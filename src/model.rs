use chrono::NaiveDate;
use macros::Getters;
use std::fmt::Display;

/// Represents kind of transaction.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TxKind {
    Income,
    Expence,
}

/// Represents transaction.
#[derive(Debug, PartialEq, Eq, Getters)]
pub struct Tx {
    date: NaiveDate,
    cat: String,
    kind: TxKind,
    amount: u64,
}

/// Represents result of comparison of two transaction lists.
#[derive(Debug)]
pub enum TxsCmpRes {
    /// Transaction lists completely identical.
    Equals,

    /// Transaction lists have different lengths.
    CountMismatch,

    /// Transaction lists differ.
    /// - index of first difference,
    /// - description of first field that differ.
    Different(usize, String),
}

impl Display for TxKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Income => write!(f, "Income"),
            Self::Expence => write!(f, "Expence"),
        }
    }
}

impl Tx {
    /// Creates an instance of `Tx`.\
    /// Panics when provided args are violate domain constraints:
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

/// Compares two lists of transactions.
/// Size of lists and order of items matter.
pub fn compare_txs(txs1: &Vec<Tx>, txs2: &Vec<Tx>) -> TxsCmpRes {
    if txs1.len() != txs2.len() {
        return TxsCmpRes::CountMismatch;
    }

    if let Some(first_diff) = txs1
        .iter()
        .zip(txs2)
        .enumerate()
        .find(|(_, (tx1, tx2))| tx1 != tx2)
    {
        let (idx, (tx1, tx2)) = first_diff;
        let reason = find_diff_field(tx1, tx2);
        return TxsCmpRes::Different(idx, reason);
    }

    TxsCmpRes::Equals
}

fn find_diff_field(tx1: &Tx, tx2: &Tx) -> String {
    if tx1.get_date() != tx2.get_date() {
        format!("Date: {} != {}", tx1.get_date(), tx2.get_date())
    } else if tx1.get_cat() != tx2.get_cat() {
        format!("Category: {} != {}", tx1.get_cat(), tx2.get_cat())
    } else if tx1.get_kind() != tx2.get_kind() {
        format!("Kind: {} != {}", tx1.get_kind(), tx2.get_kind())
    } else if tx1.get_amount() != tx2.get_amount() {
        format!("Amount: {} != {}", tx1.get_amount(), tx2.get_amount())
    } else {
        panic!("Values of the unknown field differ.");
    }
}
