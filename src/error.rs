use std::{error::Error, fmt::Display};

/// Creates an instance of `FinToolsError::ParsingError` from literal or format args.
macro_rules! parsing_err {
    ($msg:literal) => {
        FinToolsError::ParsingError($msg.into())
    };

    ($($arg:tt)*) => {
        FinToolsError::ParsingError(format!($($arg)*))
    }
}

/// Creates an instance of `FinToolsError::SerializingError` from literal or format args.
macro_rules! serializing_err {
    ($msg:literal) => {
        FinToolsError::SerializingError($msg.into())
    };

    ($($arg:tt)*) => {
        FinToolsError::SerializingError(format!($($arg)*))
    }
}

/// Creates an instance of `FinToolsError::ValidationError` from literal or format args.
macro_rules! validation_err {
    ($msg:literal) => {
        FinToolsError::ValidationError($msg.into())
    };

    ($($arg:tt)*) => {
        FinToolsError::ValidationError(format!($($arg)*))
    }
}

pub(crate) use parsing_err;
pub(crate) use serializing_err;
pub(crate) use validation_err;

/// Represents `fin_tools` library error variants.
#[derive(Debug)]
pub enum FinToolsError {
    /// Represents an I/O error which occurred while reading transactions list.
    ReadError(std::io::Error),

    /// Represents an I/O error which occurred while writing transactions list.
    WriteError(std::io::Error),

    /// Represents an error which occurred while deserializing transactions list.
    ParsingError(String),

    /// Represents an error which occurred while serializing transactions list.
    SerializingError(String),

    /// Represents an error which occurred while validation of domain model.
    ValidationError(String),
}

impl Display for FinToolsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // FOR_REVIEWER: I don't like this. So, is it reading or writing failure?
            // Are there ways to specify this in a more granular fashion without losing
            // ability to use operator "?" via implementing From<T>?
            // I would appreciate some advice.
            FinToolsError::ReadError(_) => write!(f, "Failed to read transactions."),
            FinToolsError::WriteError(_) => write!(f, "Failed to write transactions."),
            FinToolsError::ParsingError(reason) => {
                write!(f, "Failed to parse transactions. {reason}")
            }
            FinToolsError::SerializingError(reason) => {
                write!(f, "Failed to serialize transactions. {reason}")
            }
            FinToolsError::ValidationError(reason) => {
                write!(f, "{reason}")
            }
        }
    }
}

impl Error for FinToolsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ReadError(err) | Self::WriteError(err) => Some(err),
            Self::ParsingError(_) | Self::SerializingError(_) | Self::ValidationError(_) => None,
        }
    }
}
