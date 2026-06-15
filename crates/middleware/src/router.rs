use std::collections::HashMap;

use axum::{
    Json, Router,
    extract::{Path, Query},
    middleware::from_fn,
    routing::{get, post},
};
use serde::Deserialize;

use crate::mid::{MyLayer, simple_logging};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub id: String,
    pub name: String,
}

pub fn app_router() -> Router {
    let routes = Router::new()
        .route("/api/users/{user_id}", get(get_user_id))
        .route("/api/users", post(create_user))
        .route("/api/posts", get(query))
        .layer(from_fn(simple_logging))
        .layer(MyLayer {});

    routes
}

// path parameter extractor
async fn get_user_id(Path(user_id): Path<String>) -> String {
    println!("getting user_id from path parameter: {}", user_id);
    format!("user_id: {}", user_id)
}

// post user data
async fn create_user(Json(user): Json<CreateUserRequest>) -> String {
    println!("get create user request: {:?}", user);
    format!("created user: {:?}", user)
}

// get query paramter
async fn query(Query(data): Query<HashMap<String, String>>) -> String {
    println!("getting query params: {:?}", data);
    format!("params: {:?}", data)
}
