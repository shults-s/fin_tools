use crate::error::{FinToolsError, parsing_err, validation_err};
use crate::format::DATE_FMT_STR;
use crate::model::{Tx, TxKind};
use chrono::NaiveDate;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::num::NonZeroU64;

const COLUMNS_NUM: usize = 4;
const DEFAULT_COLUMNS: [&str; COLUMNS_NUM] = ["date", "category", "kind", "amount"];

#[inline]
fn get_column_idx(col_name: &str, columns: &[impl AsRef<str>]) -> Result<usize, FinToolsError> {
    columns
        .into_iter()
        .position(|col| col.as_ref() == col_name)
        .ok_or_else(|| parsing_err!("Column '{}' not found.", col_name))
}

#[inline]
fn get_kind_from_str(str: &str) -> Option<TxKind> {
    match str.to_ascii_lowercase().as_str() {
        "income" => Some(TxKind::Income),
        "expence" => Some(TxKind::Expence),
        _ => None,
    }
}

#[inline]
fn get_str_from_kind(kind: &TxKind) -> String {
    format!("{kind}").to_ascii_lowercase()
}

pub(super) fn write_txs_as_text(
    dst: &mut dyn std::io::Write,
    txs: &[Tx],
    add_header: bool,
    field_sep: &str,
) -> Result<(), FinToolsError> {
    let mut writer = BufWriter::new(dst);

    if add_header {
        let header = DEFAULT_COLUMNS.join(field_sep);
        writeln!(writer, "{header}").map_err(FinToolsError::WriteError)?;
    }

    for tx in txs {
        writeln!(
            writer,
            "{1}{0}{2}{0}{3}{0}{4}",
            field_sep,
            tx.get_date().format(DATE_FMT_STR),
            tx.get_cat(),
            get_str_from_kind(tx.get_kind()),
            tx.get_amount()
        )
        .map_err(FinToolsError::WriteError)?;
    }

    writer.flush().map_err(FinToolsError::WriteError)?;
    Ok(())
}

pub(super) fn read_txs_from_text(
    src: &mut dyn std::io::Read,
    has_header: bool,
    field_sep: &str,
) -> Result<Vec<Tx>, FinToolsError> {
    let mut txs = vec![];

    let mut lines = BufReader::new(src).lines();

    let columns = if has_header && let Some(header) = lines.next() {
        // FOR_REVIEWER: I don't like this stupid allocations, but I didn't come up
        // with a more idiomatic way to read header on-demand.
        // I would appreciate some advice.
        header
            .map_err(FinToolsError::ReadError)?
            .split(field_sep)
            .map(|column| column.trim().to_ascii_lowercase())
            .collect::<Vec<_>>()
    } else {
        DEFAULT_COLUMNS
            .iter()
            .map(|column| column.to_string())
            .collect::<Vec<_>>()
    };

    let date_idx = get_column_idx("date", &columns)?;
    let cat_idx = get_column_idx("category", &columns)?;
    let kind_idx = get_column_idx("kind", &columns)?;
    let amount_idx = get_column_idx("amount", &columns)?;

    let mut line_num = if has_header { 1 } else { 0 };
    for line in lines {
        line_num += 1;

        let line = line.map_err(FinToolsError::ReadError)?;
        if line.trim().is_empty() {
            continue;
        }

        let fields = line.split(field_sep).map(str::trim).collect::<Vec<_>>();
        if fields.len() != COLUMNS_NUM {
            return Err(parsing_err!(
                "Invalid transaction format at line {}. Expected {} fields, found {}.",
                line_num,
                COLUMNS_NUM,
                fields.len()
            ));
        }

        let date = NaiveDate::parse_from_str(fields[date_idx], DATE_FMT_STR).map_err(|_| {
            parsing_err!(
                "Invalid transaction date '{}' at line {}. Expected format is YYYY-MM-DD.",
                fields[date_idx],
                line_num
            )
        })?;

        let kind = get_kind_from_str(fields[kind_idx]).ok_or_else(|| {
            parsing_err!(
                "Invalid transaction kind '{}' at line {}.",
                fields[kind_idx],
                line_num
            )
        })?;

        let amount = fields[amount_idx].parse::<u64>().map_err(|_| {
            parsing_err!(
                "Invalid transaction amount '{}' at line {}. Expected positive integer.",
                fields[amount_idx],
                line_num
            )
        })?;
        let Some(amount) = NonZeroU64::new(amount) else {
            return Err(parsing_err!(
                "Invalid transaction amount '{}' at line {}. Value must not be zero.",
                amount,
                line_num,
            ));
        };

        let cat = fields[cat_idx].to_string();
        if !cat.is_ascii() || cat.len() == 0 {
            return Err(parsing_err!(
                "Invalid transaction category '{}' at line {}. Expected non empty ASCII string.",
                cat,
                line_num
            ));
        }

        match Tx::try_new(date, cat, kind, amount) {
            Ok(tx) => txs.push(tx),
            Err(err) => {
                return Err(validation_err!(
                    "Invalid transaction field at line {}. {}",
                    line_num,
                    err
                ));
            }
        };
    }

    Ok(txs)
}
