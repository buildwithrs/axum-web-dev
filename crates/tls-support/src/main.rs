use axum::{Router, routing::get};
use axum_server::tls_rustls::RustlsConfig;
use std::net::SocketAddr;


const HELLO: &'static str = "<html><head></head><body><h1>Hello, Rust TLS Support Web</h1></body></html>";

#[tokio::main]
async fn main() ->anyhow::Result<()> {
    let app = Router::new().route("/index", get(|| async { HELLO }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8088));
    println!("listening on {}", addr);

    let config = RustlsConfig::from_pem_file("certs/cert.pem", "certs/key.pem").await?;

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
