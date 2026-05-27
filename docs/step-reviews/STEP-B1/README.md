# STEP-B1 — Define core type enum `Type`

## Step

**STEP-B1** — Define core type enum `Type` (Phase B — AST and hashing)

## What shipped

- [`crates/alfdf-ast/src/types.rs`](../../../crates/alfdf-ast/src/types.rs) — `Type` enum: `Var`, `Con`, `Arrow`, `Tuple`
- [`crates/alfdf-ast/src/lib.rs`](../../../crates/alfdf-ast/src/lib.rs) — re-export `Type`
- [`crates/alfdf-ast/tests/type_roundtrip.rs`](../../../crates/alfdf-ast/tests/type_roundtrip.rs) — 100-case JSON round-trip + trait bounds
- [`crates/alfdf-ast/tests/support/type_json_cases.rs`](../../../crates/alfdf-ast/tests/support/type_json_cases.rs) — hand-written fixtures

## Whole-project progress

- **Phase A:** STEP-A1–A4 done (A3 ADR pending merge in [PR #4](https://github.com/QuickLeopard/ALFDF/pull/4))
- **Phase B:** STEP-B1 done; STEP-B2 next
- **MVP0:** 5 / ~80+ steps — see [`.DOCS/ALFDF-MVP0-Stepbystep-Guide-v0.md`](../../../.DOCS/ALFDF-MVP0-Stepbystep-Guide-v0.md)

## Commands run

```text
$ cargo test -p alfdf-ast
# PASS — 3 tests (1 unit, 2 integration), 1 doctest

$ just verify
# PASS — build, clippy, fmt, nextest, jscpd, deny
```

## Tests

- `type_json_roundtrip_hand_written_cases` — 100 JSON fixtures, serde round-trip
- `type_traits_object_safe_bounds` — `Clone + Eq + Hash` compile-time check
- `types::tests::var_equality` — unit smoke

Benchmarks: not applicable — no benchmark defined in this step.

## Risks and follow-ups

- JSON shape uses serde externally-tagged enum; canonical JSON ordering may differ from future S-expression serializer (STEP-B4).
- Next: **STEP-B2** — `Expr` enum with `Span`.

## Review

- TDD: RED `test(alfdf-ast): STEP-B1 failing …` then GREEN `feat(alfdf-ast): STEP-B1 …`.
- `Type` derives `Clone`, `Eq`, `Hash`, `Serialize`, `Deserialize` as required.
- No new workspace dependencies.
