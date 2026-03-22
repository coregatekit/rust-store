use std::{net::SocketAddr, sync::Arc, time::Duration};

use anyhow::Result;
use axum::{
    Router,
    http::{Method, StatusCode},
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

use crate::{config::config_model::DotEnvyConfig, infrastructure::axum_http::default_routers};

pub async fn start(config: Arc<DotEnvyConfig>) -> Result<()> {
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
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_origin(Any),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));

    let listener = TcpListener::bind(addr).await?;

    info!("Server is listening on {}", config.server.port);

    axum::serve(listener, app).await?;

    Ok(())
}
