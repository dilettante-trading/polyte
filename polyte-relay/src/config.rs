use alloy::primitives::Address;
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue};
use sha2::Sha256;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use base64::{Engine as _, engine::general_purpose::STANDARD};

pub struct ContractConfig {
    pub safe_factory: Address,
    pub safe_multisend: Address,
}

pub fn get_contract_config(chain_id: u64) -> Option<ContractConfig> {
    match chain_id {
        137 | 80002 => Some(ContractConfig {
            safe_factory: Address::from_str("0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b").unwrap(),
            safe_multisend: Address::from_str("0xA238CBeb142c10Ef7Ad8442C6D1f9E89e07e7761")
                .unwrap(),
        }),
        _ => None,
    }
}

pub struct BuilderConfig {
    pub key: String,
    pub secret: String,
    pub passphrase: Option<String>,
}

impl BuilderConfig {
    pub fn new(key: String, secret: String, passphrase: Option<String>) -> Self {
        Self {
            key,
            secret,
            passphrase,
        }
    }

    pub fn generate_headers(
        &self,
        method: &str,
        path: &str,
        body: Option<&str>,
    ) -> Result<HeaderMap, String> {
        let mut headers = HeaderMap::new();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        let body_str = body.unwrap_or("");
        let message = format!("{}{}{}{}", timestamp, method, path, body_str);

        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret.as_bytes())
            .map_err(|e| format!("Invalid secret: {}", e))?;
        mac.update(message.as_bytes());
        let result = mac.finalize();
        let signature = STANDARD.encode(result.into_bytes());

        headers.insert("POLY-API-KEY", HeaderValue::from_str(&self.key).unwrap());
        headers.insert(
            "POLY-TIMESTAMP",
            HeaderValue::from_str(&timestamp).unwrap(),
        );
        headers.insert(
            "POLY-SIGNATURE",
            HeaderValue::from_str(&signature).unwrap(),
        );

        if let Some(passphrase) = &self.passphrase {
            headers.insert(
                "POLY-PASSPHRASE",
                HeaderValue::from_str(passphrase).unwrap(),
            );
        }

        Ok(headers)
    }
}
