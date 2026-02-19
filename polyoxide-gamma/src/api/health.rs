use polyoxide_core::RequestError;
use reqwest::Client;
use std::time::{Duration, Instant};
use url::Url;

use crate::error::GammaError;

/// Health namespace for API health and latency operations
#[derive(Clone)]
pub struct Health {
    pub(crate) client: Client,
    pub(crate) base_url: Url,
}

impl Health {
    /// Measure the round-trip time (RTT) to the Polymarket Gamma API.
    ///
    /// Makes a GET request to the API root and returns the latency.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use polyoxide_gamma::Gamma;
    ///
    /// # async fn example() -> Result<(), polyoxide_gamma::GammaError> {
    /// let client = Gamma::new()?;
    /// let latency = client.health().ping().await?;
    /// println!("API latency: {}ms", latency.as_millis());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn ping(&self) -> Result<Duration, GammaError> {
        let start = Instant::now();
        let response = self.client.get(self.base_url.clone()).send().await?;
        let latency = start.elapsed();

        if !response.status().is_success() {
            return Err(GammaError::from_response(response).await);
        }

        Ok(latency)
    }
}
