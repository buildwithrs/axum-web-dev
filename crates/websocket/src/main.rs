use anyhow::Context;
use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tracing::info;
use websocket::{init_tracing, ws::ws_handler};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let app = Router::new().route("/ws", get(ws_handler));

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("failed to bind {addr}"))?;

    info!("server listen on: {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}
