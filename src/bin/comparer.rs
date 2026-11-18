use fin_tools::arg_utils::{collect_args, get_required_arg_val};
use fin_tools::format::get_reader;
use fin_tools::fs_utils::{get_format_from_path, get_path};
use fin_tools::model::{TxsCmpRes, compare_txs};
use std::error::Error;
use std::fs::File;

const USAGE_STR: &str = r#"Usage: comparer --txs1="<file_path>" --txs2="<file_path>"
Format will inferred from file extension: ftx, csv or txt."#;

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args();
    if args.len() != 2 {
        println!("{USAGE_STR}");
        return Ok(());
    }

    let arg_map = collect_args(args)?;
    let path1 = get_required_arg_val("txs1", &arg_map).and_then(get_path)?;
    let path2 = get_required_arg_val("txs2", &arg_map).and_then(get_path)?;

    let fmt1 = get_format_from_path(path1)?;
    let fmt2 = get_format_from_path(path2)?;

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
