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
