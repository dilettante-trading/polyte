//! Example: Test order placement
//!
//! Run with:
//! ```
//! cargo run --example test_order
//! ```
//!
//! Create a `.env` file in the polyte directory with:
//! ```
//! POLYMARKET_PRIVATE_KEY=0x...
//! POLYMARKET_API_KEY=...
//! POLYMARKET_API_SECRET=...
//! POLYMARKET_API_PASSPHRASE=...
//! ```

use polyte_clob::{Account, Clob, CreateOrderParams, OrderKind, OrderSide};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file if present
    dotenvy::dotenv().ok();
    
    tracing_subscriber::fmt::init();

    println!("Loading account from environment variables...");
    let account = Account::from_env()?;
    println!("Account address: {:?}", account.address());

    let clob = Clob::from_account(account)?;

    // Example token ID - replace with a real one from your market
    // You can get token IDs from polyte-gamma or the Polymarket website
    let token_id =
        "102559817034631022221500208641784929295731053857601013029449249654006364919935"
            .to_string();

    let params = CreateOrderParams {
        token_id,
        price: 0.5,
        size: 5.0, // Small size for testing (5 shares = $2.50 at 0.5 price)
        side: OrderSide::Buy,
        order_type: OrderKind::Gtc,
        post_only: false,
        expiration: None,
    };

    println!("\n📝 Creating order with params:");
    println!("   Token ID: {}...", &params.token_id[..30]);
    println!("   Price: {}", params.price);
    println!("   Size: {} shares", params.size);
    println!("   Side: {:?}", params.side);
    println!("   Order Type: {:?}", params.order_type);

    println!("\n🔄 Creating and signing order...");
    let order = clob.create_order(&params).await?;
    println!("   Salt: {}", order.salt);
    println!("   Maker: {:?}", order.maker);
    println!("   Maker Amount: {}", order.maker_amount);
    println!("   Taker Amount: {}", order.taker_amount);

    println!("\n✍️ Signing order...");
    let signed_order = clob.sign_order(&order).await?;
    println!(
        "   Signature: {}...{}",
        &signed_order.signature[..10],
        &signed_order.signature[signed_order.signature.len() - 6..]
    );

    println!("\n🚀 Posting order to CLOB...");
    match clob
        .post_order(&signed_order, params.order_type, params.post_only)
        .await
    {
        Ok(response) => {
            println!("\n✅ Order placed successfully!");
            println!("   Success: {}", response.success);
            if let Some(order_id) = &response.order_id {
                println!("   Order ID: {}", order_id);
            }
            if let Some(error) = &response.error_msg {
                println!("   Error: {}", error);
            }
            if !response.transaction_hashes.is_empty() {
                println!("   Tx Hashes: {:?}", response.transaction_hashes);
            }
        }
        Err(e) => {
            println!("\n❌ Order failed: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
