//! Account module for credential management and signing operations.
//!
//! This module provides a unified abstraction for managing Polymarket CLOB authentication,
//! including wallet management, API credentials, and signing operations.

mod credentials;
mod signer;
mod wallet;

use std::path::Path;

use alloy::primitives::Address;
pub use credentials::Credentials;
use serde::{Deserialize, Serialize};
pub use signer::Signer;
pub use wallet::Wallet;

use crate::{
    core::eip712::{sign_clob_auth, sign_order},
    error::ClobError,
    types::{Order, SignedOrder},
};

/// Environment variable names for account configuration
pub mod env {
    pub const PRIVATE_KEY: &str = "POLYMARKET_PRIVATE_KEY";
    pub const API_KEY: &str = "POLYMARKET_API_KEY";
    pub const API_SECRET: &str = "POLYMARKET_API_SECRET";
    pub const API_PASSPHRASE: &str = "POLYMARKET_API_PASSPHRASE";
}

/// Account configuration for file-based loading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfig {
    pub private_key: String,
    #[serde(flatten)]
    pub credentials: Credentials,
}

/// Unified account primitive for credential management and signing operations.
///
/// `Account` combines wallet (private key), API credentials, and signing capabilities
/// into a single abstraction. It provides factory methods for loading credentials from
/// various sources (environment variables, files) and handles both EIP-712 order signing
/// and HMAC-based L2 API authentication.
///
/// # Example
///
/// ```no_run
/// use polyoxide_clob::Account;
///
/// // Load from environment variables
/// let account = Account::from_env()?;
///
/// // Or load from a JSON file
/// let account = Account::from_file("config/account.json")?;
///
/// // Get the wallet address
/// println!("Address: {:?}", account.address());
/// # Ok::<(), polyoxide_clob::ClobError>(())
/// ```
#[derive(Clone, Debug)]
pub struct Account {
    wallet: Wallet,
    credentials: Credentials,
    signer: Signer,
}

impl Account {
    /// Create a new account from private key and credentials.
    ///
    /// # Arguments
    ///
    /// * `private_key` - Hex-encoded private key (with or without 0x prefix)
    /// * `credentials` - API credentials for L2 authentication
    ///
    /// # Example
    ///
    /// ```no_run
    /// use polyoxide_clob::{Account, Credentials};
    ///
    /// let credentials = Credentials {
    ///     key: "api_key".to_string(),
    ///     secret: "api_secret".to_string(),
    ///     passphrase: "passphrase".to_string(),
    /// };
    ///
    /// let account = Account::new("0x...", credentials)?;
    /// # Ok::<(), polyoxide_clob::ClobError>(())
    /// ```
    pub fn new(
        private_key: impl Into<String>,
        credentials: Credentials,
    ) -> Result<Self, ClobError> {
        let wallet = Wallet::from_private_key(&private_key.into())?;
        let signer = Signer::new(&credentials.secret);

        Ok(Self {
            wallet,
            credentials,
            signer,
        })
    }

    /// Load account from environment variables.
    ///
    /// Reads the following environment variables:
    /// - `POLYMARKET_PRIVATE_KEY`: Hex-encoded private key
    /// - `POLYMARKET_API_KEY`: API key
    /// - `POLYMARKET_API_SECRET`: API secret (base64 encoded)
    /// - `POLYMARKET_API_PASSPHRASE`: API passphrase
    ///
    /// # Example
    ///
    /// ```no_run
    /// use polyoxide_clob::Account;
    ///
    /// let account = Account::from_env()?;
    /// # Ok::<(), polyoxide_clob::ClobError>(())
    /// ```
    pub fn from_env() -> Result<Self, ClobError> {
        let private_key = std::env::var(env::PRIVATE_KEY).map_err(|_| {
            ClobError::validation(format!(
                "Missing environment variable: {}",
                env::PRIVATE_KEY
            ))
        })?;

        let credentials = Credentials {
            key: std::env::var(env::API_KEY).map_err(|_| {
                ClobError::validation(format!("Missing environment variable: {}", env::API_KEY))
            })?,
            secret: std::env::var(env::API_SECRET).map_err(|_| {
                ClobError::validation(format!("Missing environment variable: {}", env::API_SECRET))
            })?,
            passphrase: std::env::var(env::API_PASSPHRASE).map_err(|_| {
                ClobError::validation(format!(
                    "Missing environment variable: {}",
                    env::API_PASSPHRASE
                ))
            })?,
        };

        Self::new(private_key, credentials)
    }

    /// Load account from a JSON configuration file.
    ///
    /// The file should contain:
    /// ```json
    /// {
    ///     "private_key": "0x...",
    ///     "key": "api_key",
    ///     "secret": "api_secret",
    ///     "passphrase": "passphrase"
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```no_run
    /// use polyoxide_clob::Account;
    ///
    /// let account = Account::from_file("config/account.json")?;
    /// # Ok::<(), polyoxide_clob::ClobError>(())
    /// ```
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ClobError> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path).map_err(|e| {
            ClobError::validation(format!(
                "Failed to read config file {}: {}",
                path.display(),
                e
            ))
        })?;

        Self::from_json(&content)
    }

    /// Load account from a JSON string.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use polyoxide_clob::Account;
    ///
    /// let json = r#"{
    ///     "private_key": "0x...",
    ///     "key": "api_key",
    ///     "secret": "api_secret",
    ///     "passphrase": "passphrase"
    /// }"#;
    ///
    /// let account = Account::from_json(json)?;
    /// # Ok::<(), polyoxide_clob::ClobError>(())
    /// ```
    pub fn from_json(json: &str) -> Result<Self, ClobError> {
        let config: AccountConfig = serde_json::from_str(json)
            .map_err(|e| ClobError::validation(format!("Failed to parse JSON config: {}", e)))?;

        Self::new(config.private_key, config.credentials)
    }

    /// Get the wallet address.
    pub fn address(&self) -> Address {
        self.wallet.address()
    }

    /// Get a reference to the wallet.
    pub fn wallet(&self) -> &Wallet {
        &self.wallet
    }

    /// Get a reference to the credentials.
    pub fn credentials(&self) -> &Credentials {
        &self.credentials
    }

    /// Get a reference to the HMAC signer.
    pub fn signer(&self) -> &Signer {
        &self.signer
    }

    /// Sign an order using EIP-712.
    ///
    /// # Arguments
    ///
    /// * `order` - The unsigned order to sign
    /// * `chain_id` - The chain ID for EIP-712 domain
    ///
    /// # Example
    ///
    /// ```no_run
    /// use polyoxide_clob::{Account, Order};
    ///
    /// async fn example(account: &Account, order: &Order) -> Result<(), Box<dyn std::error::Error>> {
    ///     let signed_order = account.sign_order(order, 137).await?;
    ///     println!("Signature: {}", signed_order.signature);
    ///     Ok(())
    /// }
    /// ```
    pub async fn sign_order(&self, order: &Order, chain_id: u64) -> Result<SignedOrder, ClobError> {
        let signature = sign_order(order, self.wallet.signer(), chain_id).await?;

        Ok(SignedOrder {
            order: order.clone(),
            signature,
        })
    }

    /// Sign a CLOB authentication message for API key creation (L1 auth).
    ///
    /// # Arguments
    ///
    /// * `chain_id` - The chain ID for EIP-712 domain
    /// * `timestamp` - Unix timestamp in seconds
    /// * `nonce` - Random nonce value
    pub async fn sign_clob_auth(
        &self,
        chain_id: u64,
        timestamp: u64,
        nonce: u32,
    ) -> Result<String, ClobError> {
        sign_clob_auth(self.wallet.signer(), chain_id, timestamp, nonce).await
    }

    /// Sign an L2 API request message using HMAC.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Unix timestamp in seconds
    /// * `method` - HTTP method (GET, POST, DELETE)
    /// * `path` - Request path (e.g., "/order")
    /// * `body` - Optional request body
    pub fn sign_l2_request(
        &self,
        timestamp: u64,
        method: &str,
        path: &str,
        body: Option<&str>,
    ) -> Result<String, ClobError> {
        let message = Signer::create_message(timestamp, method, path, body);
        self.signer.sign(&message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_json() {
        let json = r#"{
            "private_key": "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
            "key": "test_key",
            "secret": "c2VjcmV0",
            "passphrase": "test_pass"
        }"#;

        let account = Account::from_json(json).unwrap();
        assert_eq!(account.credentials().key, "test_key");
        assert_eq!(account.credentials().passphrase, "test_pass");
    }

    #[test]
    fn test_sign_l2_request() {
        let json = r#"{
            "private_key": "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
            "key": "test_key",
            "secret": "c2VjcmV0",
            "passphrase": "test_pass"
        }"#;

        let account = Account::from_json(json).unwrap();
        let signature = account
            .sign_l2_request(1234567890, "GET", "/api/test", None)
            .unwrap();

        // Should be URL-safe base64
        assert!(!signature.contains('+'));
        assert!(!signature.contains('/'));
    }
}
