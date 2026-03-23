use async_trait::async_trait;
use mockall::automock;

use anyhow::Result;

#[automock]
#[async_trait]
pub trait ProductsRepository {
    async fn get_products() -> Result<()>;
}
