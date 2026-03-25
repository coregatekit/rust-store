use std::{net::SocketAddr, sync::Arc, time::Duration};

use anyhow::{Result, anyhow};
use axum::{
    Router,
    http::{HeaderValue, Method, StatusCode},
    routing::get,
};
use scalar_api_reference::axum::router;
use serde_json::json;
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::info;
use utoipa::OpenApi;

use crate::{
    config::{config_loader::get_stage, config_model::DotEnvyConfig, stage::Stage},
    infrastructure::{
        axum_http::{default_routers, routers},
        postgres::connection::PgPoolSquad,
    },
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::infrastructure::axum_http::routers::products::list_products
    ),
    components(schemas(
        crate::domain::entities::products::ProductEntity,
        crate::domain::entities::products::ProductCursorPage
    )),
    info(
        title = "Rust Store API",
        version = "1.0.0",
        description = "API for Rust Store application"
    ),
    tags(
        (name = "products", description = "Operations related to products")
    ),
)]
struct ApiDoc;

pub async fn start(config: Arc<DotEnvyConfig>, db_pool: Arc<PgPoolSquad>) -> Result<()> {
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

    let spec_json = ApiDoc::openapi().to_pretty_json().unwrap();
    let spec_route = Router::new().route(
        "/api-docs/openapi.json",
        get(move || async move {
            axum::response::Response::builder()
                .header("Content-Type", "application/json")
                .body(spec_json.clone())
                .unwrap()
        }),
    );

    let api_reference_configuration = json!({
        "url": "/api-docs/openapi.json",
        "name": "Scalar API Reference",
    });

    let v1 = Router::new().nest("/products", routers::products::routes(Arc::clone(&db_pool)));

    let app = Router::new()
        .fallback(default_routers::not_found)
        .merge(spec_route) // serve OpenAPI spec at /api-docs/openapi.json
        .merge(router("/scalar", &api_reference_configuration)) // serve Scalar API Reference UI at /scalar
        .nest("/api/v1", v1)
        .route("/health-check", get(default_routers::health_check))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(config.server.time_out),
        ))
        .layer(RequestBodyLimitLayer::new(
            (config.server.body_limit * 1024 * 1024).try_into()?,
        ))
        .layer(cors_layer.allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ]))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));

    let listener = TcpListener::bind(addr).await?;

    info!("Server is listening on {}", config.server.port);

    axum::serve(listener, app).await?;

    Ok(())
}
