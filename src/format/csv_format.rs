use crate::error::FinToolsError;
use crate::format::utils::{read_tx_from_text, write_tx_as_text};
use crate::format::{TxReader, TxWriter};
use crate::model::Tx;

pub(super) struct CsvTxReader;
pub(super) struct CsvTxWriter;

const FIELD_SEP: char = ',';

impl TxReader for CsvTxReader {
    fn read(&self, src: &mut dyn std::io::Read) -> Result<Vec<Tx>, FinToolsError> {
        read_tx_from_text(src, true, FIELD_SEP)
    }
}

impl TxWriter for CsvTxWriter {
    fn write(&self, dst: &mut dyn std::io::Write, txs: &Vec<Tx>) -> Result<(), FinToolsError> {
        write_tx_as_text(
            dst,
            txs,
            Some("date,category,kind,amount".into()),
            FIELD_SEP,
        )
    }
}
