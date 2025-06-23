use sqlx::PgPool;
use std::env;

pub struct Db(PgPool);

pub async fn init_pool() -> Result<PgPool, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // PgPool:: .max_connections(5).connect(&db_url).await
    todo!()
}
