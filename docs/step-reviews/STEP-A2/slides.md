# STEP-A2 — CI pipeline

---

## What changed

- CI split into fmt, clippy, test, build, deny, jscpd jobs
- `deny.toml` + full `just verify`

---

## Project progress

- Phase A: **A1–A2 done**, A3–A4 open

---

## Verification

- `just verify` — **PASS** locally
- CI on PR — pending merge

---

## Review / follow-ups

- nextest `--no-tests pass` until tests land in Phase B
- STEP-A3: ADR + crate naming ADR
