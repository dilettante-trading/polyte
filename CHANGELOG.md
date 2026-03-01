## [0.9.2] - 2026-03-01

### 🚀 Features

- *(core)* Parse `Retry-After` header for server-guided backoff delays
- *(core)* Expose `RetryConfig` through all high-level client builders (`Clob`, `Gamma`, `DataApi`, `RelayClient`)

### 🐛 Bug Fixes

- *(core)* Add segment-boundary-aware endpoint matching to prevent `/price` from matching `/prices-history`
- *(core)* Replace `SystemTime` nanos with `fastrand` for uniform backoff jitter
- *(clob)* Generate fresh L1 auth timestamp on each retry to avoid staleness
- *(relay)* Add retry loops with 429 handling to all relay endpoints

## [0.9.1] - 2026-02-28

### ⚙️ Miscellaneous Tasks

- Prune unused deps, tokio/alloy features, and fix TLS duplication
- Apply rustfmt formatting across workspace

### 📚 Documentation

- Add testing conventions and module organization to CLAUDE.md

### 🔧 CI

- Replace rust-cache with sccache for shared compilation caching

## [0.9.0] - 2026-02-28

### 🚀 Features

- *(core)* Add per-endpoint rate limiting with configurable quotas, retry-on-429 backoff with jitter, and governor-based throttling

### 🐛 Bug Fixes

- *(core)* Fix rate limit quota precision, backoff jitter range, and add missing endpoint quota
- *(core)* Carry message context in RateLimit error variant and downgrade retry log level
- *(core)* Redact secrets from Debug impls to prevent log leakage
- *(clob)* Use BUY/SELL strings for price endpoint side parameter
- *(clob)* Use typed request for `get_fee_rate` with correct field and token_id
- *(clob)* Fix tautological assertion in salt test
- *(clob)* Reject NaN and infinity in order parameter validation
- *(clob)* Classify service errors as Api instead of Validation
- *(clob)* Return None on insufficient liquidity and increase salt entropy
- *(data)* Route all HTTP calls through Request<T> for rate limiting and 429 retries
- *(data)* Align Display impls with serde SCREAMING_SNAKE_CASE for sort enums
- *(relay)* Replace unwraps with error propagation and compile-time address validation
- *(cli)* Replace `process::exit` with Result-based error handling in WS credentials and completions
- *(cli)* Reject invalid activity types with error instead of silently dropping

### 🚜 Refactor

- *(core)* Make `Signer::new` infallible

### 🧪 Tests

- *(core)* Add unit tests for Request query builder and typed request
- *(clob)* Add unit tests for EIP-712 signing, WS types, and auth credentials
- *(clob)* Add live integration tests for CLOB public endpoints
- *(data)* Add unit tests for enum serialization, builders, and type serde
- *(data)* Add live integration tests for data API public endpoints
- *(gamma)* Add unit tests for type deserialization and client builder
- *(relay)* Add unit tests for types serde, address derivation, signature packing, hex constants, contract config, and builder defaults
- *(cli)* Add unit tests for argument parsing across all subcommands
- Add live integration tests for all API endpoints

## [0.8.1] - 2026-02-26

### 🚜 Refactor

- *(core)* Remove verbose request/response body logging from HTTP clients
- *(clob)* Remove verbose request/response body logging from HTTP clients
- *(relay)* Remove verbose request/response body logging and leftover `eprintln!` debug statements

## [0.8.0] - 2026-02-25

### 🚀 Features

- Migrate price and size fields from String to Decimal with `serde(with = "rust_decimal::serde::str")` for accurate serialization

## [0.7.1] - 2026-02-24

### 🐛 Bug Fixes

- *(clob)* Add `canceled_order_id` and `message` fields to `CancelResponse` and mark `success` as default.

## [0.7.0] - 2026-02-24

### 🚀 Features

- *(relay)* Update builder to default to Polygon Mainnet (137) and relay V2 (`https://relayer-v2.polymarket.com/`)
- *(relay)* Update `RelayClientBuilder` to implement `Default`

## [0.6.1] - 2026-02-20

### 🚀 Features

- *(core)* Add unified authentication module with HMAC signing and timestamp generation
- *(core)* Add `Signer` struct supporting multiple base64 formats (URL-safe and standard)
- *(core)* Add `current_timestamp()` function for safe Unix timestamp generation
- *(core)* Add `Base64Format` enum to support both URL-safe and standard base64 encoding
- *(core)* Add `impl_api_error_conversions!` macro to reduce error conversion boilerplate

### 🚜 Refactor

- *(core)* Consolidate HMAC signing logic from CLOB and Relay into shared `Signer` implementation
- *(core)* Consolidate timestamp generation into single safe implementation
- *(clob)* Refactor `Signer` to use `polyoxide_core::Signer` as thin wrapper with CLOB-specific error handling
- *(clob)* Extract market metadata fetching into `get_market_metadata()` helper method
- *(clob)* Extract fee rate fetching into `get_fee_rate()` helper method
- *(clob)* Extract maker address resolution into `resolve_maker_address()` helper method
- *(clob)* Extract order building into `build_order()` helper method
- *(clob)* Simplify `create_order()` and `create_market_order()` by using extracted helpers (~140 lines removed)
- *(relay)* Update to use `polyoxide_core::Signer` and `current_timestamp()` for authentication
- *(gamma)* Use `impl_api_error_conversions!` macro to reduce error conversion boilerplate
- *(data)* Use `impl_api_error_conversions!` macro to reduce error conversion boilerplate

## [0.6.0] - 2026-02-19

### 🚀 Features

- *(relay)* Add gas estimation for redemption transactions with safety buffer and relayer overhead
- *(relay)* Add `estimate_redemption_gas` method to estimate gas costs using RPC provider simulation
- *(relay)* Add `submit_gasless_redemption_with_gas_estimation` method for redemptions with optional gas estimation
- *(relay)* Add default RPC URLs to contract configuration for Polygon mainnet and Amoy testnet
- *(repo)* Rename project from `polyte` to `polyoxide`

## [0.5.0] - 2026-02-19

### 🚀 Features

- *(clob)* Add health API namespace with ping method
- *(relay)* Introduce `polyte-relay` crate for interacting with relayer services
- *(relay)* Add gasless redemption functionality via relayer v2 API
- *(relay)* Introduce `BuilderAccount` for centralized signer and config management
- *(clob)* Introduce `MarketOrderArgs` and market order calculation utilities
- *(clob)* Enhance order creation logic with maker address determination and optional funder parameter
- *(clob)* Integrate polyte-gamma client into Clob and ClobBuilder
- *(clob)* Add `neg_risk` and `tick_size` methods to markets search
- *(clob)* Add `neg_risk` support for orders
- *(clob)* Implement funder and signature type support
- *(clob)* Add `get_by_token_ids` method to retrieve markets by token IDs
- *(clob)* Add `prices_history` method for historical token prices
- *(clob)* Add Display impl for OrderKind and SignatureType
- *(clob)* Introduce PartialCreateOrderOptions for enhanced order creation flexibility
- *(gamma)* Introduce Gamma User API
- *(gamma)* Add `volume_1yr` field to match Gamma API naming conventions
- *(data)* Add USDC balance endpoint to account API
- *(data)* Update `BalanceAllowanceResponse` to use HashMap for allowances
- *(polyte)* Add DataApi to unified Polymarket client
- *(types)* Add `is_proxy` method to `SignatureType` enum
- *(error)* Add service error creation method to ClobError

### 🐛 Bug Fixes

- *(clob)* Use precise decimal arithmetic and explicit TickSize parsing
- *(clob)* Update order amount calculations to support 6 decimal places
- *(clob)* Update owner field in order payload to use account address
- *(clob)* Add custom deserialization for minimum_tick_size to handle both string and number formats
- *(gamma)* Correct typos in Market and Event field names
- *(error)* Enhance API error logging by capturing raw response body
- *(tests)* Update salt generation test to check for non-empty output

### 🚜 Refactor

- *(clob)* Serialize OrderSide enum variants as 'BUY' and 'SELL' strings
- *(clob)* Update ClobBuilder to use optional account and introduce with_account method
- *(clob)* Restructure EIP-712 domain and order definitions into protocol module
- *(clob)* Implement custom serialization and deserialization for `SignatureType` enum
- *(gamma)* Rename `active` filter to `open` for market and series listing
- *(gamma)* Rename user proxy field to `proxyWallet` in API response
- *(gamma)* Rename `wallet_address` query parameter to `address` in public profile API
- *(core)* Add shared HTTP client infrastructure
- Remove Result type aliases in favor of explicit types
- Refactor amount calculations to use f64 arithmetic

### ⚙️ Miscellaneous Tasks

- Add CLAUDE.md with project guidance and architecture overview
- Update `thiserror` dependency to version 2.0.17
- Add `specta` support in multiple modules
- Add `dotenvy` dependency

## [0.4.0] - 2026-01-05

### 🐛 Bug Fixes

- *(clob)* Correct the type of the OrderBook timestamp

### ⚙️ Miscellaneous Tasks

- Add changelog and publish it on Github Releases page
## [cli-v0.3.2] - 2025-12-04

### 🐛 Bug Fixes

- *(cli)* Use limit flag instead of hardcorded value
- *(gamma)* Typo

### 🚜 Refactor

- *(cli)* Move duplicates into `common` module
- Use clap `value_parser` for comma-separated arguments

### ⚙️ Miscellaneous Tasks

- Format
- Remove unnecessary doc
## [cli-v0.3.1] - 2025-12-04

### 🚜 Refactor

- *(cli)* Improve credential error messages for `ws user` command
## [cli-v0.3.0] - 2025-12-03

### 🚀 Features

- *(clob)* Add websocket support
- *(cli)* Add support for Clob websockets

### 🚜 Refactor

- Consolidate auth into account module

### 📚 Documentation

- Update Clob documentation

### ⚙️ Miscellaneous Tasks

- Remove clob examples
## [cli-v0.2.4] - 2025-12-01

### 🐛 Bug Fixes

- Change `comment_count` type from u32 to i64 to prevent sentinel value issues

### 🚜 Refactor

- Extract common Request builder to `polyte-core`

### 📚 Documentation

- Update CLI README
- Update `polyte` README

### ⚙️ Miscellaneous Tasks

- Remove gamma examples
- Update Event type in Gamma
## [cli-v0.2.1] - 2025-12-01

### 🚀 Features

- Add support for Builders API

### 📚 Documentation

- Fix typo
## [cli-v0.2.0] - 2025-11-30

### 🚀 Features

- Add support for Data API

### 🚜 Refactor

- Remove deprecated code
- Reuse `SortOrder` enum
## [cli-v0.1.5] - 2025-11-28

### 🐛 Bug Fixes

- *(gamma)* Change `order_min_price_tick_size` and `order_min_size` to `f64`

### 🚜 Refactor

- *(cli)* Chain builder methods for request construction
## [cli-v0.1.4] - 2025-11-28

### 🚀 Features

- Bump versions
- Release cli-v0.1.4

### 🐛 Bug Fixes

- Clean-up types and make them more exhaustive
- Typo

### ⚙️ Miscellaneous Tasks

- *(cli)* Set default values to flags
- Enable retrieving a market by its slug
## [cli-v0.1.3] - 2025-11-28

### 🚀 Features

- Add cli commands presets and more flags

### 🐛 Bug Fixes

- Deserialize API responses into correct structs

### ⚙️ Miscellaneous Tasks

- Run `cargo fmt`
## [cli-v0.1.2] - 2025-11-27

### 🚀 Features

- *(cli)* Add command to display CLI version

### ⚙️ Miscellaneous Tasks

- Add more unit tests for utils
## [cli-v0.1.1] - 2025-11-27

### 🚀 Features

- Enable generating shell completions
## [cli-v0.1.0] - 2025-11-27

### 🚀 Features

- Add cli

### 📚 Documentation

- Add links to crates documentation
- Say it's wip in README

### ⚙️ Miscellaneous Tasks

- Make Polymarket client clonable
- Bump deps
- Bump `alloy` to latest and move it clob crate
- Add install script and workflow to release binaries on Github Releases
- Fix release workflow
