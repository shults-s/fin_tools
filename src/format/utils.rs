use crate::error::{FinToolsError, parsing_err};
use crate::format::DATE_FMT_STR;
use crate::model::{Tx, TxKind};
use chrono::NaiveDate;
use std::io::{BufRead, BufReader, BufWriter, Write};

pub(crate) const TX_FIELDS_NUM: usize = 4;

#[inline]
fn get_kind_from_str(str: &str) -> Option<TxKind> {
    match str.to_ascii_lowercase().as_str() {
        "income" => Some(TxKind::Income),
        "expence" => Some(TxKind::Expence),
        _ => None,
    }
}

#[inline]
fn get_str_from_kind(kind: TxKind) -> String {
    format!("{:?}", kind).to_lowercase()
}

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
            get_str_from_kind(*tx.get_kind()),
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
            return Err(parsing_err!(
                "Invalid TX format at line {}. Expected {} fields, found {}.",
                line_num,
                TX_FIELDS_NUM,
                fields.len()
            ));
        }

        let date = NaiveDate::parse_from_str(fields[0], DATE_FMT_STR).map_err(|_| {
            parsing_err!(
                "Invalid date '{}' at line {}. Expected format is YYYY-MM-DD.",
                fields[0],
                line_num
            )
        })?;

        let cat = fields[1].to_ascii_lowercase();
        if !cat.is_ascii() || cat.len() == 0 {
            return Err(parsing_err!(
                "Invalid category '{}' at line {}. Expected non empty ASCII string.",
                cat,
                line_num
            ));
        }

        let kind = get_kind_from_str(fields[2]).ok_or_else(|| {
            parsing_err!(
                "Invalid transaction kind '{}' at line {}.",
                fields[2],
                line_num
            )
        })?;

        let amount = fields[3].parse::<u64>().map_err(|_| {
            parsing_err!(
                "Invalid amount '{}' at line {}. Expected positive integer.",
                fields[3],
                line_num
            )
        })?;
        if amount == 0 {
            return Err(parsing_err!(
                "Invalid amount '{}' at line {}. Amount must be positive.",
                amount,
                line_num
            ));
        }

        txs.push(Tx::new(date, cat, kind, amount));
    }

    Ok(txs)
}
