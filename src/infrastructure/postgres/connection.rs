use anyhow::Result;
use diesel::{PgConnection, r2d2::{ConnectionManager, Pool}};

pub type PgPoolSquad = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(url: &str) -> Result<PgPoolSquad> {
  let manager = ConnectionManager::<PgConnection>::new(url);
  let pool = Pool::builder().build(manager)?;

  Ok(pool)    
}