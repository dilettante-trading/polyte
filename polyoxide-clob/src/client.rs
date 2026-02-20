use polyoxide_core::{HttpClient, HttpClientBuilder, DEFAULT_POOL_SIZE, DEFAULT_TIMEOUT_MS};
use reqwest::Client;
use url::Url;

use crate::{
    account::{Account, Credentials},
    api::{account::AccountApi, orders::OrderResponse, Health, Markets, Orders},
    core::chain::Chain,
    error::ClobError,
    request::{AuthMode, Request},
    types::*,
    utils::{
        calculate_market_order_amounts, calculate_market_price, calculate_order_amounts,
        generate_salt,
    },
};
use alloy::primitives::Address;
use polyoxide_gamma::Gamma;

const DEFAULT_BASE_URL: &str = "https://clob.polymarket.com";

#[derive(Clone)]
pub struct Clob {
    pub(crate) client: Client,
    pub(crate) base_url: Url,
    pub(crate) chain_id: u64,
    pub(crate) account: Option<Account>,
    pub(crate) gamma: Gamma,
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

    /// Get health namespace for latency and health checks
    pub fn health(&self) -> Health {
        Health {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
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
    pub async fn create_order(
        &self,
        params: &CreateOrderParams,
        options: Option<PartialCreateOrderOptions>,
    ) -> Result<Order, ClobError> {
        let account = self
            .account
            .as_ref()
            .ok_or_else(|| ClobError::validation("Account required to create order"))?;

        params.validate()?;

        // Fetch market metadata (neg_risk and tick_size)
        let (neg_risk, tick_size) = self.get_market_metadata(&params.token_id, options).await?;

        // Get fee rate
        let fee_rate_bps = self.get_fee_rate().await?;

        // Calculate amounts
        let (maker_amount, taker_amount) =
            calculate_order_amounts(params.price, params.size, params.side, tick_size);

        // Resolve maker address
        let signature_type = params.signature_type.unwrap_or_default();
        let maker = self
            .resolve_maker_address(params.funder, signature_type, account)
            .await?;

        // Build order
        Ok(Self::build_order(
            params.token_id.clone(),
            maker,
            account.address(),
            maker_amount,
            taker_amount,
            fee_rate_bps,
            params.side,
            signature_type,
            neg_risk,
            params.expiration,
        ))
    }

    /// Create an unsigned market order from parameters
    pub async fn create_market_order(
        &self,
        params: &MarketOrderArgs,
        options: Option<PartialCreateOrderOptions>,
    ) -> Result<Order, ClobError> {
        let account = self
            .account
            .as_ref()
            .ok_or_else(|| ClobError::validation("Account required to create order"))?;

        if params.amount <= 0.0 {
            return Err(ClobError::validation(format!(
                "Amount must be positive, got {}",
                params.amount
            )));
        }

        // Fetch market metadata (neg_risk and tick_size)
        let (neg_risk, tick_size) = self.get_market_metadata(&params.token_id, options).await?;

        // Determine price
        let price = if let Some(p) = params.price {
            p
        } else {
            // Fetch orderbook and calculate price
            let book = self
                .markets()
                .order_book(params.token_id.clone())
                .send()
                .await?;

            let levels = match params.side {
                OrderSide::Buy => book.asks,
                OrderSide::Sell => book.bids,
            };

            calculate_market_price(&levels, params.amount, params.side)
                .ok_or_else(|| ClobError::validation("Not enough liquidity to fill market order"))?
        };

        // Get fee rate
        let fee_rate_bps = self.get_fee_rate().await?;

        // Calculate amounts
        let (maker_amount, taker_amount) =
            calculate_market_order_amounts(params.amount, price, params.side, tick_size);

        // Resolve maker address
        let signature_type = params.signature_type.unwrap_or_default();
        let maker = self
            .resolve_maker_address(params.funder, signature_type, account)
            .await?;

        // Build order with expiration set to 0 for market orders
        Ok(Self::build_order(
            params.token_id.clone(),
            maker,
            account.address(),
            maker_amount,
            taker_amount,
            fee_rate_bps,
            params.side,
            signature_type,
            neg_risk,
            Some(0),
        ))
    }
    pub async fn sign_order(&self, order: &Order) -> Result<SignedOrder, ClobError> {
        let account = self
            .account
            .as_ref()
            .ok_or_else(|| ClobError::validation("Account required to sign order"))?;
        account.sign_order(order, self.chain_id).await
    }

    // Helper methods for order creation

    /// Fetch market metadata (neg_risk and tick_size) for a token
    async fn get_market_metadata(
        &self,
        token_id: &str,
        options: Option<PartialCreateOrderOptions>,
    ) -> Result<(bool, TickSize), ClobError> {
        // Fetch or use provided neg_risk status
        let neg_risk = if let Some(neg_risk) = options.and_then(|o| o.neg_risk) {
            neg_risk
        } else {
            let neg_risk_resp = self.markets().neg_risk(token_id.to_string()).send().await?;
            neg_risk_resp.neg_risk
        };

        // Fetch or use provided tick size
        let tick_size = if let Some(tick_size) = options.and_then(|o| o.tick_size) {
            tick_size
        } else {
            let tick_size_resp = self
                .markets()
                .tick_size(token_id.to_string())
                .send()
                .await?;
            let tick_size_val = tick_size_resp
                .minimum_tick_size
                .parse::<f64>()
                .map_err(|e| {
                    ClobError::validation(format!("Invalid minimum_tick_size field: {}", e))
                })?;
            TickSize::try_from(tick_size_val)?
        };

        Ok((neg_risk, tick_size))
    }

    /// Fetch the current fee rate from the API
    async fn get_fee_rate(&self) -> Result<String, ClobError> {
        let fee_rate_response: serde_json::Value = self
            .client
            .get(self.base_url.join("/fee-rate")?)
            .send()
            .await?
            .json()
            .await?;

        Ok(fee_rate_response["feeRateBps"]
            .as_str()
            .unwrap_or("0")
            .to_string())
    }

    /// Resolve the maker address based on funder and signature type
    async fn resolve_maker_address(
        &self,
        funder: Option<Address>,
        signature_type: SignatureType,
        account: &Account,
    ) -> Result<Address, ClobError> {
        if let Some(funder) = funder {
            Ok(funder)
        } else if signature_type.is_proxy() {
            // Fetch proxy from Gamma
            let profile = self
                .gamma
                .user()
                .get(account.address().to_string())
                .send()
                .await
                .map_err(|e| ClobError::service(format!("Failed to fetch user profile: {}", e)))?;

            profile
                .proxy
                .ok_or_else(|| {
                    ClobError::validation(format!(
                        "Signature type {:?} requires proxy, but none found for {}",
                        signature_type,
                        account.address()
                    ))
                })?
                .parse::<Address>()
                .map_err(|e| {
                    ClobError::validation(format!("Invalid proxy address format from Gamma: {}", e))
                })
        } else {
            Ok(account.address())
        }
    }

    /// Build an Order struct from the provided parameters
    #[allow(clippy::too_many_arguments)]
    fn build_order(
        token_id: String,
        maker: Address,
        signer: Address,
        maker_amount: String,
        taker_amount: String,
        fee_rate_bps: String,
        side: OrderSide,
        signature_type: SignatureType,
        neg_risk: bool,
        expiration: Option<u64>,
    ) -> Order {
        Order {
            salt: generate_salt(),
            maker,
            signer,
            taker: alloy::primitives::Address::ZERO,
            token_id,
            maker_amount,
            taker_amount,
            expiration: expiration.unwrap_or(0).to_string(),
            nonce: "0".to_string(),
            fee_rate_bps,
            side,
            signature_type,
            neg_risk,
        }
    }

    /// Post a signed order
    /// Post a signed order
    pub async fn post_order(
        &self,
        signed_order: &SignedOrder,
        order_type: OrderKind,
        post_only: bool,
    ) -> Result<OrderResponse, ClobError> {
        let account = self
            .account
            .as_ref()
            .ok_or_else(|| ClobError::validation("Account required to post order"))?;

        let auth = AuthMode::L2 {
            address: account.address(),
            credentials: account.credentials().clone(),
            signer: account.signer().clone(),
        };

        // Create the payload wrapping the signed order
        let payload = serde_json::json!({
            "order": signed_order,
            "owner": account.credentials().key,
            "orderType": order_type,
            "postOnly": post_only,
        });

        Request::post(
            self.client.clone(),
            self.base_url.clone(),
            "/order".to_string(),
            auth,
            self.chain_id,
        )
        .body(&payload)?
        .send()
        .await
    }

    /// Create, sign, and post an order (convenience method)
    pub async fn place_order(
        &self,
        params: &CreateOrderParams,
        options: Option<PartialCreateOrderOptions>,
    ) -> Result<OrderResponse, ClobError> {
        let order = self.create_order(params, options).await?;
        let signed_order = self.sign_order(&order).await?;
        self.post_order(&signed_order, params.order_type, params.post_only)
            .await
    }

    /// Create, sign, and post a market order (convenience method)
    pub async fn place_market_order(
        &self,
        params: &MarketOrderArgs,
        options: Option<PartialCreateOrderOptions>,
    ) -> Result<OrderResponse, ClobError> {
        let order = self.create_market_order(params, options).await?;
        let signed_order = self.sign_order(&order).await?;

        let order_type = params.order_type.unwrap_or(OrderKind::Fok);
        // Market orders are usually FOK

        self.post_order(&signed_order, order_type, false) // Market orders cannot be post_only
            .await
    }
}

/// Parameters for creating an order
#[derive(Debug, Clone)]
pub struct CreateOrderParams {
    pub token_id: String,
    pub price: f64,
    pub size: f64,
    pub side: OrderSide,
    pub order_type: OrderKind,
    pub post_only: bool,
    pub expiration: Option<u64>,
    pub funder: Option<Address>,
    pub signature_type: Option<SignatureType>,
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
    gamma: Option<Gamma>,
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
            gamma: None,
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

    /// Set Gamma client
    pub fn gamma(mut self, gamma: Gamma) -> Self {
        self.gamma = Some(gamma);
        self
    }

    /// Build the CLOB client
    pub fn build(self) -> Result<Clob, ClobError> {
        let HttpClient { client, base_url } = HttpClientBuilder::new(&self.base_url)
            .timeout_ms(self.timeout_ms)
            .pool_size(self.pool_size)
            .build()?;

        let gamma = if let Some(gamma) = self.gamma {
            gamma
        } else {
            polyoxide_gamma::Gamma::builder()
                .timeout_ms(self.timeout_ms)
                .pool_size(self.pool_size)
                .build()
                .map_err(|e| {
                    ClobError::service(format!("Failed to build default Gamma client: {}", e))
                })?
        };

        Ok(Clob {
            client,
            base_url,
            chain_id: self.chain.chain_id(),
            account: self.account,
            gamma,
        })
    }
}

impl Default for ClobBuilder {
    fn default() -> Self {
        Self::new()
    }
}
