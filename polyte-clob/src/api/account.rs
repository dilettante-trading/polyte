use alloy::primitives::Address;
use std::collections::HashMap;
use polyte_core::QueryBuilder;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    account::{Credentials, Signer, Wallet},
    request::{AuthMode, Request},
    types::OrderSide,
};

/// Account API namespace for account-related operations
#[derive(Clone)]
pub struct AccountApi {
    pub(crate) client: Client,
    pub(crate) base_url: Url,
    pub(crate) wallet: Wallet,
    pub(crate) credentials: Credentials,
    pub(crate) signer: Signer,
    pub(crate) chain_id: u64,
}

impl AccountApi {
    /// Get balance and allowance for a token
    pub fn balance_allowance(
        &self,
        token_id: impl Into<String>,
    ) -> Request<BalanceAllowanceResponse> {
        Request::get(
            self.client.clone(),
            self.base_url.clone(),
            "/balance-allowance",
            AuthMode::L2 {
                address: self.wallet.clone().address(),
                credentials: self.credentials.clone(),
                signer: self.signer.clone(),
            },
            self.chain_id,
        )
        .query("token_id", token_id.into())
    }

    pub fn usdc_balance(&self) -> Request<BalanceAllowanceResponse> {
         Request::get(
            self.client.clone(),
            self.base_url.clone(),
            "/balance-allowance",
            AuthMode::L2 {
                address: self.wallet.clone().address(),
                credentials: self.credentials.clone(),
                signer: self.signer.clone(),
            },
            self.chain_id,
        )
        .query("asset_type", "COLLATERAL")
        .query("signature_type", 1)
    }

    /// Get trades
    pub fn trades(&self) -> Request<Vec<Trade>> {
        Request::get(
            self.client.clone(),
            self.base_url.clone(),
            "/trades",
            AuthMode::L2 {
                address: self.wallet.clone().address(),
                credentials: self.credentials.clone(),
                signer: self.signer.clone(),
            },
            self.chain_id,
        )
    }
}

/// Trade information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub id: String,
    pub taker_order_id: String,
    pub market: String,
    pub asset_id: String,
    pub side: OrderSide,
    pub size: String,
    pub fee_rate_bps: String,
    pub price: String,
    pub status: String,
    pub match_time: String,
    #[serde(default)]
    pub last_update: Option<String>,
    pub outcome: String,
    #[serde(default)]
    pub bucket_index: Option<u32>,
    pub owner: Address,
    pub transaction_hash: String,
}

/// Balance and allowance response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceAllowanceResponse {
    pub balance: String,
    pub allowances: HashMap<Address, String>,
}
