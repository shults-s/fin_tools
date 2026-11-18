mod error;
mod format;
mod model;
mod utils;

use crate::format::get_reader;
use crate::model::{TxsCmpRes, compare_txs};
use crate::utils::{ArgMap, get_format_from_path, get_path};
use std::error::Error;
use std::fs::File;

const USAGE_STR: &str = r#"Usage: converter <src> <dst>
  <src> = --src="<file_path>" | --stdin=<fmt>
  <dst> = --dst="<file_path>" | --stdout=<fmt>
  <fmt> = "ftx" | "csv" | "txt"
When a file path is provided, format will inferred from file extension: ftx, csv or txt."#;

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args();
    if args.len() != 2 {
        println!("{USAGE_STR}");
        return Ok(());
    }

    let arg_map = ArgMap::from(args)?;

    Ok(())
}
