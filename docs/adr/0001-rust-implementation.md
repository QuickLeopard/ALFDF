# 1. Rust implementation language and workspace layout

- **Date:** 2026-05-27
- **Status:** Accepted
- **Deciders:** ALFDF MVP0 implementers
- **Related:** [Project spec § crate layout](../../.DOCS/ALFDF-MVP0-Project-Spec-v0.1.md), [Step guide Phase A–O](../../.DOCS/ALFDF-MVP0-Stepbystep-Guide-v0.md)

## Context and Problem Statement

ALFDF MVP0 requires a systems implementation language for fourteen library crates (AST, parser, type checker, storage, MCP server, etc.), strict CI gates, and content-addressed persistence. The project spec names Rust (edition 2024+) and shows a workspace with mixed `abal-*` (language) and `alfdf-*` (framework) crate names. The step-by-step guide defines **fourteen `alfdf-*` crates** and phases B–O that map one crate per concern.

We need a single, documented choice for toolchain, workspace shape, and crate naming so agents and contributors do not split between spec diagram and guide implementation.

## Decision Drivers

- **Performance and safety** — parsing, hashing, and storage paths are hot; memory safety without a GC.
- **Ecosystem** — serde, SQLite/LanceDB bindings, MCP/JSON-RPC, property testing (proptest), benchmarks (criterion).
- **Agent workflow** — pinned toolchain, `just verify`, workspace lints, and step-by-step guide are already Rust-centric (STEP-A1, STEP-A2).
- **Naming clarity** — ABAL is the *language*; ALFDF is the *framework*. Crate names should reflect ownership boundaries.

## Considered Options

1. **Rust with spec diagram names (`abal-*` + `alfdf-*`)** — matches spec § crate layout literally.
2. **Rust with guide names (all `alfdf-*`)** — matches step guide and existing workspace (STEP-A1).
3. **Alternative language (e.g. TypeScript, Go)** — rejected; spec and guides target Rust.

## Decision Outcome

**Chosen:** Option 2 — **Rust 1.95**, **edition 2024**, **fourteen `alfdf-*` library crates** under `crates/`, as implemented in STEP-A1 and enumerated in the step guide (Phases B–O).

Toolchain and workspace policy:

| Item | Value |
|------|--------|
| Toolchain | `1.95.0` (`rust-toolchain.toml`) |
| Edition / MSRV | 2024 / `1.95` |
| Resolver | `3` |
| Lints | Workspace `[workspace.lints]` — `warnings = deny`, `clippy::all = deny` |
| Local gate | `just verify` — build, clippy, fmt, nextest, jscpd, deny (STEP-A2) |

**ABAL vs ALFDF naming:** JSON Schema and wire identifiers keep the **`abal-*` namespace** where the language is referenced (e.g. `abal-ast/1.0.0` in STEP-B6). **Cargo crate names** use **`alfdf-*`** for all fourteen workspace members to avoid split prefixes in `Cargo.toml` and CI.

### Consequences

- **Positive:** One prefix in the workspace; guide steps map 1:1 to crate directories; agents follow a single layout.
- **Positive:** Rust + pinned stack align with spec goals (total functions, hashing, MCP JSON surfaces).
- **Negative:** Spec § crate layout diagram is **stale** until a future spec-amendment step aligns prose with this ADR.
- **Neutral:** `alfdf-bench` from the spec diagram is **`alfdf-metrics`** in the guide (Phase O); benchmarks live there until renamed in a later ADR if needed.

## Confirmation

- [x] Workspace builds with `cargo build --workspace` (STEP-A1).
- [x] CI and `just verify` green on `main` after STEP-A2 merge (PR #3).
- [ ] Spec text updated — deferred to explicit spec-amendment PR (not mixed with this ADR-only step).

## Pros and Cons of the Options

### Option 1 — Spec diagram names (`abal-*` + `alfdf-*`)

- Good: Matches spec figure verbatim.
- Bad: Requires renaming existing crates and updating every guide reference; two prefixes complicate workspace dependency graphs.

### Option 2 — Guide names (all `alfdf-*`) — **selected**

- Good: Matches merged STEP-A1; single naming rule; simpler for agents.
- Bad: Spec diagram needs amendment.

### Option 3 — Non-Rust

- Bad: Contradicts spec, guides, and shipped workspace.

## Links

- [STEP-A1 review](../step-reviews/STEP-A1/README.md) — crate naming note
- [MADR template](https://adr.github.io/madr/)
