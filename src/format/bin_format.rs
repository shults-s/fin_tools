use crate::error::FinToolsError;
use crate::format::{TxReader, TxWriter};
use crate::model::Tx;

pub(super) struct BinTxReader;
pub(super) struct BinTxWriter;

const SIG: &[u8; 2] = b"FT";

impl TxReader for BinTxReader {
    fn read(&self, src: &dyn std::io::Read) -> Result<Vec<Tx>, FinToolsError> {
        //let txs = Vec::with_capacity(N);
        todo!()
    }
}

impl TxWriter for BinTxWriter {
    fn write(&self, dst: &dyn std::io::Write) -> Result<(), FinToolsError> {
        todo!()
    }
}
