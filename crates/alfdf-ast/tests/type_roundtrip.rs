//! Integration tests for ABAL `Type` JSON round-trip (STEP-B1).

mod support;

use alfdf_ast::Type;
use support::type_json_cases::json_cases;

#[test]
fn type_json_roundtrip_hand_written_cases() {
    for (index, json) in json_cases().iter().enumerate() {
        let parsed: Type = serde_json::from_str(json)
            .unwrap_or_else(|err| panic!("case {index}: deserialize failed: {err}"));
        let encoded = serde_json::to_string(&parsed)
            .unwrap_or_else(|err| panic!("case {index}: serialize failed: {err}"));
        let reparsed: Type = serde_json::from_str(&encoded)
            .unwrap_or_else(|err| panic!("case {index}: re-deserialize failed: {err}"));
        assert_eq!(parsed, reparsed, "case {index}: value round-trip mismatch");
        let reparsed_from_original: Type = serde_json::from_str(json)
            .unwrap_or_else(|err| panic!("case {index}: second deserialize failed: {err}"));
        assert_eq!(
            parsed, reparsed_from_original,
            "case {index}: json canonical mismatch"
        );
    }
}

#[test]
fn type_traits_object_safe_bounds() {
    fn assert_traits<T: Clone + Eq + std::hash::Hash>() {}
    assert_traits::<Type>();
}
