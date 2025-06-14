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

To install `ratefy`, clone the repository and build it using Cargo:

```bash
git clone https://github.com/yourusername/ratefy.git
cd ratefy
cargo build --release
