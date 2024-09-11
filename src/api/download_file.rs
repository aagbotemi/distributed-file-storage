use std::sync::Arc;

use axum::{
    extract::Path,
    http::{
        header::{HeaderValue, CONTENT_DISPOSITION, CONTENT_TYPE},
        StatusCode,
    },
    response::IntoResponse,
    Extension,
};

use futures::future::try_join_all;
use tokio::task::JoinHandle;
use uuid::Uuid;

use crate::{config::Config, db::Database, errors::AppError};

// pub async fn download_file(
//     Extension(db): Extension<Arc<Database>>,
//     Path(file_id): Path<Uuid>,
//     Extension(config): Extension<Arc<Config>>,
// ) -> Result<impl IntoResponse, AppError> {
//     let chunks = db
//         .get_file_chunks(file_id)
//         .await
//         .map_err(|e| AppError::DatabaseError(e.to_string()))?;

//     // Combine all chunks into one file
//     let mut file_data = Vec::new();
//     for chunk in chunks {
//         file_data.extend_from_slice(&chunk);
//     }

//     let file_name = format!("{}.png", file_id); // Adjust file extension accordingly
//     let content_disposition = format!("attachment; filename={}", file_name);

//     Ok((
//         StatusCode::OK,
//         [
//             (header::CONTENT_TYPE, "image/png"),
//             (
//                 header::CONTENT_DISPOSITION,
//                 content_disposition.as_str(),
//             ),
//         ],
//         file_data,
//     ))
// }

pub async fn download_file(
    Extension(db): Extension<Arc<Database>>,
    Path(file_id): Path<Uuid>,
    Extension(config): Extension<Arc<Config>>,
) -> Result<impl IntoResponse, AppError> {
    let chunks = db
        .get_file_chunks(file_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Combine all chunks into one file
    let mut file_data = Vec::new();
    for chunk in chunks {
        file_data.extend_from_slice(&chunk);
    }

    let file_name = format!("{}.png", file_id); // Adjust file extension accordingly
    let content_disposition = format!("attachment; filename={}", file_name);

    // Use HeaderValue for headers that take ownership of the string
    let headers = [
        (CONTENT_TYPE, HeaderValue::from_static("image/png")),
        (
            CONTENT_DISPOSITION,
            HeaderValue::from_str(&content_disposition).map_err(|_| AppError::InvalidHeader)?,
        ),
    ];

    Ok((StatusCode::OK, headers, file_data))
}
