//! DB Tests
//!
//! Specifically tests db queries and operations.
//! Does not test a gtfs dataset properly.

#[cfg(test)]
use sqlx::PgPool;

#[cfg(test)]
#[sqlx::test(migrator = "super::MIGRATOR")]
pub fn magic(pool: PgPool) -> sqlx::Result<()> {
    Ok(())
}
