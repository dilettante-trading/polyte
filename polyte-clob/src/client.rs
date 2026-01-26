use polyte_core::{HttpClient, HttpClientBuilder, DEFAULT_POOL_SIZE, DEFAULT_TIMEOUT_MS};
use reqwest::Client;
use url::Url;

use crate::{
    account::{Account, Credentials},
    api::{account::AccountApi, orders::OrderResponse, Markets, Orders},
    core::chain::Chain,
    error::ClobError,
    request::{AuthMode, Request},
    types::*,
    utils::{calculate_order_amounts, current_timestamp, generate_salt},
};

const DEFAULT_BASE_URL: &str = "https://clob.polymarket.com";

#[derive(Clone)]
pub struct Clob {
    pub(crate) client: Client,
    pub(crate) base_url: Url,
    pub(crate) chain_id: u64,
    pub(crate) account: Option<Account>,
}

impl Clob {
    /// Create a new CLOB client with default configuration
    pub fn new(
        private_key: impl Into<String>,
        credentials: Credentials,
    ) -> Result<Self, ClobError> {
        Self::builder(private_key, credentials)?.build()
    }

    /// Create a new public CLOB client (read-only)
    pub fn public() -> Self {
        ClobBuilder::new().build().unwrap() // unwrap safe because default build never fails
    }

    /// Create a new CLOB client builder with required authentication
    pub fn builder(
        private_key: impl Into<String>,
        credentials: Credentials,
    ) -> Result<ClobBuilder, ClobError> {
        let account = Account::new(private_key, credentials)?;
        Ok(ClobBuilder::new().with_account(account))
    }

    /// Create a new CLOB client from an Account
    pub fn from_account(account: Account) -> Result<Self, ClobError> {
        ClobBuilder::new().with_account(account).build()
    }

    /// Get a reference to the account
    pub fn account(&self) -> Option<&Account> {
        self.account.as_ref()
    }

    /// Get markets namespace
    pub fn markets(&self) -> Markets {
        Markets {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
            chain_id: self.chain_id,
        }
    }

    /// Get orders namespace
    pub fn orders(&self) -> Result<Orders, ClobError> {
        let account = self
            .account
            .as_ref()
            .ok_or_else(|| ClobError::validation("Account required for orders API"))?;

        Ok(Orders {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
            wallet: account.wallet().clone(),
            credentials: account.credentials().clone(),
            signer: account.signer().clone(),
            chain_id: self.chain_id,
        })
    }

    /// Get account API namespace
    pub fn account_api(&self) -> Result<AccountApi, ClobError> {
        let account = self
            .account
            .as_ref()
            .ok_or_else(|| ClobError::validation("Account required for account API"))?;

        Ok(AccountApi {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
            wallet: account.wallet().clone(),
            credentials: account.credentials().clone(),
            signer: account.signer().clone(),
            chain_id: self.chain_id,
        })
    }

    /// Create an unsigned order from parameters
    pub async fn create_order(&self, params: &CreateOrderParams) -> Result<Order, ClobError> {
        let account = self
            .account
            .as_ref()
            .ok_or_else(|| ClobError::validation("Account required to create order"))?;

        params.validate()?;

        // Fetch market info for tick size
        let market = self.markets().get(&params.token_id).send().await?;
        let tick_size = TickSize::try_from(market.minimum_tick_size)?;

        // Get fee rate
        let fee_rate_response: serde_json::Value = self
            .client
            .get(self.base_url.join("/fee-rate")?)
            .send()
            .await?
            .json()
            .await?;

        let fee_rate_bps = fee_rate_response["feeRateBps"]
            .as_str()
            .unwrap_or("0")
            .to_string();

        // Calculate amounts
        let (maker_amount, taker_amount) =
            calculate_order_amounts(params.price, params.size, params.side, tick_size);

        Ok(Order {
            salt: generate_salt(),
            maker: account.address(),
            signer: account.address(),
            taker: alloy::primitives::Address::ZERO,
            token_id: params.token_id.clone(),
            maker_amount,
            taker_amount,
            expiration: params.expiration.unwrap_or(0).to_string(),
            nonce: current_timestamp().to_string(),
            fee_rate_bps,
            side: params.side,
            signature_type: SignatureType::default(),
        })
    }

    /// Sign an order
    pub async fn sign_order(&self, order: &Order) -> Result<SignedOrder, ClobError> {
        let account = self
            .account
            .as_ref()
            .ok_or_else(|| ClobError::validation("Account required to sign order"))?;
        account.sign_order(order, self.chain_id).await
    }

    /// Post a signed order
    pub async fn post_order(&self, signed_order: &SignedOrder) -> Result<OrderResponse, ClobError> {
        let account = self
            .account
            .as_ref()
            .ok_or_else(|| ClobError::validation("Account required to post order"))?;

        let auth = AuthMode::L2 {
            address: account.address(),
            credentials: account.credentials().clone(),
            signer: account.signer().clone(),
        };

        Request::post(
            self.client.clone(),
            self.base_url.clone(),
            "/order".to_string(),
            auth,
            self.chain_id,
        )
        .body(signed_order)?
        .send()
        .await
    }

    /// Create, sign, and post an order (convenience method)
    pub async fn place_order(
        &self,
        params: &CreateOrderParams,
    ) -> Result<OrderResponse, ClobError> {
        let order = self.create_order(params).await?;
        let signed_order = self.sign_order(&order).await?;
        self.post_order(&signed_order).await
    }
}

/// Parameters for creating an order
#[derive(Debug, Clone)]
pub struct CreateOrderParams {
    pub token_id: String,
    pub price: f64,
    pub size: f64,
    pub side: OrderSide,
    pub expiration: Option<u64>,
}

impl CreateOrderParams {
    pub fn validate(&self) -> Result<(), ClobError> {
        if self.price <= 0.0 || self.price > 1.0 {
            return Err(ClobError::validation(format!(
                "Price must be between 0.0 and 1.0, got {}",
                self.price
            )));
        }
        if self.size <= 0.0 {
            return Err(ClobError::validation(format!(
                "Size must be positive, got {}",
                self.size
            )));
        }
        if self.price.is_nan() || self.size.is_nan() {
            return Err(ClobError::validation("NaN values not allowed"));
        }
        Ok(())
    }
}

/// Builder for CLOB client
pub struct ClobBuilder {
    base_url: String,
    timeout_ms: u64,
    pool_size: usize,
    chain: Chain,
    account: Option<Account>,
}

impl ClobBuilder {
    /// Create a new builder with default configuration
    pub fn new() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            timeout_ms: DEFAULT_TIMEOUT_MS,
            pool_size: DEFAULT_POOL_SIZE,
            chain: Chain::PolygonMainnet,
            account: None,
        }
    }

    /// Set account for the client
    pub fn with_account(mut self, account: Account) -> Self {
        self.account = Some(account);
        self
    }

    /// Set base URL for the API
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Set request timeout in milliseconds
    pub fn timeout_ms(mut self, timeout: u64) -> Self {
        self.timeout_ms = timeout;
        self
    }

    /// Set connection pool size
    pub fn pool_size(mut self, size: usize) -> Self {
        self.pool_size = size;
        self
    }

    /// Set chain
    pub fn chain(mut self, chain: Chain) -> Self {
        self.chain = chain;
        self
    }

    /// Build the CLOB client
    pub fn build(self) -> Result<Clob, ClobError> {
        let HttpClient { client, base_url } = HttpClientBuilder::new(&self.base_url)
            .timeout_ms(self.timeout_ms)
            .pool_size(self.pool_size)
            .build()?;

        Ok(Clob {
            client,
            base_url,
            chain_id: self.chain.chain_id(),
            account: self.account,
        })
    }
}

impl Default for ClobBuilder {
    fn default() -> Self {
        Self::new()
    }
}
