use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use clap::{Args, ValueEnum};
use color_eyre::eyre::Result;
use futures_util::StreamExt;
use polyoxide_clob::ws::{ApiCredentials, Channel, UserMessage, WebSocket};

use crate::commands::common::parsing::parse_duration;

/// User event types to filter
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
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

    /// API key
    #[arg(long, env = "POLYMARKET_API_KEY")]
    api_key: Option<String>,

    /// API secret
    #[arg(long, env = "POLYMARKET_API_SECRET")]
    api_secret: Option<String>,

    /// API passphrase
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

#[derive(Debug, Clone, Copy, ValueEnum, Default)]
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
    let credentials = get_credentials(args.api_key, args.api_secret, args.api_passphrase);

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

fn get_credentials(
    api_key: Option<String>,
    api_secret: Option<String>,
    api_passphrase: Option<String>,
) -> ApiCredentials {
    match (api_key, api_secret, api_passphrase) {
        (Some(key), Some(secret), Some(passphrase)) => ApiCredentials::new(key, secret, passphrase),
        (key, secret, passphrase) => {
            let mut missing = Vec::new();
            if key.is_none() {
                missing.push("--api-key / POLYMARKET_API_KEY");
            }
            if secret.is_none() {
                missing.push("--api-secret / POLYMARKET_API_SECRET");
            }
            if passphrase.is_none() {
                missing.push("--api-passphrase / POLYMARKET_API_PASSPHRASE");
            }
            eprintln!("Error: Missing required credentials:\n");
            for m in &missing {
                eprintln!("  - {}", m);
            }
            std::process::exit(1);
        }
    }
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
