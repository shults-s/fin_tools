use crate::error::FinToolsError;
use crate::format::{TxReader, TxWriter};
use crate::model::Tx;

pub(super) struct CsvTxReader;
pub(super) struct CsvTxWriter;

impl TxReader for CsvTxReader {
    fn read(&self, src: &dyn std::io::Read) -> Result<Vec<Tx>, FinToolsError> {
        todo!()
    }
}

impl TxWriter for CsvTxWriter {
    fn write(&self, dst: &dyn std::io::Write) -> Result<(), FinToolsError> {
        todo!()
    }
}
