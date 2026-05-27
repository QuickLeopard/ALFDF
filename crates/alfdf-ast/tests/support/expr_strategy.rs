//! `proptest` strategies for `Expr` (STEP-B2).

use alfdf_ast::{Expr, Literal, Pattern, Span, Type};
use proptest::prelude::*;

prop_compose! {
    pub fn span_strategy()(file in r"[a-z][a-z0-9_/]{0,12}\.abal", start in 0u32..1000, len in 0u32..200)
        -> Span {
        Span {
            file,
            start,
            end: start.saturating_add(len),
        }
    }
}

prop_compose! {
    fn literal_strategy()(value in prop_oneof![
        any::<bool>().prop_map(Literal::Bool),
        any::<i64>().prop_map(Literal::Int),
        r"[a-zA-Z0-9_]{0,16}".prop_map(Literal::String),
    ]) -> Literal {
        value
    }
}

prop_compose! {
    fn type_strategy()(depth in 0u8..4)(ty in type_tree(depth)) -> Type {
        ty
    }
}

fn type_tree(depth: u8) -> impl Strategy<Value = Type> {
    let leaf = prop_oneof![
        r"[a-z]{1,4}".prop_map(Type::Var),
        (r"[A-Z][a-zA-Z0-9]{0,8}", Just(vec![])).prop_map(|(n, a)| Type::Con(n, a)),
    ];
    if depth == 0 {
        leaf.boxed()
    } else {
        prop_oneof![
            leaf,
            (type_tree(depth - 1), type_tree(depth - 1))
                .prop_map(|(l, r)| Type::Arrow(Box::new(l), Box::new(r))),
            prop::collection::vec(type_tree(depth - 1), 0..3).prop_map(Type::Tuple),
        ]
        .boxed()
    }
}

prop_compose! {
    pub fn pattern_strategy()(pat in pattern_tree(3)) -> Pattern {
        pat
    }
}

fn pattern_tree(depth: u8) -> impl Strategy<Value = Pattern> {
    let leaf = prop_oneof![
        r"[a-z]{1,6}".prop_map(Pattern::Var),
        Just(Pattern::Wildcard),
    ];
    if depth == 0 {
        leaf.boxed()
    } else {
        prop_oneof![
            leaf,
            (
                r"[A-Z][a-zA-Z0-9]{0,8}",
                prop::collection::vec(pattern_tree(depth - 1), 0..3),
            )
                .prop_map(|(name, args)| Pattern::Ctor { name, args }),
        ]
        .boxed()
    }
}

prop_compose! {
    pub fn expr_strategy()(depth in 0u8..6)(expr in expr_tree(depth)) -> Expr {
        expr
    }
}

fn expr_tree(depth: u8) -> impl Strategy<Value = Expr> {
    let leaf = prop_oneof![
        (span_strategy(), literal_strategy())
            .prop_map(|(span, value)| Expr::Lit { span, value }),
        (span_strategy(), r"[a-z][a-z0-9_]{0,8}")
            .prop_map(|(span, name)| Expr::Var { span, name }),
    ];
    if depth == 0 {
        leaf.boxed()
    } else {
        let child = || expr_tree(depth - 1);
        prop_oneof![
            leaf,
            (span_strategy(), child(), prop::collection::vec(child(), 0..3))
                .prop_map(|(span, func, args)| Expr::App {
                    span,
                    func: Box::new(func),
                    args,
                }),
            (
                span_strategy(),
                prop::collection::vec(r"[a-z]{1,6}", 0..3),
                type_strategy(),
                child(),
            )
                .prop_map(|(span, params, ret_type, body)| Expr::Lambda {
                    span,
                    params,
                    ret_type,
                    body: Box::new(body),
                }),
            (
                span_strategy(),
                r"[a-z]{1,6}",
                type_strategy(),
                child(),
                child(),
            )
                .prop_map(|(span, name, ty, value, body)| Expr::Let {
                    span,
                    name,
                    ty,
                    value: Box::new(value),
                    body: Box::new(body),
                }),
            (span_strategy(), child(), prop::collection::vec((pattern_strategy(), child()), 0..3))
                .prop_map(|(span, scrutinee, arms)| Expr::Match {
                    span,
                    scrutinee: Box::new(scrutinee),
                    arms,
                }),
            (
                span_strategy(),
                r"[A-Z][a-zA-Z0-9]{0,8}",
                prop::collection::vec(child(), 0..3),
            )
                .prop_map(|(span, name, args)| Expr::Ctor { span, name, args }),
            (span_strategy(), prop::collection::vec(child(), 0..4))
                .prop_map(|(span, elems)| Expr::TupleLit { span, elems }),
        ]
        .boxed()
    }
}
