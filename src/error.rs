use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum FinToolsError {
    IoError(std::io::Error),
    ParsingError(String),
    SerializingError(String),
}

impl Display for FinToolsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // FOR_REVIEWER: I don't like this. So, is it reading or writing?
            // Are there ways to specify this in a more granular fashion without losing
            // ability to use operator "?" via implementing From<T>?
            // I would appreciate some advice.
            FinToolsError::IoError(_) => write!(f, "Failed to read/write transactions."),
            FinToolsError::ParsingError(reason) => {
                write!(f, "Failed to parse transactions. {reason}")
            }
            FinToolsError::SerializingError(reason) => {
                write!(f, "Failed to serialize transactions. {reason}")
            }
        }
    }
}

impl Error for FinToolsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FinToolsError::IoError(err) => Some(err),
            FinToolsError::ParsingError(_) | FinToolsError::SerializingError(_) => None,
        }
    }
}

impl From<std::io::Error> for FinToolsError {
    fn from(val: std::io::Error) -> Self {
        FinToolsError::IoError(val)
    }
}

macro_rules! parsing_err {
    ($msg:literal) => {
        FinToolsError::ParsingError($msg.into())
    };

    ($($arg:tt)*) => {
        FinToolsError::ParsingError(format!($($arg)*))
    }
}

macro_rules! serializing_err {
    ($msg:literal) => {
        FinToolsError::SerializingError($msg.into())
    };

    ($($arg:tt)*) => {
        FinToolsError::SerializingError(format!($($arg)*))
    }
}

pub(crate) use parsing_err;
pub(crate) use serializing_err;
