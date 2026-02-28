//! Live integration tests against the Polymarket Data API.
//!
//! These tests hit the real API and require network access.
//! They are gated behind `#[ignore]` so they don't run in CI.
//!
//! Run manually with:
//! ```sh
//! cargo test -p polyoxide-data --test live_api -- --ignored
//! ```

use polyoxide_data::DataApi;
use std::time::Duration;

fn client() -> DataApi {
    DataApi::new().expect("data api client")
}

// An address to test user endpoints (doesn't need to be active)
const TEST_USER: &str = "0x0000000000000000000000000000000000000001";

// ── Health ───────────────────────────────────────────────────────

#[tokio::test]
#[ignore]
async fn live_health_check() {
    let client = client();
    let health = client.health().check().await.expect("health check");
    assert_eq!(health.data, "OK", "health response should be OK");
}

#[tokio::test]
#[ignore]
async fn live_ping() {
    let client = client();
    let latency = client.health().ping().await.expect("ping");
    assert!(
        latency < Duration::from_secs(10),
        "latency too high: {:?}",
        latency
    );
}

// ── Open Interest ────────────────────────────────────────────────

#[tokio::test]
#[ignore]
async fn live_open_interest() {
    let client = client();
    let oi = client
        .open_interest()
        .get()
        .send()
        .await
        .expect("open interest");
    assert!(!oi.is_empty(), "should return at least one market's OI");
}

// ── Trades ───────────────────────────────────────────────────────

#[tokio::test]
#[ignore]
async fn live_list_trades() {
    let client = client();
    let trades = client
        .trades()
        .list()
        .limit(5)
        .send()
        .await
        .expect("list trades");
    assert!(!trades.is_empty(), "should return at least one trade");
}

// ── User endpoints ───────────────────────────────────────────────

#[tokio::test]
#[ignore]
async fn live_user_traded() {
    let client = client();
    // Verify the endpoint responds and deserializes correctly
    let traded = client
        .user(TEST_USER)
        .traded()
        .await
        .expect("user traded should deserialize");
    assert_eq!(traded.user, TEST_USER, "should echo back the user address");
}

#[tokio::test]
#[ignore]
async fn live_user_positions() {
    let client = client();
    // Just verify the endpoint responds and deserializes — user may have 0 open positions
    let _positions = client
        .user(TEST_USER)
        .list_positions()
        .limit(5)
        .send()
        .await
        .expect("list positions should succeed");
}

// ── Builders ─────────────────────────────────────────────────────

#[tokio::test]
#[ignore]
async fn live_builder_leaderboard() {
    let client = client();
    let leaderboard = client
        .builders()
        .leaderboard()
        .limit(5)
        .send()
        .await
        .expect("builder leaderboard");
    assert!(
        !leaderboard.is_empty(),
        "should return at least one builder"
    );
}
