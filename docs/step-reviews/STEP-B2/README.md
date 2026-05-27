# STEP-B2 — Define core expression enum `Expr`

## Step

**STEP-B2** — Define core expression enum `Expr` (Phase B — AST and hashing)

## What shipped

- [`crates/alfdf-ast/src/span.rs`](../../../crates/alfdf-ast/src/span.rs) — `Span { file, start, end }`
- [`crates/alfdf-ast/src/literal.rs`](../../../crates/alfdf-ast/src/literal.rs) — `Literal` (Bool, Int, String)
- [`crates/alfdf-ast/src/pattern.rs`](../../../crates/alfdf-ast/src/pattern.rs) — `Pattern` for match arms
- [`crates/alfdf-ast/src/expr.rs`](../../../crates/alfdf-ast/src/expr.rs) — `Expr` variants with per-node `Span`
- [`crates/alfdf-ast/tests/expr_roundtrip_proptest.rs`](../../../crates/alfdf-ast/tests/expr_roundtrip_proptest.rs) — 10k-case JSON round-trip
- [`crates/alfdf-ast/tests/support/expr_strategy.rs`](../../../crates/alfdf-ast/tests/support/expr_strategy.rs) — `proptest` generators

## Whole-project progress

- **Phase B:** STEP-B1–B2 (B1 in [PR #6](https://github.com/QuickLeopard/ALFDF/pull/6)); STEP-B3 next
- **Phase A:** A3 ADR still open in [PR #4](https://github.com/QuickLeopard/ALFDF/pull/4)
- **MVP0:** 6 / ~80+ steps

## Commands run

```text
$ cargo test -p alfdf-ast
# PASS — 4 tests (incl. 10k proptest ~93s debug)

$ just verify
# PASS
```

## Tests

- `expr_json_roundtrip` — 10_000 random `Expr` values, `serde_json` round-trip
- Existing STEP-B1 tests unchanged

Benchmarks: not applicable.

## Risks and follow-ups

- Branch stacks on STEP-B1 until #6 merges.
- `Lambda`/`Let` carry `Type` annotations; parser may refine arity later.
- Next: **STEP-B3** — top-level items (`Module`, `FnDecl`, …).

## Review

- TDD: RED proptest commit, then GREEN `Expr` + supporting types.
- Every `Expr` variant includes `span: Span` per guide.
