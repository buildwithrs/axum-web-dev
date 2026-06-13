use request_extractor::router::app_router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = app_router();

    let server_addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(server_addr).await?;
    println!("http server listen on: {}", server_addr);

    axum::serve(listener, router.into_make_service()).await?;
    Ok(())
}
