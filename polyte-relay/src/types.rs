use alloy::sol;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonceResponse {
    pub nonce: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionStatusResponse {
    pub state: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<String>,
}
