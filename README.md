# io_error_serde
Provides [`serde`](https://serde.rs/) de/serialization for `std::io::Error` kinds.

## Example
```rust
use std::io;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Error(#[serde(with = "io_error_serde::ErrorKind")] io::ErrorKind);
```

## Use
In `Cargo.toml`
```toml
io_error_serde = { git = "https://github.com/syre-data/io_error_serde.git" }
```
