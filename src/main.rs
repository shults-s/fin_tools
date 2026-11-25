//! Contains CLI application for converting transaction list formats.

use fin_tools::format::{TxFormat, get_format, get_reader, get_writer};
use fin_tools::utils::{ArgMap, get_format_from_path};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Write, stdin, stdout};
use std::path::Path;
use std::process::ExitCode;

enum Src<'a> {
    File(&'a Path, TxFormat),
    StdIn(TxFormat),
}

enum Dst<'a> {
    File(&'a Path, TxFormat),
    StdOut(TxFormat),
}

const USAGE_STR: &str = r#"Usage: converter <src> <dst>
  <src> = --src="<file_path>" | --stdin=<fmt>
  <dst> = --dst="<file_path>" | --stdout=<fmt>
  <fmt> = "ftx" | "csv" | "txt"
When a file path is provided, format will inferred from file extension: ftx, csv or txt."#;

fn main() -> ExitCode {
    match converter() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}

fn converter() -> Result<(), Box<dyn Error>> {
    let args = std::env::args();
    if args.len() != 3 {
        return Err(USAGE_STR.to_string().into());
    }

    let arg_map = ArgMap::from(args)?;

    let src = if let Some(src_path) = arg_map.get_arg("src")
        && !arg_map.check_val_arg("stdin")
    {
        let path = Path::new(src_path);
        let format = get_format_from_path(path)?;
        Src::File(path, format)
    } else if let Some(format) = arg_map.get_arg("stdin") {
        let format = get_format(format).ok_or_else(|| format!("Unsupported format '{format}'."))?;
        Src::StdIn(format)
    } else {
        return Err("Invalid <src>. Must be set one of --src or --stdin.".into());
    };

    let dst = if let Some(dst_path) = arg_map.get_arg("dst")
        && !arg_map.check_val_arg("stdout")
    {
        let path = Path::new(dst_path);
        let format = get_format_from_path(path)?;
        Dst::File(path, format)
    } else if let Some(format) = arg_map.get_arg("stdout") {
        let format = get_format(format).ok_or_else(|| format!("Unsupported format '{format}'."))?;
        Dst::StdOut(format)
    } else {
        return Err("Invalid <dst>. Must be set one of --dst or --stdout.".into());
    };

    let txs = match src {
        Src::File(path, format) => {
            let src = File::open(path)?;
            get_reader(format).read(&mut BufReader::new(src))
        }
        Src::StdIn(format) => {
            let mut src = stdin().lock();
            get_reader(format).read(&mut src)
        }
    }?;

    let mut buffer = Vec::new();
    match dst {
        Dst::File(path, format) => {
            get_writer(format).write(&mut buffer, &txs)?;
            let mut dst = File::create(path)?;
            dst.write_all(&buffer)?;
        }
        Dst::StdOut(format) => {
            get_writer(format).write(&mut buffer, &txs)?;
            let mut dst = stdout().lock();
            dst.write_all(&buffer)?;
        }
    };

    Ok(())
}
