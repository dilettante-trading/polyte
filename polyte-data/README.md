# polyte-data

Rust client library for Polymarket Data API.

The Data API provides access to Polymarket's data endpoints, including user positions, trades, holders, open interest, and live volume.

More information about this crate can be found in the [crate documentation](https://docs.rs/polyte-data/).

## Installation

```
cargo add polyte-data
```

## Usage

### Basic Example

```rust
use polyte_data::DataApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = DataApi::new()?;

    // Get positions for a user
    let positions = data.user("0x1234567890123456789012345678901234567890")
        .list_positions()
        .limit(10)
        .send()
        .await?;

    for position in positions {
        println!("{}: size {}", position.title, position.size);
    }

    Ok(())
}
```

### Get User Traded Markets

```rust
let traded = data.user("0x...")
    .traded()
    .await?;

println!("Total markets traded: {}", traded.total_traded);
```

### Get Trades

```rust
let trades = data.trades()
    .list()
    .limit(10)
    .send()
    .await?;

for trade in trades {
    println!("Trade: {} @ {}", trade.size, trade.price);
}
```

### Get Holders

```rust
let holders = data.holders()
    .get("token_id_here")
    .send()
    .await?;

println!("Total holders: {}", holders.len());
```

### Get Open Interest

```rust
let open_interest = data.open_interest()
    .get("condition_id_here")
    .send()
    .await?;

println!("Open interest: {}", open_interest.open_interest);
```

### Get Live Volume

```rust
let volume = data.live_volume()
    .get("clob_token_id_here")
    .send()
    .await?;

println!("Live volume: {}", volume.volume);
```

## API Coverage

- **Users**: User positions and traded markets
- **Trades**: Trade history
- **Holders**: Token holder information
- **Open Interest**: Market open interest data
- **Live Volume**: Real-time trading volume
- **Health**: API health checks

## License

This project is licensed under the [MIT License](https://github.com/roushou/polyte/blob/main/LICENSE).
