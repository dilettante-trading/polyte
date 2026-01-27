use polyte_core::{QueryBuilder, Request};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::error::GammaError;

/// User API namespace
#[derive(Clone)]
pub struct User {
    pub(crate) client: Client,
    pub(crate) base_url: Url,
}

impl User {
    /// Get user details
    pub fn get(&self, signer_address: impl Into<String>) -> Request<UserResponse, GammaError> {
        Request::new(
            self.client.clone(),
            self.base_url.clone(),
            "/public-profile",
        )
        .query("address", signer_address.into())
    }
}

/// User details response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    /// The user's proxy wallet address (Treasury)
    #[serde(rename = "proxyWallet")]
    pub proxy: Option<String>,
    /// The user's EOA address (Signer)
    pub address: Option<String>,
    /// Account ID
    pub id: Option<String>,
    /// Username/Display name
    pub name: Option<String>,
}
