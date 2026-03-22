use std::{net::SocketAddr, sync::Arc, time::Duration};

use anyhow::{Result, anyhow};
use axum::{
    Router,
    http::{HeaderValue, Method, StatusCode},
    routing::get,
};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::info;

use crate::{
    config::{config_loader::get_stage, config_model::DotEnvyConfig, stage::Stage},
    infrastructure::axum_http::default_routers,
};

pub async fn start(config: Arc<DotEnvyConfig>) -> Result<()> {
    let cors_layer = match get_stage() {
        Stage::Local | Stage::Development => CorsLayer::new().allow_origin(Any),
        Stage::Production => {
            let origins = config
                .server
                .allow_origins
                .split(',')
                .map(str::trim)
                .filter(|origin| !origin.is_empty())
                .map(HeaderValue::from_str)
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(|err| anyhow!("Invalid SERVER_ALLOW_ORIGINS value: {err}"))?;

            if origins.is_empty() {
                return Err(anyhow!(
                    "SERVER_ALLOW_ORIGINS must contain at least one origin in production"
                ));
            }

            CorsLayer::new().allow_origin(origins)
        }
    };
    let app = Router::new()
        .fallback(default_routers::not_found)
        .route("/health-check", get(default_routers::health_check))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(config.server.time_out),
        ))
        .layer(RequestBodyLimitLayer::new(
            (config.server.body_limit * 1024 * 1024).try_into()?,
        ))
        .layer(
            cors_layer
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ]),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));

    let listener = TcpListener::bind(addr).await?;

    info!("Server is listening on {}", config.server.port);

    axum::serve(listener, app).await?;

    Ok(())
}
