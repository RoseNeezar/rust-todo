use shuttle_runtime::SecretStore;
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn init(secrets: SecretStore) -> Config {
        let database_url = secrets
            .get("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let jwt_secret = secrets.get("JWT_SECRET").expect("JWT_SECRET must be set");

        Config {
            database_url,
            jwt_secret,
        }
    }
}
