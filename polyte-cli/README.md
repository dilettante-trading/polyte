# polyte-cli

CLI tool for querying Polymarket APIs.


## Installation

Install using cargo

```bash
cargo install polyte-cli
```

Or download binaries directly from Github releases

```
curl -fsSL https://raw.githubusercontent.com/roushou/polyte/main/scripts/install.sh | sh
```

## Usage

```bash
polyte <API> <COMMAND> [OPTIONS]
```

### Gamma API

Query market data from the Gamma API.

#### Markets

```bash
# List markets
polyte gamma markets list --limit 10 --active true

# Get a market by condition ID
polyte gamma markets get <CONDITION_ID>
```

Display all supported features

```
polyte gamma --help
```

### Data API

Query Data API.

#### Markets

```bash
# List user activity
polyte data positions --user 0x56687bf447db6ffa42ffe2204a05edaa20f55839 activity

# List builders leaderboard
polyte data builders leaderboard
```

Display all supported features

```
polyte data --help
```

### WebSocket

Subscribe to real-time market data and user updates.

```bash
# Subscribe to market channel (order book, prices)
polyte ws market <ASSET_ID>

# Only show trades, exit after 10 messages
polyte ws market --filter trade -n 10 <ASSET_ID>

# Run for 30 seconds with summary output
polyte ws market -t 30s -f summary <ASSET_ID>

# Subscribe to user channel (requires API credentials)
polyte ws user <MARKET_ID>
```

Display all supported features

```
polyte ws --help
```

## Shell completions

For convenience, shell completions can be generated too

```
# Generate completions for Fish shell
polyte completions fish
```

## License

This project is licensed under the [MIT License](https://github.com/roushou/polyte/blob/main/LICENSE).
