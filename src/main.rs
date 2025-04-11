mod config;
mod domain;
mod adapter;
mod infrastructure;
mod shared;

use anyhow::Result;
use std::error::Error;
use log::error;
use crate::infrastructure::connect_with_adapter;
#[cfg(feature = "dev")]
use crate::infrastructure::db::env_auth::EnvDBAuth;

#[cfg(feature = "azure")]
use crate::infrastructure::db::azure_kv_auth::AzureKeyVaultDBAuth;

#[actix_web::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    dotenv::dotenv().ok();


    #[cfg(feature = "dev")]
    let auth = EnvDBAuth;

    #[cfg(feature = "azure")]
    let auth = AzureKeyVaultDBAuth;

    let pool = connect_with_adapter(&auth).await;
    let pool = match pool {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to connect to the database: {}", e);
            return Err(e.into());
        }
    };


    Ok(())
}
