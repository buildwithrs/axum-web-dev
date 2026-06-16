use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;


#[derive(Debug, Error)]
pub enum AppError {
    #[error("invalid argument")]
    InvalidArgument(String),

    #[error("result not found")]
    ResultNotFound(String)
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::InvalidArgument(s) => {
                (StatusCode::BAD_REQUEST, s).into_response()
            }
            AppError::ResultNotFound(s) => {
                (StatusCode::NOT_FOUND, s).into_response()
            }
        }
    }
}