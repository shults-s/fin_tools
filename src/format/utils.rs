use crate::error::{FinToolsError, parsing_err};
use crate::format::DATE_FMT_STR;
use crate::model::{Tx, TxKind};
use chrono::NaiveDate;
use std::io::{BufRead, BufReader, BufWriter, Write};

const COLUMNS_NUM: usize = 4;
const DEFAULT_COLUMNS: [&str; COLUMNS_NUM] = ["date", "category", "kind", "amount"];

#[inline]
// fn get_column_idx(col_name: &str, columns: impl IntoIterator<Item: AsRef<str>>) -> Option<usize> {
fn get_column_idx(col_name: &str, columns: &[impl AsRef<str>]) -> Option<usize> {
    columns.into_iter().position(|col| col.as_ref() == col_name)
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
    format!("{:?}", kind).to_lowercase()
}

pub(super) fn write_txs_as_text(
    dst: &mut dyn std::io::Write,
    txs: &Vec<Tx>,
    add_header: bool,
    field_sep: &str,
) -> Result<(), FinToolsError> {
    let mut writer = BufWriter::new(dst);

    if add_header {
        let header = DEFAULT_COLUMNS.join(field_sep);
        writeln!(writer, "{header}")?;
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

    let mut lines = BufReader::new(src).lines();

    let columns = if has_header && let Some(header) = lines.next() {
        // FOR_REVIEWER: I don't like this stupid allocations, but I didn't come up
        // with a more idiomatic way to read header on-demand.
        // I would appreciate some advice.
        header?
            .split(field_sep)
            .map(|column| column.trim().to_ascii_lowercase())
            .collect::<Vec<String>>()
    } else {
        DEFAULT_COLUMNS
            .iter()
            .map(|column| column.to_string())
            .collect::<Vec<String>>()
    };

    let date_idx =
        get_column_idx("date", &columns).ok_or_else(|| parsing_err!("Column 'date' not found."))?;
    let cat_idx = get_column_idx("category", &columns)
        .ok_or_else(|| parsing_err!("Column 'category' not found."))?;
    let kind_idx =
        get_column_idx("kind", &columns).ok_or_else(|| parsing_err!("Column 'kind' not found."))?;
    let amount_idx = get_column_idx("amount", &columns)
        .ok_or_else(|| parsing_err!("Column 'amount' not found."))?;

    let mut line_num = if has_header { 1 } else { 0 };
    for line in lines {
        line_num += 1;

        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let fields = line.split(field_sep).map(str::trim).collect::<Vec<_>>();
        if fields.len() != COLUMNS_NUM {
            return Err(parsing_err!(
                "Invalid TX format at line {}. Expected {} fields, found {}.",
                line_num,
                COLUMNS_NUM,
                fields.len()
            ));
        }

        let date = NaiveDate::parse_from_str(fields[date_idx], DATE_FMT_STR).map_err(|_| {
            parsing_err!(
                "Invalid date '{}' at line {}. Expected format is YYYY-MM-DD.",
                fields[date_idx],
                line_num
            )
        })?;

        let cat = fields[cat_idx].to_string();
        if !cat.is_ascii() || cat.len() == 0 {
            return Err(parsing_err!(
                "Invalid category '{}' at line {}. Expected non empty ASCII text.",
                cat,
                line_num
            ));
        }

        let kind = get_kind_from_str(fields[kind_idx]).ok_or_else(|| {
            parsing_err!(
                "Invalid transaction kind '{}' at line {}.",
                fields[kind_idx],
                line_num
            )
        })?;

        let amount = fields[amount_idx].parse::<u64>().map_err(|_| {
            parsing_err!(
                "Invalid amount '{}' at line {}. Expected positive integer.",
                fields[amount_idx],
                line_num
            )
        })?;
        if amount == 0 {
            return Err(parsing_err!(
                "Invalid amount '{}' at line {}. Value must be a positive integer.",
                amount,
                line_num
            ));
        }

        txs.push(Tx::new(date, cat, kind, amount));
    }

    Ok(txs)
}
