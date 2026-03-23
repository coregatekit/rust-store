use anyhow::{Ok, Result};
use std::sync::Arc;

use crate::{
    domain::{
        entities::products::{ProductCursorPage, ProductEntity},
        repositories::products::ProductsRepository,
    },
    infrastructure::postgres::cursor::Cursor,
};

pub struct ProductsUseCase<T>
where
    T: ProductsRepository + Send + Sync,
{
    products_repository: Arc<T>,
}

impl<T> ProductsUseCase<T>
where
    T: ProductsRepository + Send + Sync,
{
    pub fn new(products_repository: Arc<T>) -> Self {
        Self {
            products_repository,
        }
    }

    pub async fn get_products(
        &self,
        cursor: Option<String>,
        size: usize,
    ) -> Result<ProductCursorPage> {
        let cursor_str = cursor.unwrap_or_default(); // empty string = first page

        let items = self
            .products_repository
            .get_products_cursor(cursor_str, size)
            .await?;

        let next_cursor = Self::build_next_cursor(&items, size);

        Ok(ProductCursorPage { items, next_cursor })
    }

    fn build_next_cursor(items: &[ProductEntity], page_size: usize) -> Option<String> {
        if items.len() < page_size {
            return None;
        }

        let last = items.last()?;
        Cursor {
            id: last.id,
            created_at: last.created_at,
        }
        .encode()
        .ok()
    }
}
