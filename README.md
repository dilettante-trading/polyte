# Polyte

Rust client for Polymarket APIs.

More information about this crate can be found in the [crate documentation](https://docs.rs/polyte/).

> [!WARNING]
> This is currently work-in-progress so the API may change and some features may be missing

## Installation

```
cargo add polyte
```

Or install individual APIs:

```
# Market data only
cargo add polyte --no-default-features --features gamma

# Trading only
cargo add polyte --no-default-features --features clob
```

## Usage

```rust
use polyte::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = Credentials {
        key: "api_key".to_string(),
        secret: "secret".to_string(),
        passphrase: "passphrase".to_string(),
    };

    let client = Polymarket::builder("0x...", credentials)
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
