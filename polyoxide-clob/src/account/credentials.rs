use std::fmt;

use serde::{Deserialize, Serialize};

/// API credentials for L2 authentication
#[derive(Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub key: String,
    pub secret: String,
    pub passphrase: String,
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("key", &"<redacted>")
            .field("secret", &"<redacted>")
            .field("passphrase", &"<redacted>")
            .finish()
    }
}
