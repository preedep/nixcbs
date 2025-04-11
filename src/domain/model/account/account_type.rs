use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq)]
pub struct AccountType {
    pub id: i16,
    pub name: String,
    pub interest_rate: f64,
    pub minimum_balance: f64,
    pub created_at: Option<NaiveDateTime>,
}