use axum::{extract::multipart::MultipartError, http::StatusCode, response::IntoResponse};
use thiserror::Error;
use tokio::io;


#[derive(Debug, Error)]
pub enum AppError {
    #[error("upload error: {0}")]
    UploadError(#[from] MultipartError),

    #[error("write file error: {0}")]
    IOError(#[from] io::Error)
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::UploadError(e) => {
                (StatusCode::BAD_REQUEST, e.to_string()).into_response()
            }
            AppError::IOError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
}