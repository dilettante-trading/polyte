use std::fmt;

use alloy::primitives::Address;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error when parsing a tick size from an invalid value
#[derive(Error, Debug, Clone, PartialEq)]
#[error("invalid tick size: {0}. Valid values are 0.1, 0.01, 0.001, or 0.0001")]
pub struct ParseTickSizeError(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderSide {
    Buy,
    Sell,
}

impl Serialize for OrderSide {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            // Python client serializes as "0" and "1"
            Self::Buy => serializer.serialize_str("0"),
            Self::Sell => serializer.serialize_str("1"),
        }
    }
}

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Buy => write!(f, "0"),
            Self::Sell => write!(f, "1"),
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

impl fmt::Display for OrderKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gtc => write!(f, "GTC"),
            Self::Fok => write!(f, "FOK"),
            Self::Gtd => write!(f, "GTD"),
            Self::Fak => write!(f, "FAK"),
        }
    }
}

/// Signature type
/// Signature type
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SignatureType {
    #[default]
    Eoa = 0,
    PolyProxy = 1,
    PolyGnosisSafe = 2,
}

impl Serialize for SignatureType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for SignatureType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = u8::deserialize(deserializer)?;
        match v {
            0 => Ok(Self::Eoa),
            1 => Ok(Self::PolyProxy),
            2 => Ok(Self::PolyGnosisSafe),
            _ => Err(serde::de::Error::custom(format!(
                "invalid signature type: {}",
                v
            ))),
        }
    }
}

impl fmt::Display for SignatureType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eoa => write!(f, "eoa"),
            Self::PolyProxy => write!(f, "poly-proxy"),
            Self::PolyGnosisSafe => write!(f, "poly-gnosis-safe"),
        }
    }
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

/// Options for creating an order
#[derive(Debug, Clone, Copy, Default)]
pub struct PartialCreateOrderOptions {
    pub tick_size: Option<TickSize>,
    pub neg_risk: Option<bool>,
}

impl TryFrom<&str> for TickSize {
    type Error = ParseTickSizeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "0.1" => Ok(Self::Tenth),
            "0.01" => Ok(Self::Hundredth),
            "0.001" => Ok(Self::Thousandth),
            "0.0001" => Ok(Self::TenThousandth),
            _ => Err(ParseTickSizeError(s.to_string())),
        }
    }
}

impl TryFrom<f64> for TickSize {
    type Error = ParseTickSizeError;

    fn try_from(n: f64) -> Result<Self, Self::Error> {
        const EPSILON: f64 = 1e-10;
        if (n - 0.1).abs() < EPSILON {
            Ok(Self::Tenth)
        } else if (n - 0.01).abs() < EPSILON {
            Ok(Self::Hundredth)
        } else if (n - 0.001).abs() < EPSILON {
            Ok(Self::Thousandth)
        } else if (n - 0.0001).abs() < EPSILON {
            Ok(Self::TenThousandth)
        } else {
            Err(ParseTickSizeError(n.to_string()))
        }
    }
}

impl std::str::FromStr for TickSize {
    type Err = ParseTickSizeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

fn serialize_salt<S>(salt: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    // Parse the string as u128 and serialize it as a number
    let val = salt
        .parse::<u128>()
        .map_err(|_| serde::ser::Error::custom("invalid salt"))?;
    serializer.serialize_u128(val)
}

/// Unsigned order
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    #[serde(serialize_with = "serialize_salt")]
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
    #[serde(skip)]
    pub neg_risk: bool,
}

/// Signed order
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedOrder {
    #[serde(flatten)]
    pub order: Order,
    pub signature: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Address;
    use std::str::FromStr;

    #[test]
    fn test_order_serialization() {
        let order = Order {
            salt: "123".to_string(),
            maker: Address::from_str("0x0000000000000000000000000000000000000001").unwrap(),
            signer: Address::from_str("0x0000000000000000000000000000000000000002").unwrap(),
            taker: Address::ZERO,
            token_id: "456".to_string(),
            maker_amount: "1000".to_string(),
            taker_amount: "2000".to_string(),
            expiration: "0".to_string(),
            nonce: "789".to_string(),
            fee_rate_bps: "0".to_string(),
            side: OrderSide::Buy,
            signature_type: SignatureType::Eoa,
            neg_risk: false,
        };

        let signed_order = SignedOrder {
            order,
            signature: "0xabc".to_string(),
        };

        let json = serde_json::to_value(&signed_order).unwrap();

        // Check camelCase
        assert!(json.get("makerAmount").is_some());
        assert!(json.get("takerAmount").is_some());
        assert!(json.get("tokenId").is_some());
        assert!(json.get("feeRateBps").is_some());
        assert!(json.get("signatureType").is_some());

        // Check flattened fields
        assert!(json.get("signature").is_some());
        assert!(json.get("salt").is_some());

        // Check values
        assert_eq!(json["makerAmount"], "1000");
        assert_eq!(json["side"], "0");
        assert_eq!(json["signatureType"], 0);
        assert_eq!(json["nonce"], "789");
    }
}
