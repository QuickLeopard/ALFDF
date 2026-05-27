# STEP-A3 — ADR directory + first ADR

## Step

**STEP-A3** — ADR directory + first ADR (Phase A — Foundations)

## What shipped

- [`docs/adr/README.md`](../../../docs/adr/README.md) — ADR index (MADR)
- [`docs/adr/0001-rust-implementation.md`](../../../docs/adr/0001-rust-implementation.md) — Rust toolchain, workspace layout, `alfdf-*` crate naming vs spec `abal-*` diagram

## Whole-project progress

- **Phase A:** STEP-A1–A3 done; STEP-A4 pending
- **MVP0:** 3 / ~80+ steps

## Commands run

```text
$ just verify
# PASS — all gates (build, clippy, fmt, nextest, jscpd, deny)
```

Docs-only step; no new Rust code.

## Tests

Benchmarks: not applicable.

## Risks and follow-ups

- Spec § crate layout still shows `abal-*` names — track via future **spec-amendment** step (ADR § Confirmation).
- Next: **STEP-A4** — `[workspace.dependencies]` for serde, tracing, etc.

## Review

- MADR format: Context, Decision Drivers, Options, Outcome, Consequences.
- Records STEP-A1 naming drift decision for agents and reviewers.
- No production code in this PR (ADR-only).
