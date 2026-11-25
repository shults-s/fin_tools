#![warn(missing_docs)]
// or `rustc -W missing_docs src/lib.rs`
// or `RUSTDOCFLAGS="-W missing_docs" cargo doc --no-deps`
// or #![doc = include_str!("../README.md")]
// or see Cargo.toml

//! Contains reusable logic of financial tool.

/// Contains errors that may occur while working with library.
pub mod error;

/// Contains representation formats of domain logic.
pub mod format;

/// Contains domain logic of library.
pub mod model;

/// Contains auxiliary: args parser, file system path utils.
pub mod utils;
