use crate::error::{FinToolsError, parsing_err, serializing_err, validation_err};
use crate::format::{CAT_MAX_SIZE, Days, MAX_DATE, MIN_DATE, TxReader, TxWriter};
use crate::model::{Tx, TxKind};
use chrono::NaiveDate;
use std::io::{BufReader, BufWriter, ErrorKind, Read, Write};
use std::num::NonZeroU64;

pub(super) struct BinTxReader;
pub(super) struct BinTxWriter;

// | Bytes |  0  |  1  |    2    |
// |:-----:|:---:|:---:|:-------:|
// |  Type |  u8 |  u8 |    u8   |
// | Field | 'F' | 'T' | Version |
const SIG_SIZE: usize = 3;
const FIXED_SIG_BYTES: &[u8; 2] = b"FT";
const VERSION: u8 = 1;
const SIG_BYTES: &[u8; 3] = &[FIXED_SIG_BYTES[0], FIXED_SIG_BYTES[1], VERSION];

// | Bytes | 0..=1 |   2  | 3..=10 | 11 |   13..l  |
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
    // FOR_REVIEWER: For prod should be used explicit match kind => u8 instead of
    // relying on order of values in enum, but I cover this in tests for now.
    kind as u8
}

impl TxReader for BinTxReader {
    fn read(&self, src: &mut dyn std::io::Read) -> Result<Vec<Tx>, FinToolsError> {
        let mut txs = vec![];

        let mut reader = BufReader::new(src);

        let mut sig = [0u8; SIG_SIZE];
        reader
            .read_exact(&mut sig)
            .map_err(FinToolsError::ReadError)?;

        if sig[..2].as_ref() != FIXED_SIG_BYTES {
            return Err(parsing_err!("Bad FTX signature bytes."));
        }

        if sig[2] != VERSION {
            return Err(parsing_err!(
                "Unsupported FTX version. Expected = {}, found {}.",
                VERSION,
                sig[2]
            ));
        }

        let mut entry_buf = [0u8; ENTRY_SIZE];
        let mut cat_buf = [0u8; CAT_MAX_SIZE];
        let mut offset = SIG_SIZE;
        loop {
            if let Err(e) = reader.read_exact(&mut entry_buf[..1]) {
                match e.kind() {
                    ErrorKind::UnexpectedEof => break,
                    _ => return Err(FinToolsError::WriteError(e)),
                }
            }

            reader
                .read_exact(&mut entry_buf[1..])
                .map_err(FinToolsError::ReadError)?;

            let [d0, d1, kind, a0, a1, a2, a3, a4, a5, a6, a7, l] = entry_buf;

            let days = Days::from_le_bytes([d0, d1]);
            let date = NaiveDate::from_epoch_days(days as i32).ok_or_else(|| {
                parsing_err!(
                    "Invalid transaction date '{}' for TX with offset {}. Value is out of range 0..={}.",
                    days,
                    offset,
                    Days::MAX
                )
            })?;

            let kind = get_kind_from_byte(kind).ok_or_else(|| {
                parsing_err!(
                    "Invalid transaction kind '{}' for TX with offset {}.",
                    kind,
                    offset
                )
            })?;

            let amount = u64::from_le_bytes([a0, a1, a2, a3, a4, a5, a6, a7]);
            let Some(amount) = NonZeroU64::new(amount) else {
                return Err(parsing_err!(
                    "Invalid transaction amount '{}' for TX with offset {}. Value must not be zero.",
                    amount,
                    offset,
                ));
            };

            let cat_len = l as usize;
            if cat_len == 0 {
                return Err(parsing_err!(
                    "Invalid transaction category len for TX with offset {}. Value must not be zero.",
                    offset,
                ));
            }

            reader
                .read_exact(&mut cat_buf[..cat_len])
                .map_err(FinToolsError::ReadError)?;
            let cat_bytes = (&cat_buf[..cat_len]).iter().copied().collect::<Vec<_>>();
            let cat = match String::from_utf8(cat_bytes) {
                Ok(str) => str,
                Err(err) => return Err(parsing_err!("{}", err)),
            };
            if !cat.is_ascii() {
                return Err(parsing_err!(
                    "Invalid transaction category '{}' for TX with offset {}. Expected ASCII string.",
                    cat,
                    offset
                ));
            }

            offset += ENTRY_SIZE + cat_len;

            match Tx::try_new(date, cat, kind, amount) {
                Ok(tx) => txs.push(tx),
                Err(err) => {
                    return Err(validation_err!(
                        "Invalid transaction field for TX with offset {}. {}",
                        offset,
                        err
                    ));
                }
            };
        }

        Ok(txs)
    }
}

impl TxWriter for BinTxWriter {
    fn write(&self, dst: &mut dyn std::io::Write, txs: &Vec<Tx>) -> Result<(), FinToolsError> {
        let mut writer = BufWriter::new(dst);

        writer
            .write_all(SIG_BYTES)
            .map_err(FinToolsError::WriteError)?;

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
            writer.write_all(&days).map_err(FinToolsError::WriteError)?;

            let kind = get_byte_from_kind(*tx.get_kind()).to_le_bytes();
            writer.write_all(&kind).map_err(FinToolsError::WriteError)?;

            let amount = tx.get_amount().get().to_le_bytes();
            writer
                .write_all(&amount)
                .map_err(FinToolsError::WriteError)?;

            let cat = tx.get_cat().as_str();
            let l = cat.len();
            if cat.len() > CAT_MAX_SIZE {
                return Err(serializing_err!(
                    "Unable to serialize category longer than {} characters, given {} for TX #{}.",
                    CAT_MAX_SIZE,
                    cat.len(),
                    tx_num
                ));
            }
            let l = (l as u8).to_le_bytes();
            let cat = cat.as_bytes();
            writer.write_all(&l).map_err(FinToolsError::WriteError)?;
            writer.write_all(&cat).map_err(FinToolsError::WriteError)?;
        }

        writer.flush().map_err(FinToolsError::WriteError)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::error::FinToolsError;
    use crate::format::{BinTxReader, TxReader};
    use crate::model::{Tx, TxKind};
    use chrono::NaiveDate;
    use std::io::Cursor;
    use std::num::NonZeroU64;

    #[test]
    fn test_bin_format_reader_success() {
        let mut src = Cursor::new([
            70, 84, 1, 37, 81, 1, 188, 2, 0, 0, 0, 0, 0, 0, 3, 101, 97, 116, 36, 81, 0, 232, 3, 0,
            0, 0, 0, 0, 0, 4, 109, 101, 111, 119,
        ]);
        let expected = vec![
            Tx::try_new(
                NaiveDate::from_ymd_opt(2026, 11, 16).unwrap(),
                "eat".into(),
                TxKind::Expence,
                NonZeroU64::new(700).unwrap(),
            )
            .unwrap(),
            Tx::try_new(
                NaiveDate::from_ymd_opt(2026, 11, 15).unwrap(),
                "meow".into(),
                TxKind::Income,
                NonZeroU64::new(1000).unwrap(),
            )
            .unwrap(),
        ];
        let actual = BinTxReader.read(&mut src).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_bin_format_reader_missing_signature_failure() {
        let mut src = Cursor::new([
            37, 81, 1, 188, 2, 0, 0, 0, 0, 0, 0, 3, 101, 97, 116, 36, 81, 0, 232, 3, 0, 0, 0, 0, 0,
            0, 4, 109, 101, 111, 119,
        ]);
        let actual = BinTxReader.read(&mut src).unwrap_err();
        assert!(matches!(actual, FinToolsError::ParsingError(_)));
    }
}
