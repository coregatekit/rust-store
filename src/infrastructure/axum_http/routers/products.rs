use std::{collections::HashMap, sync::Arc};

use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};

use crate::{
    application::usecases::products::ProductsUseCase,
    domain::repositories::products::ProductsRepository,
    infrastructure::postgres::{connection::PgPoolSquad, products::ProductPostgres},
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let products_repository = ProductPostgres::new(Arc::clone(&db_pool));

    let products_use_case = ProductsUseCase::new(Arc::new(products_repository));

    Router::new()
        .route("/", get(list_products))
        .with_state(Arc::new(products_use_case))
}

pub async fn list_products<T>(
    State(uc): State<Arc<ProductsUseCase<T>>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse
where
    T: ProductsRepository + Send + Sync,
{
    let cursor = params.get("cursor").cloned();
    let size = params
        .get("size")
        .and_then(|s| s.parse().ok())
        .unwrap_or(20);

    match uc.get_products(cursor, size).await {
        Ok(page) => Json(page).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get products: {}", e),
        )
            .into_response(),
    }
}
