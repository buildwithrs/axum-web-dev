use std::path::PathBuf;

use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart},
    response::IntoResponse,
    routing::post,
};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::warn;

use crate::errors::AppError;

const FILE_PATH: &'static str = "/tmp";
const MAX_UPLOAD: usize = 2 * 1024 * 1024; // 2 MB

pub fn app_router() -> Router {
    Router::new().route(
        "/upload",
        post(upload_handler).layer(DefaultBodyLimit::max(MAX_UPLOAD)),
    )
}

async fn upload_handler(mut multipart: Multipart) -> Result<impl IntoResponse, AppError> {
    while let Some(mut field) = multipart.next_field().await? {
        let Some(f_name) = field.file_name() else {
            continue;
        };

        let name = safe_name(f_name);
        let path = PathBuf::from(FILE_PATH).join(name);

        match File::create_new(path.clone()).await {
            Ok(mut f) => {
                // let _ = f.write_buf(&mut data).await;
                while let Some(chunk) = field.chunk().await? {
                    f.write_all(&chunk).await?;
                }
                f.flush().await?;

                return Ok(format!("upload success to: {:?}", path));
            }
            Err(e) => {
                warn!("failed upload file: {}", e);
                return Ok(format!("failed to upload file"));
            }
        }
    }

    Ok(format!("failed to upload file"))
}

fn safe_name(name: &str) -> String {
    PathBuf::from(name)
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "upload.data".into())
}
