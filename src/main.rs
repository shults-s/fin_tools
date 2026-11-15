use chrono::{Datelike, NaiveDate};

use crate::model::Tx;

mod error;
mod format;
mod model;

fn main() {
    // converter
    let date = NaiveDate::parse_from_str("2026-11-15", "%Y-%m-%d").unwrap();
    let s: i32 = date.num_days_from_ce();
    let tx = Tx::new(date, "meow".into(), model::TxKind::Income, 100_000);
}
