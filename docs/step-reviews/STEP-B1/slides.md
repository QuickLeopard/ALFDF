# STEP-B1 — Define core type enum `Type`

---

## What changed

- `Type` enum in `alfdf-ast`: Var, Con, Arrow, Tuple
- 100 hand-written JSON round-trip tests

---

## Type shape

```rust
enum Type {
    Var(String),
    Con(String, Vec<Type>),
    Arrow(Box<Type>, Box<Type>),
    Tuple(Vec<Type>),
}
```

---

## Progress

- Phase B started; B1 done
- Next: STEP-B2 — `Expr`

---

## Verification

- `cargo test -p alfdf-ast` — 3 tests PASS
- `just verify` — PASS

---

## Follow-ups

- STEP-B2 Expr + Span
- STEP-B4 canonical S-expr may refine JSON canonical form
