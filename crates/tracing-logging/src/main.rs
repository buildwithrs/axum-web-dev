use tracing::info;
use tracing_logging::{observe::init_tracing, router::app_router};
use tokio::net::TcpListener;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();
    
    let router = app_router();

    let server_addr = "0.0.0.0:8181";
    let listener = TcpListener::bind(server_addr).await?;
    info!("web server listen on: {}", server_addr);

    axum::serve(listener, router.into_make_service()).await?;
    Ok(())
}
