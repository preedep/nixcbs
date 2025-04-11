use uuid::Uuid;
use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionType {
    Deposit,
    Withdraw,
    TransferIn,
    TransferOut,
    Interest,
}

impl TransactionType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "deposit" => Some(Self::Deposit),
            "withdraw" => Some(Self::Withdraw),
            "transfer_in" => Some(Self::TransferIn),
            "transfer_out" => Some(Self::TransferOut),
            "interest" => Some(Self::Interest),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub id: Uuid,
    pub account_id: Uuid,
    pub txn_type: TransactionType,
    pub amount: f64,
    pub balance_after: f64,
    pub description: Option<String>,
    pub txn_time: NaiveDate,
}