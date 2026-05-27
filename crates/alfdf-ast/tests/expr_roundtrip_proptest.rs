//! Property test: `Expr` JSON round-trip (STEP-B2).

mod support;

use alfdf_ast::Expr;
use proptest::prelude::*;
use support::expr_strategy::expr_strategy;

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 10_000,
        ..ProptestConfig::default()
    })]

    #[test]
    fn expr_json_roundtrip(expr in expr_strategy()) {
        let json = serde_json::to_string(&expr).expect("serialize");
        let decoded: Expr = serde_json::from_str(&json).expect("deserialize");
        prop_assert_eq!(expr, decoded);
    }
}
