use polyoxide_core::{HttpClient, QueryBuilder, Request};
use serde::{Deserialize, Serialize};

use crate::error::GammaError;

/// User API namespace
#[derive(Clone)]
pub struct User {
    pub(crate) http_client: HttpClient,
}

impl User {
    /// Get user details
    pub fn get(&self, signer_address: impl Into<String>) -> Request<UserResponse, GammaError> {
        Request::new(self.http_client.clone(), "/public-profile")
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
