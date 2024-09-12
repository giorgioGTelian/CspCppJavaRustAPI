# Setting Up a Basic API in Rust

## Prerequisites

To follow along, ensure you have Rust and Cargo installed. You can install them using the official Rustup installer.

## Creating a New Rust Project

Start by creating a new Rust project:

```bash
cargo new rust_api
cd rust_api
```

use the follogin command to run the project

```bash
cd rust_api
cargo run
```

## Adding Dependencies

actix-web = "4"
serde = { version = "1.0", features = ["derive"] }
anyhow = "1.0"
thiserror = "1.0"
log = "0.4"
env_logger = "0.9"
