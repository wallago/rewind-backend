pub(crate) use std::sync::LazyLock;

use anyhow::Result;
use dotenvy::dotenv;
use sqlx::PgPool;

#[derive(Debug)]
pub struct Config {
    pub log_level: String,
    pub app: Application,
    pub db: String,
}

impl Config {
    fn from_env() -> Self {
        let _ = dotenv();
        Self {
            log_level: load_env("RUST_LOG").unwrap_or("info".to_string()),
            app: Application::from_env(),
            db: load_env("DATABASE_URL").unwrap_or("postgres://rewind@localhost:5432".to_string()),
        }
    }
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::from_env);

#[derive(Debug)]
pub struct Application {
    pub host: String,
    pub port: String,
}

impl Application {
    fn from_env() -> Self {
        Self {
            host: load_env("APP_HOST").unwrap_or("0.0.0.0".to_string()),
            port: load_env("APP_PORT").unwrap_or("8080".to_string()),
        }
    }

    pub fn server_url(&self) -> String {
        let Self { host, port, .. } = self;

        format!("{host}:{port}")
    }
}

fn load_env<T: std::str::FromStr>(key: &str) -> Option<T> {
    match std::env::var(key) {
        Ok(val) => val.parse::<T>().ok(),
        Err(err) => {
            tracing::error!("Failed to load {key} from env ({err})");
            None
        }
    }
}

pub type DbPool = PgPool;

pub async fn init_db_pool() -> Result<DbPool> {
    Ok(PgPool::connect(&CONFIG.db).await?)
}
