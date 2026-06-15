use std::collections::HashMap;

use axum::{
    Json, Router, extract::{Path, Query}, response::IntoResponse, routing::{get, post}
};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResp {
    pub id: String,
    pub name: String,
}

pub fn app_router() -> Router {
    let routes = Router::new()
        .route("/api/users/{user_id}", get(get_user_id))
        .route("/api/users", post(create_user))
        .route("/api/posts", get(query));

    routes
}

// path parameter extractor
async fn get_user_id(Path(user_id): Path<String>) -> Result<String, AppError> {
    println!("getting user_id from path parameter: {}", user_id);
    if &user_id == "1001" {
        Ok(format!("user_id: {}", user_id))

    } else {
        Err(AppError::ResultNotFound("user not found".to_string()))
    }
}

// post user data
async fn create_user(Json(user): Json<CreateUserRequest>) -> Result<impl IntoResponse, AppError> {
    println!("get create user request: {:?}", user);
    if user.id == "" || user.id.len() < 5 {
        return Err(AppError::InvalidArgument("user id empty or too short".to_string()));
    }
    Ok(Json(CreateUserResp {id: user.id.clone(), name: user.name.clone()}))
}

// get query paramter
async fn query(Query(data): Query<HashMap<String, String>>) -> String {
    println!("getting query params: {:?}", data);
    format!("params: {:?}", data)
}
