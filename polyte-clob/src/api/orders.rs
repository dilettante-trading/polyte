use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    error::Result,
    request::{AuthMode, Request},
    signer::Signer,
    types::{Credentials, SignedOrder},
    wallet::Wallet,
};

/// Orders namespace for order-related operations
#[derive(Clone)]
pub struct Orders {
    pub(crate) client: Client,
    pub(crate) base_url: Url,
    pub(crate) wallet: Wallet,
    pub(crate) credentials: Credentials,
    pub(crate) signer: Signer,
    pub(crate) chain_id: u64,
}

impl Orders {
    /// List user's orders
    pub fn list(&self) -> Request<Vec<OpenOrder>> {
        Request::get(
            self.client.clone(),
            self.base_url.clone(),
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
            client: self.client.clone(),
            base_url: self.base_url.clone(),
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
    client: Client,
    base_url: Url,
    auth: AuthMode,
    chain_id: u64,
    order_id: String,
}

impl CancelOrderRequest {
    /// Execute the cancel request
    pub async fn send(self) -> Result<CancelResponse> {
        #[derive(serde::Serialize)]
        struct CancelRequest {
            #[serde(rename = "orderID")]
            order_id: String,
        }

        let request = CancelRequest {
            order_id: self.order_id,
        };

        Request::delete(
            self.client,
            self.base_url,
            "/order",
            self.auth,
            self.chain_id,
        )
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
    pub success: bool,
    pub error_msg: Option<String>,
}
