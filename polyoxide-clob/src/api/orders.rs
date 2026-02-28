use polyoxide_core::HttpClient;
use serde::{Deserialize, Serialize};

use crate::{
    account::{Credentials, Signer, Wallet},
    error::ClobError,
    request::{AuthMode, Request},
    types::SignedOrder,
};

/// Orders namespace for order-related operations
#[derive(Clone)]
pub struct Orders {
    pub(crate) http_client: HttpClient,
    pub(crate) wallet: Wallet,
    pub(crate) credentials: Credentials,
    pub(crate) signer: Signer,
    pub(crate) chain_id: u64,
}

impl Orders {
    /// List user's orders
    pub fn list(&self) -> Request<Vec<OpenOrder>> {
        Request::get(
            self.http_client.clone(),
            "/data/orders",
            AuthMode::L2 {
                address: self.wallet.address(),
                credentials: self.credentials.clone(),
                signer: self.signer.clone(),
            },
            self.chain_id,
        )
    }

    /// Cancel an order
    pub fn cancel(&self, order_id: impl Into<String>) -> CancelOrderRequest {
        CancelOrderRequest {
            http_client: self.http_client.clone(),
            auth: AuthMode::L2 {
                address: self.wallet.address(),
                credentials: self.credentials.clone(),
                signer: self.signer.clone(),
            },
            chain_id: self.chain_id,
            order_id: order_id.into(),
        }
    }
}

/// Request builder for canceling an order
pub struct CancelOrderRequest {
    http_client: HttpClient,
    auth: AuthMode,
    chain_id: u64,
    order_id: String,
}

impl CancelOrderRequest {
    /// Execute the cancel request
    pub async fn send(self) -> Result<CancelResponse, ClobError> {
        #[derive(serde::Serialize)]
        struct CancelRequest {
            #[serde(rename = "orderID")]
            order_id: String,
        }

        let request = CancelRequest {
            order_id: self.order_id,
        };

        Request::delete(self.http_client, "/order", self.auth, self.chain_id)
            .body(&request)?
            .send()
            .await
    }
}

/// Open order from API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct OpenOrder {
    pub id: String,
    pub market: String,
    pub asset_id: String,
    #[serde(flatten)]
    pub order: SignedOrder,
    pub status: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

/// Response from posting an order
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct OrderResponse {
    pub success: bool,
    pub error_msg: Option<String>,
    pub order_id: Option<String>,
    #[serde(default)]
    pub transaction_hashes: Vec<String>,
}

/// Response from canceling an order
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct CancelResponse {
    #[serde(default)]
    pub success: bool,
    pub error_msg: Option<String>,
    pub canceled_order_id: Option<String>,
    pub message: Option<String>,
}
