use share_state::{router::app_router, state::AppState};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "redis://127.0.0.1/?protocol=resp3";
    let redis_cli = redis::Client::open(addr)?;

    let state = AppState::new(redis_cli);
    let router = app_router(state);

    let server_addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(server_addr).await?;
    println!("http server listen on: {}", server_addr);

    axum::serve(listener, router.into_make_service()).await?;
    Ok(())
}
