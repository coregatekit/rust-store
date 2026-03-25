use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

use crate::infrastructure::postgres::schema::products;
use utoipa::ToSchema;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable, Serialize, ToSchema)]
#[diesel(table_name = products)]
pub struct ProductEntity {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    #[schema(value_type = f64)] // BigDecimal doesn't implement ToSchema natively
    pub price: BigDecimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ProductCursorPage {
    pub items: Vec<ProductEntity>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = products)]
pub struct CreateProductEntity {
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub price: BigDecimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
