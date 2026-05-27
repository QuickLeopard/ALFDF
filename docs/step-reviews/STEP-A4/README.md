# STEP-A4 — Add core utility crates

## Step

**STEP-A4** — Add core utility crates (Phase A — Foundations)

## What shipped

- [`Cargo.toml`](../../../Cargo.toml) — `[workspace.dependencies]` for serde, serde_json, thiserror, anyhow, tracing, blake3, proptest, insta, criterion (latest stable pins; criterion `0.8.2`)
- [`Cargo.lock`](../../../Cargo.lock) — resolved dependency graph
- All fourteen `crates/alfdf-*/Cargo.toml` — `[dependencies]` and `[dev-dependencies]` via `workspace = true`
- [`scripts/wire_workspace_deps.sh`](../../../scripts/wire_workspace_deps.sh) — idempotent wiring helper for future crate bootstrap
- [`deny.toml`](../../../deny.toml) — allow `BSD-2-Clause` (blake3 → arrayref) and `Unicode-3.0` (proc-macro chain)

## Whole-project progress

- **Phase A:** STEP-A1–A4 done; Phase B (AST) next
- **MVP0:** 4 / ~80+ steps — see [`.DOCS/ALFDF-MVP0-Stepbystep-Guide-v0.md`](../../../.DOCS/ALFDF-MVP0-Stepbystep-Guide-v0.md)

## Commands run

```text
$ just verify
# PASS — build, clippy, fmt, nextest --no-tests pass, jscpd, deny
```

## Tests

- `cargo nextest run --workspace --no-tests pass` — PASS (no tests yet; workspace compiles)
- No new unit tests (dependency wiring only)

Benchmarks: not applicable — no benchmarks defined in this step.

## Risks and follow-ups

- New transitive licenses must stay within `deny.toml` allow-list; re-run `just deny` when adding deps.
- Workspace dep version policy: `.cursor/rules/06-workspace-deps.mdc`; authoritative pin table in `05-tech-stack.mdc`.
- `scripts/bootstrap_crates.sh` does not yet append workspace dep blocks — wire manually or extend bootstrap in a later step.
- Next: **STEP-B1** — `Type` enum in `alfdf-ast`.

## Review

- Every member crate shares one version pin per dependency via workspace inheritance.
- `cargo deny check` green after expanding license allow-list for blake3 and serde/proptest transitive crates.
- No production Rust source changes; manifest-only step.
