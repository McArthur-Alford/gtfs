pub mod queries;
pub mod types;

use crate::vars;
use anyhow::Result;
use sqlx::{PgPool, migrate};
use tracing::{info, instrument};

#[derive(Debug)]
pub struct Db(pub PgPool);

#[instrument]
pub async fn connect() -> Result<Db> {
    info!("Attempting to connect to db");
    let db_url = vars::db_url();
    let pool = PgPool::connect(&db_url).await?;
    info!("Connected to db");
    Ok(Db(pool))
}

impl Db {
    #[instrument(name = "db_migrations", skip(self))]
    pub async fn run_migrations(&mut self) -> Result<()> {
        info!("Attempting DB migrations");
        sqlx::migrate!("./migrations").run(&self.0).await?;
        info!("Db migrations complete");
        Ok(())
    }
}
