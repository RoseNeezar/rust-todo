// region:    --- Modules

mod error;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

// endregion: --- Modules

pub type Db = Pool<Postgres>;

pub async fn new_db_pool(db_url: &str) -> eyre::Result<Db> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .map_err(|ex| eyre::eyre!("{}", ex.to_string()))
}
