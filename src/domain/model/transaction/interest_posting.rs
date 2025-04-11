use uuid::Uuid;
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Clone, PartialEq)]
pub struct InterestPosting {
    pub id: Uuid,
    pub account_id: Uuid,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub interest_amount: f64,
    pub posted_at: NaiveDateTime,
    pub transaction_id: Option<Uuid>, // soft link to transaction
}