//! Match patterns. Spec §3.

use serde::{Deserialize, Serialize};

/// Pattern for `match` expressions.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Pattern {
    /// Bind a variable.
    Var(String),
    /// Constructor pattern with sub-patterns.
    Ctor {
        /// Constructor name.
        name: String,
        /// Argument patterns.
        args: Vec<Self>,
    },
    /// Wildcard `_`.
    Wildcard,
}
