use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum FinToolsError {
    TxReadingError(std::io::Error),
    TxWritingError(std::io::Error),
    TxParsingError(String),
}

impl Display for FinToolsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FinToolsError::TxReadingError(_) => write!(f, "Failed to read transactions."),
            FinToolsError::TxWritingError(_) => write!(f, "Failed to write transactions."),
            FinToolsError::TxParsingError(reason) => {
                write!(f, "Failed to parse transaction. {reason}")
            }
        }
    }
}

impl Error for FinToolsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FinToolsError::TxReadingError(err) => Some(err),
            FinToolsError::TxWritingError(err) => Some(err),
            FinToolsError::TxParsingError(_) => None,
        }
    }
}
