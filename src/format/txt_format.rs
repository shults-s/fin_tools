use crate::error::FinToolsError;
use crate::format::utils::{read_tx_from_text, write_tx_as_text};
use crate::format::{TxReader, TxWriter};
use crate::model::Tx;

pub(super) struct TxtTxReader;
pub(super) struct TxtTxWriter;

const FIELD_SEP: char = ';';

impl TxReader for TxtTxReader {
    fn read(&self, src: &mut dyn std::io::Read) -> Result<Vec<Tx>, FinToolsError> {
        read_tx_from_text(src, false, FIELD_SEP)
    }
}

impl TxWriter for TxtTxWriter {
    fn write(&self, dst: &mut dyn std::io::Write, txs: &Vec<Tx>) -> Result<(), FinToolsError> {
        write_tx_as_text(dst, txs, None, FIELD_SEP)
    }
}
