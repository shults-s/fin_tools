use chrono::NaiveDate;

/// Represents the type of transaction.
#[derive(Debug, PartialEq, Eq)]
pub enum TxKind {
    Income,
    Expence,
}

/// Represents the transaction.
#[derive(Debug, PartialEq, Eq)]
pub struct Tx {
    date: NaiveDate,
    cat: String,
    kind: TxKind,
    amount: u64,
}

impl Tx {
    /// Creates a new transaction. Panics if `cat` is not an ASCII string.
    pub fn new(date: NaiveDate, cat: String, kind: TxKind, amount: u64) -> Self {
        if !cat.is_ascii() {
            panic!("Transaction category must be an ASCII string.");
        }
        Self {
            date,
            cat: cat.to_ascii_lowercase(),
            kind,
            amount,
        }
    }
}
