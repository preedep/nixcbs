// src/infrastructure/db/connector.rs
use sqlx::{PgPool, postgres::PgPoolOptions};
use crate::infrastructure::db::auth::DBAuthAdapter;


pub async fn connect_with_adapter<A: DBAuthAdapter>(
    adapter: &A
) -> anyhow::Result<PgPool> {
    let creds = adapter.get_credentials().await?;
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        creds.username, creds.password, creds.host, creds.port, creds.dbname
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    Ok(pool)
}