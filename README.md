# axum-web-dev

A Rust workspace for building web services with [axum](https://github.com/tokio-rs/axum).

## Layout

```
axum-web-dev/
├── Cargo.toml            # workspace root, shared deps
└── crates/
    ├── server/           # binary — wires up the HTTP server
    ├── api/              # library — axum router & handlers
    └── domain/           # library — business logic & types
```

## Run

```sh
cargo run -p server
# then: curl http://localhost:3000/health
```

## Common commands

```sh
cargo build            # build all crates
cargo test             # run all tests
cargo clippy --all-targets --all-features
cargo fmt --all
```
