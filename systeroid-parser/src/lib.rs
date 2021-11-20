//! systeroid-parser

#![warn(missing_docs, clippy::unwrap_used)]

/// Export regex crate.
pub use regex;

/// Document parser.
pub mod parser;

/// Parse results.
pub mod document;

/// Error implementation.
pub mod error;

/// File reader.
pub mod reader;