use std::collections::HashMap;

use axum::{
    Json, Router,
    extract::{Path, Query},
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Product {
    pub id: String,
    pub kind: String,
    pub name: String,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub product: Product,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResp {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateProductResp {
    pub product: Product,
}

#[derive(Debug, Deserialize)]
pub struct ListProductsReq {
    pub offset: u32,
    pub limit: u32,
}

pub fn app_router() -> Router {
    let user_route = Router::new()
        .route("/users/{user_id}", get(get_user_id))
        .route("/users", post(create_user))
        .route("/posts", get(query));

    let product_route = Router::new()
        .route("/products/{product_id}", get(get_product_by_id))
        .route("/products", post(create_product).get(list_products));

    let all_routes = Router::new()
        .nest("/api/u", user_route)
        .nest("/api/p", product_route);
    all_routes
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

// path parameter extractor
async fn get_product_by_id(Path(product_id): Path<String>) -> Result<String, AppError> {
    println!("getting product from path parameter: {}", product_id);
    if &product_id == "9090" {
        Ok(format!("product_id: {}", product_id))
    } else {
        Err(AppError::ResultNotFound("product not found".to_string()))
    }
}

// post user data
async fn create_user(Json(user): Json<CreateUserRequest>) -> Result<impl IntoResponse, AppError> {
    println!("get create user request: {:?}", user);
    if user.id == "" || user.id.len() < 5 {
        return Err(AppError::InvalidArgument(
            "user id empty or too short".to_string(),
        ));
    }
    Ok(Json(CreateUserResp {
        id: user.id.clone(),
        name: user.name.clone(),
    }))
}

// post user data
async fn create_product(
    Json(req): Json<CreateProductRequest>,
) -> Result<impl IntoResponse, AppError> {
    println!("get create product request: {:?}", req);
    if req.product.id == "" || req.product.id.len() < 5 {
        return Err(AppError::InvalidArgument(
            "user id empty or too short".to_string(),
        ));
    }
    let prd = req.product;
    Ok(Json(CreateProductResp {
        product: prd.clone(),
    }))
}

// get query paramter
async fn list_products(Query(req): Query<ListProductsReq>) -> impl IntoResponse {
    println!("list products req: {:?}", req);
    let prd1 = Product::default();
    Json(vec![prd1.clone(), prd1])
}

// get query paramter
async fn query(Query(data): Query<HashMap<String, String>>) -> String {
    println!("getting query params: {:?}", data);
    format!("params: {:?}", data)
}
