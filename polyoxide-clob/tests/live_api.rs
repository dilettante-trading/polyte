//! Live integration tests against the Polymarket CLOB API.
//!
//! These tests hit the real API and require network access.
//! They are gated behind `#[ignore]` so they don't run in CI.
//!
//! Run manually with:
//! ```sh
//! cargo test -p polyoxide-clob --test live_api -- --ignored
//! ```

use polyoxide_clob::{Clob, OrderSide};
use polyoxide_core::QueryBuilder;
use polyoxide_gamma::Gamma;
use std::time::Duration;

fn public_client() -> Clob {
    Clob::public()
}

/// Find a token_id with an active order book using Gamma.
///
/// The CLOB `/markets` listing returns mostly resolved markets. Gamma's
/// `closed=false` filter reliably returns markets with live order books.
async fn find_active_token_id() -> String {
    let gamma = Gamma::builder().build().expect("gamma client");
    let markets = gamma
        .markets()
        .list()
        .closed(false)
        .send()
        .await
        .expect("gamma list markets");

    markets
        .iter()
        .find_map(|m| {
            // clob_token_ids is a JSON-encoded array string: '["id1", "id2"]'
            m.clob_token_ids.as_ref().and_then(|ids| {
                serde_json::from_str::<Vec<String>>(ids)
                    .ok()
                    .and_then(|v| v.into_iter().next())
            })
        })
        .expect("should find at least one active market with a token_id via Gamma")
}

// ── Health ───────────────────────────────────────────────────────

#[tokio::test]
#[ignore]
async fn live_ping() {
    let client = public_client();
    let latency = client.health().ping().await.expect("ping should succeed");
    assert!(
        latency < Duration::from_secs(10),
        "latency too high: {:?}",
        latency
    );
}

// ── Markets ──────────────────────────────────────────────────────

#[tokio::test]
#[ignore]
async fn live_list_markets() {
    let client = public_client();
    let resp = client.markets().list().send().await.expect("list markets");
    assert!(!resp.data.is_empty(), "should return at least one market");
}

#[tokio::test]
#[ignore]
async fn live_fee_rate() {
    let token_id = find_active_token_id().await;
    let client = public_client();

    let resp = client
        .markets()
        .fee_rate(&token_id)
        .send()
        .await
        .expect("fee_rate should deserialize");

    assert!(
        resp.base_fee <= 10_000,
        "fee rate {} bps seems unreasonably high",
        resp.base_fee
    );
}

#[tokio::test]
#[ignore]
async fn live_midpoint() {
    let token_id = find_active_token_id().await;
    let client = public_client();

    let resp = client
        .markets()
        .midpoint(&token_id)
        .send()
        .await
        .expect("midpoint should succeed");

    let mid: f64 = resp.mid.parse().expect("mid should be a number");
    assert!(
        (0.0..=1.0).contains(&mid),
        "midpoint {mid} should be between 0 and 1"
    );
}

#[tokio::test]
#[ignore]
async fn live_order_book() {
    let token_id = find_active_token_id().await;
    let client = public_client();

    let book = client
        .markets()
        .order_book(&token_id)
        .send()
        .await
        .expect("order book should succeed");

    assert!(
        !book.bids.is_empty() || !book.asks.is_empty(),
        "order book should have at least some levels"
    );
}

#[tokio::test]
#[ignore]
async fn live_price() {
    let token_id = find_active_token_id().await;
    let client = public_client();

    let resp = client
        .markets()
        .price(&token_id, OrderSide::Buy)
        .send()
        .await
        .expect("price should succeed");

    let price: f64 = resp.price.parse().expect("price should be a number");
    assert!(
        (0.0..=1.0).contains(&price),
        "price {price} should be between 0 and 1"
    );
}

#[tokio::test]
#[ignore]
async fn live_prices_history() {
    let token_id = find_active_token_id().await;
    let client = public_client();

    let resp = client
        .markets()
        .prices_history(&token_id)
        .query("interval", "max")
        .send()
        .await
        .expect("prices_history should succeed");

    assert!(
        !resp.history.is_empty(),
        "prices history should be non-empty"
    );
}

#[tokio::test]
#[ignore]
async fn live_neg_risk() {
    let token_id = find_active_token_id().await;
    let client = public_client();

    let _resp = client
        .markets()
        .neg_risk(&token_id)
        .send()
        .await
        .expect("neg_risk should deserialize");
}

#[tokio::test]
#[ignore]
async fn live_tick_size() {
    let token_id = find_active_token_id().await;
    let client = public_client();

    let resp = client
        .markets()
        .tick_size(&token_id)
        .send()
        .await
        .expect("tick_size should succeed");

    let tick: f64 = resp
        .minimum_tick_size
        .parse()
        .expect("minimum_tick_size should be a number");
    assert!(tick > 0.0, "tick size {tick} should be positive");
}

#[tokio::test]
#[ignore]
async fn live_get_market() {
    let client = public_client();

    // Get a condition_id from the market list
    let list = client
        .markets()
        .list()
        .send()
        .await
        .expect("list markets");
    let condition_id = &list
        .data
        .first()
        .expect("should have at least one market")
        .condition_id;

    let market = client
        .markets()
        .get(condition_id)
        .send()
        .await
        .expect("get market should succeed");

    assert_eq!(
        &market.condition_id, condition_id,
        "returned market should match requested condition_id"
    );
}

#[tokio::test]
#[ignore]
async fn live_get_markets_by_token_ids() {
    let token_id = find_active_token_id().await;
    let client = public_client();

    let resp = client
        .markets()
        .get_by_token_ids(vec![token_id.clone()])
        .send()
        .await
        .expect("get_by_token_ids should succeed");

    assert!(
        !resp.data.is_empty(),
        "should return at least one market for the given token_id"
    );
}
