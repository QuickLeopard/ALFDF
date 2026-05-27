# STEP-B2 — Define core expression enum `Expr`

---

## What changed

- `Span`, `Literal`, `Pattern`, `Expr` in `alfdf-ast`
- Variants: Lit, Var, App, Lambda, Let, Match, Ctor, TupleLit
- 10k proptest JSON round-trip

---

## Expr shape

Each variant carries `span: Span` for diagnostics.

---

## Progress

- Phase B: B1 + B2 on branch (B1 PR #6)
- Next: STEP-B3 top-level items

---

## Verification

- `cargo test -p alfdf-ast` — PASS (10k proptest)
- `just verify` — PASS

---

## Follow-ups

- Merge PR #6 (B1) before or with this PR
- STEP-B3 `Module` / decls
