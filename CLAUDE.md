# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Polyte is a Rust SDK toolkit for Polymarket APIs. It provides library crates for interacting with Polymarket's trading and market data services, plus a standalone CLI.

## Build Commands

```bash
# Build all crates
cargo build

# Build specific crate
cargo build -p polyte-clob

# Run tests
cargo test

# Run tests for specific crate
cargo test -p polyte-gamma

# Run single test
cargo test test_name

# Run CLI
cargo run -p polyte-cli -- <command>

# Run example
cargo run -p polyte-gamma --example retrieve_markets

# Check formatting
cargo fmt --check

# Lint
cargo clippy
```

## Architecture

### Workspace Structure

The workspace contains 7 crates organized in layers:

**Unified Client** (`polyte/`): Re-exports all API clients through a single `Polymarket` struct with optional features (`clob`, `gamma`, `data`, `ws`). Entry point is `Polymarket::builder(account).chain(...).build()`.

**API Clients**:
- `polyte-clob/`: Trading API (CLOB) - order placement, signing (EIP-712), account management, WebSocket streaming
- `polyte-gamma/`: Market data API - markets, events, series, tags, sports
- `polyte-data/`: Data API - positions, trades, holders, open interest, volume, builder leaderboard
- `polyte-relay/`: Relayer API - gasless redemption, uses `alloy` for Ethereum interactions

**Shared**:
- `polyte-core/`: HTTP client building, error types, request utilities shared across clients
- `polyte-cli/`: CLI binary using clap, commands for gamma/data/ws

### Key Patterns

**Builder Pattern**: All clients use builders (`ClobBuilder`, `Gamma::builder()`, `DataApiBuilder`) for configuration.

**Account/Credentials**: `Account` holds wallet (private key) and API credentials. Load via:
- `Account::from_env()` - reads `POLYMARKET_PRIVATE_KEY`, `POLYMARKET_API_KEY`, `POLYMARKET_API_SECRET`, `POLYMARKET_API_PASSPHRASE`
- `Account::from_file(path)` - JSON config file
- Direct construction with `Credentials` struct

**Request Builders**: API methods return request builders with chainable methods (`.limit()`, `.open()`, etc.) finalized with `.send().await`.

**Chain Configuration**: `Chain::PolygonMainnet` or `Chain::PolygonMumbai` determines contract addresses and API endpoints.

### polyte-clob Internals

- `account/`: Wallet (alloy LocalSigner), Signer trait, Credentials
- `core/`: Chain config, EIP-712 typed data for order signing
- `api/`: REST endpoints (orders, markets, account)
- `ws/`: WebSocket client with market (public) and user (authenticated) channels
- `request.rs`: Authenticated request building with HMAC signing

### polyte-relay

Uses `alloy` crate for Ethereum/Polygon interactions. Provides gasless redemption via relayer v2 API with HMAC authentication headers.
