use alloy::{
    primitives::{keccak256, Address, U256},
    signers::Signer as AlloySigner,
    sol,
    sol_types::SolStruct,
};

use crate::{
    core::chain::Chain,
    error::ClobError,
    types::{Order as ClobOrder, SignatureType},
};

mod protocol {
    use super::*;
    sol! {
        #[derive(Debug, PartialEq, Eq)]
        struct EIP712Domain {
            string name;
            string version;
            uint256 chainId;
            address verifyingContract;
        }

        #[derive(Debug, PartialEq, Eq)]
        struct Order {
            uint256 salt;
            address maker;
            address signer;
            address taker;
            uint256 tokenId;
            uint256 makerAmount;
            uint256 takerAmount;
            uint256 expiration;
            uint256 nonce;
            uint256 feeRateBps;
            uint8 side;
            uint8 signatureType;
        }

        #[derive(Debug, PartialEq, Eq)]
        struct ClobAuth {
            string message;
        }
    }
}

/// Sign an order with EIP-712
pub async fn sign_order<S: AlloySigner>(
    order: &ClobOrder,
    signer: &S,
    chain_id: u64,
) -> Result<String, ClobError> {
    let chain = Chain::from_chain_id(chain_id)
        .ok_or_else(|| ClobError::Crypto(format!("Unsupported chain ID: {}", chain_id)))?;
    let contracts = chain.contracts();

    let verifying_contract = if order.neg_risk {
        contracts.neg_risk_exchange
    } else {
        contracts.exchange
    };

    // Create EIP-712 domain
    let domain = protocol::EIP712Domain {
        name: "Polymarket CTF Exchange".to_string(),
        version: "1".to_string(),
        chainId: U256::from(chain_id),
        verifyingContract: verifying_contract,
    };

    // Convert order to struct
    let order_struct = protocol::Order {
        salt: U256::from_str_radix(&order.salt, 10)
            .map_err(|e| ClobError::Crypto(format!("Invalid salt: {}", e)))?,
        maker: order.maker,
        signer: order.signer,
        taker: order.taker,
        tokenId: U256::from_str_radix(&order.token_id, 10)
            .map_err(|e| ClobError::Crypto(format!("Invalid token_id: {}", e)))?,
        makerAmount: U256::from_str_radix(&order.maker_amount, 10)
            .map_err(|e| ClobError::Crypto(format!("Invalid maker_amount: {}", e)))?,
        takerAmount: U256::from_str_radix(&order.taker_amount, 10)
            .map_err(|e| ClobError::Crypto(format!("Invalid taker_amount: {}", e)))?,
        expiration: U256::from_str_radix(&order.expiration, 10)
            .map_err(|e| ClobError::Crypto(format!("Invalid expiration: {}", e)))?,
        nonce: U256::from_str_radix(&order.nonce, 10)
            .map_err(|e| ClobError::Crypto(format!("Invalid nonce: {}", e)))?,
        feeRateBps: U256::from_str_radix(&order.fee_rate_bps, 10)
            .map_err(|e| ClobError::Crypto(format!("Invalid fee_rate_bps: {}", e)))?,
        side: match order.side {
            crate::types::OrderSide::Buy => 0,
            crate::types::OrderSide::Sell => 1,
        },
        signatureType: match order.signature_type {
            SignatureType::Eoa => 0,
            SignatureType::PolyProxy => 1,
            SignatureType::PolyGnosisSafe => 2,
        },
    };

    // Compute struct hash and domain separator (Alloy's eip712_hash_struct already performs keccak256)
    let struct_hash = order_struct.eip712_hash_struct();
    let domain_separator = domain.eip712_hash_struct();

    // Compute final hash
    let mut message = Vec::new();
    message.extend_from_slice(b"\x19\x01");
    message.extend_from_slice(domain_separator.as_slice());
    message.extend_from_slice(struct_hash.as_slice());
    let digest = keccak256(&message);

    // Sign the digest
    let signature = signer.sign_hash(&digest).await?;

    Ok(format!("0x{}", hex::encode(signature.as_bytes())))
}

/// Sign CLOB auth message for API key creation
pub async fn sign_clob_auth<S: AlloySigner>(
    signer: &S,
    chain_id: u64,
    timestamp: u64,
    nonce: u32,
) -> Result<String, ClobError> {
    let domain = protocol::EIP712Domain {
        name: "ClobAuthDomain".to_string(),
        version: "1".to_string(),
        chainId: U256::from(chain_id),
        verifyingContract: Address::ZERO,
    };

    let message = format!(
        "This message attests that I control the given wallet\ntimestamp: {}\nnonce: {}",
        timestamp, nonce
    );

    let clob_auth = protocol::ClobAuth { message };

    // Compute struct hash and domain separator
    let struct_hash = clob_auth.eip712_hash_struct();
    let domain_separator = domain.eip712_hash_struct();

    // Compute final hash
    let mut digest_message = Vec::new();
    digest_message.extend_from_slice(b"\x19\x01");
    digest_message.extend_from_slice(domain_separator.as_slice());
    digest_message.extend_from_slice(struct_hash.as_slice());
    let digest = keccak256(&digest_message);

    // Sign the digest
    let signature = signer.sign_hash(&digest).await?;

    Ok(format!("0x{}", hex::encode(signature.as_bytes())))
}
