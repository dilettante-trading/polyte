use crate::account::BuilderAccount;
use crate::config::{get_contract_config, BuilderConfig, ContractConfig};
use crate::error::RelayError;
use crate::types::{
    NonceResponse, RelayerTransactionResponse, SafeTransaction, SafeTx, TransactionStatusResponse,
    WalletType,
};
use alloy::hex;
use alloy::network::TransactionBuilder;
use alloy::primitives::{keccak256, Address, Bytes, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::TransactionRequest;
use alloy::signers::Signer;
use alloy::sol_types::{Eip712Domain, SolCall, SolStruct, SolValue};
use reqwest::Client;
use serde::Serialize;
use std::time::{Duration, Instant};
use url::Url;

// Safe Init Code Hash from constants.py
const SAFE_INIT_CODE_HASH: &str =
    "2bce2127ff07fb632d16c8347c4ebf501f4841168bed00d9e6ef715ddb6fcecf";

// From Polymarket Relayer Client
const PROXY_INIT_CODE_HASH: &str =
    "0xd21df8dc65880a8606f09fe0ce3df9b8869287ab0b058be05aa9e8af6330a00b";

#[derive(Debug, Clone)]
pub struct RelayClient {
    client: Client,
    base_url: Url,
    chain_id: u64,
    account: Option<BuilderAccount>,
    contract_config: ContractConfig,
    wallet_type: WalletType,
}

impl RelayClient {
    /// Create a new Relay client with authentication
    pub fn new(
        private_key: impl Into<String>,
        config: Option<BuilderConfig>,
    ) -> Result<Self, RelayError> {
        let account = BuilderAccount::new(private_key, config)?;
        Self::builder()?.with_account(account).build()
    }

    /// Create a new Relay client builder
    pub fn builder() -> Result<RelayClientBuilder, RelayError> {
        RelayClientBuilder::new()
    }

    /// Create a new Relay client builder pulling settings from environment
    pub fn default_builder() -> Result<RelayClientBuilder, RelayError> {
        Ok(RelayClientBuilder::default())
    }

    /// Create a new Relay client from a BuilderAccount
    pub fn from_account(account: BuilderAccount) -> Result<Self, RelayError> {
        Self::builder()?.with_account(account).build()
    }

    pub fn address(&self) -> Option<Address> {
        self.account.as_ref().map(|a| a.address())
    }

    /// Measure the round-trip time (RTT) to the Relay API.
    ///
    /// Makes a GET request to the API base URL and returns the latency.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use polyoxide_relay::RelayClient;
    ///
    /// # async fn example() -> Result<(), polyoxide_relay::RelayError> {
    /// let client = RelayClient::builder()?.build()?;
    /// let latency = client.ping().await?;
    /// println!("API latency: {}ms", latency.as_millis());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn ping(&self) -> Result<Duration, RelayError> {
        let start = Instant::now();
        let response = self.client.get(self.base_url.clone()).send().await?;
        let latency = start.elapsed();

        if !response.status().is_success() {
            let text = response.text().await?;
            return Err(RelayError::Api(format!("Ping failed: {}", text)));
        }

        Ok(latency)
    }

    pub async fn get_nonce(&self, address: Address) -> Result<u64, RelayError> {
        let url = self.base_url.join(&format!(
            "nonce?address={}&type={}",
            address,
            self.wallet_type.as_str()
        ))?;
        let resp = self.client.get(url).send().await?;

        if !resp.status().is_success() {
            let text = resp.text().await?;
            return Err(RelayError::Api(format!("get_nonce failed: {}", text)));
        }

        let data = resp.json::<NonceResponse>().await?;
        Ok(data.nonce)
    }

    pub async fn get_transaction(
        &self,
        transaction_id: &str,
    ) -> Result<TransactionStatusResponse, RelayError> {
        let url = self
            .base_url
            .join(&format!("transaction?id={}", transaction_id))?;
        let resp = self.client.get(url).send().await?;

        if !resp.status().is_success() {
            let text = resp.text().await?;
            return Err(RelayError::Api(format!("get_transaction failed: {}", text)));
        }

        resp.json::<TransactionStatusResponse>()
            .await
            .map_err(Into::into)
    }

    pub async fn get_deployed(&self, safe_address: Address) -> Result<bool, RelayError> {
        #[derive(serde::Deserialize)]
        struct DeployedResponse {
            deployed: bool,
        }
        let url = self
            .base_url
            .join(&format!("deployed?address={}", safe_address))?;
        let resp = self.client.get(url).send().await?;

        if !resp.status().is_success() {
            let text = resp.text().await?;
            return Err(RelayError::Api(format!("get_deployed failed: {}", text)));
        }

        let data = resp.json::<DeployedResponse>().await?;
        Ok(data.deployed)
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

    fn derive_proxy_wallet(&self, owner: Address) -> Result<Address, RelayError> {
        let proxy_factory = self.contract_config.proxy_factory.ok_or_else(|| {
            RelayError::Api("Proxy wallet not supported on this chain".to_string())
        })?;

        // Salt = keccak256(encodePacked(["address"], [address]))
        // encodePacked for address uses the 20 bytes directly.
        let salt = keccak256(owner.as_slice());

        let init_code_hash = hex::decode(PROXY_INIT_CODE_HASH).unwrap();

        // CREATE2: keccak256(0xff ++ factory ++ salt ++ init_code_hash)[12..]
        let mut input = Vec::new();
        input.push(0xff);
        input.extend_from_slice(proxy_factory.as_slice());
        input.extend_from_slice(salt.as_slice());
        input.extend_from_slice(&init_code_hash);

        let hash = keccak256(input);
        Ok(Address::from_slice(&hash[12..]))
    }

    pub fn get_expected_proxy_wallet(&self) -> Result<Address, RelayError> {
        let account = self.account.as_ref().ok_or(RelayError::MissingSigner)?;
        self.derive_proxy_wallet(account.address())
    }

    /// Get relay payload for PROXY wallets (returns relay address and nonce)
    pub async fn get_relay_payload(&self, address: Address) -> Result<(Address, u64), RelayError> {
        #[derive(serde::Deserialize)]
        struct RelayPayload {
            address: String,
            #[serde(deserialize_with = "crate::types::deserialize_nonce")]
            nonce: u64,
        }

        let url = self
            .base_url
            .join(&format!("relay-payload?address={}&type=PROXY", address))?;
        let resp = self.client.get(url).send().await?;

        if !resp.status().is_success() {
            let text = resp.text().await?;
            return Err(RelayError::Api(format!(
                "get_relay_payload failed: {}",
                text
            )));
        }

        let data = resp.json::<RelayPayload>().await?;
        let relay_address: Address = data
            .address
            .parse()
            .map_err(|e| RelayError::Api(format!("Invalid relay address: {}", e)))?;
        Ok((relay_address, data.nonce))
    }

    /// Create the proxy struct hash for signing (EIP-712 style but with specific fields)
    #[allow(clippy::too_many_arguments)]
    fn create_proxy_struct_hash(
        &self,
        from: Address,
        to: Address,
        data: &[u8],
        tx_fee: U256,
        gas_price: U256,
        gas_limit: U256,
        nonce: u64,
        relay_hub: Address,
        relay: Address,
    ) -> [u8; 32] {
        let mut message = Vec::new();

        // "rlx:" prefix
        message.extend_from_slice(b"rlx:");
        // from address (20 bytes)
        message.extend_from_slice(from.as_slice());
        // to address (20 bytes) - This must be the ProxyFactory address
        message.extend_from_slice(to.as_slice());
        // data (raw bytes)
        message.extend_from_slice(data);
        // txFee as 32-byte big-endian
        message.extend_from_slice(&tx_fee.to_be_bytes::<32>());
        // gasPrice as 32-byte big-endian
        message.extend_from_slice(&gas_price.to_be_bytes::<32>());
        // gasLimit as 32-byte big-endian
        message.extend_from_slice(&gas_limit.to_be_bytes::<32>());
        // nonce as 32-byte big-endian
        message.extend_from_slice(&U256::from(nonce).to_be_bytes::<32>());
        // relayHub address (20 bytes)
        message.extend_from_slice(relay_hub.as_slice());
        // relay address (20 bytes)
        message.extend_from_slice(relay.as_slice());

        keccak256(&message).into()
    }

    /// Encode proxy transactions into calldata for the proxy wallet
    fn encode_proxy_transaction_data(&self, txns: &[SafeTransaction]) -> Vec<u8> {
        // ProxyTransaction struct: (uint8 typeCode, address to, uint256 value, bytes data)
        // Function selector for proxy(ProxyTransaction[])
        // IMPORTANT: Field order must match the ABI exactly!
        alloy::sol! {
            struct ProxyTransaction {
                uint8 typeCode;
                address to;
                uint256 value;
                bytes data;
            }
            function proxy(ProxyTransaction[] txns);
        }

        let proxy_txns: Vec<ProxyTransaction> = txns
            .iter()
            .map(|tx| {
                ProxyTransaction {
                    typeCode: 1, // 1 = Call (CallType.Call)
                    to: tx.to,
                    value: tx.value,
                    data: tx.data.clone(),
                }
            })
            .collect();

        // Encode the function call: proxy([ProxyTransaction, ...])
        let call = proxyCall { txns: proxy_txns };
        call.abi_encode()
    }

    fn create_safe_multisend_transaction(&self, txns: &[SafeTransaction]) -> SafeTransaction {
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

    fn split_and_pack_sig_safe(&self, sig: alloy::primitives::Signature) -> String {
        // Alloy's v() returns a boolean y_parity: false = 0, true = 1
        // For Safe signatures, v must be adjusted: 0/1 + 31 = 31/32
        let v_raw = if sig.v() { 1u8 } else { 0u8 };
        let v = v_raw + 31;

        // Pack r, s, v
        let mut packed = Vec::new();
        packed.extend_from_slice(&sig.r().to_be_bytes::<32>());
        packed.extend_from_slice(&sig.s().to_be_bytes::<32>());
        packed.push(v);

        format!("0x{}", hex::encode(packed))
    }

    fn split_and_pack_sig_proxy(&self, sig: alloy::primitives::Signature) -> String {
        // For Proxy signatures, use standard v value: 27 or 28
        let v = if sig.v() { 28u8 } else { 27u8 };

        // Pack r, s, v
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
        self.execute_with_gas(transactions, metadata, None).await
    }

    pub async fn execute_with_gas(
        &self,
        transactions: Vec<SafeTransaction>,
        metadata: Option<String>,
        gas_limit: Option<u64>,
    ) -> Result<RelayerTransactionResponse, RelayError> {
        match self.wallet_type {
            WalletType::Safe => self.execute_safe(transactions, metadata).await,
            WalletType::Proxy => self.execute_proxy(transactions, metadata, gas_limit).await,
        }
    }

    async fn execute_safe(
        &self,
        transactions: Vec<SafeTransaction>,
        metadata: Option<String>,
    ) -> Result<RelayerTransactionResponse, RelayError> {
        let account = self.account.as_ref().ok_or(RelayError::MissingSigner)?;
        let from_address = account.address();

        let safe_address = self.derive_safe_address(from_address);

        if !self.get_deployed(safe_address).await? {
            return Err(RelayError::Api(format!(
                "Safe {} is not deployed",
                safe_address
            )));
        }

        let nonce = self.get_nonce(from_address).await?;

        let aggregated = self.create_safe_multisend_transaction(&transactions);

        let safe_tx = SafeTx {
            to: aggregated.to,
            value: aggregated.value,
            data: aggregated.data,
            operation: aggregated.operation,
            safeTxGas: U256::ZERO,
            baseGas: U256::ZERO,
            gasPrice: U256::ZERO,
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
        let signature = account
            .signer()
            .sign_message(struct_hash.as_slice())
            .await
            .map_err(|e| RelayError::Signer(e.to_string()))?;
        let packed_sig = self.split_and_pack_sig_safe(signature);

        #[derive(Serialize)]
        struct SigParams {
            #[serde(rename = "gasPrice")]
            gas_price: String,
            operation: String,
            #[serde(rename = "safeTxnGas")]
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

        self._post_request("submit", &body).await
    }

    async fn execute_proxy(
        &self,
        transactions: Vec<SafeTransaction>,
        metadata: Option<String>,
        gas_limit: Option<u64>,
    ) -> Result<RelayerTransactionResponse, RelayError> {
        let account = self.account.as_ref().ok_or(RelayError::MissingSigner)?;
        let from_address = account.address();

        let proxy_wallet = self.derive_proxy_wallet(from_address)?;
        let relay_hub = self
            .contract_config
            .relay_hub
            .ok_or_else(|| RelayError::Api("Relay hub not configured".to_string()))?;
        let proxy_factory = self
            .contract_config
            .proxy_factory
            .ok_or_else(|| RelayError::Api("Proxy factory not configured".to_string()))?;

        // Get relay payload (relay address + nonce)
        let (relay_address, nonce) = self.get_relay_payload(from_address).await?;

        // Encode all transactions into proxy calldata
        let encoded_data = self.encode_proxy_transaction_data(&transactions);

        // Constants for proxy transactions
        let tx_fee = U256::ZERO;
        let gas_price = U256::ZERO;
        let gas_limit = U256::from(gas_limit.unwrap_or(10_000_000u64));

        // Create struct hash for signing
        // In original code, "to" was set to proxy_wallet I think? Or proxy_factory?
        // Let's use proxy_wallet as "to" for now (based on safe logic) but verify if it should be factory.
        // Actually, Python client says `const to = proxyWalletFactory`.
        // So we must use proxy_factory as "to".
        let struct_hash = self.create_proxy_struct_hash(
            from_address,
            proxy_factory, // CORRECTED: Use proxy_factory
            &encoded_data,
            tx_fee,
            gas_price,
            gas_limit,
            nonce,
            relay_hub,
            relay_address,
        );

        // Sign the struct hash with EIP191 prefix
        let signature = account
            .signer()
            .sign_message(&struct_hash)
            .await
            .map_err(|e| RelayError::Signer(e.to_string()))?;
        let packed_sig = self.split_and_pack_sig_proxy(signature);

        #[derive(Serialize)]
        struct SigParams {
            #[serde(rename = "relayerFee")]
            relayer_fee: String,
            #[serde(rename = "gasLimit")]
            gas_limit: String,
            #[serde(rename = "gasPrice")]
            gas_price: String,
            #[serde(rename = "relayHub")]
            relay_hub: String,
            relay: String,
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
            nonce: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            metadata: Option<String>,
        }

        let body = Body {
            type_: "PROXY".to_string(),
            from: from_address.to_string(),
            to: proxy_factory.to_string(),
            proxy_wallet: proxy_wallet.to_string(),
            data: format!("0x{}", hex::encode(&encoded_data)),
            signature: packed_sig,
            signature_params: SigParams {
                relayer_fee: "0".to_string(),
                gas_limit: gas_limit.to_string(),
                gas_price: "0".to_string(),
                relay_hub: relay_hub.to_string(),
                relay: relay_address.to_string(),
            },
            nonce: nonce.to_string(),
            metadata,
        };

        self._post_request("submit", &body).await
    }

    /// Estimate gas required for a redemption transaction.
    ///
    /// Returns the estimated gas limit with relayer overhead and safety buffer included.
    /// Uses the default RPC URL configured for the current chain.
    ///
    /// # Arguments
    ///
    /// * `condition_id` - The condition ID to redeem
    /// * `index_sets` - The index sets to redeem
    ///
    /// # Example
    ///
    /// ```no_run
    /// use polyoxide_relay::{RelayClient, BuilderAccount, BuilderConfig, WalletType};
    /// use alloy::primitives::{U256, hex};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let builder_config = BuilderConfig::new(
    ///     "key".to_string(),
    ///     "secret".to_string(),
    ///     None,
    /// );
    /// let account = BuilderAccount::new("0x...", Some(builder_config))?;
    /// let client = RelayClient::builder()?
    ///     .with_account(account)
    ///     .wallet_type(WalletType::Proxy)
    ///     .build()?;
    ///
    /// let condition_id = [0u8; 32];
    /// let index_sets = vec![U256::from(1)];
    /// let estimated_gas = client
    ///     .estimate_redemption_gas(condition_id, index_sets)
    ///     .await?;
    /// println!("Estimated gas: {}", estimated_gas);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn estimate_redemption_gas(
        &self,
        condition_id: [u8; 32],
        index_sets: Vec<U256>,
    ) -> Result<u64, RelayError> {
        // 1. Define the redemption interface
        alloy::sol! {
            function redeemPositions(address collateral, bytes32 parentCollectionId, bytes32 conditionId, uint256[] indexSets);
        }

        // 2. Setup constants
        let collateral =
            Address::parse_checksummed("0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174", None)
                .map_err(|e| RelayError::Api(format!("Invalid collateral address: {}", e)))?;
        let ctf_exchange =
            Address::parse_checksummed("0x4D97DCd97eC945f40cF65F87097ACe5EA0476045", None)
                .map_err(|e| RelayError::Api(format!("Invalid CTF exchange address: {}", e)))?;
        let parent_collection_id = [0u8; 32];

        // 3. Encode the redemption calldata
        let call = redeemPositionsCall {
            collateral,
            parentCollectionId: parent_collection_id.into(),
            conditionId: condition_id.into(),
            indexSets: index_sets,
        };
        let redemption_calldata = Bytes::from(call.abi_encode());

        // 4. Get the proxy wallet address
        let proxy_wallet = match self.wallet_type {
            WalletType::Proxy => self.get_expected_proxy_wallet()?,
            WalletType::Safe => self.get_expected_safe()?,
        };

        // 5. Create provider using the configured RPC URL
        let provider = ProviderBuilder::new().connect_http(
            self.contract_config
                .rpc_url
                .parse()
                .map_err(|e| RelayError::Api(format!("Invalid RPC URL: {}", e)))?,
        );

        // 6. Construct a mock transaction exactly as the proxy will execute it
        let tx = TransactionRequest::default()
            .with_from(proxy_wallet)
            .with_to(ctf_exchange)
            .with_input(redemption_calldata);

        // 7. Ask the Polygon node to simulate it and return the base computational cost
        let inner_gas_used = provider
            .estimate_gas(tx)
            .await
            .map_err(|e| RelayError::Api(format!("Gas estimation failed: {}", e)))?;

        // 8. Add relayer execution overhead + a 20% safety buffer
        let relayer_overhead: u64 = 50_000;
        let safe_gas_limit = (inner_gas_used + relayer_overhead) * 120 / 100;

        Ok(safe_gas_limit)
    }

    pub async fn submit_gasless_redemption(
        &self,
        condition_id: [u8; 32],
        index_sets: Vec<alloy::primitives::U256>,
    ) -> Result<RelayerTransactionResponse, RelayError> {
        self.submit_gasless_redemption_with_gas_estimation(condition_id, index_sets, false)
            .await
    }

    pub async fn submit_gasless_redemption_with_gas_estimation(
        &self,
        condition_id: [u8; 32],
        index_sets: Vec<alloy::primitives::U256>,
        estimate_gas: bool,
    ) -> Result<RelayerTransactionResponse, RelayError> {
        // 1. Define the specific interface for redemption
        alloy::sol! {
            function redeemPositions(address collateral, bytes32 parentCollectionId, bytes32 conditionId, uint256[] indexSets);
        }

        // 2. Setup Constants
        // USDC on Polygon
        let collateral =
            Address::parse_checksummed("0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174", None).unwrap();
        // CTF Exchange Address on Polygon
        let ctf_exchange =
            Address::parse_checksummed("0x4D97DCd97eC945f40cF65F87097ACe5EA0476045", None).unwrap();
        let parent_collection_id = [0u8; 32];

        // 3. Encode the Calldata
        let call = redeemPositionsCall {
            collateral,
            parentCollectionId: parent_collection_id.into(),
            conditionId: condition_id.into(),
            indexSets: index_sets.clone(),
        };
        let data = call.abi_encode();

        // 4. Estimate gas if requested
        let gas_limit = if estimate_gas {
            Some(
                self.estimate_redemption_gas(condition_id, index_sets.clone())
                    .await?,
            )
        } else {
            None
        };

        // 5. Construct the SafeTransaction
        let tx = SafeTransaction {
            to: ctf_exchange,
            value: U256::ZERO,
            data: data.into(),
            operation: 0, // 0 = Call (Not DelegateCall)
        };

        // 6. Use the execute_with_gas method
        // This handles Nonce fetching, EIP-712 Signing, and Relayer submission.
        self.execute_with_gas(vec![tx], None, gas_limit).await
    }

    async fn _post_request<T: Serialize>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<RelayerTransactionResponse, RelayError> {
        let url = self.base_url.join(endpoint)?;
        let body_str = serde_json::to_string(body)?;

        let mut headers = if let Some(account) = &self.account {
            if let Some(config) = account.config() {
                config
                    .generate_relayer_v2_headers("POST", url.path(), Some(&body_str))
                    .map_err(RelayError::Api)?
            } else {
                return Err(RelayError::Api(
                    "Builder config missing - cannot authenticate request".to_string(),
                ));
            }
        } else {
            return Err(RelayError::Api(
                "Account missing - cannot authenticate request".to_string(),
            ));
        };

        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let resp = self
            .client
            .post(url.clone())
            .headers(headers)
            .body(body_str.clone())
            .send()
            .await?;

        let status = resp.status();
        tracing::debug!("Response status for {}: {}", endpoint, status);

        if !status.is_success() {
            let text = resp.text().await?;
            tracing::error!(
                "Request to {} failed with status {}: {}",
                endpoint,
                status,
                text
            );
            return Err(RelayError::Api(format!("Request failed: {}", text)));
        }

        let response_text = resp.text().await?;

        // Try to deserialize
        serde_json::from_str(&response_text).map_err(|e| {
            tracing::error!(
                "Failed to decode response from {}: {}. Raw body: {}",
                endpoint,
                e,
                response_text
            );
            RelayError::SerdeJson(e)
        })
    }
}

pub struct RelayClientBuilder {
    base_url: String,
    chain_id: u64,
    account: Option<BuilderAccount>,
    wallet_type: WalletType,
}

impl Default for RelayClientBuilder {
    fn default() -> Self {
        let relayer_url = std::env::var("RELAYER_URL")
            .unwrap_or_else(|_| "https://relayer-v2.polymarket.com/".to_string());
        let chain_id = std::env::var("CHAIN_ID")
            .unwrap_or("137".to_string())
            .parse::<u64>()
            .unwrap_or(137);

        Self::new()
            .unwrap()
            .url(&relayer_url)
            .unwrap()
            .chain_id(chain_id)
    }
}

impl RelayClientBuilder {
    pub fn new() -> Result<Self, RelayError> {
        let mut base_url = Url::parse("https://relayer-v2.polymarket.com")?;
        if !base_url.path().ends_with('/') {
            base_url.set_path(&format!("{}/", base_url.path()));
        }

        Ok(Self {
            base_url: base_url.to_string(),
            chain_id: 137,
            account: None,
            wallet_type: WalletType::default(),
        })
    }

    pub fn chain_id(mut self, chain_id: u64) -> Self {
        self.chain_id = chain_id;
        self
    }

    pub fn url(mut self, url: &str) -> Result<Self, RelayError> {
        let mut base_url = Url::parse(url)?;
        if !base_url.path().ends_with('/') {
            base_url.set_path(&format!("{}/", base_url.path()));
        }
        self.base_url = base_url.to_string();
        Ok(self)
    }

    pub fn with_account(mut self, account: BuilderAccount) -> Self {
        self.account = Some(account);
        self
    }

    pub fn wallet_type(mut self, wallet_type: WalletType) -> Self {
        self.wallet_type = wallet_type;
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
            wallet_type: self.wallet_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ping() {
        let client = RelayClient::builder().unwrap().build().unwrap();
        let result = client.ping().await;
        assert!(result.is_ok(), "ping failed: {:?}", result.err());
    }
}
