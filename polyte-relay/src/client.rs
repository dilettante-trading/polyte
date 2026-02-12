use crate::account::BuilderAccount;
use crate::config::{get_contract_config, BuilderConfig, ContractConfig};
use crate::error::RelayError;
use crate::types::{
    NonceResponse, RelayerTransactionResponse, SafeTransaction, SafeTx, TransactionStatusResponse,
};
use alloy::hex;
use alloy::primitives::{keccak256, Address, Bytes, U256};
use alloy::signers::Signer;
use alloy::sol_types::{Eip712Domain, SolCall, SolStruct, SolValue};
use reqwest::Client;
use serde::Serialize;
use url::Url;

// Safe Init Code Hash from constants.py
const SAFE_INIT_CODE_HASH: &str =
    "2bce2127ff07fb632d16c8347c4ebf501f4841168bed00d9e6ef715ddb6fcecf";

#[derive(Clone)]
pub struct RelayClient {
    client: Client,
    base_url: Url,
    chain_id: u64,
    account: Option<BuilderAccount>,
    contract_config: ContractConfig,
}

impl RelayClient {
    /// Create a new Relay client with authentication
    pub fn new(
        base_url: &str,
        chain_id: u64,
        private_key: impl Into<String>,
        config: Option<BuilderConfig>,
    ) -> Result<Self, RelayError> {
        let account = BuilderAccount::new(private_key, config)?;
        Self::builder(base_url, chain_id)?
            .with_account(account)
            .build()
    }

    /// Create a new Relay client builder
    pub fn builder(base_url: &str, chain_id: u64) -> Result<RelayClientBuilder, RelayError> {
        RelayClientBuilder::new(base_url, chain_id)
    }

    /// Create a new Relay client from a BuilderAccount
    pub fn from_account(
        base_url: &str,
        chain_id: u64,
        account: BuilderAccount,
    ) -> Result<Self, RelayError> {
        Self::builder(base_url, chain_id)?.with_account(account).build()
    }

    pub fn address(&self) -> Option<Address> {
        self.account.as_ref().map(|a| a.address())
    }

    pub async fn get_nonce(&self, address: Address) -> Result<u64, RelayError> {
        let url = self
            .base_url
            .join("get-nonce")?
            .join(&format!("?address={}&type=SAFE", address))?;
        let resp = self
            .client
            .get(url)
            .send()
            .await?
            .json::<NonceResponse>()
            .await?;
        Ok(resp.nonce)
    }

    pub async fn get_transaction(
        &self,
        transaction_id: &str,
    ) -> Result<TransactionStatusResponse, RelayError> {
        let url = self
            .base_url
            .join("get-transaction")?
            .join(&format!("?id={}", transaction_id))?;
        let resp = self
            .client
            .get(url)
            .send()
            .await?
            .json::<TransactionStatusResponse>()
            .await?;
        Ok(resp)
    }

    pub async fn get_deployed(&self, safe_address: Address) -> Result<bool, RelayError> {
        #[derive(serde::Deserialize)]
        struct DeployedResponse {
            deployed: bool,
        }
        let url = self
            .base_url
            .join("get-deployed")?
            .join(&format!("?address={}", safe_address))?;
        let resp = self
            .client
            .get(url)
            .send()
            .await?
            .json::<DeployedResponse>()
            .await?;
        Ok(resp.deployed)
    }

    fn derive_safe_address(&self, owner: Address) -> Address {
        let salt = keccak256(owner.abi_encode());
        let init_code_hash = hex::decode(SAFE_INIT_CODE_HASH).unwrap();
        
        // CREATE2: keccak256(0xff ++ address ++ salt ++ keccak256(init_code))[12..]
        let mut input = Vec::new();
        input.push(0xff);
        input.extend_from_slice(self.contract_config.safe_factory.as_slice());
        input.extend_from_slice(salt.as_slice());
        input.extend_from_slice(&init_code_hash);
        
        let hash = keccak256(input);
        Address::from_slice(&hash[12..])
    }

    pub fn get_expected_safe(&self) -> Result<Address, RelayError> {
        let account = self.account.as_ref().ok_or(RelayError::MissingSigner)?;
        Ok(self.derive_safe_address(account.address()))
    }

    fn create_safe_multisend_transaction(&self, txns: Vec<SafeTransaction>) -> SafeTransaction {
        if txns.len() == 1 {
            return txns[0].clone();
        }

        let mut encoded_txns = Vec::new();
        for tx in txns {
            // Packed: [uint8 operation, address to, uint256 value, uint256 data_len, bytes data]
            let mut packed = Vec::new();
            packed.push(tx.operation);
            packed.extend_from_slice(tx.to.as_slice());
            packed.extend_from_slice(&tx.value.to_be_bytes::<32>());
            packed.extend_from_slice(&U256::from(tx.data.len()).to_be_bytes::<32>());
            packed.extend_from_slice(&tx.data);
            encoded_txns.extend_from_slice(&packed);
        }

        // encoded_txns now needs to be wrapped in multiSend(bytes)
        // selector: 8d80ff0a
        let mut data = hex::decode("8d80ff0a").unwrap();
        
        // Use alloy to encode `(bytes)` tuple.
        let multisend_data = (Bytes::from(encoded_txns),).abi_encode();
        data.extend_from_slice(&multisend_data);

        SafeTransaction {
            to: self.contract_config.safe_multisend,
            operation: 1, // DelegateCall
            data: data.into(),
            value: U256::ZERO,
        }
    }

    fn split_and_pack_sig(&self, sig: alloy::primitives::Signature) -> String {
        // Alloy's v() returns a boolean y_parity for EIP-1559/2930 and others.
        // False = 0 (27), True = 1 (28).
        let v = if sig.v() { 28 } else { 27 };

        // Pack r, s, v
        // abi.encodePacked(uint256 r, uint256 s, uint8 v)
        let mut packed = Vec::new();
        packed.extend_from_slice(&sig.r().to_be_bytes::<32>());
        packed.extend_from_slice(&sig.s().to_be_bytes::<32>());
        packed.push(v);
        
        format!("0x{}", hex::encode(packed))
    }

    pub async fn execute(
        &self,
        transactions: Vec<SafeTransaction>,
        metadata: Option<String>,
    ) -> Result<RelayerTransactionResponse, RelayError> {
        let account = self.account.as_ref().ok_or(RelayError::MissingSigner)?;
        let from_address = account.address();
        
        let safe_address = self.derive_safe_address(from_address);
        
        if !self.get_deployed(safe_address).await? {
            return Err(RelayError::Api(format!("Safe {} is not deployed", safe_address)));
        }

        let nonce = self.get_nonce(from_address).await?;

        let aggregated = self.create_safe_multisend_transaction(transactions);

        let safe_tx = SafeTx {
            to: aggregated.to,
            value: aggregated.value,
            data: aggregated.data,
            operation: aggregated.operation,
            safeTxGas: U256::ZERO,
            baseGas: U256::ZERO,
            gasPrice: U256::ZERO, // Assuming 0
            gasToken: Address::ZERO,
            refundReceiver: Address::ZERO,
            nonce: U256::from(nonce),
        };

        let domain = Eip712Domain {
            name: None,
            version: None,
            chain_id: Some(U256::from(self.chain_id)),
            verifying_contract: Some(safe_address),
            salt: None,
        };

        let struct_hash = safe_tx.eip712_signing_hash(&domain);
        let signature = account.signer().sign_hash(&struct_hash).await.map_err(|e| RelayError::Signer(e.to_string()))?;
        let packed_sig = self.split_and_pack_sig(signature);
        
        #[derive(Serialize)]
        struct SigParams {
            #[serde(rename = "gasPrice")]
            gas_price: String,
            operation: String,
            #[serde(rename = "safeTxGas")]
            safe_tx_gas: String,
            #[serde(rename = "baseGas")]
            base_gas: String,
            #[serde(rename = "gasToken")]
            gas_token: String,
            #[serde(rename = "refundReceiver")]
            refund_receiver: String,
        }
        
        #[derive(Serialize)]
        struct Body {
            #[serde(rename = "type")]
            type_: String,
            from: String,
            to: String,
            #[serde(rename = "proxyWallet")]
            proxy_wallet: String,
            data: String,
            signature: String,
            #[serde(rename = "signatureParams")]
            signature_params: SigParams,
            #[serde(rename = "value")]
            value: String,
            nonce: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            metadata: Option<String>,
        }

        let body = Body {
            type_: "SAFE".to_string(),
            from: from_address.to_string(),
            to: safe_tx.to.to_string(),
            proxy_wallet: safe_address.to_string(),
            data: safe_tx.data.to_string(),
            signature: packed_sig,
            signature_params: SigParams {
                gas_price: "0".to_string(),
                operation: safe_tx.operation.to_string(),
                safe_tx_gas: "0".to_string(),
                base_gas: "0".to_string(),
                gas_token: Address::ZERO.to_string(),
                refund_receiver: Address::ZERO.to_string(),
            },
            value: safe_tx.value.to_string(),
            nonce: nonce.to_string(),
            metadata,
        };

        self._post_request("submit-transaction", &body).await
    }

    pub async fn submit_gasless_redemption(
        &self,
        condition_id: [u8; 32],
        index_sets: Vec<alloy::primitives::U256>,
    ) -> Result<RelayerTransactionResponse, RelayError> {
        let account = self.account.as_ref().ok_or(RelayError::MissingSigner)?;
        let safe_address = self.derive_safe_address(account.address());

        // CTF Contract Interface
        alloy::sol! {
            function redeemPositions(address collateral, bytes32 parentCollectionId, bytes32 conditionId, uint256[] indexSets);
        }

        let collateral = Address::parse_checksummed("0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174", None).unwrap();
        let parent_collection_id = [0u8; 32]; // bytes32(0)

        // Encode calldata
        let call = redeemPositionsCall {
            collateral,
            parentCollectionId: parent_collection_id.into(),
            conditionId: condition_id.into(),
            indexSets: index_sets,
        };
        let data = call.abi_encode();
        let data_hex = format!("0x{}", hex::encode(data));

        // Construct Body
        #[derive(Serialize)]
        struct InnerTx {
            to: String,
            value: String,
            data: String,
            operation: u8,
        }

        #[derive(Serialize)]
        struct RedemptionBody {
            #[serde(rename = "chainId")]
            chain_id: u64,
            #[serde(rename = "safeAddress")]
            safe_address: String,
            transactions: Vec<InnerTx>,
        }

        let body = RedemptionBody {
            chain_id: 137,
            safe_address: safe_address.to_string(),
            transactions: vec![InnerTx {
                to: "0x4D97DCd97eC945f40cF65F87097ACe5EA0476045".to_string(), // CTF Exchange
                value: "0".to_string(),
                data: data_hex,
                operation: 0,
            }],
        };

        // Send Request
        // Note: Using hardcoded URL as per requirements
        let url = Url::parse("https://relayer-v2.polymarket.com/transactions").map_err(|e| RelayError::Api(e.to_string()))?;
        let body_str = serde_json::to_string(&body)?;

        let headers = if let Some(config) = account.config() {
            config
                .generate_relayer_v2_headers("POST", url.path(), Some(&body_str))
                .map_err(RelayError::Api)?
        } else {
            return Err(RelayError::Api(
                "Builder config missing - cannot authenticate request".to_string(),
            ));
        };

        let resp = self
            .client
            .post(url)
            .headers(headers)
            .body(body_str)
            .send()
            .await?;

        if !resp.status().is_success() {
            let text = resp.text().await?;
            return Err(RelayError::Api(format!("Request failed: {}", text)));
        }

        Ok(resp.json().await?)
    }

    async fn _post_request<T: Serialize>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<RelayerTransactionResponse, RelayError> {
        let url = self.base_url.join(endpoint)?;
        let body_str = serde_json::to_string(body)?;
        
        let headers = if let Some(account) = &self.account {
            if let Some(config) = account.config() {
                config.generate_headers("POST", url.path(), Some(&body_str))
                    .map_err(RelayError::Api)?
            } else {
                return Err(RelayError::Api("Builder config missing - cannot authenticate request".to_string()));
            }
        } else {
             return Err(RelayError::Api("Account missing - cannot authenticate request".to_string()));
        };

        let resp = self.client
            .post(url)
            .headers(headers)
            .body(body_str)
            .send()
            .await?;
            
        if !resp.status().is_success() {
             let text = resp.text().await?;
             return Err(RelayError::Api(format!("Request failed: {}", text)));
        }

        Ok(resp.json().await?)
    }
}

pub struct RelayClientBuilder {
    base_url: String,
    chain_id: u64,
    account: Option<BuilderAccount>,
}

impl RelayClientBuilder {
    pub fn new(base_url: &str, chain_id: u64) -> Result<Self, RelayError> {
        let mut base_url = Url::parse(base_url)?;
        if !base_url.path().ends_with('/') {
            base_url.set_path(&format!("{}/", base_url.path()));
        }
        
        Ok(Self {
            base_url: base_url.to_string(),
            chain_id,
            account: None,
        })
    }

    pub fn with_account(mut self, account: BuilderAccount) -> Self {
        self.account = Some(account);
        self
    }

    pub fn build(self) -> Result<RelayClient, RelayError> {
        let mut base_url = Url::parse(&self.base_url)?;
        if !base_url.path().ends_with('/') {
            base_url.set_path(&format!("{}/", base_url.path()));
        }

        let contract_config = get_contract_config(self.chain_id)
            .ok_or_else(|| RelayError::Api(format!("Unsupported chain ID: {}", self.chain_id)))?;

        Ok(RelayClient {
            client: Client::new(),
            base_url,
            chain_id: self.chain_id,
            account: self.account,
            contract_config,
        })
    }
}
