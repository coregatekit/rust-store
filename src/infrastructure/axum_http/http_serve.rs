use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tracing::info;

use crate::{config::config_model::DotEnvyConfig, infrastructure::axum_http::default_routers};

pub async fn start(config: Arc<DotEnvyConfig>) -> Result<()> {
    let app = Router::new()
        .fallback(default_routers::not_found)
        .route("/health-check", get(default_routers::health_check));

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));

    let listener = TcpListener::bind(addr).await?;

    info!("Server is listening on {}", config.server.port);

    axum::serve(listener, app).await?;

    Ok(())
}
