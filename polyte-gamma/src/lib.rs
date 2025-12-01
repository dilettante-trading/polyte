//! # polyte-gamma
//!
//! Rust client library for Polymarket Gamma (market data) API.
//!
//! ## Features
//!
//! - Market data retrieval with filtering and pagination
//! - Event and series (tournament/season) information
//! - Tags and sports metadata
//! - Comments on markets, events, and series
//! - Type-safe API with idiomatic Rust patterns
//! - Request builder pattern for flexible, composable queries
//!
//! ## Example
//!
//! ```no_run
//! use polyte_gamma::Gamma;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a new Gamma client
//!     let gamma = Gamma::new()?;
//!
//!     // List active markets with fluent builder pattern
//!     let markets = gamma.markets()
//!         .list()
//!         .active(true)
//!         .limit(10)
//!         .send()
//!         .await?;
//!
//!     for market in markets {
//!         println!("Market: {}", market.question);
//!     }
//!
//!     // Get a specific market
//!     let market = gamma.markets()
//!         .get("condition-id")
//!         .send()
//!         .await?;
//!
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod client;
pub mod error;
pub mod types;

pub use client::{Gamma, GammaBuilder};
pub use error::{GammaError, Result};
