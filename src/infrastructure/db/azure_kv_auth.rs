use async_trait::async_trait;
use crate::infrastructure::db::auth::{DBAuthAdapter, DBCredentials};


pub struct AzureKeyVaultDBAuth;

#[async_trait]
impl DBAuthAdapter for AzureKeyVaultDBAuth {
    async fn get_credentials(&self) -> anyhow::Result<DBCredentials> {
        // ใช้ Azure SDK ดึง secret จาก Key Vault ด้วย Managed Identity
        // เช่นผ่าน azure_identity + azure_key_vault
        todo!("fetch secret from Azure Key Vault")
    }
}