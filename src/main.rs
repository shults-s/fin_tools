mod arg_utils;
mod error;
mod format;
mod fs_utils;
mod model;

use crate::arg_utils::{collect_args, get_required_arg_val};
use crate::format::get_reader;
use crate::fs_utils::{get_format_from_path, get_path};
use crate::model::{TxsCmpRes, compare_txs};
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

    let arg_map = collect_args(args)?;

    Ok(())
}
