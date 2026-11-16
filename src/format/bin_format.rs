use crate::error::{FinToolsError, parsing_err, serializing_err};
use crate::format::{CAT_MAX_SIZE, MAX_DATE, MIN_DATE, TxReader, TxWriter};
use crate::model::{Tx, TxKind};
use chrono::NaiveDate;
use std::io::{BufReader, BufWriter, Read, Write};

pub(super) struct BinTxReader;
pub(super) struct BinTxWriter;

// | Bytes |  0  |  1  |    2    |
// |:-----:|:---:|:---:|:-------:|
// |  Type |  u8 |  u8 |    u8   |
// | Field | 'F' | 'T' | Version |
const SIG_SIZE: usize = 3;
const FIXED_SIG_BYTES: &[u8; 2] = b"FT";
const VERSION: u8 = 1;

// | Bytes | 0..=1 |   2  | 3..=11 | 12 |  13..=l  |
// |:-----:|:-----:|:----:|:------:|:--:|:--------:|
// |  Type |  u16  |  u8  |   u64  | u8 |   &str   |
// | Field |  Date | Kind | Amount |  l | Category |
const ENTRY_SIZE: usize = 12;

// macro_rules! as_u16 {
//     ($slice:expr) => {
//         (($slice[0] as u16) << 8) | $slice[1] as u16
//     };
// }

#[inline]
fn get_kind_from_byte(byte: u8) -> Option<TxKind> {
    match byte {
        0u8 => Some(TxKind::Income),
        1u8 => Some(TxKind::Expence),
        _ => None,
    }
}

#[inline]
fn get_byte_from_kind(kind: TxKind) -> u8 {
    kind as u8
}

impl TxReader for BinTxReader {
    fn read(&self, src: &mut dyn std::io::Read) -> Result<Vec<Tx>, FinToolsError> {
        let mut txs = vec![];

        let mut reader = BufReader::new(src);

        let mut sig = [0u8; SIG_SIZE];
        reader.read_exact(&mut sig)?;

        if !sig[..2].eq(FIXED_SIG_BYTES) {
            return Err(parsing_err!("Bad signature bytes."));
        }

        if sig[2] != VERSION {
            return Err(parsing_err!(
                "Unsupported version. Expected <= {}, found {}.",
                VERSION,
                sig[2]
            ));
        }

        let mut entry_buf = [0u8; ENTRY_SIZE];
        let mut cat_buf = [0u8; CAT_MAX_SIZE];
        let mut offset = SIG_SIZE;
        loop {
            let readed = reader.read(&mut entry_buf)?;
            if readed == 0 {
                break; // EOF
            }

            if readed < ENTRY_SIZE {
                return Err(parsing_err!("Unexpected end of data."));
            }

            // FOR_REVIEWER: Here and throughout, inability to read values from a slice
            // is considered as a violation of an invariant (fucked idxs).
            let days = u16::from_le_bytes(entry_buf[..=1].try_into().unwrap());
            let date = NaiveDate::from_epoch_days(days as i32).ok_or_else(|| {
                parsing_err!(
                    "Date '{}' is out of range 0..={} for TX with offset {}.",
                    u16::MAX,
                    days,
                    offset
                )
            })?;

            let kind = get_kind_from_byte(entry_buf[2]).ok_or_else(|| {
                parsing_err!(
                    "Invalid transaction kind '{}' for TX with offset {}.",
                    entry_buf[2],
                    offset
                )
            })?;

            let amount = u64::from_le_bytes(entry_buf[3..=11].try_into().unwrap());
            if amount == 0 {
                return Err(parsing_err!(
                    "Invalid amount '{}' for TX with offset {}. Amount must be positive.",
                    amount,
                    offset,
                ));
            }

            let cat_len = entry_buf[12] as usize;
            if cat_len == 0 || cat_len > CAT_MAX_SIZE {
                return Err(parsing_err!(
                    "Invalid category len '{}' for TX with offset {}. Length must be in 1..={}.",
                    cat_len,
                    offset,
                    CAT_MAX_SIZE
                ));
            }

            reader.read_exact(&mut cat_buf[..cat_len])?;
            let cat = String::from_utf8_lossy(&cat_buf[..cat_len]).to_string();
            if !cat.is_ascii() {
                return Err(parsing_err!(
                    "Invalid category '{}' for TX with offset {}. Allowed ASCII characters only.",
                    cat,
                    offset
                ));
            }

            offset += ENTRY_SIZE + cat_len;

            txs.push(Tx::new(date, cat, kind, amount));
        }

        Ok(txs)
    }
}

impl TxWriter for BinTxWriter {
    fn write(&self, dst: &mut dyn std::io::Write, txs: &Vec<Tx>) -> Result<(), FinToolsError> {
        let mut writer = BufWriter::new(dst);

        for (idx, tx) in txs.iter().enumerate() {
            let tx_num = idx + 1;

            let date = *tx.get_date();
            if date < MIN_DATE || date > MAX_DATE {
                return Err(serializing_err!(
                    "Unable to serialize date outside of range {}..={}, given {} for TX #{}.",
                    MIN_DATE,
                    MAX_DATE,
                    date,
                    tx_num
                ));
            }

            let days = (date.to_epoch_days() as u16).to_le_bytes();
            writer.write(&days)?;

            let kind = get_byte_from_kind(*tx.get_kind()).to_le_bytes();
            writer.write(&kind)?;

            let amount = tx.get_amount().to_le_bytes();
            writer.write(&amount)?;

            let cat = tx.get_cat().as_bytes();
            let l = cat.len().to_le_bytes();
            if cat.len() > CAT_MAX_SIZE {
                return Err(serializing_err!(
                    "Unable to serialize category longer than {} characters, given {} for TX #{}.",
                    CAT_MAX_SIZE,
                    cat.len(),
                    tx_num
                ));
            }
            writer.write(&l)?;
            writer.write(&cat)?;
        }

        writer.flush()?;
        Ok(())
    }
}
