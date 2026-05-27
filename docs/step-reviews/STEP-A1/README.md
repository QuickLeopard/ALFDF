# STEP-A1 — Bootstrap Rust workspace

## Step

**STEP-A1** — Bootstrap Rust workspace (Phase A — Foundations)

## What shipped

- Workspace root [`Cargo.toml`](../../../Cargo.toml): resolver 3, Rust 1.95, edition 2024, workspace lints (warnings/clippy deny)
- [`rust-toolchain.toml`](../../../rust-toolchain.toml), [`rustfmt.toml`](../../../rustfmt.toml), [`.editorconfig`](../../../.editorconfig)
- 14 empty library crates under `crates/alfdf-*` (generated via [`scripts/bootstrap_crates.sh`](../../../scripts/bootstrap_crates.sh))
- [`justfile`](../../../justfile) with STEP-A1-scoped `verify` (build, clippy, fmt-check)
- Minimal [`.github/workflows/ci.yml`](../../../.github/workflows/ci.yml) (build, clippy, fmt)
- Root [`README.md`](../../../README.md)

## Whole-project progress

- **Phase A:** STEP-A1 complete; STEP-A2–A4 pending
- **MVP0:** 1 / ~80+ steps (see [step guide](../../../.DOCS/ALFDF-MVP0-Stepbystep-Guide-v0.md))

## Commands run

```text
$ just verify
cargo build --workspace    # Finished dev profile in ~1.84s — PASS
cargo clippy ...           # Finished dev profile in ~4.23s — PASS
cargo fmt --all -- --check # PASS (no output)
```

Run on: Linux (WSL2), toolchain `1.95.0-x86_64-unknown-linux-gnu` (from `rust-toolchain.toml`).

## Tests

- Acceptance per guide: `cargo build --workspace` — **PASS** (via `just verify`)
- No unit tests in A1 (scaffold only; TDD exception b)

## Benchmarks

Benchmarks: not applicable — STEP-A1 defines no performance targets.

## Risks and follow-ups

- **Spec vs guide crate names:** spec § crate layout lists 12 `abal-*` crates; step guide uses 14 `alfdf-*` crates. A1 follows the **guide**. Record in ADR at STEP-A3.
- **`just verify` subset:** deny, nextest, jscpd deferred to STEP-A2 per plan.
- **`alfdf-common`:** in build guide MVP0.0.3 but not in A1’s 14 crates; add when a step requires it.

## Review

- Verified all 14 workspace members compile with workspace `warnings = deny` and `clippy::all = deny`.
- CI workflow matches A1 “Done when” (build + clippy + fmt); test/deny jobs land in STEP-A2.
- Watch: first real code in STEP-B1 (`alfdf-ast`); ensure workspace.dependencies wired in STEP-A4 before heavy deps.
