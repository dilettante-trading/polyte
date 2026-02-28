use polyoxide_core::HttpClient;
use std::time::{Duration, Instant};

use crate::error::ClobError;

/// Health namespace for API health and latency operations
#[derive(Clone)]
pub struct Health {
    pub(crate) http_client: HttpClient,
}

impl Health {
    /// Measure the round-trip time (RTT) to the Polymarket CLOB API.
    ///
    /// Makes a GET request to the API root and returns the latency.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use polyoxide_clob::Clob;
    ///
    /// # async fn example() -> Result<(), polyoxide_clob::ClobError> {
    /// let client = Clob::public();
    /// let latency = client.health().ping().await?;
    /// println!("API latency: {}ms", latency.as_millis());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn ping(&self) -> Result<Duration, ClobError> {
        let start = Instant::now();
        let response = self
            .http_client
            .client
            .get(self.http_client.base_url.clone())
            .send()
            .await?;
        let latency = start.elapsed();

        if !response.status().is_success() {
            return Err(ClobError::from_response(response).await);
        }

        Ok(latency)
    }
}
