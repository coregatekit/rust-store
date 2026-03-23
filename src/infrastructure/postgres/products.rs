use anyhow::{Error, Result};
use diesel::prelude::*;
use std::sync::Arc;
use tokio::task::spawn_blocking;

use async_trait::async_trait;

use crate::{
    domain::{entities::products::ProductEntity, repositories::products::ProductsRepository},
    infrastructure::postgres::{
        connection::PgPoolSquad,
        cursor::Cursor,
        schema::products::dsl::{self, products},
    },
};

pub struct ProductPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl ProductPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool: db_pool }
    }
}

#[async_trait]
impl ProductsRepository for ProductPostgres {
    async fn get_products_cursor(&self, cursor: String, size: usize) -> Result<Vec<ProductEntity>> {
        let pool = Arc::clone(&self.db_pool);
        let limit = (size + 1) as i64; // Fetch one extra to determine if there's a next page

        // Decode the cursor if it's not empty
        let parsed: Option<Cursor> = if cursor.is_empty() {
            None
        } else {
            Some(Cursor::decode(&cursor)?)
        };

        let mut rows: Vec<ProductEntity> = spawn_blocking(move || {
            let mut conn = pool.get()?;

            let mut query = products
                .order((dsl::created_at.desc(), dsl::id.desc()))
                .limit(limit)
                .select(ProductEntity::as_select())
                .into_boxed();

            // Apply keyset predicate only when a cursor is provided
            if let Some(c) = parsed {
                query = query.filter(
                    dsl::created_at
                        .lt(c.created_at)
                        .or(dsl::created_at.eq(c.created_at).and(dsl::id.lt(c.id))),
                );
            }

            query.load(&mut conn).map_err(Error::from)
        })
        .await??;

        rows.truncate(size);

        Ok(rows)
    }
}
