use std::marker::PhantomData;

use alloy::primitives::Address;
use polyoxide_core::request::QueryBuilder;
use reqwest::{Client, Method, Response};
use serde::de::DeserializeOwned;
use url::Url;

use crate::{
    account::{Credentials, Signer, Wallet},
    error::ClobError,
    utils::current_timestamp,
};

/// Authentication mode for requests
#[derive(Debug, Clone)]
pub enum AuthMode {
    None,
    L1 {
        wallet: Wallet,
        nonce: u32,
        timestamp: u64,
    },
    L2 {
        address: Address,
        credentials: Credentials,
        signer: Signer,
    },
}

/// Generic request builder for CLOB API
pub struct Request<T> {
    pub(crate) client: Client,
    pub(crate) base_url: Url,
    pub(crate) path: String,
    pub(crate) method: Method,
    pub(crate) query: Vec<(String, String)>,
    pub(crate) body: Option<serde_json::Value>,
    pub(crate) auth: AuthMode,
    pub(crate) chain_id: u64,
    pub(crate) _marker: PhantomData<T>,
}

impl<T> Request<T> {
    /// Create a new GET request
    pub(crate) fn get(
        client: Client,
        base_url: Url,
        path: impl Into<String>,
        auth: AuthMode,
        chain_id: u64,
    ) -> Self {
        Self {
            client,
            base_url,
            path: path.into(),
            method: Method::GET,
            query: Vec::new(),
            body: None,
            auth,
            chain_id,
            _marker: PhantomData,
        }
    }

    /// Create a new POST request
    pub(crate) fn post(
        client: Client,
        base_url: Url,
        path: String,
        auth: AuthMode,
        chain_id: u64,
    ) -> Self {
        Self {
            client,
            base_url,
            path,
            method: Method::POST,
            query: Vec::new(),
            body: None,
            auth,
            chain_id,
            _marker: PhantomData,
        }
    }

    /// Create a new DELETE request
    pub(crate) fn delete(
        client: Client,
        base_url: Url,
        path: impl Into<String>,
        auth: AuthMode,
        chain_id: u64,
    ) -> Self {
        Self {
            client,
            base_url,
            path: path.into(),
            method: Method::DELETE,
            query: Vec::new(),
            body: None,
            auth,
            chain_id,
            _marker: PhantomData,
        }
    }

    /// Set request body
    pub fn body<B: serde::Serialize>(mut self, body: &B) -> Result<Self, ClobError> {
        self.body = Some(serde_json::to_value(body)?);
        Ok(self)
    }
}

impl<T> QueryBuilder for Request<T> {
    fn add_query(&mut self, key: String, value: String) {
        self.query.push((key, value));
    }
}

impl<T: DeserializeOwned> Request<T> {
    /// Execute the request and deserialize response
    pub async fn send(self) -> Result<T, ClobError> {
        let response = self.send_raw().await?;

        // Get text for debugging
        let text = response.text().await?;

        tracing::debug!("Response body: {}", text);

        // Deserialize and provide better error context
        serde_json::from_str(&text).map_err(|e| {
            tracing::error!("Deserialization failed: {}", e);
            tracing::error!("Failed to deserialize: {}", text);
            e.into()
        })
    }

    /// Execute the request and return raw response
    pub async fn send_raw(self) -> Result<Response, ClobError> {
        let url = self.base_url.join(&self.path)?;

        // Build the base request
        let mut request = match self.method {
            Method::GET => self.client.get(url),
            Method::POST => {
                let mut req = self.client.post(url);
                if let Some(body) = &self.body {
                    req = req.header("Content-Type", "application/json").json(body);
                }
                req
            }
            Method::DELETE => {
                let mut req = self.client.delete(url);
                if let Some(body) = &self.body {
                    req = req.header("Content-Type", "application/json").json(body);
                }
                req
            }
            _ => return Err(ClobError::validation("Unsupported HTTP method")),
        };

        // Add query parameters
        if !self.query.is_empty() {
            request = request.query(&self.query);
        }

        // Add authentication headers
        request = self.add_auth_headers(request).await?;

        tracing::debug!("Sending {} request to: {:?}", self.method, request);

        // Execute request
        let response = request.send().await?;
        let status = response.status();

        tracing::debug!("Response status: {}", status);

        if !status.is_success() {
            let error = ClobError::from_response(response).await;
            tracing::error!("Request failed: {:?}", error);
            return Err(error);
        }

        Ok(response)
    }

    /// Add authentication headers based on auth mode
    async fn add_auth_headers(
        &self,
        mut request: reqwest::RequestBuilder,
    ) -> Result<reqwest::RequestBuilder, ClobError> {
        match &self.auth {
            AuthMode::None => Ok(request),
            AuthMode::L1 {
                wallet,
                nonce,
                timestamp,
            } => {
                use crate::core::eip712::sign_clob_auth;

                let signature =
                    sign_clob_auth(wallet.signer(), self.chain_id, *timestamp, *nonce).await?;

                request = request
                    .header("POLY_ADDRESS", format!("{:?}", wallet.address()))
                    .header("POLY_SIGNATURE", signature)
                    .header("POLY_TIMESTAMP", timestamp.to_string())
                    .header("POLY_NONCE", nonce.to_string());

                Ok(request)
            }
            AuthMode::L2 {
                address,
                credentials,
                signer,
            } => {
                let timestamp = current_timestamp();
                let body_str = self.body.as_ref().map(|b| b.to_string());
                let message = Signer::create_message(
                    timestamp,
                    self.method.as_str(),
                    &self.path,
                    body_str.as_deref(),
                );
                let signature = signer.sign(&message)?;

                request = request
                    .header("POLY_ADDRESS", format!("{:?}", address))
                    .header("POLY_SIGNATURE", signature)
                    .header("POLY_TIMESTAMP", timestamp.to_string())
                    .header("POLY_API_KEY", &credentials.key)
                    .header("POLY_PASSPHRASE", &credentials.passphrase);

                Ok(request)
            }
        }
    }
}
