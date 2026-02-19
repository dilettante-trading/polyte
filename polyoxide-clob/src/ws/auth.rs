//! API credentials for WebSocket authentication.

use std::fmt;

use serde::{Deserialize, Serialize};

/// API credentials for WebSocket user channel authentication.
///
/// These credentials can be obtained from your Polymarket account settings
/// or derived using the CLOB API.
///
/// # Example
///
/// ```
/// use polyoxide_clob::ws::ApiCredentials;
///
/// let creds = ApiCredentials::new("api_key", "api_secret", "passphrase");
/// ```
#[derive(Clone, Serialize, Deserialize)]
pub struct ApiCredentials {
    /// API key
    #[serde(rename = "apiKey")]
    pub api_key: String,
    /// API secret
    pub secret: String,
    /// API passphrase
    pub passphrase: String,
}

impl ApiCredentials {
    /// Create new API credentials.
    pub fn new(
        api_key: impl Into<String>,
        secret: impl Into<String>,
        passphrase: impl Into<String>,
    ) -> Self {
        Self {
            api_key: api_key.into(),
            secret: secret.into(),
            passphrase: passphrase.into(),
        }
    }

    /// Load credentials from environment variables.
    ///
    /// Reads:
    /// - `POLYMARKET_API_KEY`
    /// - `POLYMARKET_API_SECRET`
    /// - `POLYMARKET_API_PASSPHRASE`
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            api_key: std::env::var("POLYMARKET_API_KEY")?,
            secret: std::env::var("POLYMARKET_API_SECRET")?,
            passphrase: std::env::var("POLYMARKET_API_PASSPHRASE")?,
        })
    }
}

impl fmt::Debug for ApiCredentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApiCredentials")
            .field("api_key", &"<redacted>")
            .field("secret", &"<redacted>")
            .field("passphrase", &"<redacted>")
            .finish()
    }
}
