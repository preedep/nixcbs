
use async_trait::async_trait;
use crate::infrastructure::db::auth::{DBAuthAdapter, DBCredentials};

pub struct EnvDBAuth;

#[async_trait]
impl DBAuthAdapter for EnvDBAuth {
    async fn get_credentials(&self) -> anyhow::Result<DBCredentials> {
        Ok(DBCredentials {
            username: std::env::var("DB_USER")?,
            password: std::env::var("DB_PASS")?,
            host: std::env::var("DB_HOST")?,
            port: std::env::var("DB_PORT")?.parse()?,
            dbname: std::env::var("DB_NAME")?,
            pool_size: std::env::var("DB_POOL_SIZE")?.parse()?,
            ssl_mode: std::env::var("DB_SSL_MODE")?,
        })
    }
}