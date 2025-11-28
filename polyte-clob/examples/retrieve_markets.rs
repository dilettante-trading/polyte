use polyte_clob::{Chain, Clob, Credentials, OrderSide};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber to see debug logs
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("debug")),
        )
        .init();

    // Create a CLOB client
    // Note: Even though market data is public, authentication is required to initialize the client
    let private_key = std::env::var("PRIVATE_KEY").unwrap_or_else(|_| {
        "0x0000000000000000000000000000000000000000000000000000000000000001".to_string()
    });
    let credentials = Credentials {
        key: std::env::var("API_KEY").unwrap_or_else(|_| "dummy".to_string()),
        secret: std::env::var("API_SECRET").unwrap_or_else(|_| "dummy".to_string()),
        passphrase: std::env::var("API_PASSPHRASE").unwrap_or_else(|_| "dummy".to_string()),
    };

    let clob = Clob::builder(private_key, credentials)?
        .chain(Chain::PolygonMainnet)
        .build()?;

    println!("=== Polymarket Markets Example ===\n");

    // Example 1: List all markets
    println!("1. Listing markets...");
    let markets_response = clob.markets().list().send().await?;
    println!("   Found {} markets", markets_response.data.len());
    if let Some(next_cursor) = &markets_response.next_cursor {
        println!("   Next cursor: {}", next_cursor);
    }
    if let Some(first_market) = markets_response.data.first() {
        println!("   First market: {}\n", first_market.question);
        println!("   token: {:?}\n", first_market.tokens[0]);
    }

    // Example 2: Get a specific market by condition ID
    if let Some(market_preview) = markets_response.data.first() {
        println!("2. Getting a specific market by condition ID...");
        let market = clob
            .markets()
            .get(&market_preview.condition_id)
            .send()
            .await?;
        println!("   Market: {}", market.question);
        println!("   Active: {}", market.active);
        println!("   Min Tick Size: {}\n", market.minimum_tick_size);
    }

    // Example 3: Get order book for a token
    println!("3. Getting order book...");
    // Find a market with order book enabled
    if let Some(market) = markets_response
        .data
        .iter()
        .find(|m| m.enable_order_book == Some(true) && m.active)
    {
        if let Some(token) = market.tokens.first() {
            if let Some(token_id) = &token.token_id {
                match clob.markets().order_book(token_id).send().await {
                    Ok(order_book) => {
                        println!("   Market: {}", market.question);
                        println!("   Token: {}", token.outcome);
                        println!("   Buy orders: {}", order_book.bids.len());
                        println!("   Sell orders: {}\n", order_book.asks.len());
                    }
                    Err(e) => {
                        println!("   Skipping (no order book available): {}\n", e);
                    }
                }
            }
        }
    } else {
        println!("   No markets with order book enabled found\n");
    }

    // Example 4: Get price for a token
    println!("4. Getting price information...");
    if let Some(market) = markets_response.data.iter().find(|m| m.active) {
        if let Some(token) = market.tokens.first() {
            if let Some(token_id) = &token.token_id {
                match clob.markets().price(token_id, OrderSide::Buy).send().await {
                    Ok(price) => {
                        println!("   Market: {}", market.question);
                        println!("   Token: {}", token.outcome);
                        println!("   Buy price: {}\n", price.price);
                    }
                    Err(e) => {
                        println!("   Skipping (no price available): {}\n", e);
                    }
                }
            }
        }
    }

    // Example 5: Get midpoint price
    println!("5. Getting midpoint price...");
    if let Some(market) = markets_response.data.iter().find(|m| m.active) {
        if let Some(token) = market.tokens.first() {
            if let Some(token_id) = &token.token_id {
                match clob.markets().midpoint(token_id).send().await {
                    Ok(midpoint) => {
                        println!("   Market: {}", market.question);
                        println!("   Token: {}", token.outcome);
                        println!("   Midpoint: {}", midpoint.mid);
                        println!();
                    }
                    Err(e) => {
                        println!("   Skipping (no midpoint available): {}\n", e);
                    }
                }
            }
        }
    }

    println!("=== Example Complete ===");
    Ok(())
}
