use std::fmt;

use alloy::primitives::Address;
use serde::{Deserialize, Serialize};

/// API credentials for L2 authentication
#[derive(Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub key: String,
    pub secret: String,
    pub passphrase: String,
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("key", &"<redacted>")
            .field("secret", &"<redacted>")
            .field("passphrase", &"<redacted>")
            .finish()
    }
}

/// Order side (buy or sell)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderSide {
    Buy,
    Sell,
}

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Buy => write!(f, "BUY"),
            Self::Sell => write!(f, "SELL"),
        }
    }
}

/// Order type/kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderKind {
    /// Good-till-Cancelled
    Gtc,
    /// Fill-or-Kill
    Fok,
    /// Good-till-Date
    Gtd,
    /// Fill-and-Kill
    Fak,
}

/// Signature type
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SignatureType {
    #[default]
    Eoa,
    PolyProxy,
    PolyGnosisSafe,
}

/// Tick size (minimum price increment)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TickSize {
    /// 0.1
    Tenth,
    /// 0.01
    Hundredth,
    /// 0.001
    Thousandth,
    /// 0.0001
    TenThousandth,
}

impl TickSize {
    pub fn as_f64(&self) -> f64 {
        match self {
            Self::Tenth => 0.1,
            Self::Hundredth => 0.01,
            Self::Thousandth => 0.001,
            Self::TenThousandth => 0.0001,
        }
    }

    pub fn decimals(&self) -> u32 {
        match self {
            Self::Tenth => 1,
            Self::Hundredth => 2,
            Self::Thousandth => 3,
            Self::TenThousandth => 4,
        }
    }
}

impl From<&str> for TickSize {
    fn from(s: &str) -> Self {
        match s {
            "0.1" => Self::Tenth,
            "0.01" => Self::Hundredth,
            "0.001" => Self::Thousandth,
            "0.0001" => Self::TenThousandth,
            _ => Self::Hundredth, // Default
        }
    }
}

impl From<f64> for TickSize {
    fn from(n: f64) -> Self {
        const EPSILON: f64 = 1e-10;
        if (n - 0.1).abs() < EPSILON {
            Self::Tenth
        } else if (n - 0.01).abs() < EPSILON {
            Self::Hundredth
        } else if (n - 0.001).abs() < EPSILON {
            Self::Thousandth
        } else if (n - 0.0001).abs() < EPSILON {
            Self::TenThousandth
        } else {
            Self::Hundredth // Default
        }
    }
}

/// Unsigned order
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Order {
    pub salt: String,
    pub maker: Address,
    pub signer: Address,
    pub taker: Address,
    pub token_id: String,
    pub maker_amount: String,
    pub taker_amount: String,
    pub expiration: String,
    pub nonce: String,
    pub fee_rate_bps: String,
    pub side: OrderSide,
    pub signature_type: SignatureType,
}

/// Signed order
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SignedOrder {
    #[serde(flatten)]
    pub order: Order,
    pub signature: String,
}
