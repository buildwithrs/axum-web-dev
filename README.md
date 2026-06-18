# axum-web-dev

A Rust workspace of small, focused examples that explore the building blocks of
the [axum](https://github.com/tokio-rs/axum) web framework. Each crate is a
self-contained binary that demonstrates one concept (routing, state, errors,
TLS, …) and can be run on its own.

## Workspace layout

```
axum-web-dev/
├── Cargo.toml                 # workspace root, shared deps
├── certs/                     # local TLS cert/key (used by tls-support)
├── docs/                      # design notes / supporting docs
└── crates/
    ├── server/                # minimal HTTP server binary (root example)
    ├── api/                   # router & handler library used by `server`
    ├── domain/                # shared types/constants (e.g. version string)
    ├── share-state/           # Axum `State` extractor + shared `AppState`
    ├── request-extractor/     # Path / Query / Json extractors
    ├── middleware/            # custom `tower::Layer` + `from_fn` middleware
    ├── error-handling/        # typed `AppError` + `IntoResponse` mapping
    ├── route-group/           # grouping routes with `Router::nest`
    ├── handler-test/          # route-group handlers + `#[tokio::test]` cases
    ├── tracing-logging/       # `tracing`/`tracing-subscriber` + request-id mw
    ├── websocket/             # WebSocket upgrade + echo handler
    ├── file-upload/           # multipart upload to disk
    └── tls-support/           # HTTPS via `axum-server` + rustls
```

## Modules at a glance

| Crate | What it demonstrates |
| --- | --- |
| `server` | Minimal binary that wires up `api::router()`, applies `TraceLayer`, binds a TCP listener and serves via `axum::serve`. |
| `api` | Tiny library that exposes a `Router` with `/` and `/health`, returning a JSON status that pulls the version from `domain`. |
| `domain` | Holds shared business constants/types (currently `VERSION`, exposed via `CARGO_PKG_VERSION`). |
| `share-state` | Defines an `AppState` wrapping a `redis::Client` and a `RwLock<HashMap>` cache; `/index` reads a key from Redis and renders HTML. |
| `request-extractor` | Shows the three core extractors: `Path<T>` for path params, `Query<T>` for query strings, and `Json<T>` for request bodies. |
| `middleware` | Two flavours of middleware: an `axum::middleware::from_fn` logger and a hand-written `tower::Layer`/`Service` (`MyLayer`) that wraps every request. |
| `error-handling` | A typed `AppError` enum (built with `thiserror`) that implements `IntoResponse`, mapping domain errors to HTTP status codes (e.g. `400`/`404`). |
| `route-group` | Uses `Router::nest` to mount separate user and product sub-routers under `/api/u` and `/api/p`, with shared error handling. |
| `handler-test` | Same routed app as `route-group`, plus a `#[cfg(test)]` module that drives the router with `tower::ServiceExt::oneshot` for end-to-end handler tests. |
| `tracing-logging` | Initialises `tracing-subscriber`, adds a request-id middleware that propagates an `x-request-id` header, and layers `TraceLayer::new_for_http` for structured request logs. |
| `websocket` | Upgrades an HTTP request to a WebSocket at `/ws` and echoes every received text frame back to the client, with `tracing`-based logs. |
| `file-upload` | Accepts `multipart/form-data` at `/upload`, streams each part to `/tmp` with a 2 MB body limit and converts IO/multipart errors into HTTP responses. |
| `tls-support` | Serves an HTTPS endpoint with `axum-server` and rustls, loading the PEM cert/key from the repo's `certs/` directory. |

## Run

Most crates are independent binaries — pick the concept you want to explore and
launch it with `cargo run -p <name>`:

```sh
cargo run -p server              # minimal HTTP server  -> http://localhost:8080/health
cargo run -p request-extractor   # extractors demo      -> http://localhost:8080
cargo run -p middleware          # custom middleware    -> http://localhost:8080
cargo run -p error-handling      # typed error mapping  -> http://localhost:8080
cargo run -p route-group         # nested routers       -> http://localhost:8181
cargo run -p handler-test        # nested + tests
cargo run -p tracing-logging     # tracing + request id -> http://localhost:8181
cargo run -p websocket           # WS echo              -> ws://localhost:8080/ws
cargo run -p file-upload         # multipart upload     -> http://localhost:8080/upload
cargo run -p tls-support         # HTTPS via rustls     -> https://localhost:8088/index
cargo run -p share-state         # Redis-backed state   -> http://localhost:8080/index
```

For the stateful ones, expect external dependencies:

- `share-state` needs a local Redis at `redis://127.0.0.1/?protocol=resp3`.
- `tls-support` reads `certs/cert.pem` and `certs/key.pem` from the repo root.

Sample REST calls live next to the code: `test.rest`, `route_group_test.rest`,
`tracing_logging.rest`, `file-upload.rest`.

## Common commands

```sh
cargo build                                    # build every crate
cargo test                                     # run all tests (handler-test, etc.)
cargo run -p <crate>                           # run a single example
cargo clippy --all-targets --all-features
cargo fmt --all
```

## Conventions

- Each example is small and self-contained — read one crate's `main.rs` and
  `router.rs` to understand the full picture.
- Shared dependencies (axum, tokio, serde, tracing, …) live in the workspace
  root `Cargo.toml`; individual crates pull them with `workspace = true`.
- Error mapping uses `thiserror` enums with `IntoResponse` impls.