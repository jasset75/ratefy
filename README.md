# Ratefy

**Ratefy** is a modular, terminal-based application for interacting with and analyzing financial, economic, and exchange rate data. Designed with extensibility and separation of concerns in mind, it uses a layered architecture with reusable libraries.

## Project Structure

This project is organized as a Rust workspace with multiple crates:

- [`ratefy-cli`](./crates/ratefy-cli): The main command-line interface.
- [`ratefy-lib`](./crates/ratefy-lib): The core business logic and data handling engine.
- [`ratefy-menu`](./crates/ratefy-menu): An interactive menu engine built with Ratatui for terminal-based UIs.

## Features

- 💱 Modular exchange rate analysis
- 📊 CLI-based dashboards with Ratatui
- ⚙️ Clean separation between UI, logic, and storage
- 🧪 Tested and type-safe Rust architecture

## Installation

If you don't have Rust installed, you can install it along with other tools using [mise](#toolchain-management-with-mise).

To install `ratefy`, clone the repository and build it using Cargo:

```bash
git clone https://github.com/jasset75/ratefy.git
cd ratefy
cargo build --release
```

## Git Hooks (Lefthook)

This project uses [Lefthook](https://github.com/evilmartians/lefthook) to run automated checks before every commit.  
The following commands are currently enforced:

- `cargo fmt -- --check`: Ensures the code is properly formatted.
- `cargo clippy -- -D warnings`: Runs the linter and treats all warnings as errors.
- `cargo test`: Runs the full test suite.

## Toolchain Management with mise

We use [`mise`](https://mise.jdx.dev/) to manage project tools like Rust and Lefthook in a reproducible way.

### Installing `mise`

If you don't have `mise` installed yet:

#### macOS / Linux

```bash
curl https://mise.jdx.dev/install.sh | bash
```

Then restart your shell or add the following to your shell config file:

```bash
eval "$(~/.local/bin/mise activate bash)"
```

For updated details you can check https://mise.jdx.dev/installing-mise.html

---

### Setting up the environment

After cloning the repository:

```bash
mise install
```

This will automatically install:

- The latest stable version of Rust
- The latest version of Lefthook
- Git hooks via `lefthook install` (if configured in `.mise.toml`)

---

### Manual Lefthook Installation

If you're not using `mise`, you can install Lefthook manually by following the official instructions:

👉 https://lefthook.dev/installation/index.html

---

### Running the hooks manually

To run the pre-commit hooks on all files:

```bash
lefthook run pre-commit
```

> Make sure Rust tools like `rustfmt` and `clippy` are installed using:

```bash
rustup component add rustfmt clippy
```
