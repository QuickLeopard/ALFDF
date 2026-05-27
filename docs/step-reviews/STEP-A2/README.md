# STEP-A2 — CI pipeline

## Step

**STEP-A2** — CI pipeline (Phase A — Foundations)

## What shipped

- [`.github/workflows/ci.yml`](../../../.github/workflows/ci.yml) — six parallel jobs: `fmt`, `clippy`, `test`, `build`, `deny`, `jscpd`
- [`deny.toml`](../../../deny.toml) — MIT/Apache-2.0 license policy, advisories, bans, sources
- [`justfile`](../../../justfile) — full `verify`: build, clippy, fmt-check, test (nextest), jscpd, deny
- [`README.md`](../../../README.md) — prerequisites (nextest, deny, Node 20)

## Whole-project progress

- **Phase A:** STEP-A1–A2 done; STEP-A3–A4 pending
- **MVP0:** 2 / ~80+ steps (see [step guide](../../../.DOCS/ALFDF-MVP0-Stepbystep-Guide-v0.md))

## Commands run

```text
$ just verify
cargo build --workspace     # PASS
cargo clippy ...            # PASS
cargo fmt --all -- --check  # PASS
cargo nextest run --workspace --no-tests pass  # PASS (0 tests, skeleton workspace)
./scripts/jscpd_gate.sh     # PASS
cargo deny check            # PASS (advisories, bans, licenses, sources ok)
```

## Tests

- Empty workspace: nextest uses `--no-tests pass` until crates add tests (STEP-B+).
- **Gating demos:** this PR validates passing CI; fmt-fail demo: run `cargo fmt` without check locally on a dirty branch — CI `fmt` job would fail on push.

## Benchmarks

Benchmarks: not applicable — STEP-A2 defines no performance targets.

## Risks and follow-ups

- **`scripts/tdd-order.sh`:** still deferred; step-ship documents manual RED-before-GREEN until wired.
- **Pre-commit hooks:** optional follow-up (not in STEP-A2 scope).
- Next: **STEP-A3** ADR directory + `0001-rust-implementation.md`.

## Review

- CI mirrors [05-tech-stack.mdc](../../../.cursor/rules/05-tech-stack.mdc): Rust 1.95, nextest, deny, jscpd (Node 20).
- `just verify` now matches project `system.md` rule 2 for full local gate.
- Watch: first crate with external deps must pass `cargo deny check` in CI.
