# Getting started

To install Njord into your Rust project we need to include the dependencies:

**Cargo.toml**
```toml
[dependencies]

# The core APIs, including the Table trait. Always
# required when using njord. The "derive" feature is only required when
# using #[derive(Table)] to make njord work with structs
# and enums defined in your crate.
njord = { version = "0.1.0", features = ["derive"] }
```