//! ABAL type AST nodes. Spec §3.

use serde::{Deserialize, Serialize};

/// ABAL type expression: variable, constructor application, function, or tuple.
///
/// # Examples
///
/// ```
/// use alfdf_ast::Type;
///
/// let var = Type::Var("a".into());
/// let nat = Type::Con("Nat".into(), vec![]);
/// let fn_type = Type::Arrow(Box::new(var.clone()), Box::new(nat));
/// assert_eq!(var, Type::Var("a".into()));
/// let _ = fn_type;
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Type {
    /// Type variable (e.g. polymorphic `a`).
    Var(String),
    /// Named type constructor with type arguments (e.g. `List a`).
    Con(String, Vec<Self>),
    /// Function type `domain -> codomain`.
    Arrow(Box<Self>, Box<Self>),
    /// Tuple of types.
    Tuple(Vec<Self>),
}

#[cfg(test)]
mod tests {
    use super::Type;

    #[test]
    fn var_equality() {
        assert_eq!(Type::Var("x".into()), Type::Var("x".into()));
    }
}
