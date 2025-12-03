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
use polyte_clob::ws::{ApiCredentials, Channel, UserMessage, WebSocket};

/// User event types to filter
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum UserEventType {
    /// Order updates
    Order,
    /// Trade updates
    Trade,
}

#[derive(Args)]
pub struct UserArgs {
    /// Market IDs (condition IDs) to subscribe to
    #[arg(required = true)]
    market_ids: Vec<String>,

    /// API key (defaults to POLYMARKET_API_KEY env var)
    #[arg(long, env = "POLYMARKET_API_KEY")]
    api_key: Option<String>,

    /// API secret (defaults to POLYMARKET_API_SECRET env var)
    #[arg(long, env = "POLYMARKET_API_SECRET")]
    api_secret: Option<String>,

    /// API passphrase (defaults to POLYMARKET_API_PASSPHRASE env var)
    #[arg(long, env = "POLYMARKET_API_PASSPHRASE")]
    api_passphrase: Option<String>,

    /// Output format
    #[arg(short, long, value_enum, default_value = "pretty")]
    format: OutputFormat,

    /// Filter by event type (can be specified multiple times)
    #[arg(long, value_enum)]
    filter: Vec<UserEventType>,

    /// Exit after receiving N messages
    #[arg(short = 'n', long)]
    count: Option<u64>,

    /// Exit after specified duration (e.g., "30s", "5m", "1h")
    #[arg(short, long, value_parser = parse_duration)]
    timeout: Option<Duration>,
}

fn parse_duration(s: &str) -> Result<Duration, String> {
    let s = s.trim();
    if s.is_empty() {
        return Err("empty duration".to_string());
    }

    let (num, unit) = if let Some(n) = s.strip_suffix("ms") {
        (n, "ms")
    } else if let Some(n) = s.strip_suffix('s') {
        (n, "s")
    } else if let Some(n) = s.strip_suffix('m') {
        (n, "m")
    } else if let Some(n) = s.strip_suffix('h') {
        (n, "h")
    } else {
        // Default to seconds if no unit
        (s, "s")
    };

    let num: u64 = num
        .parse()
        .map_err(|_| format!("invalid number: {}", num))?;

    match unit {
        "ms" => Ok(Duration::from_millis(num)),
        "s" => Ok(Duration::from_secs(num)),
        "m" => Ok(Duration::from_secs(num * 60)),
        "h" => Ok(Duration::from_secs(num * 3600)),
        _ => Err(format!("unknown unit: {}", unit)),
    }
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

pub async fn run(args: UserArgs) -> Result<()> {
    let credentials = match (args.api_key, args.api_secret, args.api_passphrase) {
        (Some(key), Some(secret), Some(passphrase)) => {
            ApiCredentials::new(key, secret, passphrase)
        }
        _ => ApiCredentials::from_env().map_err(|e| {
            color_eyre::eyre::eyre!(
                "Missing API credentials. Set POLYMARKET_API_KEY, POLYMARKET_API_SECRET, and POLYMARKET_API_PASSPHRASE environment variables, or provide --api-key, --api-secret, and --api-passphrase flags. Error: {}",
                e
            )
        })?,
    };

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    eprintln!(
        "Connecting to user channel for {} market(s)...",
        args.market_ids.len()
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

    let mut ws = WebSocket::connect_user(args.market_ids, credentials).await?;
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

fn should_print(channel: &Channel, filters: &[UserEventType]) -> bool {
    if filters.is_empty() {
        return true;
    }

    match channel {
        Channel::User(msg) => {
            let event_type = match msg {
                UserMessage::Order(_) => UserEventType::Order,
                UserMessage::Trade(_) => UserEventType::Trade,
            };
            filters.contains(&event_type)
        }
        Channel::Market(_) => false,
    }
}

fn print_message(channel: &Channel, format: OutputFormat) -> Result<()> {
    match channel {
        Channel::User(msg) => match format {
            OutputFormat::Pretty => {
                println!("{}", serde_json::to_string_pretty(&msg)?);
            }
            OutputFormat::Json => {
                println!("{}", serde_json::to_string(&msg)?);
            }
            OutputFormat::Summary => {
                print_user_summary(msg);
            }
        },
        Channel::Market(_) => {
            // Shouldn't happen on user channel
        }
    }
    Ok(())
}

fn print_user_summary(msg: &UserMessage) {
    match msg {
        UserMessage::Order(order) => {
            println!(
                "[ORDER] id={} type={:?} side={} price={} size={} matched={}",
                &order.id[..8.min(order.id.len())],
                order.order_type,
                order.side,
                order.price,
                order.original_size,
                order.size_matched
            );
        }
        UserMessage::Trade(trade) => {
            println!(
                "[TRADE] id={} side={} price={} size={} status={:?}",
                &trade.id[..8.min(trade.id.len())],
                trade.side,
                trade.price,
                trade.size,
                trade.status
            );
        }
    }
}
