//! AST types, schema, and hashing. Spec §3.

#![warn(
    missing_docs,
    clippy::pedantic,
    clippy::nursery,
    clippy::cognitive_complexity
)]

mod expr;
mod literal;
mod pattern;
mod span;
mod types;

pub use expr::Expr;
pub use literal::Literal;
pub use pattern::Pattern;
pub use span::Span;
pub use types::Type;
