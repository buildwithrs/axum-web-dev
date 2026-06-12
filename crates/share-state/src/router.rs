use axum::{Router, extract::State, routing::get};
use redis::AsyncCommands;

use crate::state::AppState;

pub fn app_router(state: AppState) -> Router {
    let router = Router::new().route("/index", get(index)).with_state(state);

    router
}

async fn index(State(state): State<AppState>) -> String {
    let mut conn = match state.redis_conn.get_multiplexed_async_connection().await {
        Ok(conn) => conn,
        Err(e) => return format!("failed to get redis conn: {}", e),
    };

    let value: Option<String> = match conn.get("hello").await {
        Ok(v) => v,
        Err(e) => return format!("failed to get value: {}", e),
    };

    format!("<html><body><h1>{:?}<h1></body></html>", value)
}
