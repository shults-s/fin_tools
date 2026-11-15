use crate::error::FinToolsError;
use crate::format::DATE_FMT_STR;
use crate::model::{TX_FIELDS_NUM, Tx, TxKind};
use chrono::NaiveDate;
use std::io::{BufRead, BufReader, BufWriter, Write};

pub(super) fn write_txs_as_text(
    dst: &mut dyn std::io::Write,
    txs: &Vec<Tx>,
    header: Option<String>,
    field_sep: &str,
) -> Result<(), FinToolsError> {
    let mut writer = BufWriter::new(dst);

    if let Some(header) = header {
        writeln!(writer, "{header}")?;
    }

    for tx in txs {
        writeln!(
            writer,
            "{1}{0}{2}{0}{3}{0}{4}",
            field_sep,
            tx.get_date().format(DATE_FMT_STR),
            tx.get_cat(),
            format!("{:?}", tx.get_kind()).to_lowercase(),
            tx.get_amount()
        )?;
    }

    writer.flush()?;
    Ok(())
}

pub(super) fn read_txs_from_text(
    src: &mut dyn std::io::Read,
    has_header: bool,
    field_sep: &str,
) -> Result<Vec<Tx>, FinToolsError> {
    let mut txs = vec![];

    for (idx, line) in BufReader::new(src).lines().enumerate() {
        if has_header && idx == 0 {
            continue;
        }

        let line_num = idx + 1;
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let fields = line.split(field_sep).map(str::trim).collect::<Vec<_>>();
        if fields.len() != TX_FIELDS_NUM {
            let msg = format!(
                "Invalid entry format at line {}. Expected {} fields, found {}.",
                line_num,
                TX_FIELDS_NUM,
                fields.len()
            );
            return Err(FinToolsError::ParsingError(msg));
        }

        let date = NaiveDate::parse_from_str(fields[0], DATE_FMT_STR).map_err(|_| {
            FinToolsError::ParsingError(format!(
                "Invalid date format at line {}. Expected format is YYYY-MM-DD.",
                line_num
            ))
        })?;

        let cat = fields[1].to_ascii_lowercase();
        if !cat.is_ascii() {
            let msg = format!(
                "Invalid category at line {}. Allowed ASCII characters only.",
                line_num,
            );
            return Err(FinToolsError::ParsingError(msg));
        }

        let kind = match fields[2].to_ascii_lowercase().as_str() {
            "income" => Ok(TxKind::Income),
            "expence" => Ok(TxKind::Expence),
            _ => {
                let msg = format!("Invalid transaction kind at line {}.", line_num);
                Err(FinToolsError::ParsingError(msg))
            }
        }?;

        let amount = fields[3].parse::<u64>().map_err(|_| {
            let msg = format!("Invalid amount at line {}. Expected 64-bit uint.", line_num);
            FinToolsError::ParsingError(msg)
        })?;

        txs.push(Tx::new(date, cat, kind, amount));
    }

    Ok(txs)
}
