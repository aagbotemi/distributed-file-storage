use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use uuid::Uuid;

use crate::{db::Database, errors::AppError};

pub async fn get_file_data(
    Extension(db): Extension<Arc<Database>>,
    Path(file_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let file_data = db
        .get_file_metadata(file_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok((StatusCode::OK, Json(file_data)))
}
