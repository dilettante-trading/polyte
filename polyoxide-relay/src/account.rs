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
