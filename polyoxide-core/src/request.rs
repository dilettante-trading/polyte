use std::marker::PhantomData;

use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use url::Url;

use crate::ApiError;

/// Query parameter builder
pub trait QueryBuilder: Sized {
    /// Add a query parameter
    fn add_query(&mut self, key: String, value: String);

    /// Add a query parameter
    fn query(mut self, key: impl Into<String>, value: impl ToString) -> Self {
        self.add_query(key.into(), value.to_string());
        self
    }

    /// Add optional query parameter (only if Some)
    fn query_opt(mut self, key: impl Into<String>, value: Option<impl ToString>) -> Self {
        if let Some(v) = value {
            self.add_query(key.into(), v.to_string());
        }
        self
    }

    /// Add multiple query parameters with the same key
    fn query_many<I, V>(self, key: impl Into<String>, values: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: ToString,
    {
        let key = key.into();
        let mut result = self;
        for value in values {
            result.add_query(key.clone(), value.to_string());
        }
        result
    }

    /// Add multiple optional query parameters with the same key
    fn query_many_opt<I, V>(self, key: impl Into<String>, values: Option<I>) -> Self
    where
        I: IntoIterator<Item = V>,
        V: ToString,
    {
        if let Some(values) = values {
            self.query_many(key, values)
        } else {
            self
        }
    }
}

/// Trait for error types that can be created from API responses
pub trait RequestError: From<ApiError> + std::fmt::Debug {
    /// Create error from HTTP response
    fn from_response(response: Response) -> impl std::future::Future<Output = Self> + Send;
}

/// Generic request builder for simple GET-only APIs (Gamma, Data)
pub struct Request<T, E> {
    pub(crate) client: Client,
    pub(crate) base_url: Url,
    pub(crate) path: String,
    pub(crate) query: Vec<(String, String)>,
    pub(crate) _marker: PhantomData<(T, E)>,
}

impl<T, E> Request<T, E> {
    /// Create a new request
    pub fn new(client: Client, base_url: Url, path: impl Into<String>) -> Self {
        Self {
            client,
            base_url,
            path: path.into(),
            query: Vec::new(),
            _marker: PhantomData,
        }
    }
}

impl<T, E> QueryBuilder for Request<T, E> {
    fn add_query(&mut self, key: String, value: String) {
        self.query.push((key, value));
    }
}

impl<T: DeserializeOwned, E: RequestError> Request<T, E> {
    /// Execute the request and deserialize response
    pub async fn send(self) -> Result<T, E> {
        let response = self.send_raw().await?;

        // Get text for debugging
        let text = response
            .text()
            .await
            .map_err(|e| E::from(ApiError::from(e)))?;

        // Deserialize and provide better error context
        serde_json::from_str(&text).map_err(|e| {
            tracing::error!("Deserialization failed: {}", e);
            tracing::error!("Failed to deserialize: {}", text);
            E::from(ApiError::from(e))
        })
    }

    /// Execute the request and return raw response
    pub async fn send_raw(self) -> Result<Response, E> {
        let url = self
            .base_url
            .join(&self.path)
            .map_err(|e| E::from(ApiError::from(e)))?;

        let mut request = self.client.get(url);

        if !self.query.is_empty() {
            request = request.query(&self.query);
        }

        let response = request
            .send()
            .await
            .map_err(|e| E::from(ApiError::from(e)))?;
        let status = response.status();

        tracing::debug!("Response status: {}", status);

        if !status.is_success() {
            let error = E::from_response(response).await;
            tracing::error!("Request failed: {:?}", error);
            return Err(error);
        }

        Ok(response)
    }
}

/// Type marker for deserializable responses
pub struct TypedRequest<T> {
    pub(crate) _marker: PhantomData<T>,
}

impl<T> TypedRequest<T> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T> Default for TypedRequest<T> {
    fn default() -> Self {
        Self::new()
    }
}
