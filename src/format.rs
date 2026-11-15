use crate::error::FinToolsError;
use crate::format::bin_format::{BinTxReader, BinTxWriter};
use crate::format::csv_format::{CsvTxReader, CsvTxWriter};
use crate::format::txt_format::{TxtTxReader, TxtTxWriter};
use crate::model::Tx;

use std::io::{Read, Write};

mod bin_format;
mod csv_format;
mod txt_format;

/// Represents an formats, available for reading & writing transactions.
pub enum TxFormat {
    Bin,
    Csv,
    Txt,
}

/// Abstract transaction reader.
pub trait TxReader {
    fn read(&self, src: &dyn Read) -> Result<Vec<Tx>, FinToolsError>;
}

/// Abstract transaction writer.
pub trait TxWriter {
    fn write(&self, dst: &dyn Write) -> Result<(), FinToolsError>;
}

/// Transaction reader factory.
pub fn get_reader_for(format: TxFormat) -> Box<dyn TxReader> {
    match format {
        TxFormat::Bin => Box::new(BinTxReader),
        TxFormat::Csv => Box::new(CsvTxReader),
        TxFormat::Txt => Box::new(TxtTxReader),
    }
}

/// Transaction writer factory.
pub fn get_writer_for(format: TxFormat) -> Box<dyn TxWriter> {
    match format {
        TxFormat::Bin => Box::new(BinTxWriter),
        TxFormat::Csv => Box::new(CsvTxWriter),
        TxFormat::Txt => Box::new(TxtTxWriter),
    }
}
