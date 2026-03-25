use bigdecimal::BigDecimal;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::domain::entities::products::CreateProductEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductModel {
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub price: BigDecimal,
}

impl CreateProductModel {
    pub fn to_entity(&self) -> CreateProductEntity {
        CreateProductEntity {
            name: self.name.clone(),
            description: self.description.clone(),
            image_url: self.image_url.clone(),
            price: self.price.clone(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
