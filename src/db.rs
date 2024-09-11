use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use crate::errors::AppError;

pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn save_chunk(
        &self,
        file_id: Uuid,
        chunk_id: &i32,
        data: &[u8],
    ) -> Result<(), AppError> {
        sqlx::query("INSERT INTO file_chunks (file_id, chunk_id, data) VALUES ($1, $2, $3)")
            .bind(file_id)
            .bind(chunk_id)
            .bind(data)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn get_file_metadata(&self, file_id: Uuid) -> Result<Vec<(i32, Vec<u8>)>, AppError> {
        let chunks = sqlx::query("SELECT * FROM file_chunks WHERE file_id = $1 ORDER by chunk_id")
            .bind(file_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(chunks
            .into_iter()
            .map(|row| {
                let chunk_id: i32 = row.get("chunk_id");
                let data: Vec<u8> = row.get("data");
                (chunk_id, data)
            })
            .collect())
    }

    pub async fn get_file_chunks(&self, file_id: Uuid) -> Result<Vec<Vec<u8>>, AppError> {
        let chunks = self.get_file_metadata(file_id).await?;
        Ok(chunks.into_iter().map(|(_, data)| data).collect())
    }
}
