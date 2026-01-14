use alloy::{network::EthereumWallet, primitives::Address, signers::local::PrivateKeySigner};

use crate::error::ClobError;

/// Wallet wrapper for signing operations
#[derive(Clone, Debug)]
pub struct Wallet {
    signer: PrivateKeySigner,
    wallet: EthereumWallet,
}

impl Wallet {
    /// Create wallet from private key hex string
    pub fn from_private_key(private_key: &str) -> Result<Self, ClobError> {
        let signer = private_key
            .parse::<PrivateKeySigner>()
            .map_err(|e| ClobError::Crypto(format!("Failed to parse private key: {}", e)))?;
        let wallet = EthereumWallet::from(signer.clone());

        Ok(Self { signer, wallet })
    }

    /// Get the wallet address
    pub fn address(&self) -> Address {
        self.signer.address()
    }

    /// Get reference to the signer
    pub fn signer(&self) -> &PrivateKeySigner {
        &self.signer
    }

    /// Get reference to the Ethereum wallet
    pub fn ethereum_wallet(&self) -> &EthereumWallet {
        &self.wallet
    }
}
