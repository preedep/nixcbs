use uuid::Uuid;
use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionType {
    Deposit,// deposit
    Withdraw,// withdraw
    TransferIn,//Transfer between accounts
    TransferOut,// Transfer between accounts
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

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use chrono::NaiveDate;

    fn sample_uuid() -> Uuid {
        Uuid::new_v4()
    }

    fn sample_date() -> NaiveDate {
        NaiveDate::from_ymd_opt(2023, 10, 1).unwrap()
    }

    #[test]
    fn from_str_returns_correct_transaction_type_for_valid_strings() {
        assert_eq!(TransactionType::from_str("deposit"), Some(TransactionType::Deposit));
        assert_eq!(TransactionType::from_str("withdraw"), Some(TransactionType::Withdraw));
        assert_eq!(TransactionType::from_str("transfer_in"), Some(TransactionType::TransferIn));
        assert_eq!(TransactionType::from_str("transfer_out"), Some(TransactionType::TransferOut));
        assert_eq!(TransactionType::from_str("interest"), Some(TransactionType::Interest));
    }

    #[test]
    fn from_str_returns_none_for_invalid_strings() {
        assert_eq!(TransactionType::from_str("invalid"), None);
        assert_eq!(TransactionType::from_str(""), None);
        assert_eq!(TransactionType::from_str("123"), None);
    }

    #[test]
    fn transaction_fields_are_set_correctly() {
        let txn = Transaction {
            id: sample_uuid(),
            account_id: sample_uuid(),
            txn_type: TransactionType::Deposit,
            amount: 100.0,
            balance_after: 200.0,
            description: Some("Initial deposit".to_string()),
            txn_time: sample_date(),
        };

        assert_eq!(txn.txn_type, TransactionType::Deposit);
        assert_eq!(txn.amount, 100.0);
        assert_eq!(txn.balance_after, 200.0);
        assert_eq!(txn.description, Some("Initial deposit".to_string()));
        assert_eq!(txn.txn_time, sample_date());
    }

    #[test]
    fn transaction_allows_none_description() {
        let txn = Transaction {
            id: sample_uuid(),
            account_id: sample_uuid(),
            txn_type: TransactionType::Withdraw,
            amount: 50.0,
            balance_after: 150.0,
            description: None,
            txn_time: sample_date(),
        };

        assert_eq!(txn.description, None);
    }
}