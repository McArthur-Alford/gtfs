use std::env::var;

pub fn db_url() -> String {
    var("DATABASE_URL").expect("DATABASE_URL must be set")
}
