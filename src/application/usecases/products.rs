use std::sync::Arc;

use crate::{
    domain::{entities::products::ProductEntity, repositories::products::ProductsRepository},
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
