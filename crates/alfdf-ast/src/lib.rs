//! AST types, schema, and hashing. Spec §3.

#![warn(
    missing_docs,
    clippy::pedantic,
    clippy::nursery,
    clippy::cognitive_complexity
)]

mod types;

pub use types::Type;
