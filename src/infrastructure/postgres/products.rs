use std::sync::Arc;

use crate::infrastructure::postgres::connection::PgPoolSquad;

pub struct ProductPostgres {
  db_pool: Arc<PgPoolSquad>,
}

impl ProductPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool: db_pool }
    }
}
