# STEP-A1 — Bootstrap Rust workspace

---

## What changed

- Rust workspace + 14 `alfdf-*` lib skeletons
- Toolchain 1.95 / edition 2024 / workspace lints
- `just verify` (build, clippy, fmt)
- Minimal GitHub Actions CI

---

## Project progress

- Phase A: **A1 done**, A2–A4 open
- MVP0: first step landed

---

## Verification

- `just verify` — **PASS** (local)
- CI: build + clippy + fmt on PR

---

## Review / follow-ups

- Spec `abal-*` vs guide `alfdf-*` → ADR in A3
- Full `just verify` (deny, nextest) → STEP-A2
- Next: **STEP-A2** CI pipeline hardening
