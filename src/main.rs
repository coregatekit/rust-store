use std::sync::Arc;

use rust_store::{config::config_loader, infrastructure::axum_http::http_serve::start};
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

    info!("Environment variables loaded successfully: {:?}", dotenvy_env);

    start(Arc::new(dotenvy_env)).await.expect("Failed to start server");
}
