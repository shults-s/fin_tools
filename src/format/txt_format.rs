use crate::error::FinToolsError;
use crate::format::{TxReader, TxWriter};
use crate::model::Tx;

pub(super) struct TxtTxReader;
pub(super) struct TxtTxWriter;

impl TxReader for TxtTxReader {
    fn read(&self, src: &dyn std::io::Read) -> Result<Vec<Tx>, FinToolsError> {
        todo!()
    }
}

impl TxWriter for TxtTxWriter {
    fn write(&self, dst: &dyn std::io::Write) -> Result<(), FinToolsError> {
        todo!()
    }
}
