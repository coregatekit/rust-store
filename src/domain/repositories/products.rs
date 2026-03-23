use async_trait::async_trait;
use mockall::automock;

use anyhow::Result;

use crate::domain::entities::products::ProductEntity;

#[automock]
#[async_trait]
pub trait ProductsRepository {
    async fn get_products_cursor(&self, cursor: String, size: usize) -> Result<Vec<ProductEntity>>;
}
