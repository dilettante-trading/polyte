use std::time::{SystemTime, UNIX_EPOCH};

use rand::Rng;
use rust_decimal::{prelude::ToPrimitive, Decimal, RoundingStrategy};

use crate::types::{OrderSide, TickSize};

/// Get current Unix timestamp in seconds
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Calculate maker and taker amounts for an order using precise decimal arithmetic.
///
/// This function uses `rust_decimal` to avoid floating-point precision issues
/// that can occur with f64 arithmetic in financial calculations.
///
/// # Arguments
///
/// * `price` - Order price (0.0 to 1.0)
/// * `size` - Order size in shares
/// * `side` - Buy or Sell
/// * `tick_size` - Minimum price increment for rounding
///
/// # Returns
///
/// A tuple of (maker_amount, taker_amount) as strings suitable for the CLOB API.
/// - For BUY orders: maker = cost (USDC), taker = shares
/// - For SELL orders: maker = shares, taker = cost (USDC)
pub fn calculate_order_amounts(
    price: f64,
    size: f64,
    side: OrderSide,
    tick_size: TickSize,
) -> (String, String) {
    const SIZE_DECIMALS: u32 = 6; // shares are in 6 decimals

    let tick_decimals = tick_size.decimals();

    // Convert to Decimal for precise arithmetic
    // Using from_f64_retain to preserve the exact f64 representation
    let price_decimal = Decimal::try_from(price).unwrap_or_else(|_| {
        // Fallback: parse from string representation for edge cases
        Decimal::from_str_exact(&price.to_string()).unwrap_or(Decimal::ZERO)
    });
    let size_decimal = Decimal::try_from(size)
        .unwrap_or_else(|_| Decimal::from_str_exact(&size.to_string()).unwrap_or(Decimal::ZERO));

    // Round price to tick size using banker's rounding (round half to even)
    let price_rounded =
        price_decimal.round_dp_with_strategy(tick_decimals, RoundingStrategy::MidpointNearestEven);

    // Round size to 2 decimals
    let size_rounded =
        size_decimal.round_dp_with_strategy(SIZE_DECIMALS, RoundingStrategy::MidpointNearestEven);

    // Calculate cost with precise decimal multiplication
    let cost = price_rounded * size_rounded;
    let cost_rounded =
        cost.round_dp_with_strategy(SIZE_DECIMALS, RoundingStrategy::MidpointNearestEven);

    // Convert to raw amounts (multiply by 10^decimals and take integer part)
    let share_amount = decimal_to_raw_amount(size_rounded, SIZE_DECIMALS);
    let cost_amount = decimal_to_raw_amount(cost_rounded, SIZE_DECIMALS);

    match side {
        OrderSide::Buy => {
            // BUY: maker pays USDC, receives shares
            (cost_amount, share_amount)
        }
        OrderSide::Sell => {
            // SELL: maker pays shares, receives USDC
            (share_amount, cost_amount)
        }
    }
}

/// Convert a Decimal to a raw integer amount string.
///
/// Multiplies by 10^decimals and takes the floor to get the integer representation.
fn decimal_to_raw_amount(value: Decimal, decimals: u32) -> String {
    let multiplier = Decimal::from(10u64.pow(decimals));
    let raw = (value * multiplier).floor();
    // Convert to u128 for the string representation
    raw.to_u128()
        .map(|n| n.to_string())
        .unwrap_or_else(|| raw.to_string().split('.').next().unwrap_or("0").to_string())
}

/// Generate random salt for orders
pub fn generate_salt() -> String {
    rand::rng().random::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_order_amounts_buy() {
        let (maker, taker) =
            calculate_order_amounts(0.52, 100.0, OrderSide::Buy, TickSize::Hundredth);

        // BUY: maker = cost (52000000), taker = shares (100000000)
        assert_eq!(maker, "52000000");
        assert_eq!(taker, "100000000");
    }

    #[test]
    fn test_calculate_order_amounts_sell() {
        let (maker, taker) =
            calculate_order_amounts(0.52, 100.0, OrderSide::Sell, TickSize::Hundredth);

        // SELL: maker = shares (100000000), taker = cost (52000000)
        assert_eq!(maker, "100000000");
        assert_eq!(taker, "52000000");
    }

    #[test]
    fn test_calculate_order_amounts_tenth_tick_size() {
        let (maker, taker) = calculate_order_amounts(0.5, 50.0, OrderSide::Buy, TickSize::Tenth);

        // price=0.5, size=50 => cost=25.0
        // BUY: maker = cost (25000000), taker = shares (50000000)
        assert_eq!(maker, "25000000");
        assert_eq!(taker, "50000000");
    }

    #[test]
    fn test_calculate_order_amounts_thousandth_tick_size() {
        let (maker, taker) =
            calculate_order_amounts(0.523, 100.0, OrderSide::Buy, TickSize::Thousandth);

        // price=0.523, size=100 => cost=52.3
        // BUY: maker = cost (52300000), taker = shares (100000000)
        assert_eq!(maker, "52300000");
        assert_eq!(taker, "100000000");
    }

    #[test]
    fn test_calculate_order_amounts_ten_thousandth_tick_size() {
        let (maker, taker) =
            calculate_order_amounts(0.5234, 100.0, OrderSide::Buy, TickSize::TenThousandth);

        // price=0.5234, size=100 => cost=52.34
        // BUY: maker = cost (52340000), taker = shares (100000000)
        assert_eq!(maker, "52340000");
        assert_eq!(taker, "100000000");
    }

    #[test]
    fn test_calculate_order_amounts_price_rounding() {
        // Price 0.526 should round to 0.53 with Hundredth tick size
        let (maker, taker) =
            calculate_order_amounts(0.526, 100.0, OrderSide::Buy, TickSize::Hundredth);

        // price rounds to 0.53, size=100 => cost=53.0
        assert_eq!(maker, "53000000");
        assert_eq!(taker, "100000000");
    }

    #[test]
    fn test_calculate_order_amounts_size_rounding() {
        // Size 100.567 should round to 100.57 (now rounds to 100.567000 as decimal places increased)
        // With SIZE_DECIMALS = 6, size 100.567 is preserved.
        let (maker, taker) =
            calculate_order_amounts(0.50, 100.567, OrderSide::Buy, TickSize::Hundredth);

        // price=0.50, size=100.567 => cost=50.2835
        // Rounding cost to 6 decimals: 50.283500
        // Maker = 50.2835 * 10^6 = 50283500
        // Taker = 100.567 * 10^6 = 100567000

        // Wait, check rounding strategy.
        // size_rounded = 100.567.
        // cost = 0.50 * 100.567 = 50.2835.
        // cost_rounded = 50.283500.

        assert_eq!(maker, "50283500");
        assert_eq!(taker, "100567000");
    }

    #[test]
    fn test_calculate_order_amounts_minimum_price() {
        let (maker, taker) =
            calculate_order_amounts(0.01, 100.0, OrderSide::Buy, TickSize::Hundredth);

        // price=0.01, size=100 => cost=1.0
        assert_eq!(maker, "1000000");
        assert_eq!(taker, "100000000");
    }

    #[test]
    fn test_calculate_order_amounts_maximum_price() {
        let (maker, taker) =
            calculate_order_amounts(0.99, 100.0, OrderSide::Buy, TickSize::Hundredth);

        // price=0.99, size=100 => cost=99.0
        assert_eq!(maker, "99000000");
        assert_eq!(taker, "100000000");
    }

    #[test]
    fn test_calculate_order_amounts_small_size() {
        let (maker, taker) =
            calculate_order_amounts(0.50, 0.01, OrderSide::Buy, TickSize::Hundredth);

        // price=0.50, size=0.01 => cost=0.005
        // cost rounded to 6 decimals: 0.005000
        // Maker = 5000
        // Taker = 10000
        assert_eq!(maker, "5000");
        assert_eq!(taker, "10000");
    }

    #[test]
    fn test_calculate_order_amounts_large_size() {
        let (maker, taker) =
            calculate_order_amounts(0.50, 10000.0, OrderSide::Buy, TickSize::Hundredth);

        // price=0.50, size=10000 => cost=5000.0
        assert_eq!(maker, "5000000000");
        assert_eq!(taker, "10000000000");
    }

    #[test]
    fn test_current_timestamp_is_reasonable() {
        let timestamp = current_timestamp();

        // Should be after 2024-01-01 (1704067200)
        assert!(
            timestamp > 1704067200,
            "Timestamp should be after 2024-01-01"
        );

        // Should be before 2100-01-01 (4102444800)
        assert!(
            timestamp < 4102444800,
            "Timestamp should be before 2100-01-01"
        );
    }

    #[test]
    fn test_current_timestamp_increases() {
        let t1 = current_timestamp();
        let t2 = current_timestamp();

        // Second call should be >= first (same second or later)
        assert!(t2 >= t1);
    }

    #[test]
    fn test_generate_salt_is_positive() {
        let salt = generate_salt();
        assert!(salt > 0);
    }

    #[test]
    fn test_generate_salt_uniqueness() {
        let salt1 = generate_salt();
        let salt2 = generate_salt();
        let salt3 = generate_salt();

        // All salts should be different (statistically guaranteed)
        assert_ne!(salt1, salt2, "Salts should be unique");
        assert_ne!(salt2, salt3, "Salts should be unique");
        assert_ne!(salt1, salt3, "Salts should be unique");
    }



    #[test]
    fn test_rounding_behavior() {
        // Test banker's rounding (round half to even)
        // 0.555 rounds to 0.56 (6 is even)
        let (maker, _) = calculate_order_amounts(0.555, 100.0, OrderSide::Buy, TickSize::Hundredth);
        assert_eq!(maker, "56000000"); // 0.56 * 100 = 56.0 => 56000000

        // 0.554 rounds down to 0.55
        let (maker, _) = calculate_order_amounts(0.554, 100.0, OrderSide::Buy, TickSize::Hundredth);
        assert_eq!(maker, "55000000"); // 0.55 * 100 = 55.0 => 55000000

        // 0.545 rounds to 0.54 (4 is even) - banker's rounding
        let (maker, _) = calculate_order_amounts(0.545, 100.0, OrderSide::Buy, TickSize::Hundredth);
        assert_eq!(maker, "54000000"); // 0.54 * 100 = 54.0 => 54000000

        // 0.556 rounds up to 0.56
        let (maker, _) = calculate_order_amounts(0.556, 100.0, OrderSide::Buy, TickSize::Hundredth);
        assert_eq!(maker, "56000000"); // 0.56 * 100 = 56.0 => 56000000
    }

    #[test]
    fn test_symmetry_buy_sell() {
        // For same price/size, buy and sell should have swapped maker/taker
        let (buy_maker, buy_taker) =
            calculate_order_amounts(0.60, 50.0, OrderSide::Buy, TickSize::Hundredth);
        let (sell_maker, sell_taker) =
            calculate_order_amounts(0.60, 50.0, OrderSide::Sell, TickSize::Hundredth);

        assert_eq!(buy_maker, sell_taker, "Buy maker should equal sell taker");
        assert_eq!(buy_taker, sell_maker, "Buy taker should equal sell maker");
    }

    #[test]
    fn test_decimal_precision() {
        // This test verifies that decimal arithmetic is precise.
        // The classic f64 precision issue: 0.1 + 0.2 != 0.3 in IEEE 754
        // Our implementation uses rust_decimal to avoid such issues.

        // Test a value that causes f64 precision issues
        // 0.33 * 100.0 = 33.0 exactly, but intermediate f64 ops can introduce error
        let (maker, taker) =
            calculate_order_amounts(0.33, 100.0, OrderSide::Buy, TickSize::Hundredth);
        assert_eq!(maker, "33000000");
        assert_eq!(taker, "100000000");

        // Another precision-sensitive case: 0.07 * 1000.0
        let (maker, taker) =
            calculate_order_amounts(0.07, 1000.0, OrderSide::Buy, TickSize::Hundredth);
        assert_eq!(maker, "70000000"); // Should be exactly 70000000
        assert_eq!(taker, "1000000000");

        // Test with small values that stress precision
        let (maker, taker) =
            calculate_order_amounts(0.01, 0.01, OrderSide::Buy, TickSize::Hundredth);
        assert_eq!(maker, "100"); // 0.01 * 0.01 = 0.0001 => 0.0001 * 10^6 = 100
        assert_eq!(taker, "10000"); // 0.01 * 10^6 = 10000
    }
}
