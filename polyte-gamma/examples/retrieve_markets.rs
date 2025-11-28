use polyte_gamma::Gamma;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber to see logs
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let gamma = Gamma::builder().build()?;

    println!("=== Polymarket Gamma Markets Example ===\n");

    // Example 1: List all markets
    println!("1. Listing all markets...");
    let markets = gamma.markets().list().send().await?;
    println!("   Found {} markets", markets.len());
    println!();

    // Example 2: Get a specific market by ID (if we have one)
    if let Some(first_market) = markets.first() {
        let condition_id = &first_market.condition_id;
        println!("2. Getting market details for: {}", first_market.question);
        println!("   Market ID: {}", condition_id);

        match gamma.markets().get(condition_id).send().await {
            Ok(market) => {
                println!("   Question: {}", market.question);
                println!("   Description: {}", market.description);
                println!("   Active: {:?}", market.active);
                println!("   Closed: {:?}", market.closed);
                println!("   Volume: {:?}", market.volume);
                println!("   Liquidity: {:?}", market.liquidity);
                println!("   Tokens:");
                for token in &market.tokens {
                    println!("     - {}: {}", token.outcome, token.token_id);
                    if let Some(price) = &token.price {
                        println!("       Price: {}", price);
                    }
                }
            }
            Err(e) => {
                eprintln!("   Error: {}", e);
            }
        }
        println!();
    }

    // Example 3: List active markets only
    println!("3. Listing active markets...");
    let active_markets = gamma.markets().list().active(true).limit(10).send().await?;
    println!(
        "   Found {} active markets (limited to 10)",
        active_markets.len()
    );
    for market in active_markets.iter().take(5) {
        println!("   - {}", market.question);
    }
    println!();

    // Example 4: List closed markets
    println!("4. Listing closed markets...");
    let closed_markets = gamma.markets().list().closed(true).limit(5).send().await?;
    println!(
        "   Found {} closed markets (limited to 5)",
        closed_markets.len()
    );
    for market in &closed_markets {
        println!("   - {}", market.question);
    }
    println!();

    // Example 5: Show market statistics
    println!("5. Market statistics from first page:");
    let total_volume: f64 = markets
        .iter()
        .filter_map(|m| m.volume.as_ref().and_then(|v| v.parse::<f64>().ok()))
        .sum();
    let total_liquidity: f64 = markets
        .iter()
        .filter_map(|m| m.liquidity.as_ref().and_then(|l| l.parse::<f64>().ok()))
        .sum();
    let active_count = markets.iter().filter(|m| m.active == Some(true)).count();
    let closed_count = markets.iter().filter(|m| m.closed == Some(true)).count();

    println!("   Total volume: ${:.2}", total_volume);
    println!("   Total liquidity: ${:.2}", total_liquidity);
    println!("   Active markets: {}", active_count);
    println!("   Closed markets: {}", closed_count);

    println!("\n=== Example Complete ===");
    Ok(())
}
