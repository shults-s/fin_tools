use crate::error::{FinToolsError, validation_err};
use chrono::NaiveDate;
use macros::Getters;
use std::{fmt::Display, num::NonZeroU64};

/// Represents kind of transaction.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TxKind {
    /// Represents an inflow of funds.
    Income,

    /// Represents an outflow of funds.
    Expense,
}

/// Represents transaction category.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxCat(String);

/// Represents transaction.
#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct Tx {
    date: NaiveDate,
    cat: TxCat,
    kind: TxKind,
    amount: NonZeroU64,
}

/// Represents result of comparison of two transaction lists.
#[derive(Debug, PartialEq, Eq)]
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

impl Display for TxCat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TxCat {
    /// Creates an instance of `TxCat`.\
    /// Returns an error when `cat` is empty or is not an ASCII string.
    pub fn try_new(cat: String) -> Result<Self, FinToolsError> {
        if !cat.is_ascii() {
            return Err(validation_err!(
                "Transaction category must be an ASCII string."
            ));
        }
        if cat.is_empty() {
            return Err(validation_err!("Transaction category can't be empty."));
        }
        Ok(Self { 0: cat })
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Tx {
    /// Creates an instance of `Tx`.\
    /// Returns an error when `TxCat` can't be constructed from `cat`.
    pub fn try_new(
        date: NaiveDate,
        cat: String,
        kind: TxKind,
        amount: NonZeroU64,
    ) -> Result<Self, FinToolsError> {
        let cat = TxCat::try_new(cat)?;
        Ok(Self {
            date,
            cat,
            kind,
            amount,
        })
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

#[cfg(test)]
mod tests {
    use std::num::NonZeroU64;

    use crate::model::{Tx, TxKind, TxsCmpRes, compare_txs};
    use chrono::NaiveDate;

    #[test]
    fn test_comparer_equals() {
        let txs1 = vec![
            Tx::try_new(
                NaiveDate::from_ymd_opt(2026, 11, 16).unwrap(),
                "eat".into(),
                TxKind::Expence,
                NonZeroU64::new(700).unwrap(),
            )
            .unwrap(),
            Tx::try_new(
                NaiveDate::from_ymd_opt(2026, 11, 15).unwrap(),
                "meow".into(),
                TxKind::Income,
                NonZeroU64::new(1000).unwrap(),
            )
            .unwrap(),
        ];
        let txs2 = txs1.clone();

        assert_eq!(TxsCmpRes::Equals, compare_txs(&txs1, &txs2));
    }

    #[test]
    fn test_comparer_count_mismatch() {
        let txs1 = vec![
            Tx::try_new(
                NaiveDate::from_ymd_opt(2026, 11, 16).unwrap(),
                "eat".into(),
                TxKind::Expence,
                NonZeroU64::new(700).unwrap(),
            )
            .unwrap(),
            Tx::try_new(
                NaiveDate::from_ymd_opt(2026, 11, 15).unwrap(),
                "meow".into(),
                TxKind::Income,
                NonZeroU64::new(1000).unwrap(),
            )
            .unwrap(),
        ];
        let txs2 = txs1.iter().skip(1).cloned().collect::<Vec<_>>();

        assert_eq!(TxsCmpRes::CountMismatch, compare_txs(&txs1, &txs2));
    }

    #[test]
    fn test_comparer_different() {
        let txs1 = vec![
            Tx::try_new(
                NaiveDate::from_ymd_opt(2026, 11, 16).unwrap(),
                "eat".into(),
                TxKind::Expence,
                NonZeroU64::new(700).unwrap(),
            )
            .unwrap(),
            Tx::try_new(
                NaiveDate::from_ymd_opt(2026, 11, 15).unwrap(),
                "meow".into(),
                TxKind::Income,
                NonZeroU64::new(1000).unwrap(),
            )
            .unwrap(),
        ];

        let txs2 = vec![
            Tx::try_new(
                NaiveDate::from_ymd_opt(2026, 11, 16).unwrap(),
                "eat".into(),
                TxKind::Expence,
                NonZeroU64::new(700).unwrap(),
            )
            .unwrap(),
            Tx::try_new(
                NaiveDate::from_ymd_opt(2026, 11, 15).unwrap(),
                "meow".into(),
                TxKind::Income,
                NonZeroU64::new(2300).unwrap(),
            )
            .unwrap(),
        ];

        assert!(matches!(
            compare_txs(&txs1, &txs2),
            TxsCmpRes::Different(1, _)
        ));
    }
}
