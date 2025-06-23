use crate::vars;
use anyhow::Result;
use sqlx::{PgPool, migrate};
use tracing::instrument;

#[derive(Debug)]
pub struct Db(pub PgPool);

#[instrument]
pub async fn connect() -> Result<Db> {
    let db_url = vars::db_url();
    let pool = PgPool::connect(&db_url).await?;
    Ok(Db(pool))
}

impl Db {
    #[instrument(name = "db_migrations", skip(self))]
    pub async fn run_migrations(&mut self) -> Result<()> {
        sqlx::migrate!("./migrations").run(&self.0).await?;
        Ok(())
    }
}
