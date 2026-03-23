use std::sync::Arc;

use rust_store::{config::config_loader, infrastructure::{axum_http::http_serve::start, postgres::connection::establish_connection}};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let dotenvy_env = match config_loader::load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load environment variables: {}", e);
            std::process::exit(1);
        } 
    };

    info!("Environment variables loaded successfully:"); 

    let postgres_pool = match establish_connection(&dotenvy_env.database.url) {
        Ok(pool) => pool,
        Err(err) => {
            error!("Failed to establish database connection: {err}");
            std::process::exit(1);
        },
    };

    info!("Database connection established successfully");

    start(Arc::new(dotenvy_env), Arc::new(postgres_pool)).await.expect("Failed to start server");
}
