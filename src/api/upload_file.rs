use std::sync::Arc;

use axum::{body::Bytes, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{config::Config, db::Database, errors::AppError, utils::split_file};

#[derive(Serialize, Deserialize)]
pub struct UploadResponse {
    file_id: Uuid,
}

pub async fn upload_file(
    Extension(db): Extension<Arc<Database>>,
    Extension(config): Extension<Arc<Config>>,
    payload: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let file_id = Uuid::new_v4();
    let chunks = split_file(&payload, config.chunk_size);

    let handles: Vec<_> = chunks
        .iter()
        .enumerate()
        .map(|(i, chunk)| {
            let db = db.clone();
            let chunk = chunk.clone();
            tokio::spawn(async move { db.save_chunk(file_id, &(i as i32), &chunk).await })
        })
        .collect();

    for handle in handles {
        handle
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    }

    Ok((StatusCode::OK, Json(UploadResponse { file_id })))
}
