// src/infrastructure/db/env_auth.rs
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
        })
    }
}