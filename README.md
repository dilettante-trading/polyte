# Polyte

Rust SDK toolkit for Polymarket APIs. It includes library crates for use in your projects and a standalone CLI.

> [!WARNING]
> This is currently work-in-progress so the API may change and some features may be missing

## Crates

| Crate | Description |
|-------|-------------|
| [polyte](./polyte) | Unified client for Polymarket APIs (CLOB, Gamma, Data) |
| [polyte-cli](./polyte-cli) | CLI tool for querying Polymarket APIs |
| [polyte-clob](./polyte-clob) | Client library for Polymarket CLOB (order book) API |
| [polyte-core](./polyte-core) | Core utilities and shared types |
| [polyte-data](./polyte-data) | Client library for Polymarket Data API |
| [polyte-gamma](./polyte-gamma) | Client library for Polymarket Gamma (market data) API |

## Installation

### Libraries

```
cargo add polyte
```

Or install individual APIs:

```
# Market data only
cargo add polyte --no-default-features --features gamma

# Trading only
cargo add polyte --no-default-features --features clob

# Data API only
cargo add polyte --no-default-features --features data
```

### CLI

Install using cargo

```
cargo install polyte-cli
```

Or download binaries directly from Github releases

```
curl -fsSL https://raw.githubusercontent.com/roushou/polyte/main/scripts/install.sh | sh
```

See more information [here](./polyte-cli/README.md).

## Usage

```rust
use polyte::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load account from environment variables
    let account = Account::from_env()?;

    let client = Polymarket::builder(account)
        .chain(Chain::PolygonMainnet)
        .build()?;

    // Get markets
    let markets = client.gamma.markets().list().send().await?;

    // Get balance
    let balance = client.clob.balance_allowance().await?;

    Ok(())
}
```

## License

This project is licensed under the [MIT](./LICENSE) License.
