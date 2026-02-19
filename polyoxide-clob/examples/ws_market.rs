//! Example: Connect to the market WebSocket channel
//!
//! Run with:
//! ```
//! cargo run --example ws_market
//! ```

use futures_util::StreamExt;
use polyoxide_clob::ws::{Channel, MarketMessage, WebSocket};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Example asset IDs - these are token IDs from Polymarket markets
    // You can get these from the Gamma API or Polymarket website
    let asset_ids = vec![
        "11011989236331164054844679338128881575446679057930991995271488699513863012857".to_string(),
    ];

    println!("Connecting to Polymarket WebSocket...");
    println!("Subscribing to {} asset(s)", asset_ids.len());

    let mut ws = WebSocket::connect_market(asset_ids).await?;

    println!("Connected! Waiting for messages...\n");

    let mut count = 0;
    let limit = 100;
    while let Some(msg) = ws.next().await {
        match msg {
            Ok(Channel::Market(market_msg)) => {
                match market_msg {
                    MarketMessage::Book(book) => {
                        println!("📚 Order Book for {}", &book.asset_id[..20]);
                        println!("   Bids: {} levels", book.bids.len());
                        if let Some(best_bid) = book.bids.first() {
                            println!("   Best bid: {} @ {}", best_bid.size, best_bid.price);
                        }
                        println!("   Asks: {} levels", book.asks.len());
                        if let Some(best_ask) = book.asks.first() {
                            println!("   Best ask: {} @ {}", best_ask.size, best_ask.price);
                        }
                        println!();
                    }
                    MarketMessage::PriceChange(pc) => {
                        println!("💰 Price Change for market {}", &pc.market[..20]);
                        for change in &pc.price_changes {
                            println!(
                                "   {} {} @ {} (size: {})",
                                change.side,
                                &change.asset_id[..20],
                                change.price,
                                change.size
                            );
                        }
                        println!();
                    }
                    MarketMessage::LastTradePrice(ltp) => {
                        println!("🔄 Last Trade: {} @ {}", ltp.size, ltp.price);
                        println!("   Side: {}, Asset: {}...", ltp.side, &ltp.asset_id[..20]);
                        println!();
                    }
                    MarketMessage::TickSizeChange(tsc) => {
                        println!("📏 Tick Size Change");
                        println!(
                            "   {} -> {} for {}",
                            tsc.old_tick_size, tsc.new_tick_size, tsc.side
                        );
                        println!();
                    }
                }
                count += 1;
                if count >= limit {
                    println!("Received {limit} messages, exiting...");
                    break;
                }
            }
            Ok(Channel::User(_)) => {
                // Won't happen on market channel
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }

    ws.close().await?;
    println!("Connection closed.");

    Ok(())
}
