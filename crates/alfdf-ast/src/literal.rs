//! Expression literals. Spec §3.

use serde::{Deserialize, Serialize};

/// Literal values in ABAL expressions.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Literal {
    /// Boolean literal.
    Bool(bool),
    /// Signed integer literal.
    Int(i64),
    /// String literal.
    String(String),
}
