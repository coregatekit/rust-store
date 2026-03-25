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

#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use bigdecimal::BigDecimal;
    use chrono::NaiveDateTime;

    use crate::domain::repositories::products::MockProductsRepository;

    use super::*;

    // Helper function to create a ProductEntity for testing
    fn make_product(id: i32) -> ProductEntity {
        ProductEntity {
            id,
            name: format!("Product {id}"),
            description: None,
            image_url: None,
            price: BigDecimal::from(100),
            created_at: NaiveDateTime::parse_from_str("2025-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            updated_at: NaiveDateTime::parse_from_str("2025-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
        }
    }

    // Helper function to create a ProductsUseCase with a MockProductsRepository
    fn make_use_case(mock: MockProductsRepository) -> ProductsUseCase<MockProductsRepository> {
        ProductsUseCase::new(Arc::new(mock))
    }

    // First page (no cursor) returns items and a next cursor
    #[tokio::test]
    async fn get_products_first_page_returns_next_cursor() {
        let mut mock = MockProductsRepository::new();

        // Return exactly `size` items - signals there is a next page
        let _items: Vec<ProductEntity> = (1..=5).map(make_product).collect();
        mock.expect_get_products_cursor()
            .withf(|cursor, size| cursor.is_empty() && *size == 5)
            .returning(|_, _| Ok((1..=5).map(make_product).collect()));

        let uc = make_use_case(mock);
        let page = uc.get_products(None, 5).await.unwrap();

        assert_eq!(page.items.len(), 5);
        assert!(
            page.next_cursor.is_some(),
            "expected a next cursor when page is full"
        );
    }

    // Last page returns no next_cursor
    #[tokio::test]
    async fn get_products_last_page_has_no_next_cursor() {
        let mut mock = MockProductsRepository::new();

        // Return fewer items than `size` - signals this is the last page
        mock.expect_get_products_cursor()
            .returning(|_, _| Ok((1..=3).map(make_product).collect()));

        let uc = make_use_case(mock);
        let page = uc.get_products(None, 5).await.unwrap();

        assert_eq!(page.items.len(), 3);
        assert!(
            page.next_cursor.is_none(),
            "expected no next cursor when page is not full"
        );
    }

    // Repository error propagates
    #[tokio::test]
    async fn get_products_propagates_repository_error() {
        let mut mock = MockProductsRepository::new();

        mock.expect_get_products_cursor()
            .returning(|_, _| Err(anyhow!("db connection failed")));

        let uc = make_use_case(mock);
        let result = uc.get_products(None, 5).await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("db connection failed")
        );
    }

    // A valid cursor is passed through to the repository
    #[tokio::test]
    async fn get_products_passes_cursor_to_repository() {
        let mut mock = MockProductsRepository::new();

        mock.expect_get_products_cursor()
            .withf(|cursor, _| !cursor.is_empty()) // cursor must be not empty
            .returning(|_, _| Ok(vec![make_product(99)]));

        let uc = make_use_case(mock);

        use crate::infrastructure::postgres::cursor::Cursor;
        let token = Cursor {
            id: 10,
            created_at: NaiveDateTime::parse_from_str("2025-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
        }
        .encode()
        .unwrap();

        let page = uc.get_products(Some(token), 5).await.unwrap();
        assert_eq!(page.items[0].id, 99);
    }
}
