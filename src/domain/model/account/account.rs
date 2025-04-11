use uuid::Uuid;
use chrono::{NaiveDateTime};
use super::account_type::AccountType;

#[derive(Debug, Clone, PartialEq)]
pub enum AccountStatus {
    Active,
    Closed,
    Frozen,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    pub id: Uuid,
    pub account_number: String,
    pub customer_id: Uuid,
    pub account_type_id: i16,
    pub account_type: Option<AccountType>, // lazy loading (optional)
    pub balance: f64,
    pub status: AccountStatus,
    pub opened_at: NaiveDateTime,
    pub closed_at: Option<NaiveDateTime>,
}