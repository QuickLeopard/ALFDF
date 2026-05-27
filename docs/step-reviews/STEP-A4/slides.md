# STEP-A4 — Add core utility crates

---

## What changed

- `[workspace.dependencies]` in root `Cargo.toml`
- All 14 crates use `workspace = true` for shared deps
- `scripts/wire_workspace_deps.sh` for idempotent wiring
- `deny.toml`: +BSD-2-Clause, +Unicode-3.0

---

## Dependencies pinned

serde, serde_json, thiserror, anyhow, tracing, blake3  
dev: proptest, insta, criterion `0.8.2`

---

## Progress

- Phase A complete (A1–A4)
- Next: STEP-B1 — `Type` enum

---

## Verification

- `just verify` — PASS

---

## Follow-ups

- Extend `bootstrap_crates.sh` with dep blocks
- STEP-B1 AST types
