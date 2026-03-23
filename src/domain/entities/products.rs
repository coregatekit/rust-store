use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::infrastructure::postgres::schema::products;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = products)]
pub struct ProductEntity {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub price: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct ProductCursorPage {
    pub items: Vec<ProductEntity>,
    pub next_cursor: Option<String>,
}
