use crate::config::BuilderConfig;
use crate::error::RelayError;
use alloy::primitives::Address;
use alloy::signers::local::PrivateKeySigner;

#[derive(Clone, Debug)]
pub struct BuilderAccount {
    pub(crate) signer: PrivateKeySigner,
    pub(crate) config: Option<BuilderConfig>,
}

impl BuilderAccount {
    pub fn new(
        private_key: impl Into<String>,
        config: Option<BuilderConfig>,
    ) -> Result<Self, RelayError> {
        let signer = private_key
            .into()
            .parse::<PrivateKeySigner>()
            .map_err(|e| RelayError::Signer(format!("Failed to parse private key: {}", e)))?;

        Ok(Self { signer, config })
    }

    pub fn address(&self) -> Address {
        self.signer.address()
    }

    pub fn signer(&self) -> &PrivateKeySigner {
        &self.signer
    }

    pub fn config(&self) -> Option<&BuilderConfig> {
        self.config.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A well-known test private key (DO NOT use for real funds)
    // Address: 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 (anvil/hardhat default #0)
    const TEST_PRIVATE_KEY: &str =
        "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

    #[test]
    fn test_new_valid_private_key() {
        let account = BuilderAccount::new(TEST_PRIVATE_KEY, None);
        assert!(account.is_ok());
    }

    #[test]
    fn test_new_with_0x_prefix() {
        let key = format!("0x{}", TEST_PRIVATE_KEY);
        let account = BuilderAccount::new(key, None);
        // alloy accepts 0x-prefixed keys
        assert!(account.is_ok());
    }

    #[test]
    fn test_new_invalid_private_key() {
        let result = BuilderAccount::new("not_a_valid_key", None);
        assert!(result.is_err());
        let err = result.unwrap_err();
        match err {
            RelayError::Signer(msg) => {
                assert!(
                    msg.contains("Failed to parse private key"),
                    "unexpected: {msg}"
                );
            }
            other => panic!("Expected Signer error, got: {other:?}"),
        }
    }

    #[test]
    fn test_new_empty_key() {
        let result = BuilderAccount::new("", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_address_derivation_deterministic() {
        let a1 = BuilderAccount::new(TEST_PRIVATE_KEY, None).unwrap();
        let a2 = BuilderAccount::new(TEST_PRIVATE_KEY, None).unwrap();
        assert_eq!(a1.address(), a2.address());
    }

    #[test]
    fn test_address_matches_known_value() {
        // The first anvil/hardhat default account
        let account = BuilderAccount::new(TEST_PRIVATE_KEY, None).unwrap();
        let expected: Address = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
            .parse()
            .unwrap();
        assert_eq!(account.address(), expected);
    }

    #[test]
    fn test_config_none() {
        let account = BuilderAccount::new(TEST_PRIVATE_KEY, None).unwrap();
        assert!(account.config().is_none());
    }

    #[test]
    fn test_config_some() {
        let config = BuilderConfig::new("key".into(), "secret".into(), None);
        let account = BuilderAccount::new(TEST_PRIVATE_KEY, Some(config)).unwrap();
        assert!(account.config().is_some());
    }
}
