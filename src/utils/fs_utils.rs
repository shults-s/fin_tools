use crate::format::{TxFormat, get_format};
use std::path::PathBuf;

/// Gets lowercased file extension (excl. dot) from `std::path::Path`.
/// Invalid UTF-8 sequences in path will be lost.\
/// Returns error when path does not contains file extension.
pub fn get_ext(path: &PathBuf) -> Result<String, String> {
    if let Some(ext) = path.extension() {
        Ok(ext.to_string_lossy().to_ascii_lowercase())
    } else {
        Err(format!("Path '{}' has no extension.", path.display()))
    }
}

/// Gets transaction format from file path, relying on file extension.\
/// Returns error when path does not contains supported file extension.
pub fn get_format_from_path(path: &PathBuf) -> Result<TxFormat, String> {
    get_ext(path).map(get_format).and_then(|fmt| {
        fmt.ok_or_else(|| format!("Unsupported format of file '{}'.", path.display()))
    })
}
