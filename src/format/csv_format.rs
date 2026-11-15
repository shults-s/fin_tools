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
    fn write(&self, dst: &mut dyn std::io::Write, txs: &Vec<Tx>) -> Result<(), FinToolsError> {
        let header = ["date", "category", "kind", "amount"].join(FIELD_SEP);
        write_txs_as_text(dst, txs, Some(header), FIELD_SEP)
    }
}
