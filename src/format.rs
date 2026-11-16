use crate::error::FinToolsError;
use crate::format::bin_format::{BinTxReader, BinTxWriter};
use crate::format::csv_format::{CsvTxReader, CsvTxWriter};
use crate::format::txt_format::{TxtTxReader, TxtTxWriter};
use crate::model::Tx;
use chrono::NaiveDate;
use std::io::{Read, Write};

mod bin_format;
mod csv_format;
mod txt_format;
mod utils;

// FOR_REVIEWER: Format constraints, not a domain. None only if date is out of range.
const MAX_DATE: NaiveDate = NaiveDate::from_epoch_days(u16::MAX as i32).unwrap();
const MIN_DATE: NaiveDate = NaiveDate::from_epoch_days(0).unwrap();
const DATE_FMT_STR: &str = "%Y-%m-%d";
const CAT_MAX_SIZE: usize = 255;

/// Represents an formats, available for reading & writing transactions.
pub enum TxFormat {
    Bin,
    Csv,
    Txt,
}

/// Abstract transaction reader.
pub trait TxReader {
    fn read(&self, src: &mut dyn Read) -> Result<Vec<Tx>, FinToolsError>;
}

/// Abstract transaction writer.
pub trait TxWriter {
    fn write(&self, dst: &mut dyn Write, txs: &Vec<Tx>) -> Result<(), FinToolsError>;
}

/// Transaction reader factory.
pub fn get_reader(format: TxFormat) -> Box<dyn TxReader> {
    // FOR_REVIEWER: using dynamic dispatching there feels like over-engineering, although...
    match format {
        TxFormat::Bin => Box::new(BinTxReader),
        TxFormat::Csv => Box::new(CsvTxReader),
        TxFormat::Txt => Box::new(TxtTxReader),
    }
}

/// Transaction writer factory.
pub fn get_writer(format: TxFormat) -> Box<dyn TxWriter> {
    match format {
        TxFormat::Bin => Box::new(BinTxWriter),
        TxFormat::Csv => Box::new(CsvTxWriter),
        TxFormat::Txt => Box::new(TxtTxWriter),
    }
}
