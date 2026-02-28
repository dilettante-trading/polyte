//! Live integration tests against the Polymarket Relay API.
//!
//! These tests hit the real API and require network access.
//! They are gated behind `#[ignore]` so they don't run in CI.
//!
//! Run manually with:
//! ```sh
//! cargo test -p polyoxide-relay --test live_api -- --ignored
//! ```

use alloy::primitives::Address;
use polyoxide_relay::RelayClient;
use std::time::Duration;

fn client() -> RelayClient {
    RelayClient::builder()
        .expect("default builder URL is valid")
        .build()
        .expect("relay client should build without account")
}

// ── Health ───────────────────────────────────────────────────────

#[tokio::test]
#[ignore]
async fn live_ping() {
    let client = client();
    let latency = client.ping().await.expect("ping should succeed");
    assert!(
        latency < Duration::from_secs(10),
        "latency too high: {:?}",
        latency
    );
}

// ── Deployed ────────────────────────────────────────────────────

#[tokio::test]
#[ignore]
async fn live_get_deployed_zero_address() {
    let client = client();
    let deployed = client
        .get_deployed(Address::ZERO)
        .await
        .expect("get_deployed should succeed for zero address");
    // The zero address is almost certainly not a deployed Safe
    assert!(!deployed, "zero address should not be deployed");
}

#[tokio::test]
#[ignore]
async fn live_get_deployed_known_address() {
    // Use the Safe factory address itself as a test -- it exists on-chain
    // but is not a deployed Safe wallet, so result should be false.
    let addr: Address = "0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b"
        .parse()
        .expect("valid address");
    let client = client();
    let deployed = client
        .get_deployed(addr)
        .await
        .expect("get_deployed should deserialize");
    // We just care that it returns a bool without error
    let _ = deployed;
}

// ── Nonce ───────────────────────────────────────────────────────

#[tokio::test]
#[ignore]
async fn live_get_nonce() {
    let client = client();
    // Query nonce for the zero address -- should succeed and return 0
    let nonce = client
        .get_nonce(Address::ZERO)
        .await
        .expect("get_nonce should succeed for zero address");
    assert_eq!(nonce, 0, "zero address should have nonce 0");
}
