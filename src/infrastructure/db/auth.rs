// src/infrastructure/db/auth.rs
pub struct DBCredentials {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub dbname: String,
}

#[async_trait::async_trait]
pub trait DBAuthAdapter: Send + Sync {
    async fn get_credentials(&self) -> anyhow::Result<DBCredentials>;
}