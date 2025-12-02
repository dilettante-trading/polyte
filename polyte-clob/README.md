# polyte-clob

Rust client library for Polymarket CLOB (Central Limit Order Book) API.

The CLOB API enables trading operations on Polymarket, including order creation, signing, posting, and account management.

More information about this crate can be found in the [crate documentation](https://docs.rs/polyte-clob/).

## Features

- **Account Management**: Check balances, allowances, and trade history
- **Order Management**: List and cancel orders
- **Market Data**: Get order books, prices, and market information

## Installation

```
cargo add polyte-clob
```

## Usage

### Setup

```rust
use polyte_clob::{Account, Chain, ClobBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load account from environment variables
    let account = Account::from_env()?;

    // Create CLOB client
    let clob = ClobBuilder::new(account)
        .chain(Chain::PolygonMainnet)
        .build()?;

    Ok(())
}
```

#### Account Configuration

The `Account` abstraction provides multiple ways to load credentials:

```rust
use polyte_clob::{Account, Credentials};

// Option 1: From environment variables
// Reads: POLYMARKET_PRIVATE_KEY, POLYMARKET_API_KEY,
//        POLYMARKET_API_SECRET, POLYMARKET_API_PASSPHRASE
let account = Account::from_env()?;

// Option 2: From a JSON file
let account = Account::from_file("config/account.json")?;

// Option 3: Direct construction
let credentials = Credentials {
    key: "api_key".to_string(),
    secret: "api_secret".to_string(),
    passphrase: "passphrase".to_string(),
};
let account = Account::new("0x...", credentials)?;
```

### Place an Order

```rust
use polyte_clob::{CreateOrderParams, OrderSide};

let params = CreateOrderParams {
    token_id: "token_id_here".to_string(),
    price: 0.52,
    size: 100.0,
    side: OrderSide::Buy,
    expiration: None,
};

// Single method to create, sign, and post order
let response = clob.place_order(&params).await?;

if response.success {
    println!("Order placed: {:?}", response.order_id);
} else {
    eprintln!("Order failed: {:?}", response.error_msg);
}
```

## License

This project is licensed under the [MIT License](https://github.com/roushou/polyte/blob/main/LICENSE).
