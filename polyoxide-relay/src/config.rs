use alloy::primitives::Address;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE},
    Engine as _,
};
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue};
use sha2::Sha256;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

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

#[derive(Clone, Debug)]
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
        headers.insert("POLY-TIMESTAMP", HeaderValue::from_str(&timestamp).unwrap());
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
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        let body_str = body.unwrap_or("");
        // Signature logic: timestamp + method + path + body
        let message = format!("{}{}{}{}", timestamp, method, path, body_str);

        // Try URL-safe decode first, fallback to standard
        let secret_bytes = URL_SAFE
            .decode(&self.secret)
            .or_else(|_| STANDARD.decode(&self.secret))
            .map_err(|e| format!("Invalid base64 secret: {}", e))?;

        let mut mac = Hmac::<Sha256>::new_from_slice(&secret_bytes)
            .map_err(|e| format!("Invalid secret: {}", e))?;
        mac.update(message.as_bytes());
        let result = mac.finalize();
        // Use URL-safe encoding for signature (matching Python's urlsafe_b64encode)
        let signature = URL_SAFE.encode(result.into_bytes());

        headers.insert(
            "POLY_BUILDER_API_KEY",
            HeaderValue::from_str(&self.key).unwrap(),
        );
        headers.insert(
            "POLY_BUILDER_TIMESTAMP",
            HeaderValue::from_str(&timestamp).unwrap(),
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
