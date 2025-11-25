//! # polyte
//!
//! Unified Rust client for Polymarket APIs, combining both CLOB (trading) and Gamma (market data) APIs.
//!
//! ## Features
//!
//! - Unified access to both CLOB and Gamma APIs
//! - Type-safe API with idiomatic Rust patterns
//! - EIP-712 order signing and HMAC authentication
//! - Comprehensive market data and trading operations
//!
//! ## Example
//!
//! ```no_run
//! use polyte::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let private_key = "0x1234..."; // Load this from environment variables
//!
//!     // Set up credentials
//!     let credentials = Credentials {
//!         key: "<api_key>".to_string(),
//!         secret: "<secret>".to_string(),
//!         passphrase: "<passphrase>".to_string(),
//!     };
//!
//!     // Create unified client
//!     let polymarket = Polymarket::builder(private_key, credentials)
//!         .chain(Chain::PolygonMainnet)
//!         .build()?;
//!
//!     // Use Gamma API to list markets
//!     let markets = polymarket.gamma.markets()
//!         .list()
//!         .active(true)
//!         .limit(10)
//!         .send()
//!         .await?;
//!
//!     // Use CLOB API to place an order
//!     if let Some(first_market) = markets.first() {
//!         if let Some(token) = first_market.tokens.first() {
//!             let order_params = CreateOrderParams {
//!                 token_id: token.token_id.clone(),
//!                 price: 0.52,
//!                 size: 100.0,
//!                 side: OrderSide::Buy,
//!                 expiration: None,
//!             };
//!
//!             let response = polymarket.clob.place_order(&order_params).await?;
//!             println!("Order placed: {:?}", response.order_id);
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```

#[cfg(all(feature = "clob", feature = "gamma"))]
use polyte_clob::{Chain, Clob, Credentials};
#[cfg(all(feature = "clob", feature = "gamma"))]
use polyte_gamma::Gamma;

#[cfg(feature = "clob")]
pub use polyte_clob;
#[cfg(feature = "gamma")]
pub use polyte_gamma;

/// Prelude module for convenient imports
pub mod prelude {
    #[cfg(all(feature = "clob", feature = "gamma"))]
    pub use crate::{Polymarket, PolymarketBuilder, PolymarketError};

    #[cfg(feature = "clob")]
    pub use polyte_clob::{Chain, Clob, ClobError, CreateOrderParams, Credentials, OrderSide};

    #[cfg(feature = "gamma")]
    pub use polyte_gamma::{Gamma, GammaError};
}

/// Error types for Polymarket operations
#[derive(Debug, thiserror::Error)]
pub enum PolymarketError {
    /// CLOB API error
    #[cfg(feature = "clob")]
    #[error("CLOB error: {0}")]
    Clob(#[from] polyte_clob::ClobError),

    /// Gamma API error
    #[cfg(feature = "gamma")]
    #[error("Gamma error: {0}")]
    Gamma(#[from] polyte_gamma::GammaError),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),
}

/// Unified Polymarket client
#[cfg(all(feature = "clob", feature = "gamma"))]
#[derive(Clone)]
pub struct Polymarket {
    /// CLOB (trading) API client
    pub clob: Clob,
    /// Gamma (market data) API client
    pub gamma: Gamma,
}

#[cfg(all(feature = "clob", feature = "gamma"))]
impl Polymarket {
    /// Create a new client
    pub fn new(
        private_key: impl Into<String>,
        credentials: Credentials,
    ) -> Result<Self, PolymarketError> {
        PolymarketBuilder::new(private_key, credentials).build()
    }

    /// Create a new builder
    pub fn builder(private_key: impl Into<String>, credentials: Credentials) -> PolymarketBuilder {
        PolymarketBuilder::new(private_key, credentials)
    }
}

/// Builder for Polymarket client
#[cfg(all(feature = "clob", feature = "gamma"))]
pub struct PolymarketBuilder {
    clob_base_url: Option<String>,
    gamma_base_url: Option<String>,
    timeout_ms: Option<u64>,
    chain: Option<Chain>,
    private_key: String,
    credentials: Credentials,
}

#[cfg(all(feature = "clob", feature = "gamma"))]
impl PolymarketBuilder {
    fn new(private_key: impl Into<String>, credentials: Credentials) -> Self {
        Self {
            clob_base_url: None,
            gamma_base_url: None,
            timeout_ms: None,
            chain: None,
            private_key: private_key.into(),
            credentials,
        }
    }

    /// Set CLOB base URL
    pub fn clob_base_url(mut self, url: impl Into<String>) -> Self {
        self.clob_base_url = Some(url.into());
        self
    }

    /// Set Gamma base URL
    pub fn gamma_base_url(mut self, url: impl Into<String>) -> Self {
        self.gamma_base_url = Some(url.into());
        self
    }

    /// Set request timeout in milliseconds
    pub fn timeout_ms(mut self, timeout: u64) -> Self {
        self.timeout_ms = Some(timeout);
        self
    }

    /// Set chain
    pub fn chain(mut self, chain: Chain) -> Self {
        self.chain = Some(chain);
        self
    }

    /// Build the Polymarket client
    pub fn build(self) -> Result<Polymarket, PolymarketError> {
        // Build Gamma client
        let mut gamma_builder = Gamma::builder();

        if let Some(url) = self.gamma_base_url {
            gamma_builder = gamma_builder.base_url(url);
        }
        if let Some(timeout) = self.timeout_ms {
            gamma_builder = gamma_builder.timeout_ms(timeout);
        }

        let gamma = gamma_builder.build()?;

        // Build CLOB client
        let mut clob_builder = Clob::builder(self.private_key, self.credentials)?;

        if let Some(url) = self.clob_base_url {
            clob_builder = clob_builder.base_url(url);
        }
        if let Some(timeout) = self.timeout_ms {
            clob_builder = clob_builder.timeout_ms(timeout);
        }
        if let Some(chain) = self.chain {
            clob_builder = clob_builder.chain(chain);
        }

        let clob = clob_builder.build()?;

        Ok(Polymarket { clob, gamma })
    }
}
