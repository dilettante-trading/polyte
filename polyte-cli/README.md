# polyte-cli

CLI tool for querying Polymarket APIs.

> [!NOTE]
> Only Gamma API is supported as of now

## Installation

```bash
cargo install polyte-cli
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

## License

This project is licensed under the [MIT License](https://github.com/roushou/polyte/blob/main/LICENSE).
