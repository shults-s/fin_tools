use crate::error::FinToolsError;
use crate::format::{TxReader, TxWriter};
use crate::model::Tx;

pub(super) struct BinTxReader;
pub(super) struct BinTxWriter;

const SIG: &[u8; 2] = b"FT";

impl TxReader for BinTxReader {
    fn read(&self, src: &mut dyn std::io::Read) -> Result<Vec<Tx>, FinToolsError> {
        todo!()
    }
}

impl TxWriter for BinTxWriter {
    fn write(&self, dst: &mut dyn std::io::Write, txs: &Vec<Tx>) -> Result<(), FinToolsError> {
        todo!()
    }
}
