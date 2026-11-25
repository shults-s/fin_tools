use crate::error::FinToolsError;
use crate::format::bin_format::{BinTxReader, BinTxWriter};
use crate::format::csv_format::{CsvTxReader, CsvTxWriter};
use crate::format::txt_format::{TxtTxReader, TxtTxWriter};
use crate::model::Tx;
use chrono::NaiveDate;
use std::io::{BufRead, Write};

mod bin_format;
mod csv_format;
mod txt_format;
mod utils;

// FOR_REVIEWER: `None` only when date is out of range, which is violation of an invariant.
type Days = u16;
const MAX_DATE: NaiveDate = NaiveDate::from_epoch_days(Days::MAX as i32).unwrap();
const MIN_DATE: NaiveDate = NaiveDate::from_epoch_days(Days::MIN as i32).unwrap();
const DATE_FMT_STR: &str = "%Y-%m-%d";
const CAT_MAX_SIZE: usize = u8::MAX as usize;

/// Represents formats, available for reading & writing transactions.
pub enum TxFormat {
    /// Represents binary format.
    Bin,

    /// Represents CSV format.
    Csv,

    /// Represents structured text format.
    Txt,
}

/// Gets transaction format from format's short name. Param `str` assumed lowercased.\
/// Returns `None` when no format corresponds to provided name.
pub fn get_format(str: impl AsRef<str>) -> Option<TxFormat> {
    match str.as_ref() {
        "ftx" => Some(TxFormat::Bin),
        "csv" => Some(TxFormat::Csv),
        "txt" => Some(TxFormat::Txt),
        _ => None,
    }
}

/// Represents abstract transactions reader.
pub trait TxReader {
    /// Reads collections list from `src` until EOF is reached.
    fn read(&self, src: &mut dyn BufRead) -> Result<Vec<Tx>, FinToolsError>;
}

/// Represents abstract transactions writer.
pub trait TxWriter {
    /// Writes collections list to `dst`.
    fn write(&self, dst: &mut dyn Write, txs: &[Tx]) -> Result<(), FinToolsError>;
}

/// Transaction reader factory.\
/// Returns an instance of a transactions list reader for provided format.
pub fn get_reader(format: TxFormat) -> Box<dyn TxReader> {
    // FOR_REVIEWER: using dynamic dispatching there feels like over-engineering, although...
    // I would appreciate some advice.
    match format {
        TxFormat::Bin => Box::new(BinTxReader),
        TxFormat::Csv => Box::new(CsvTxReader),
        TxFormat::Txt => Box::new(TxtTxReader),
    }
}

/// Transaction writer factory.\
/// Returns an instance of a transactions list writer for provided format.
pub fn get_writer(format: TxFormat) -> Box<dyn TxWriter> {
    match format {
        TxFormat::Bin => Box::new(BinTxWriter),
        TxFormat::Csv => Box::new(CsvTxWriter),
        TxFormat::Txt => Box::new(TxtTxWriter),
    }
}

#[cfg(test)]
mod tests {
    use crate::format::{TxFormat, get_reader, get_writer};
    use std::io::Cursor;

    #[test]
    fn test_converter_csv_to_bin_success() {
        let mut src = Cursor::new(
            r#"date,kind,amount,category
            2026-11-15,income,1000,meow
            2026-11-16,expence,700,eat
            "#,
        );
        let txs = get_reader(TxFormat::Csv).read(&mut src).unwrap();

        let mut dst = Cursor::new(Vec::<u8>::new());
        get_writer(TxFormat::Bin).write(&mut dst, &txs).unwrap();

        let expected: Vec<u8> = vec![
            70, 84, 1, 36, 81, 0, 232, 3, 0, 0, 0, 0, 0, 0, 4, 109, 101, 111, 119, 37, 81, 1, 188,
            2, 0, 0, 0, 0, 0, 0, 3, 101, 97, 116,
        ];
        let actual = dst.get_ref();

        assert_eq!(&expected, actual);
        //std::fs::write("./dbg.txt", format!("{dst:?}")).unwrap();
    }

    #[test]
    fn test_converter_csv_to_txt_success() {
        let mut src = Cursor::new(
            r#"date,kind,amount,category
            2026-11-15,income,1000,meow
            2026-11-16,expence,700,eat
            "#,
        );
        let txs = get_reader(TxFormat::Csv).read(&mut src).unwrap();

        let mut dst = Cursor::new(Vec::<u8>::new());
        get_writer(TxFormat::Txt).write(&mut dst, &txs).unwrap();

        let expected: Vec<u8> = b"2026-11-15;meow;income;1000\n2026-11-16;eat;expence;700\n".into();
        let actual = dst.get_ref();

        assert_eq!(&expected, actual);
    }
}
