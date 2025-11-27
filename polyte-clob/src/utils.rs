use std::time::{SystemTime, UNIX_EPOCH};

use rand::Rng;

use crate::types::{OrderSide, TickSize};

/// Get current Unix timestamp in seconds
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Calculate maker and taker amounts for an order
pub fn calculate_order_amounts(
    price: f64,
    size: f64,
    side: OrderSide,
    tick_size: TickSize,
) -> (String, String) {
    const SIZE_DECIMALS: u32 = 2; // shares are in 2 decimals

    let tick_decimals = tick_size.decimals();

    // Round price to tick size
    let price_rounded = round_to_decimals(price, tick_decimals);

    // Round size to 2 decimals
    let size_rounded = round_to_decimals(size, SIZE_DECIMALS);

    // Calculate cost
    let cost = price_rounded * size_rounded;
    let cost_rounded = round_to_decimals(cost, tick_decimals);

    // Convert to raw amounts (no decimals)
    let share_amount = to_raw_amount(size_rounded, SIZE_DECIMALS);
    let cost_amount = to_raw_amount(cost_rounded, SIZE_DECIMALS);

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

/// Round a float to specified decimal places
fn round_to_decimals(value: f64, decimals: u32) -> f64 {
    let multiplier = 10_f64.powi(decimals as i32);
    (value * multiplier).round() / multiplier
}

/// Convert float to raw integer amount
fn to_raw_amount(value: f64, decimals: u32) -> String {
    let multiplier = 10_f64.powi(decimals as i32);
    let raw = (value * multiplier).floor() as u128;
    raw.to_string()
}

/// Generate random salt for orders
pub fn generate_salt() -> String {
    rand::rng().random::<u128>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_order_amounts_buy() {
        let (maker, taker) =
            calculate_order_amounts(0.52, 100.0, OrderSide::Buy, TickSize::Hundredth);

        // BUY: maker = cost (5200), taker = shares (10000)
        assert_eq!(maker, "5200");
        assert_eq!(taker, "10000");
    }

    #[test]
    fn test_calculate_order_amounts_sell() {
        let (maker, taker) =
            calculate_order_amounts(0.52, 100.0, OrderSide::Sell, TickSize::Hundredth);

        // SELL: maker = shares (10000), taker = cost (5200)
        assert_eq!(maker, "10000");
        assert_eq!(taker, "5200");
    }

    #[test]
    fn test_calculate_order_amounts_tenth_tick_size() {
        let (maker, taker) = calculate_order_amounts(0.5, 50.0, OrderSide::Buy, TickSize::Tenth);

        // price=0.5, size=50 => cost=25.0
        // BUY: maker = cost (2500), taker = shares (5000)
        assert_eq!(maker, "2500");
        assert_eq!(taker, "5000");
    }

    #[test]
    fn test_calculate_order_amounts_thousandth_tick_size() {
        let (maker, taker) =
            calculate_order_amounts(0.523, 100.0, OrderSide::Buy, TickSize::Thousandth);

        // price=0.523, size=100 => cost=52.3
        // BUY: maker = cost (5230), taker = shares (10000)
        assert_eq!(maker, "5230");
        assert_eq!(taker, "10000");
    }

    #[test]
    fn test_calculate_order_amounts_ten_thousandth_tick_size() {
        let (maker, taker) =
            calculate_order_amounts(0.5234, 100.0, OrderSide::Buy, TickSize::TenThousandth);

        // price=0.5234, size=100 => cost=52.34
        // BUY: maker = cost (5234), taker = shares (10000)
        assert_eq!(maker, "5234");
        assert_eq!(taker, "10000");
    }

    #[test]
    fn test_calculate_order_amounts_price_rounding() {
        // Price 0.526 should round to 0.53 with Hundredth tick size
        let (maker, taker) =
            calculate_order_amounts(0.526, 100.0, OrderSide::Buy, TickSize::Hundredth);

        // price rounds to 0.53, size=100 => cost=53.0
        assert_eq!(maker, "5300");
        assert_eq!(taker, "10000");
    }

    #[test]
    fn test_calculate_order_amounts_size_rounding() {
        // Size 100.567 should round to 100.57
        let (maker, taker) =
            calculate_order_amounts(0.50, 100.567, OrderSide::Buy, TickSize::Hundredth);

        // price=0.50, size rounds to 100.57 => cost=50.285 rounds to 50.29
        assert_eq!(maker, "5029");
        assert_eq!(taker, "10057");
    }

    #[test]
    fn test_calculate_order_amounts_minimum_price() {
        let (maker, taker) =
            calculate_order_amounts(0.01, 100.0, OrderSide::Buy, TickSize::Hundredth);

        // price=0.01, size=100 => cost=1.0
        assert_eq!(maker, "100");
        assert_eq!(taker, "10000");
    }

    #[test]
    fn test_calculate_order_amounts_maximum_price() {
        let (maker, taker) =
            calculate_order_amounts(0.99, 100.0, OrderSide::Buy, TickSize::Hundredth);

        // price=0.99, size=100 => cost=99.0
        assert_eq!(maker, "9900");
        assert_eq!(taker, "10000");
    }

    #[test]
    fn test_calculate_order_amounts_small_size() {
        let (maker, taker) =
            calculate_order_amounts(0.50, 0.01, OrderSide::Buy, TickSize::Hundredth);

        // price=0.50, size=0.01 => cost=0.005 rounds to 0.01
        assert_eq!(maker, "1");
        assert_eq!(taker, "1");
    }

    #[test]
    fn test_calculate_order_amounts_large_size() {
        let (maker, taker) =
            calculate_order_amounts(0.50, 10000.0, OrderSide::Buy, TickSize::Hundredth);

        // price=0.50, size=10000 => cost=5000.0
        assert_eq!(maker, "500000");
        assert_eq!(taker, "1000000");
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
    fn test_generate_salt_is_numeric() {
        let salt = generate_salt();

        // Should parse as u128
        assert!(
            salt.parse::<u128>().is_ok(),
            "Salt should be a valid u128 string"
        );
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
    fn test_generate_salt_not_empty() {
        let salt = generate_salt();
        assert!(!salt.is_empty(), "Salt should not be empty");
    }

    #[test]
    fn test_round_to_decimals() {
        // Test through calculate_order_amounts behavior
        // 0.555 with Hundredth should round to 0.56
        let (maker, _) = calculate_order_amounts(0.555, 100.0, OrderSide::Buy, TickSize::Hundredth);
        assert_eq!(maker, "5600"); // 0.56 * 100 = 56.0 => 5600

        // 0.554 with Hundredth should round to 0.55
        let (maker, _) = calculate_order_amounts(0.554, 100.0, OrderSide::Buy, TickSize::Hundredth);
        assert_eq!(maker, "5500"); // 0.55 * 100 = 55.0 => 5500
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
}
