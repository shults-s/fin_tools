use crate::error::FinToolsError;
use crate::format::utils::{read_txs_from_text, write_txs_as_text};
use crate::format::{TxReader, TxWriter};
use crate::model::Tx;

pub(super) struct TxtTxReader;
pub(super) struct TxtTxWriter;

const FIELD_SEP: &str = ";";

impl TxReader for TxtTxReader {
    fn read(&self, src: &mut dyn std::io::Read) -> Result<Vec<Tx>, FinToolsError> {
        read_txs_from_text(src, false, FIELD_SEP)
    }
}

impl TxWriter for TxtTxWriter {
    fn write(&self, dst: &mut dyn std::io::Write, txs: &Vec<Tx>) -> Result<(), FinToolsError> {
        write_txs_as_text(dst, txs, false, FIELD_SEP)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::FinToolsError;
    use crate::format::{TxReader, TxtTxReader};
    use crate::model::{Tx, TxKind};
    use chrono::NaiveDate;
    use std::io::Cursor;

    #[test]
    fn test_txt_format_reader_success() {
        let mut src = Cursor::new(
            r#"2026-11-16;eat;expence;700
            2026-11-15;meow;income;1000
            "#,
        );
        let expected = vec![
            Tx::new(
                NaiveDate::from_ymd_opt(2026, 11, 16).unwrap(),
                "eat".into(),
                TxKind::Expence,
                700,
            ),
            Tx::new(
                NaiveDate::from_ymd_opt(2026, 11, 15).unwrap(),
                "meow".into(),
                TxKind::Income,
                1000,
            ),
        ];
        let actual = TxtTxReader.read(&mut src).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_txt_format_reader_wrong_column_order_failure() {
        let mut src = Cursor::new(
            r#"2026-11-16;expence;700;eat
            2026-11-15;income;1000;meow
            "#,
        );
        let actual = TxtTxReader.read(&mut src).unwrap_err();
        assert!(matches!(actual, FinToolsError::ParsingError(_)));
    }
}
