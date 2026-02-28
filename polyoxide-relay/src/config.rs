use alloy::primitives::Address;
use polyoxide_core::{current_timestamp, Base64Format, Signer};
use reqwest::header::{HeaderMap, HeaderValue};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct ContractConfig {
    pub safe_factory: Address,
    pub safe_multisend: Address,
    pub proxy_factory: Option<Address>,
    pub relay_hub: Option<Address>,
    pub rpc_url: &'static str,
}

pub fn get_contract_config(chain_id: u64) -> Option<ContractConfig> {
    match chain_id {
        137 => Some(ContractConfig {
            safe_factory: Address::from_str("0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b").unwrap(),
            safe_multisend: Address::from_str("0xA238CBeb142c10Ef7Ad8442C6D1f9E89e07e7761")
                .unwrap(),
            proxy_factory: Some(
                Address::from_str("0xaB45c5A4B0c941a2F231C04C3f49182e1A254052").unwrap(),
            ),
            relay_hub: Some(
                Address::from_str("0xD216153c06E857cD7f72665E0aF1d7D82172F494").unwrap(),
            ),
            rpc_url: "https://polygon.drpc.org",
        }),
        80002 => Some(ContractConfig {
            safe_factory: Address::from_str("0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b").unwrap(),
            safe_multisend: Address::from_str("0xA238CBeb142c10Ef7Ad8442C6D1f9E89e07e7761")
                .unwrap(),
            proxy_factory: None, // Proxy not supported on Amoy testnet
            relay_hub: None,
            rpc_url: "https://rpc-amoy.polygon.technology",
        }),
        _ => None,
    }
}

#[derive(Clone)]
pub struct BuilderConfig {
    pub key: String,
    pub secret: String,
    pub passphrase: Option<String>,
}

impl std::fmt::Debug for BuilderConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BuilderConfig")
            .field("key", &"[REDACTED]")
            .field("secret", &"[REDACTED]")
            .field("passphrase", &self.passphrase.as_ref().map(|_| "[REDACTED]"))
            .finish()
    }
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
        let timestamp = current_timestamp();

        // Create signer from raw string secret (Relay v1 uses raw secrets)
        let signer = Signer::from_raw(&self.secret);
        let message = Signer::create_message(timestamp, method, path, body);
        let signature = signer.sign(&message, Base64Format::Standard)?;

        headers.insert("POLY-API-KEY", HeaderValue::from_str(&self.key).unwrap());
        headers.insert(
            "POLY-TIMESTAMP",
            HeaderValue::from_str(&timestamp.to_string()).unwrap(),
        );
        headers.insert("POLY-SIGNATURE", HeaderValue::from_str(&signature).unwrap());

        if let Some(passphrase) = &self.passphrase {
            headers.insert(
                "POLY-PASSPHRASE",
                HeaderValue::from_str(passphrase).unwrap(),
            );
        }

        Ok(headers)
    }

    pub fn generate_relayer_v2_headers(
        &self,
        method: &str,
        path: &str,
        body: Option<&str>,
    ) -> Result<HeaderMap, String> {
        let mut headers = HeaderMap::new();
        let timestamp = current_timestamp();

        // Create signer from base64-encoded secret (Relay v2 uses base64 secrets)
        let signer = Signer::new(&self.secret);
        let message = Signer::create_message(timestamp, method, path, body);
        let signature = signer.sign(&message, Base64Format::UrlSafe)?;

        headers.insert(
            "POLY_BUILDER_API_KEY",
            HeaderValue::from_str(&self.key).unwrap(),
        );
        headers.insert(
            "POLY_BUILDER_TIMESTAMP",
            HeaderValue::from_str(&timestamp.to_string()).unwrap(),
        );
        headers.insert(
            "POLY_BUILDER_SIGNATURE",
            HeaderValue::from_str(&signature).unwrap(),
        );

        if let Some(passphrase) = &self.passphrase {
            headers.insert(
                "POLY_BUILDER_PASSPHRASE",
                HeaderValue::from_str(passphrase).unwrap(),
            );
        }

        Ok(headers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_config_debug_redacts_secrets() {
        let config = BuilderConfig::new(
            "my-api-key".to_string(),
            "my-secret".to_string(),
            Some("my-passphrase".to_string()),
        );
        let debug_output = format!("{:?}", config);

        assert!(debug_output.contains("[REDACTED]"));
        assert!(!debug_output.contains("my-api-key"), "Debug leaked API key: {}", debug_output);
        assert!(!debug_output.contains("my-secret"), "Debug leaked secret: {}", debug_output);
        assert!(
            !debug_output.contains("my-passphrase"),
            "Debug leaked passphrase: {}",
            debug_output
        );
    }

    #[test]
    fn test_builder_config_debug_without_passphrase() {
        let config = BuilderConfig::new("key".to_string(), "secret".to_string(), None);
        let debug_output = format!("{:?}", config);

        assert!(debug_output.contains("[REDACTED]"));
        assert!(debug_output.contains("passphrase: None"));
    }
}
