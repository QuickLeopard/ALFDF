# ALFDF MVP0 — Step-by-Step Implementation Guide

**Version:** 0.1.0 · **Date:** 2026-05-22 · **Companion to:** spec.md v0.1.0

---

## How to read this guide

- The work is split into **6 MVPs**: **MVP0.0 → MVP0.5** (all part of "MVP0" in the spec, broken into shippable sub-milestones), then **MVP1** (CLI) and **MVP2** (IDE).
- Each MVP contains **Steps**. Each Step is sized to **≤ 1 day of focused work** for one engineer.
- Every Step has the same anatomy: Goal · Inputs · Outputs · Build · Test · Bench · Refactor · Done when.
- **Golden rule:** Do not start Step N+1 until Step N's "Done when" passes in CI.
- **Branch policy:** one branch per Step, squash-merge.

---

# MVP 0.0 — Bootstrap & Foundations

**Outcome:** empty Rust workspace, CI, hashing, AST types in memory, JSON Schema.

### Step 0.0.1 — Initialize workspace
- **Goal:** create the Cargo workspace skeleton.
- **Build:**
  - `cargo new --vcs git alfdf && cd alfdf`
  - Convert root to a workspace, add empty member crates listed in spec §12.
  - Add `rust-toolchain.toml` pinning stable + clippy + rustfmt.
  - Add `.editorconfig`, `rustfmt.toml`, `clippy.toml` (deny warnings).
- **Test:** `cargo build --workspace` succeeds.
- **Done when:** CI runs `cargo build && cargo test && cargo clippy -- -D warnings` green on an empty workspace.

### Step 0.0.2 — CI pipeline
- **Goal:** GitHub Actions runs build, test, clippy, fmt, deny.
- **Build:** `.github/workflows/ci.yml` with jobs: `fmt`, `clippy`, `test`, `deny`.
- **Done when:** all four jobs pass on `main`.

### Step 0.0.3 — Logging & error scaffolding
- **Goal:** common `tracing` + `thiserror` setup.
- **Build:** `crates/alfdf-common/` with `init_tracing()`, `AlfdfError` enum stub.
- **Done when:** every other crate depends on `alfdf-common` for errors.

### Step 0.0.4 — Define AST types (Rust enums)
- **Goal:** in-memory canonical AST.
- **Build:** in `alfdf-ast`, declare `Expr`, `Type`, `FnDecl`, `DataDecl`, `Module`. Derive `Debug`, `Clone`, `PartialEq`, `serde::{Serialize,Deserialize}`.
- **Test:** construct a `List.map` AST by hand; debug-print matches expected.
- **Done when:** types compile and serialize to deterministic JSON.

### Step 0.0.5 — JSON Schema for AST
- **Goal:** publish `schemas/abal-ast-1.0.0.json`.
- **Build:** use `schemars` to derive JSON Schema; commit the generated file.
- **Test:** validate 3 example ASTs against the schema; reject a malformed one.
- **Done when:** schema validation in CI.

### Step 0.0.6 — Content-addressed hashing
- **Goal:** Blake3 Merkle hash for any AST node.
- **Build:** `alfdf-ast::hash(node) -> ContentHash` using canonical serialization.
- **Test:** identical ASTs → identical hash; differing ASTs → different hash; cross-platform deterministic.
- **Bench:** hashing a 1000-node AST < 1 ms.

### Step 0.0.7 — Canonical s-expression printer (for embeddings)
- **Build:** `alfdf-ast::to_sexpr(node) -> String`.
- **Test:** golden-file test on `List.map`.

**Sub-milestone MVP0.0 done:** workspace + CI + AST types + hashing + schema, all green.

---

# MVP 0.1 — ABAL Parser & Pretty-Printer

**Outcome:** bijective text ↔ AST.

### Step 0.1.1 — Lexer
- **Build:** hand-written lexer in `alfdf-parser`. Tokens: keywords, identifiers, type-vars, literals, punctuation.
- **Test:** lex 50 small snippets.
- **Done when:** precise span (line, col) for every token.

### Step 0.1.2 — Parser for types
- **Build:** recursive-descent parser for `Type`. Pratt-style for arrows.
- **Test:** 30 type strings round-trip.

### Step 0.1.3 — Parser for expressions
- **Build:** Pratt parser for `Expr`.
- **Test:** 40 expressions round-trip; 10 hand-written reference equality.

### Step 0.1.4 — Parser for declarations
- **Build:** `fn`, `data`, `test`, `prop`, `law`, module.
- **Test:** parse 5 full modules; fuzz with `cargo-fuzz` 100k iters → no panics.

### Step 0.1.5 — Pretty-printer
- **Build:** `pretty(ast) -> String`, 80-col wrap, deterministic.
- **Test:** snapshot test on 10 ASTs.

### Step 0.1.6 — Bijection property test
- **Goal:** `parse ∘ pretty == id` and `pretty ∘ parse == id_norm`.
- **Build:** `proptest` generators for `Expr`, `Type`, `Module`.
- **Test:** 10 000 random ASTs round-trip (AC-1).

### Step 0.1.7 — Comment & whitespace side-channel
- **Build:** attach comments to nearest node as `attrs.doc`.
- **Test:** comments survive round-trip in 20 examples.

**MVP0.1 done:** ABAL text ↔ AST is robust and bijective.

---

# MVP 0.2 — Static Verification

**Outcome:** an AST is provably well-typed, total, deterministic before any execution.

### Step 0.2.1 — Type environment & primitives
- **Build:** `TypeEnv`; bootstrap with primitives and builtin signatures.

### Step 0.2.2 — Bidirectional type checker — checking mode
- **Build:** `check(expr, expected_ty, env)`.
- **Test:** 20 positive + 20 negative cases.

### Step 0.2.3 — Synthesis mode
- **Build:** `synth(expr, env)`.
- **Test:** 20 expressions with non-trivial polymorphism.

### Step 0.2.4 — Parametric polymorphism (rank-1)
- **Build:** instantiate at use sites; generalize at top-level fn boundaries.
- **Test:** identity, const, compose type-check.

### Step 0.2.5 — Pattern type checking
- **Test:** mismatched constructor pattern → typed error.

### Step 0.2.6 — Exhaustiveness checker
- **Build:** Maranget-style algorithm (simplified for ADTs).
- **Test:** non-exhaustive matches rejected with missing constructors listed.
- **Done when:** invariant I-11 enforced.

### Step 0.2.7 — Structural recursion checker
- **Build:** verify every recursive call passes a strict subterm of a pattern-bound variable.
- **Test:** `length`, `map`, `foldr` accepted; `loop()`, `f(x)=f(x)` rejected.

### Step 0.2.8 — Mutual recursion support
- **Build:** SCC detection + common measure.
- **Test:** mutually recursive `even/odd` accepted.

### Step 0.2.9 — Determinism static check
- **Build:** reject any non-whitelisted builtin reference.

### Step 0.2.10 — Emit certificates
- **Build:** `TotalityCert` attached to FnDecl post-checks.

**MVP0.2 done:** every static gate works.

---

# MVP 0.3 — VM, Tests, Fuzz, Laws

**Outcome:** can run ABAL code, run tests, fuzz properties, verify laws.

### Step 0.3.1 — Value representation
- **Build:** `Value` enum: `Int`, `Bool`, `Str`, `List`, `Tup`, `Ctor`, `Closure`.

### Step 0.3.2 — Big-step evaluator
- **Build:** `eval(expr, env, fuel)`.
- **Test:** 30 expressions to expected values.

### Step 0.3.3 — Fuel accounting
- **Test:** infinite construct terminates with `OutOfFuel`.

### Step 0.3.4 — Pattern matcher
- **Build:** simple linear scan (exhaustiveness guaranteed).

### Step 0.3.5 — Builtin operations
- **Build:** whitelisted Int/String/Bool/List ops.
- **Test:** one unit test per builtin.

### Step 0.3.6 — Test runner
- **Build:** `run_tests(fn_decl) -> TestReport`.
- **Test:** seed `List.map` tests pass; broken `map` fails with structured report.

### Step 0.3.7 — Value equality
- **Build:** deep structural; closures equal iff identical AST hash.

### Step 0.3.8 — Random generator framework
- **Build:** `Gen<a>` registry keyed by canonical type.

### Step 0.3.9 — Shrinker framework
- **Build:** `Shrink<a>` producing smaller candidates.

### Step 0.3.10 — Property runner
- **Build:** `run_prop(prop, cases=1000) -> PropReport`.

### Step 0.3.11 — Law-template registry (16 laws)
- **Build:** templated property per law in spec §4.2.
- **Test:** `Associative` passes on `+`, fails on `-` with counterexample.

### Step 0.3.12 — Bench harness
- **Build:** `criterion`-driven; record p50/p99 per fn.
- **Bench:** AC-18 (`List.map` over 10k Ints ≤ 50 ms).

**MVP0.3 done:** language runs and is verified end-to-end in memory.

---

# MVP 0.4 — Storage Layer

**Outcome:** persistent, content-addressed, versioned DB with type/graph/vector indices.

### Step 0.4.1 — StorageAdapter trait
- **Build:** 4 sub-traits + umbrella.
- **Done when:** I-9 structurally enforceable.

### Step 0.4.2 — SQLite schema migration
- **Build:** `refinery`-based; apply spec §8.4 DDL.

### Step 0.4.3 — TruthStore (SQLite)
- **Build:** CRUD for `entities`, `versions`.
- **Test:** round-trip insert/fetch by `content_hash`.

### Step 0.4.4 — GraphIndex (SQLite edges)
- **Build:** edges + recursive-CTE traversal.

### Step 0.4.5 — TypeIndex (in-process trie)
- **Build:** normalize → trie; persist as blob.
- **Bench:** lookup p99 ≤ 1 ms on 10 000 entries.

### Step 0.4.6 — Embedding pipeline
- **Build:** `bge-small-en-v1.5` via `candle`.
- **Bench:** ≥ 100 entries/sec (AC-17).

### Step 0.4.7 — VectorIndex (LanceDB)
- **Build:** open/create, upsert, ANN with metadata filter.

### Step 0.4.8 — Provenance, versions, supersession
- **Test:** body change → new version; signature change → new entity.

### Step 0.4.9 — Failed-attempts log + TTL
- **Build:** daily sweeper; honor `interesting` flag.

### Step 0.4.10 — Replacement-review queue
- **Build:** writes from dedup decision matrix; query API.

**MVP0.4 done.**

---

# MVP 0.5 — Dedup, Synthesis, Pipeline, MCP, Stdlib

**Outcome:** the user-visible MVP0 product.

### Step 0.5.1 — Type-isomorphism normalization (L1)
- **Build:** canonical form for type signatures.

### Step 0.5.2 — β-η-normal form (L2)
- **Build:** small β-η reducer.

### Step 0.5.3 — Shared fuzz-corpus comparator (L3)
- **Build:** generate 1000 inputs per type; compare outputs.
- **Test:** recursive vs accumulator `reverse` flagged equal (AC-11).

### Step 0.5.4 — Cross-test execution (L4)
- **Build:** run candidate ↔ existing test suites.

### Step 0.5.5 — Embedding similarity (L5)
- **Build:** cosine over `bge` vectors; never decisive alone.

### Step 0.5.6 — Confidence aggregation & decision matrix
- **Build:** rule engine implementing spec §7.4.

### Step 0.5.7 — Pipeline orchestrator
- **Build:** in `alfdf-pipeline`, 9 stages with structured aggregation. See dedicated mini-spec.

### Step 0.5.8 — Type-directed synthesis
- **Build:** bounded BFS, depth 3, budget 200.
- **Bench:** ≤ 500 ms p99 on 10k-entry DB.

### Step 0.5.9 — MCP server skeleton
- **Build:** JSON-RPC 2.0 over stdio + WebSocket.

### Step 0.5.10 — Implement tools 1–9
- One sub-step per tool, in order: `describe`, `explain`, `query_by_type`, `query_by_data`, `dependencies`, `diff_versions`, `submit_data`, `submit_fn`, `synthesize`.

### Step 0.5.11 — Implementation of `explain` renderer
- **Build:** pure `render(describe_json) -> Markdown`. No storage access.
- **Test:** AC-10 — facts in markdown ⊇ facts in JSON.

### Step 0.5.12 — Seed stdlib (40+ functions, 8 data types)
- **Build:** ABAL sources under `alfdf-stdlib/src/abal/`.
- **Test:** AC-2 — full pipeline admits all stdlib entries cleanly.

### Step 0.5.13 — Metrics endpoint
- **Build:** Prometheus exposition on `/metrics`.

### Step 0.5.14 — Conformance suite (I-1…I-12)
- **Build:** one test per invariant; AC-12 passes.

### Step 0.5.15 — Grep-based architecture tests
- **Build:** AC-13, AC-14.

### Step 0.5.16 — 20-task benchmark harness
- **Build:** measure AC-26, AC-27, AC-29.

### Step 0.5.17 — Release v0.1.0
- **Done when:** all 30 acceptance criteria green; tag pushed.

**MVP0 done.** Ship `v0.1.0`.

---

# MVP 1 — CLI Tool

### Step 1.1 — `alfdf` binary scaffold (clap subcommands).
### Step 1.2 — `alfdf submit <file.abal>`.
### Step 1.3 — `alfdf query --signature "<sig>"`.
### Step 1.4 — `alfdf describe <id>` / `alfdf explain <id>`.
### Step 1.5 — `alfdf synthesize --target "<type>"`.
### Step 1.6 — `alfdf diff <id> --from v1 --to v2`.
### Step 1.7 — Config file & profiles (`~/.config/alfdf/config.toml`).
### Step 1.8 — Shell completions (bash/zsh/fish).
### Step 1.9 — Release v0.2.0.

---

# MVP 2 — IDE / LSP Plugin

### Step 2.1 — LSP server skeleton (`tower-lsp`).
### Step 2.2 — Diagnostics from the pipeline.
### Step 2.3 — Hover = `abal.describe` rendered.
### Step 2.4 — Completion via type query.
### Step 2.5 — Code action: "Replace with existing equivalent".
### Step 2.6 — Code action: "Synthesize from existing fns".
### Step 2.7 — VS Code extension (TypeScript wrapper).
### Step 2.8 — Release v0.3.0.

---

# Cross-cutting practices

- **Testing pyramid:** unit → property → golden → integration → conformance.
- **Bench discipline:** `criterion` with committed baselines; CI warns on >10% regression.
- **Refactor cadence:** every 2 steps, allocate 30 min for cleanup.
- **Doc per Step:** rustdoc updates + CHANGELOG entry on sub-milestones.
- **Simplicity bound:** if a Step's Build list exceeds 5 sub-tasks or 1 day → split.

---

# Execution order & timeline

| Sub-milestone | Steps | Effort (1 engineer) |
|---|---|---|
| MVP0.0 Bootstrap | 0.0.1 – 0.0.7 | 1 week |
| MVP0.1 Parser | 0.1.1 – 0.1.7 | 1.5 weeks |
| MVP0.2 Verification | 0.2.1 – 0.2.10 | 2 weeks |
| MVP0.3 VM + Fuzz | 0.3.1 – 0.3.12 | 2 weeks |
| MVP0.4 Storage | 0.4.1 – 0.4.10 | 2 weeks |
| MVP0.5 Dedup + MCP + Stdlib | 0.5.1 – 0.5.17 | 4 weeks |
| **MVP0 v0.1.0** | | **~12.5 weeks** |
| MVP1 CLI | 1.1 – 1.9 | 2 weeks → v0.2.0 |
| MVP2 LSP | 2.1 – 2.8 | 3 weeks → v0.3.0 |

Two engineers in parallel cut MVP0 to ~7 weeks (verification and storage tracks are independent after MVP0.1).

---

# Day 1 checklist

1. Step 0.0.1 — create workspace.
2. Step 0.0.2 — green CI.
3. Step 0.0.4 — Rust AST enums committed.
4. ADR-0001 (Rust) + ADR-0002 (LanceDB) opened.
5. Bulk-import the ~70 Step issues from `alfdf-issues-import.csv`.

---

*End of guide — ALFDF MVP0 v0.1.0 — 2026-05-22.*
