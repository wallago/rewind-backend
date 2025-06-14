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
            log_level: load_env("RUST_LOG"),
            app: Application::from_env(),
            db: load_env("DATABASE_URL"),
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
            host: load_env("APP_HOST"),
            port: load_env("APP_PORT"),
        }
    }

    pub fn server_url(&self) -> String {
        let Self { host, port, .. } = self;

        format!("{host}:{port}")
    }
}

fn load_env<T: std::convert::From<String>>(key: &str) -> T {
    std::env::var(key).map_or_else(
        |err| panic!("Failed to load {key} from env ({err})"),
        From::from,
    )
}

pub type DbPool = PgPool;

pub async fn init_db_pool() -> Result<DbPool> {
    Ok(PgPool::connect(&CONFIG.db).await?)
}
