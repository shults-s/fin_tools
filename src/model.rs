use chrono::NaiveDate;
use macros::Getters;

/// Represents the type of transaction.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TxKind {
    Income,
    Expence,
}

/// Represents the transaction.
#[derive(Debug, PartialEq, Eq, Getters)]
pub struct Tx {
    date: NaiveDate,
    cat: String,
    kind: TxKind,
    amount: u64,
}

impl Tx {
    /// Creates a new transaction.
    /// Panics if args are violate domain constraints:
    /// - `cat` is empty or is not an ASCII string;
    /// - `amount` is zero.
    pub fn new(date: NaiveDate, cat: String, kind: TxKind, amount: u64) -> Self {
        if !cat.is_ascii() {
            panic!("Transaction category must be an ASCII string.");
        }
        if cat.is_empty() {
            panic!("Transaction category can't be empty.");
        }
        if amount == 0 {
            panic!("Transaction amount must be positive.");
        }
        Self {
            date,
            cat: cat.to_ascii_lowercase(),
            kind,
            amount,
        }
    }
}
