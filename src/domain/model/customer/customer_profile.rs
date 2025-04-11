#[derive(Debug, Clone, PartialEq)]
pub struct CustomerProfile {
    pub customer_id: Uuid,
    pub full_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub synced_at: Option<NaiveDateTime>,
}