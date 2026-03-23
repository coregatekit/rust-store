use std::sync::Arc;

use crate::domain::repositories::products::ProductsRepository;

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
}
