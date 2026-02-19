use base64::{engine::general_purpose::STANDARD, prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::error::ClobError;

/// HMAC signer for API authentication
#[derive(Clone, Debug)]
pub struct Signer {
    secret: Vec<u8>,
}

impl Signer {
    /// Create a new signer from base64-encoded secret (supports multiple formats)
    pub fn new(secret: &str) -> Result<Self, ClobError> {
        // Try different base64 formats:
        // 1. URL-safe without padding (most common for API keys)
        // 2. URL-safe with padding
        // 3. Standard base64
        // 4. Use raw bytes if all else fails
        let decoded = BASE64_URL_SAFE_NO_PAD
            .decode(secret)
            .or_else(|_| base64::engine::general_purpose::URL_SAFE.decode(secret))
            .or_else(|_| STANDARD.decode(secret))
            .unwrap_or_else(|_| secret.as_bytes().to_vec());

        Ok(Self { secret: decoded })
    }

    /// Sign a message with HMAC-SHA256
    pub fn sign(&self, message: &str) -> Result<String, ClobError> {
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.secret)
            .map_err(|e| ClobError::Crypto(format!("Failed to create HMAC: {}", e)))?;

        mac.update(message.as_bytes());
        let result = mac.finalize();
        let signature = STANDARD.encode(result.into_bytes());

        // Convert to URL-safe base64
        let signature = signature.replace('+', "-").replace('/', "_");

        Ok(signature)
    }

    /// Create signature message for API request
    pub fn create_message(timestamp: u64, method: &str, path: &str, body: Option<&str>) -> String {
        format!("{}{}{}{}", timestamp, method, path, body.unwrap_or(""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign() {
        // Test secret (base64)
        let secret = "c2VjcmV0"; // "secret" in base64
        let signer = Signer::new(secret).unwrap();

        let message = Signer::create_message(1234567890, "GET", "/api/test", None);
        let signature = signer.sign(&message).unwrap();

        // Signature should be URL-safe base64
        assert!(!signature.contains('+'));
        assert!(!signature.contains('/'));
    }
}
