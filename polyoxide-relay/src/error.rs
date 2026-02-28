use thiserror::Error;

#[derive(Error, Debug)]
pub enum RelayError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Serde JSON error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Signer error: {0}")]
    Signer(String),

    #[error("Relayer API error: {0}")]
    Api(String),

    #[error("Rate limit exceeded")]
    RateLimit,

    #[error("Missing signer")]
    MissingSigner,

    #[error("Core API error: {0}")]
    Core(#[from] polyoxide_core::ApiError),
}
