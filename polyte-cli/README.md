# polyte-cli

CLI tool for querying Polymarket APIs.

> [!NOTE]
> Only Gamma API is supported as of now

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

## Shell completions

For convenience, shell completions can be generated too

```
# Generate completions for Fish shell
polyte completions fish
```

## License

This project is licensed under the [MIT License](https://github.com/roushou/polyte/blob/main/LICENSE).
