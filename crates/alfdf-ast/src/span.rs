//! Source spans for diagnostics. Spec §3.

use serde::{Deserialize, Serialize};

/// Byte range in a source file.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Span {
    /// Source path or logical file name.
    pub file: String,
    /// Inclusive start byte offset.
    pub start: u32,
    /// Exclusive end byte offset.
    pub end: u32,
}

impl Span {
    /// Creates a span for the given file and byte range.
    #[must_use]
    pub const fn new(file: String, start: u32, end: u32) -> Self {
        Self { file, start, end }
    }
}
