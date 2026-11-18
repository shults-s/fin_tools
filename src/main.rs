mod error;
mod format;
mod model;
mod utils;

use crate::format::{get_format, get_reader, get_writer};
use crate::utils::{ArgMap, get_format_from_path, get_path_checked};
use std::error::Error;
use std::fs::File;
use std::io::{stdin, stdout};

const USAGE_STR: &str = r#"Usage: converter <src> <dst>
  <src> = --src="<file_path>" | --stdin=<fmt>
  <dst> = --dst="<file_path>" | --stdout=<fmt>
  <fmt> = "ftx" | "csv" | "txt"
When a file path is provided, format will inferred from file extension: ftx, csv or txt."#;

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args();
    if args.len() != 3 {
        println!("{USAGE_STR}");
        return Ok(());
    }

    let arg_map = ArgMap::from(args)?;

    let has_src = arg_map.check_val_arg("src") ^ arg_map.check_val_arg("stdin");
    if !has_src {
        return Err("Invalid <src>. Must be set one of --src or --stdin.".into());
    }

    let has_dst = arg_map.check_val_arg("dst") ^ arg_map.check_val_arg("stdout");
    if !has_dst {
        return Err("Invalid <dst>. Must be set one of --dst or --stdout.".into());
    }

    let txs = if let Some(src_path) = arg_map.get_arg("src") {
        let path = get_path_checked(src_path)?;
        let format = get_format_from_path(path)?;
        let mut src = File::open(path)?;
        get_reader(format).read(&mut src)
    } else if let Some(format) = arg_map.get_arg("stdin") {
        let format = get_format(format).ok_or_else(|| format!("Unsupported format '{format}'."))?;
        let mut src = stdin().lock();
        get_reader(format).read(&mut src)
    } else {
        panic!("What does the fox say?")
    }?;

    if let Some(dst_path) = arg_map.get_arg("dst") {
        let path = std::path::Path::new(dst_path);
        let format = get_format_from_path(path)?;
        let mut dst = File::create(path)?;
        get_writer(format).write(&mut dst, &txs)?;
    } else if let Some(format) = arg_map.get_arg("stdout") {
        let format = get_format(format).ok_or_else(|| format!("Unsupported format '{format}'."))?;
        let mut dst = stdout().lock();
        get_writer(format).write(&mut dst, &txs)?;
    } else {
        panic!("Ring-ding-ding-ding-dingeringeding! Wa-pa-pa-pa-pa-pa-pow!")
    }

    Ok(())
}
