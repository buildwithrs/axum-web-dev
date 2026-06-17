use std::net::SocketAddr;

use anyhow::Context;
use file_upload::router::app_router;
use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let app = app_router();

    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("failed to bind {addr}"))?;

    tracing::info!("listening on http://{addr}");
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
