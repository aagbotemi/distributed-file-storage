use serde::Deserialize;
use std::env::var;

use crate::errors::AppError;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: String,
    pub chunk_size: usize,
}

impl Config {
    pub fn load_config() -> Result<Self, AppError> {
        Ok(Config {
            database_url: var("DATABASE_URL")
                .map_err(|e| AppError::EnvVarError("DATABASE_URL".to_owned(), e))?,
            host: var("HOST").map_err(|e| AppError::EnvVarError("HOST".to_owned(), e))?,
            port: var("PORT").map_err(|e| AppError::EnvVarError("PORT".to_owned(), e))?,
            chunk_size: var("CHUNK_SIZE")
                .map_err(|e| AppError::EnvVarError("CHUNK_SIZE".to_owned(), e))?
                .parse::<usize>()
                .map_err(|e| AppError::ParseError("CHUNK_SIZE".to_owned(), e))?,
        })
    }
}
