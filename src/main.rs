use std::sync::Arc;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use distributed_file_storage::{
    api::{download_file::download_file, get_file_data::get_file_data, upload_file::upload_file},
    config::Config,
    db::Database,
    errors::AppError,
};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tokio::{net::TcpListener, task};
use tower_http::cors::{Any, CorsLayer};

// use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Load configuration
    let config = Config::load_config().expect("Failed to load configuration");
    let config = Arc::new(config);

    // Initialize logger
    // tracing_subscriber::fmt::init();

    // Initialize database connection
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| AppError::MigrationError(e.to_string()))?;

    let db = Arc::new(Database::new(pool));

    let addr = format!("{}:{}", config.host, config.port);

    // Build our application with a route
    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/upload", post(upload_file))
        .route("/file/:id", get(get_file_data))
        .route("/download/:id", get(download_file))
        .layer(cors)
        .layer(Extension(db))
        .layer(Extension(config));
    // .layer(TraceLayer::new_for_http());

    // Run it
    let listener = TcpListener::bind(addr).await.unwrap();
    dbg!(&listener.local_addr().unwrap());
    // let _ = task::spawn(async move {
    axum::serve(listener, app).await.unwrap();
    // });

    Ok(())
}
