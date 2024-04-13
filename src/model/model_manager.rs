use crate::config::envs::Config;
use sqlx::migrate;

use super::store::pool::{new_db_pool, Db};

// endregion: --- Modules

#[derive(Clone, Debug)]
pub struct ModelManager {
    pub db: Db,
}

impl ModelManager {
    /// Constructor
    pub async fn new(config: &Config) -> eyre::Result<Self> {
        let db = new_db_pool(&config.database_url).await?;

        migrate!("./migrations")
            .run(&db)
            .await
            .expect("Failed to migrate DB");

        Ok(ModelManager { db })
    }

    /// Returns the sqlx db pool reference.
    /// (Only for the model layer)
    pub fn db(&self) -> &Db {
        &self.db
    }
}
