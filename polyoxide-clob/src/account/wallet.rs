use alloy::{network::EthereumWallet, primitives::Address, signers::local::PrivateKeySigner};

use crate::error::ClobError;

/// Wallet wrapper for signing operations
#[derive(Clone)]
pub struct Wallet {
    signer: PrivateKeySigner,
    wallet: EthereumWallet,
}

impl std::fmt::Debug for Wallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Wallet")
            .field("address", &self.signer.address())
            .finish()
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    // Well-known test private key (DO NOT use in production)
    const TEST_KEY: &str = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

    #[test]
    fn test_wallet_debug_shows_address_not_key() {
        let wallet = Wallet::from_private_key(TEST_KEY).unwrap();
        let debug_output = format!("{:?}", wallet);

        // Should contain "address" field
        assert!(
            debug_output.contains("address"),
            "Debug should show address: {}",
            debug_output
        );
        // Should NOT contain the private key material
        assert!(
            !debug_output
                .contains("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"),
            "Debug should NOT contain private key: {}",
            debug_output
        );
    }
}
