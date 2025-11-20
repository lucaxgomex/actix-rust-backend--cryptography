# Rust Backend Project with Cryptography and Actix

This is a guide to setting up a simple API using **Actix Web** in a Rust project.

## Prerequisites

Before you begin, make sure you have the following installed:

- [Rust](https://www.rust-lang.org/): The programming language for the project.
- [Cargo](https://doc.rust-lang.org/cargo/): The Rust package manager and build tool.

## Dependencies
```toml
[dependencies]
actix-web = "4.0"
actix-rt = "2.5"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```
- `actix-web`: The core framework for building web applications with Actix.

- `actix-rt`: Actix runtime for async programming.

- `serde` and `serde_json`: Serialization and deserialization of data.

Run the following command to fetch the dependencies:
```bash
cargo build

```

## Run the API

To execute this project, it's recommended that you run the following command:

```rust
cargo watch -x run
```

## Test the API
Open your browser or use a tool like curl to test the API:
```bash
curl http://127.0.0.1:8080/
```