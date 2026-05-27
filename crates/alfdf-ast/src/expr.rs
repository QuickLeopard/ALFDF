//! ABAL expression AST nodes. Spec §3.

use serde::{Deserialize, Serialize};

use crate::{Literal, Pattern, Span, Type};

/// ABAL expression with source span on every variant.
///
/// # Examples
///
/// ```
/// use alfdf_ast::{Expr, Literal, Span};
///
/// let expr = Expr::Lit {
///     span: Span::new("main.abal".into(), 0, 1),
///     value: Literal::Bool(true),
/// };
/// assert!(matches!(expr, Expr::Lit { .. }));
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Expr {
    /// Literal value.
    Lit {
        /// Source span.
        span: Span,
        /// Literal payload.
        value: Literal,
    },
    /// Variable reference.
    Var {
        /// Source span.
        span: Span,
        /// Variable name.
        name: String,
    },
    /// Function application.
    App {
        /// Source span.
        span: Span,
        /// Callee expression.
        func: Box<Self>,
        /// Arguments.
        args: Vec<Self>,
    },
    /// Lambda abstraction.
    Lambda {
        /// Source span.
        span: Span,
        /// Parameter names.
        params: Vec<String>,
        /// Declared return type.
        ret_type: Type,
        /// Body expression.
        body: Box<Self>,
    },
    /// Let binding.
    Let {
        /// Source span.
        span: Span,
        /// Bound name.
        name: String,
        /// Annotated type.
        ty: Type,
        /// Right-hand side.
        value: Box<Self>,
        /// Body in scope of the binding.
        body: Box<Self>,
    },
    /// Pattern match.
    Match {
        /// Source span.
        span: Span,
        /// Scrutinee.
        scrutinee: Box<Self>,
        /// Pattern-expression arms.
        arms: Vec<(Pattern, Self)>,
    },
    /// Constructor application.
    Ctor {
        /// Source span.
        span: Span,
        /// Constructor name.
        name: String,
        /// Arguments.
        args: Vec<Self>,
    },
    /// Tuple literal.
    TupleLit {
        /// Source span.
        span: Span,
        /// Elements.
        elems: Vec<Self>,
    },
}
