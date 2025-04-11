
pub struct DBCredentials {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub dbname: String,
    pub pool_size: usize,
    pub ssl_mode: String,
}

#[async_trait::async_trait]
pub trait DBAuthAdapter: Send + Sync {
    async fn get_credentials(&self) -> anyhow::Result<DBCredentials>;
}