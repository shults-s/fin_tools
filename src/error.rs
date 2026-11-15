use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum FinToolsError {
    IoError(std::io::Error),
    ParsingError(String),
}

impl Display for FinToolsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FinToolsError::IoError(_) => write!(f, "Failed to read ot write transactions."),
            FinToolsError::ParsingError(reason) => {
                write!(f, "Failed to parse transaction. {reason}")
            }
        }
    }
}

impl Error for FinToolsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FinToolsError::IoError(err) => Some(err),
            FinToolsError::ParsingError(_) => None,
        }
    }
}

impl From<std::io::Error> for FinToolsError {
    fn from(val: std::io::Error) -> Self {
        FinToolsError::IoError(val)
    }
}
