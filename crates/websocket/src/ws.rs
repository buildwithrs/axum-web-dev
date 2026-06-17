use axum::{extract::{WebSocketUpgrade, ws::{Message, WebSocket}}, response::IntoResponse};
use tracing::{info, warn};


pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(upgrade_handler)
}

async fn upgrade_handler(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(m) => {
                info!("received msg: {:?}", m);

                let echo = format!("Echo: {:?}", m.to_text().unwrap());
                socket.send(Message::Text(echo.into())).await.unwrap();
            },
            Err(e) => {
                warn!("failed recv msg: {}", e);
            }
        }
    }
}