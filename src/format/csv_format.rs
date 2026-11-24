use crate::error::FinToolsError;
use crate::format::utils::{read_txs_from_text, write_txs_as_text};
use crate::format::{TxReader, TxWriter};
use crate::model::Tx;

pub(super) struct CsvTxReader;
pub(super) struct CsvTxWriter;

const FIELD_SEP: &str = ",";

impl TxReader for CsvTxReader {
    fn read(&self, src: &mut dyn std::io::Read) -> Result<Vec<Tx>, FinToolsError> {
        read_txs_from_text(src, true, FIELD_SEP)
    }
}

impl TxWriter for CsvTxWriter {
    fn write(&self, dst: &mut dyn std::io::Write, txs: &[Tx]) -> Result<(), FinToolsError> {
        write_txs_as_text(dst, txs, true, FIELD_SEP)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::FinToolsError;
    use crate::format::{CsvTxReader, TxReader};
    use crate::model::{Tx, TxKind};
    use chrono::NaiveDate;
    use std::io::Cursor;
    use std::num::NonZeroU64;

    #[test]
    fn test_csv_format_reader_arbitary_column_order_success() {
        let mut src = Cursor::new(
            r#"date,kind,amount,category
            2026-11-16,expence,700,eat
            2026-11-15,income,1000,meow
            "#,
        );
        let expected = vec![
            Tx::try_new(
                NaiveDate::from_ymd_opt(2026, 11, 16).unwrap(),
                "eat".into(),
                TxKind::Expense,
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
        let actual = CsvTxReader.read(&mut src).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_csv_format_reader_missing_header_failure() {
        let mut src = Cursor::new(
            r#"2026-11-15,meow,income,1000
            2026-11-16,eat,expence,700"#,
        );
        let actual = CsvTxReader.read(&mut src).unwrap_err();
        assert!(matches!(actual, FinToolsError::ParsingError(_)));
    }
}
