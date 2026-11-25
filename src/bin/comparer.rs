//! Contains CLI application for comparing transaction lists.

use fin_tools::format::get_reader;
use fin_tools::model::{TxsCmpRes, compare_txs};
use fin_tools::utils::{ArgMap, get_format_from_path};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::process::ExitCode;

const USAGE_STR: &str = r#"Usage: comparer --txs1="<file_path>" --txs2="<file_path>"
Format will inferred from file extension: ftx, csv or txt."#;

fn main() -> ExitCode {
    match comparer() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}

fn comparer() -> Result<(), Box<dyn Error>> {
    let args = std::env::args();
    if args.len() != 3 {
        return Err(format!("{USAGE_STR}").into());
    }

    let arg_map = ArgMap::from(args)?;
    let path1 = arg_map.get_required("txs1").map(PathBuf::from)?;
    let path2 = arg_map.get_required("txs2").map(PathBuf::from)?;

    let fmt1 = get_format_from_path(&path1)?;
    let fmt2 = get_format_from_path(&path2)?;

    let mut file1 = File::open(path1)?;
    let mut file2 = File::open(path2)?;

    let txs1 = get_reader(fmt1).read(&mut file1)?;
    let txs2 = get_reader(fmt2).read(&mut file2)?;
    return match compare_txs(&txs1, &txs2) {
        TxsCmpRes::Equals => {
            println!("Transaction lists are identical.");
            Ok(())
        }
        TxsCmpRes::CountMismatch => Err(format!(
            "Transaction lists have different lengths: {} != {}.",
            txs1.len(),
            txs2.len()
        )
        .into()),
        TxsCmpRes::Different(idx, reason) => {
            Err(format!("Transactions numbered {} differ. {}", idx + 1, reason).into())
        }
    };
}
