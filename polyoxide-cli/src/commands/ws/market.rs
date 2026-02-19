use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use clap::Args;
use color_eyre::eyre::Result;
use futures_util::StreamExt;
use polyoxide_clob::ws::{Channel, MarketMessage, WebSocket};

use crate::commands::common::parsing::parse_duration;

/// Market event types to filter
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum MarketEventType {
    /// Order book snapshots
    Book,
    /// Price changes
    Price,
    /// Last trade price
    Trade,
    /// Tick size changes
    Tick,
}

#[derive(Args)]
pub struct MarketArgs {
    /// Asset IDs (token IDs) to subscribe to
    #[arg(required = true)]
    asset_ids: Vec<String>,

    /// Output format
    #[arg(short, long, value_enum, default_value = "pretty")]
    format: OutputFormat,

    /// Filter by event type (can be specified multiple times)
    #[arg(long, value_enum)]
    filter: Vec<MarketEventType>,

    /// Exit after receiving N messages
    #[arg(short = 'n', long)]
    count: Option<u64>,

    /// Exit after specified duration (e.g., "30s", "5m", "1h")
    #[arg(short, long, value_parser = parse_duration)]
    timeout: Option<Duration>,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum, Default)]
pub enum OutputFormat {
    /// Pretty-printed JSON
    #[default]
    Pretty,
    /// Compact JSON (one message per line)
    Json,
    /// Human-readable summary
    Summary,
}

pub async fn run(args: MarketArgs) -> Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    eprintln!(
        "Connecting to market channel for {} asset(s)...",
        args.asset_ids.len()
    );
    if !args.filter.is_empty() {
        eprintln!("Filtering: {:?}", args.filter);
    }
    if let Some(count) = args.count {
        eprintln!("Will exit after {} message(s)", count);
    }
    if let Some(timeout) = args.timeout {
        eprintln!("Will exit after {:?}", timeout);
    }
    eprintln!("Press Ctrl+C to exit\n");

    let mut ws = WebSocket::connect_market(args.asset_ids).await?;
    let mut message_count: u64 = 0;
    let start_time = std::time::Instant::now();

    while running.load(Ordering::SeqCst) {
        // Check timeout
        if let Some(timeout) = args.timeout {
            if start_time.elapsed() >= timeout {
                eprintln!("\nTimeout reached");
                break;
            }
        }

        tokio::select! {
            msg = ws.next() => {
                match msg {
                    Some(Ok(channel)) => {
                        if should_print(&channel, &args.filter) {
                            print_message(&channel, args.format)?;
                            message_count += 1;

                            // Check count limit
                            if let Some(count) = args.count {
                                if message_count >= count {
                                    eprintln!("\nReached {} message(s)", count);
                                    break;
                                }
                            }
                        }
                    }
                    Some(Err(e)) => {
                        eprintln!("Error: {}", e);
                        break;
                    }
                    None => {
                        eprintln!("Connection closed");
                        break;
                    }
                }
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {
                if !running.load(Ordering::SeqCst) {
                    break;
                }
            }
        }
    }

    eprintln!("\nDisconnecting... ({} messages received)", message_count);
    ws.close().await?;

    Ok(())
}

fn should_print(channel: &Channel, filters: &[MarketEventType]) -> bool {
    if filters.is_empty() {
        return true;
    }

    match channel {
        Channel::Market(msg) => {
            let event_type = match msg {
                MarketMessage::Book(_) => MarketEventType::Book,
                MarketMessage::PriceChange(_) => MarketEventType::Price,
                MarketMessage::LastTradePrice(_) => MarketEventType::Trade,
                MarketMessage::TickSizeChange(_) => MarketEventType::Tick,
            };
            filters.contains(&event_type)
        }
        Channel::User(_) => false,
    }
}

fn print_message(channel: &Channel, format: OutputFormat) -> Result<()> {
    match channel {
        Channel::Market(msg) => match format {
            OutputFormat::Pretty => {
                println!("{}", serde_json::to_string_pretty(&msg)?);
            }
            OutputFormat::Json => {
                println!("{}", serde_json::to_string(&msg)?);
            }
            OutputFormat::Summary => {
                print_market_summary(msg);
            }
        },
        Channel::User(_) => {
            // Shouldn't happen on market channel
        }
    }
    Ok(())
}

fn truncate(s: &str, max_len: usize) -> &str {
    &s[..s.len().min(max_len)]
}

fn print_market_summary(msg: &MarketMessage) {
    match msg {
        MarketMessage::Book(book) => {
            println!(
                "[BOOK] asset={}.. bids={} asks={}",
                truncate(&book.asset_id, 10),
                book.bids.len(),
                book.asks.len(),
            );
        }
        MarketMessage::PriceChange(pc) => {
            for change in &pc.price_changes {
                println!(
                    "[PRICE] asset={}.. price={} side={}",
                    truncate(&change.asset_id, 10),
                    change.price,
                    change.side
                );
            }
        }
        MarketMessage::TickSizeChange(tc) => {
            println!(
                "[TICK] asset={}.. old={} new={} side={}",
                truncate(&tc.asset_id, 10),
                tc.old_tick_size,
                tc.new_tick_size,
                tc.side
            );
        }
        MarketMessage::LastTradePrice(ltp) => {
            println!(
                "[TRADE] asset={}.. price={} side={} size={}",
                truncate(&ltp.asset_id, 10),
                ltp.price,
                ltp.side,
                ltp.size
            );
        }
    }
}
