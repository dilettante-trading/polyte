use alloy::sol;
use serde::{Deserialize, Serialize};

/// Wallet type for the relayer API
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum WalletType {
    /// Safe wallet - requires explicit deployment before first transaction
    #[default]
    Safe,
    /// Proxy wallet - auto-deploys on first transaction (Magic Link users)
    Proxy,
}

impl WalletType {
    pub fn as_str(&self) -> &'static str {
        match self {
            WalletType::Safe => "SAFE",
            WalletType::Proxy => "PROXY",
        }
    }
}

sol! {
    #[derive(Debug, PartialEq, Eq)]
    struct SafeTransaction {
        address to;
        uint8 operation;
        bytes data;
        uint256 value;
    }

    #[derive(Debug, PartialEq, Eq)]
    struct SafeTransactionArgs {
        address from_address;
        uint256 nonce;
        uint256 chain_id;
        SafeTransaction[] transactions;
    }

    #[derive(Debug, PartialEq, Eq)]
    struct SafeTx {
        address to;
        uint256 value;
        bytes data;
        uint8 operation;
        uint256 safeTxGas;
        uint256 baseGas;
        uint256 gasPrice;
        address gasToken;
        address refundReceiver;
        uint256 nonce;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRequest {
    #[serde(rename = "type")]
    pub type_: String,
    pub from: String,
    pub to: String,
    #[serde(rename = "proxyWallet")]
    pub proxy_wallet: String,
    pub data: String,
    pub signature: String,
    // Add signature params if needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayerTransactionResponse {
    #[serde(rename = "transactionID")]
    pub transaction_id: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<String>,
}

pub fn deserialize_nonce<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de;

    struct NonceVisitor;

    impl<'de> de::Visitor<'de> for NonceVisitor {
        type Value = u64;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a u64 or string representing a u64")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
            Ok(v)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            v.parse().map_err(de::Error::custom)
        }
    }

    deserializer.deserialize_any(NonceVisitor)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonceResponse {
    #[serde(deserialize_with = "deserialize_nonce")]
    pub nonce: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionStatusResponse {
    pub state: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<String>,
}
