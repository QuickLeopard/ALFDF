# ALFDF — Step-by-Step Implementation Guide

**Companion to:** ALFDF MVP0 Project Specification v0.1.0
**Version:** 0.1.0
**Date:** 2026-05-22
**Implementation language:** Rust (stable, edition 2024)

---

## How to use this guide

Each **step** is an atomic unit of work designed to be:

- **Small** — ≤ 1 day of focused work for one engineer.
- **Self-contained** — clear inputs and outputs.
- **Verifiable** — explicit "Done when…" criteria.
- **Refactorable** — survives later changes without rewrites.

Each step has the same structure:

> **Goal** · **Inputs** · **Output artifact** · **Implementation notes** · **Tests** · **Benchmarks (if any)** · **Done when**

Steps are grouped into **phases**, phases into **MVPs**. Do not skip ordering: later phases assume earlier ones are merged to `main` and green in CI.

**Branch and PR discipline:**
- One step = one PR.
- PR title: `[STEP-<id>] <short title>`.
- PR template enforces: tests added, benchmarks added (if applicable), invariants checked.

---

# MVP0 — Core engine + MCP server

Estimated effort: ~16 weeks for one experienced Rust engineer, ~8–10 weeks for two.

---

## Phase A — Foundations

### STEP-A1 — Bootstrap Rust workspace
- [x] Done
- **Goal:** Empty workspace with all 14 crate skeletons compiling.
- **Inputs:** None.
- **Output:** `Cargo.toml` (workspace) + 14 empty `crates/alfdf-*/` libraries.
- **Notes:** Use `cargo new --lib`. Enable `[workspace.lints]` deny-warnings. Set MSRV. Add `rust-toolchain.toml` pinning stable.
- **Tests:** `cargo build --workspace` succeeds.
- **Done when:** CI runs `cargo build && cargo clippy -- -D warnings && cargo fmt -- --check` green.

### STEP-A2 — CI pipeline
- **Goal:** GitHub Actions (or equivalent) runs build + test + clippy + fmt + `cargo deny check`.
- **Output:** `.github/workflows/ci.yml`.
- **Tests:** PR with broken fmt fails CI; PR with passing checks succeeds.
- **Done when:** Two demo PRs (one passing, one failing) confirm gating.

### STEP-A3 — ADR directory + first ADR
- **Goal:** `docs/adr/` with `0001-rust-implementation.md`.
- **Notes:** Use MADR template.
- **Done when:** ADR merged.

### STEP-A4 — Add core utility crates
- **Goal:** Wire common deps once: `serde`, `serde_json`, `thiserror`, `anyhow`, `tracing`, `blake3`, `proptest`, `insta`, `criterion`.
- **Output:** `workspace.dependencies` block.
- **Done when:** All crates reference shared versions via `workspace = true`.

---

## Phase B — AST and hashing (`alfdf-ast`)

### STEP-B1 — Define core type enum `Type`
- **Goal:** Rust enum for ABAL types: `Var(String)`, `Con(String, Vec<Type>)`, `Arrow(Box<Type>, Box<Type>)`, `Tuple(Vec<Type>)`.
- **Tests:** Round-trip `serde_json` 100 hand-written cases.
- **Done when:** `Type` is `Clone + Eq + Hash + Serialize + Deserialize`.

### STEP-B2 — Define core expression enum `Expr`
- **Goal:** `Lit`, `Var`, `App`, `Lambda`, `Let`, `Match`, `Ctor`, `TupleLit`.
- **Notes:** Each variant carries a `Span` (file, byte range) for diagnostics.
- **Tests:** JSON round-trip property test (10k random `Expr` via `proptest`).
- **Done when:** Property test green.

### STEP-B3 — Define top-level items
- **Goal:** `DataDecl`, `FnDecl`, `LawDecl`, `TestDecl`, `PropDecl`, `Module`.
- **Tests:** JSON round-trip + golden file under `tests/golden/ast/`.
- **Done when:** 5 sample modules survive round-trip.

### STEP-B4 — Canonical S-expression serializer (for hashing)
- **Goal:** `fn to_sexp(node) -> String` producing a deterministic representation that ignores spans.
- **Tests:** Re-ordering optional fields does not change output; spans do not change output.
- **Done when:** Two ASTs with same logical content yield identical S-exp.

### STEP-B5 — Blake3 Merkle hashing
- **Goal:** `fn content_hash(node) -> [u8; 32]` over the S-exp.
- **Tests:** Identical nodes → identical hashes; one-bit change → different hash.
- **Done when:** 1000-case property test green.

### STEP-B6 — JSON Schema for AST `abal-ast/1.0.0`
- **Goal:** `schemas/abal-ast-1.0.0.json` covering all enums.
- **Tests:** Validate 5 golden modules against schema with `jsonschema` crate.
- **Done when:** Validator passes; CI step enforces it.

---

## Phase C — Parser and pretty-printer (`alfdf-parser`)

### STEP-C1 — Lexer
- **Goal:** Tokenize ABAL source: keywords, identifiers, literals, punctuation, comments (preserved out-of-band).
- **Library:** `logos`.
- **Tests:** 50 small inputs with expected token streams.
- **Done when:** All tokens produced with correct spans.

### STEP-C2 — Parser for types and literals
- **Library:** `chumsky` (or hand-written PEG).
- **Tests:** 30 type strings ↔ `Type` AST.
- **Done when:** Parses every type used in seed stdlib.

### STEP-C3 — Parser for expressions
- **Goal:** Parse `Expr` including pattern matches and lambdas.
- **Tests:** 50 expression inputs.
- **Done when:** Each expression also pretty-prints back to a parsable form.

### STEP-C4 — Parser for top-level items
- **Goal:** `DataDecl`, `FnDecl`, `TestDecl`, `PropDecl`, `LawDecl`.
- **Tests:** Parse `examples/map.abal`, `examples/list.abal`.
- **Done when:** All seed stdlib parses.

### STEP-C5 — Pretty-printer
- **Goal:** `fn pretty(module) -> String`.
- **Notes:** Use `pretty` crate or hand-write with fixed indent.
- **Tests:** Manual sample comparisons.
- **Done when:** Output is human-readable.

### STEP-C6 — Bijection property test
- **Goal:** `forall m. parse(pretty(m)) == m` (modulo spans).
- **Tests:** `proptest` with 10,000 random modules.
- **Done when:** Property green; this satisfies **AC-1**.

### STEP-C7 — Error reporting
- **Library:** `ariadne` or `miette`.
- **Goal:** Rich diagnostics with caret positions.
- **Tests:** 10 deliberately broken inputs produce useful errors.
- **Done when:** Snapshot tests via `insta` are stable.

---

## Phase D — Type checker (`alfdf-typeck`)

### STEP-D1 — Type environment
- **Goal:** `Env { vars: HashMap<String, Type>, ctors: HashMap<String, CtorSig>, fns: HashMap<String, FnSig> }`.
- **Tests:** Insert/lookup unit tests.

### STEP-D2 — Type equality and instantiation
- **Goal:** `fn equal(a: &Type, b: &Type)`, `fn instantiate(scheme, args)`.
- **Tests:** Polymorphic instantiation cases.

### STEP-D3 — Bidirectional check for literals and variables
- **Goal:** `check(expr, ty)` and `infer(expr) -> ty`.
- **Tests:** Trivial expressions.

### STEP-D4 — Application and lambda
- **Goal:** Function application typing; lambda checks against expected arrow type.
- **Tests:** `(\x: Int => x + 1)(2) : Int`.

### STEP-D5 — Let, tuples, constructors
- **Tests:** 20 small programs.

### STEP-D6 — Pattern matching
- **Goal:** Type each pattern, bind variables, check arms have unified result type.
- **Tests:** 15 match expressions with ADTs.

### STEP-D7 — Top-level decl checking
- **Goal:** Check `FnDecl` body against declared signature; ensure all `TestDecl`/`PropDecl` typecheck.
- **Tests:** Whole seed stdlib typechecks.

### STEP-D8 — Diagnostics
- **Goal:** Structured `TypeError` enum with span; integrate with parser's reporter.
- **Tests:** Snapshot tests of 15 error cases.

### STEP-D9 — Coverage to 80%
- **Done when:** `cargo tarpaulin` ≥ 80% on this crate (part of **AC-19**).

---

## Phase E — Totality and exhaustiveness (`alfdf-totality`)

### STEP-E1 — Exhaustiveness checker for matches
- **Goal:** Verify pattern matrix covers all constructors of the scrutinee's ADT.
- **Algorithm:** Maranget's "Warnings for pattern matching" (simple variant for closed ADTs, no guards).
- **Tests:** 20 cases: exhaustive accepted, non-exhaustive rejected with missing pattern hint.
- **Done when:** Snapshot tests stable.

### STEP-E2 — Decreasing-argument inference
- **Goal:** For each recursive function, identify which argument shrinks in every recursive call.
- **Algorithm:** Track sub-term relation through pattern bindings: `Cons(h, t) => ... f(t) ...` ⇒ `t` is strictly smaller than original.
- **Tests:** 10 standard recursions (map, foldr, length).

### STEP-E3 — Termination certificate
- **Goal:** Emit `TotalityCert { method: "structural_recursion", decreasing_arg: "xs" }`.
- **Tests:** Cert attached to all seed functions.

### STEP-E4 — Rejection of non-structural recursion
- **Tests:** `fn loop(): Int = loop()` rejected with `reason_code = "totality.non_structural_recursion"`. Verifies **AC-5**.

### STEP-E5 — Mutual recursion (simple case)
- **Goal:** Detect SCC; require a common decreasing measure (size sum).
- **Tests:** Even/odd example accepted.
- **Notes:** If complex, ship gated behind a flag and document as known-limited; full support deferred to MVP1.

---

## Phase F — Virtual Machine (`alfdf-vm`)

### STEP-F1 — Value representation
- **Goal:** `Value` enum: `VInt`, `VBool`, `VString`, `VList`, `VTuple`, `VCtor`, `VClosure`.
- **Tests:** Equality and Debug.

### STEP-F2 — Environment and substitution
- **Goal:** Persistent env with O(log n) extend.
- **Library:** `rpds` or simple `Arc<Frame>` chain.

### STEP-F3 — Tree-walking evaluator (core)
- **Goal:** `eval(expr, env, fuel) -> RunResult`.
- **Tests:** 30 unit tests on arithmetic, lambdas, lets.

### STEP-F4 — Pattern match runtime
- **Tests:** All match expressions used in seed stdlib evaluate correctly.

### STEP-F5 — Constructor application & ADT values
- **Tests:** `Cons(1, Nil)` evaluates and prints.

### STEP-F6 — Fuel and out-of-fuel handling
- **Default fuel:** `10_000_000`.
- **Tests:** Infinite recursion in test-only mode hits fuel cap and returns `OutOfFuel`.

### STEP-F7 — Builtin whitelist
- **Goal:** Register deterministic primitives for `Int`, `String`, `List`.
- **Tests:** Each builtin has a unit test.

### STEP-F8 — Benchmark `List.map` over 10k Int
- **Library:** `criterion`.
- **Target:** ≤ 50 ms. Verifies **AC-18**.
- **Done when:** Bench stored under `benches/vm_map.rs` with documented baseline.

---

## Phase G — Property fuzzer and laws (`alfdf-fuzz`)

### STEP-G1 — Random value generator per type
- **Goal:** `fn gen(ty: &Type, rng) -> Value`.
- **Tests:** Distribution sanity for Int, Bool, List.

### STEP-G2 — Shrinker per type
- **Goal:** `fn shrink(v: &Value) -> Vec<Value>`.
- **Tests:** Shrink List → smaller Lists; shrink Int toward 0.

### STEP-G3 — Property runner
- **Goal:** Run a `prop` function with `N=1000` random inputs; on failure, shrink to a minimal counterexample.
- **Tests:** Known-true and known-false props.

### STEP-G4 — Law template registry (16 laws)
- **Goal:** Each tag (`Associative`, `Commutative`, …) carries a code generator that emits the underlying `prop` body parameterized by the candidate function.
- **Tests:** Generate `Functor.Identity` for `List.map` and confirm pass.

### STEP-G5 — Counterexample minimization & storage
- **Tests:** A buggy `map` (off-by-one) produces a minimal counterexample.

### STEP-G6 — Seed corpora per type
- **Goal:** Save 100 representative inputs per primitive type in DB for replay.
- **Done when:** Corpora reproducible across runs (seeded RNG).

---

## Phase H — Storage (`alfdf-storage`)

### STEP-H1 — Define `StorageAdapter` trait + sub-traits
- **Goal:** Traits `TruthStore`, `TypeIndex`, `GraphIndex`, `VectorIndex` with method signatures.
- **Tests:** Compiles; no impls yet.

### STEP-H2 — SQLite migrations (DDL from spec §8.4)
- **Library:** `refinery` or `sqlx::migrate!`.
- **Tests:** Migrations apply on empty DB; idempotent.

### STEP-H3 — `TruthStore` impl over SQLite
- **Goal:** CRUD for entities, versions, tests, laws, benches.
- **Tests:** Insert + read-back + content_hash uniqueness.

### STEP-H4 — `GraphIndex` impl (edges table + recursive CTEs)
- **Tests:** Insert edges; query transitive closure of `depends_on`.

### STEP-H5 — Type-trie in-process index
- **Goal:** Trie keyed by normalized type S-expression; values are entity hashes.
- **Tests:** Insert 1000 signatures; exact lookup p99 ≤ 1 ms.

### STEP-H6 — Type-trie persistence
- **Goal:** Serialize/deserialize to SQLite blob on startup/shutdown.
- **Tests:** Round-trip with 1000 entries.

### STEP-H7 — LanceDB `VectorIndex` impl
- **Library:** `lancedb` crate.
- **Goal:** Insert (id, vector, metadata); ANN search top-k with metadata filters.
- **Tests:** Insert 10k random vectors; query latency p99 ≤ 20 ms.

### STEP-H8 — Storage Adapter isolation lint
- **Goal:** CI grep step rejects imports of `rusqlite`, `lancedb` outside `crates/alfdf-storage/`.
- **Done when:** Satisfies **I-9** and **AC-13**.

### STEP-H9 — In-memory test adapter
- **Goal:** Pure-in-memory implementation of the four sub-traits for fast unit tests.
- **Tests:** Equivalent test suite passes on both backends.

---

## Phase I — Embeddings (`alfdf-embed`)

### STEP-I1 — Wire `candle` + bge-small-en-v1.5
- **Goal:** Load model on first use; produce 384-dim vectors.
- **Tests:** Embed 5 strings, check determinism, vector norm sane.

### STEP-I2 — AST-aware serializer for embedding input
- **Goal:** `fn embed_input(entry) -> String` concatenating `name`, `docstring`, and structural S-exp.
- **Tests:** Snapshot of inputs for 5 seed functions.

### STEP-I3 — Bulk embed throughput bench
- **Target:** ≥ 100 entries/sec on dev laptop. Verifies **AC-17**.

### STEP-I4 — Embedding cache
- **Goal:** Skip re-embedding if `content_hash` already mapped to a vector.
- **Tests:** Second ingest of identical entry is zero-cost.

---

## Phase J — Dedup engine (`alfdf-dedup`)

### STEP-J1 — L1: type-signature hash + isomorphism normalization
- **Goal:** Currying + arg-reorder + α-renaming → canonical hash.
- **Tests:** `(a -> b -> c)` and `(a, b) -> c` map to same canonical form after uncurrying? **Decision:** keep as distinct in MVP0 (no auto-uncurry); document in ADR.
- **Tests:** 30 type pairs labeled iso/non-iso.

### STEP-J2 — L2: β-η-normal-form AST hash
- **Goal:** Normalize: inline let-bindings used once, η-reduce lambdas where applicable, α-rename binders to de Bruijn.
- **Tests:** Two semantically identical implementations of `identity` collide.

### STEP-J3 — L3: shared fuzz-corpus output comparison
- **Goal:** Run both candidates on the seed corpus for their input type; compare outputs.
- **Tests:** Recursive vs accumulator `reverse` detected as equal. Verifies **AC-11**.

### STEP-J4 — L4: cross-test execution
- **Goal:** Run candidate against existing entry's tests and vice versa.
- **Tests:** Same `reverse` pair confirmed.

### STEP-J5 — L5: embedding similarity scoring
- **Goal:** Cosine over embeddings; never decisive, only informs ranking.
- **Tests:** Similar-but-different functions score high but are not flagged as dupe alone.

### STEP-J6 — Decision matrix
- **Goal:** Combine confidences; emit `Reject` / `MergeAsAlias` / `StoreAsAlternative` / `Accept`.
- **Tests:** All four branches reachable from hand-crafted cases. Verifies **AC-3** and **AC-4**.

### STEP-J7 — Replacement-review queue
- **Goal:** Insert into `replacement_review` table on `MergeAsAlias`.
- **Tests:** Row inserted with deltas; status `pending`.

---

## Phase K — Synthesis (`alfdf-synthesize`)

### STEP-K1 — Type-directed candidate enumeration
- **Goal:** Given a target type, enumerate compositions of existing functions matching it, depth ≤ 3.
- **Algorithm:** Backward search guided by the type-trie.
- **Tests:** Target `(Int -> Int) -> List<Int> -> List<Int>` produces `List.map` candidate.

### STEP-K2 — Budget and pruning
- **Defaults:** depth 3, ≤ 200 candidates, ≤ 500 ms.
- **Tests:** Budget enforced.

### STEP-K3 — Candidate validation
- **Goal:** Each returned candidate is type-correct against the target.
- **Tests:** No invalid candidate slips through.

### STEP-K4 — Fallback satisfies AC-8
- **Tests:** Remove `List.map`, request synthesis, get a valid composition.

---

## Phase L — Pipeline orchestrator (`alfdf-pipeline`)

### STEP-L1 — Define `Stage` trait
- **Goal:** Uniform `fn run(&self, ctx) -> StageResult` for each pipeline stage.

### STEP-L2 — Implement stages 1–9 as `Stage` impls
- **Tests:** Each stage unit-testable in isolation with the in-memory storage adapter.

### STEP-L3 — Orchestrator with retry policy
- **Goal:** On hard-gate failure, return structured `PipelineFailure`. Caller (MCP layer) decides retry.
- **Tests:** 9 cases — one failure per stage — produce correctly typed errors.

### STEP-L4 — Failed-attempts log + 30-day TTL purge job
- **Goal:** Background task or on-demand purge.
- **Tests:** Insert with `expires_at` past; purge removes; `interesting=true` exempted.

### STEP-L5 — End-to-end submission test
- **Tests:** Submit `List.map` from scratch; observe accepted; second submission rejected (dedup). Verifies **AC-3**.

### STEP-L6 — Pipeline wall-clock budget enforcement
- **Default:** 60 s.
- **Tests:** Artificially slow stage triggers budget exceedance.

---

## Phase M — MCP server (`alfdf-mcp`)

### STEP-M1 — JSON-RPC 2.0 framing
- **Library:** `jsonrpsee` or hand-rolled.
- **Tests:** Echo handler over stdio.

### STEP-M2 — Stdio transport
- **Tests:** Smoke test with a scripted client.

### STEP-M3 — WebSocket transport
- **Tests:** Same smoke test over WS.

### STEP-M4 — Implement `abal.describe`
- **Schema:** `schemas/mcp-describe-1.0.0.json`.
- **Tests:** Output validates against schema for 5 seed entries. Verifies **AC-9**.

### STEP-M5 — Implement `abal.explain`
- **Goal:** Pure markdown renderer over `describe` output.
- **Tests:** Every fact in `describe` JSON appears in markdown (parse-back property test). Verifies **AC-10** and **I-7**.

### STEP-M6 — Implement `abal.query_by_type`
- **Tests:** AC-7 passes; ranking weights produce expected order on a seeded DB.

### STEP-M7 — Implement `abal.query_by_data`
- **Tests:** Query for "supports Functor" returns `List`.

### STEP-M8 — Implement `abal.synthesize`
- **Tests:** AC-8 passes.

### STEP-M9 — Implement `abal.submit_fn` and `abal.submit_data`
- **Tests:** End-to-end on seed stdlib.

### STEP-M10 — Implement `abal.diff_versions`
- **Tests:** Two versions of a function produce structured diff.

### STEP-M11 — Implement `abal.dependencies`
- **Tests:** Transitive closure correct for a chain of 3 dependent functions.

### STEP-M12 — Schema enforcement middleware
- **Goal:** Every outgoing response validated against the registered schema (debug builds; sampled in release).
- **Done when:** Verifies **I-8** and **AC-14**.

### STEP-M13 — Prose-output lint
- **Goal:** CI grep + unit test: only `abal.explain` returns markdown content. Verifies **I-6**.

---

## Phase N — Seed stdlib (`alfdf-stdlib`)

### STEP-N1 — `Option`, `Result`, `Either` data types
- **Tests:** Submitted through pipeline; accepted.

### STEP-N2 — `List`, `NonEmptyList`, `BinaryTree`, `Pair`, `Tuple2/3` data types
- **Tests:** All accepted.

### STEP-N3 — Core combinators
- **Functions:** `identity`, `const`, `compose`, `flip`.
- **Tests:** All accepted with `Functor.Identity` / composition laws where applicable.

### STEP-N4 — List functions (group of ~18)
- **Functions:** `map`, `filter`, `foldl`, `foldr`, `length`, `reverse`, `concat`, `take`, `drop`, `zip`, `unzip`, `all`, `any`, `find`, `head_opt`, `tail_opt`, plus 2 more as needed.
- **Tests:** Each has ≥3 tests + declared laws where applicable.

### STEP-N5 — Option/Result functions
- **Functions:** `map`, `andThen`, `getOrElse`, `isSome`, `isNone`, `mapErr`, `isOk`, `isErr`.

### STEP-N6 — Int functions
- **Functions:** `add`, `sub`, `mul`, `divOpt`, `modOpt`, `abs`, `min`, `max`, `cmp`.

### STEP-N7 — String + BinaryTree functions
- **Functions:** `length`, `concat`, `reverse`, `parseIntOpt`, `insert`, `contains`, `toList`, `depth`.

### STEP-N8 — Verify total count
- **Done when:** ≥ 40 functions and 8 data types live in the DB; verifies **AC-2**.

---

## Phase O — Metrics (`alfdf-metrics`)

### STEP-O1 — Prometheus exposition
- **Library:** `prometheus` crate.
- **Tests:** `/metrics` returns valid exposition.

### STEP-O2 — Wire counters and histograms
- **Counters:** submissions_total, rejections_by_stage, dedup_outcomes, synthesize_invocations.
- **Histograms:** pipeline_duration, query_latency, vm_eval_duration.

### STEP-O3 — Events table writer
- **Tests:** Each MCP call writes a metrics_event row.

### STEP-O4 — Token-savings instrumentation
- **Goal:** Estimate tokens saved per query (size of referenced entries' bodies vs hashes).
- **Tests:** Synthetic 20-task benchmark prints aggregate. Used for **AC-26**.

---

## Phase P — Hardening, benchmarks, acceptance

### STEP-P1 — Conformance test suite
- **Goal:** Single binary that loads a populated DB and verifies I-1…I-12.
- **Tests:** Run in CI; fails on planted violation. Verifies **AC-12**.

### STEP-P2 — Performance bench: query p99 ≤ 50 ms
- **Setup:** Seed 10,000 synthetic entries.
- **Tests:** `criterion` bench. Verifies **AC-15**.

### STEP-P3 — Performance bench: submission p99 ≤ 5 s
- **Tests:** Verifies **AC-16**.

### STEP-P4 — 20-task benchmark harness (token savings)
- **Goal:** Script runs the same tasks with and without ALFDF; records token deltas.
- **Tests:** ≥ 30% reduction. Verifies **AC-26**.

### STEP-P5 — Dedup rate measurement
- **Tests:** ≥ 20% of submissions rejected/merged on the benchmark. Verifies **AC-27**.

### STEP-P6 — Synthesis pickup measurement
- **Tests:** ≥ 15% of tasks use a synthesize result. Verifies **AC-29**.

### STEP-P7 — Capability concentration
- **Tests:** Entropy of capability tags below threshold. Verifies **AC-30**.

### STEP-P8 — Coverage gate
- **Tests:** Tarpaulin ≥ 80% on the four required crates. Verifies **AC-19**.

### STEP-P9 — Clippy + deny pass
- **Tests:** Verifies **AC-20**, **AC-21**.

### STEP-P10 — Rustdoc + cargo doc warnings free
- **Tests:** Verifies **AC-22**.

### STEP-P11 — Language reference document
- **Output:** `docs/language-reference.md` with full BNF, type rules, totality rules, ≥20 examples.
- **Verifies:** **AC-23**.

### STEP-P12 — MCP tool docs + schemas published
- **Output:** `docs/mcp-tools.md`, all 9 schemas in `schemas/`. Verifies **AC-24**.

### STEP-P13 — Remaining ADRs
- **Output:** ADRs for LanceDB choice, no type inference, no partial functions, dedup matrix. Verifies **AC-25**.

### STEP-P14 — Release `v0.1.0`
- **Done when:** Every AC-1 … AC-30 green; tag pushed; release notes posted.

---

# MVP1 — CLI tool

Estimated effort: 2–3 weeks.

### STEP-Q1 — CLI scaffold (`alfdf` binary, clap-based)
- **Subcommands:** `submit`, `query`, `describe`, `explain`, `dependencies`, `synthesize`, `diff`, `stats`.

### STEP-Q2 — `alfdf submit <file.abal>`
- **Goal:** Parse file, call pipeline, print structured report (or `--explain` for human form).
- **Tests:** End-to-end on a sample file.

### STEP-Q3 — `alfdf query --type "(a -> b) -> List<a> -> List<b>" --laws Functor.Identity`
- **Tests:** Returns ranked JSON; `--human` flag pretty-prints.

### STEP-Q4 — `alfdf describe <entity_id>` and `alfdf explain <entity_id>`
- **Tests:** Match MCP outputs exactly.

### STEP-Q5 — `alfdf stats`
- **Goal:** Show DB size, dedup rate, top capabilities.

### STEP-Q6 — Shell completions and `--help` polish
- **Tests:** Completions install on bash/zsh/fish.

### STEP-Q7 — Tarball release
- **Goal:** `cargo-dist` or equivalent multi-platform release.

**MVP1 Done when:** All commands functional, smoke tests green, binary distributed.

---

# MVP2 — IDE plugin (LSP-based)

Estimated effort: 4–6 weeks.

### STEP-R1 — LSP scaffold (`alfdf-lsp` crate)
- **Library:** `tower-lsp`.

### STEP-R2 — Document open/edit forwarding to typechecker
- **Tests:** Syntax + type errors surface as diagnostics.

### STEP-R3 — Hover → `abal.describe` summary
- **Tests:** Hover on a name shows structured facts.

### STEP-R4 — Code action: "Find similar in ALFDF"
- **Goal:** Calls `query_by_type` on the type of the symbol under cursor.

### STEP-R5 — Code action: "Submit this function"
- **Goal:** Runs full pipeline on the current function, surfaces results.

### STEP-R6 — Inline complexity & law annotations (decorations)
- **Tests:** Visual confirmation in VS Code prototype.

### STEP-R7 — VS Code extension wrapper
- **Goal:** Marketplace-ready extension consuming `alfdf-lsp`.

### STEP-R8 — Neovim and Helix configs
- **Goal:** Documented setup snippets.

**MVP2 Done when:** Extension installable, all five code actions work, hover renders `describe`.

---

# Cross-cutting practices

### Testing discipline
- **Unit tests** in every step.
- **Property tests** (`proptest`) for round-trips and invariants.
- **Snapshot tests** (`insta`) for error messages and JSON outputs.
- **Golden files** for parser and pretty-printer.
- **Conformance tests** (Phase P) enforce I-1…I-12 globally.

### Benchmark discipline
- Every performance-critical step ships a `criterion` benchmark.
- Baselines committed under `benches/baselines/` with metadata about the machine class.
- Regression alert: if p99 worsens > 20% on the same hardware, PR is blocked.

### Refactor discipline
- Public APIs marked `#[non_exhaustive]` where forward-compatible.
- `StorageAdapter` trait is the only stable boundary for persistence (I-9).
- Schemas under `schemas/` versioned semver; breaking changes require a new file (`*-2.0.0.json`).

### Review discipline
- Each PR checklist:
  - [ ] Tests added/updated
  - [ ] Benchmarks added/updated (if perf-critical)
  - [ ] Invariants checked
  - [ ] Schema validated (if MCP layer)
  - [ ] Docs updated
  - [ ] ADR added if a design choice was made

---

# Suggested two-engineer schedule (16 weeks)

| Week | Engineer A | Engineer B |
|---|---|---|
| 1 | Phase A | Phase A (parallel) |
| 2 | Phase B | Phase C STEP-C1..C3 |
| 3 | Phase C STEP-C4..C7 | Phase D STEP-D1..D5 |
| 4 | Phase D STEP-D6..D9 | Phase E |
| 5 | Phase F | Phase F (benches) |
| 6 | Phase G STEP-G1..G3 | Phase G STEP-G4..G6 |
| 7 | Phase H STEP-H1..H6 | Phase I |
| 8 | Phase H STEP-H7..H9 | Phase J STEP-J1..J3 |
| 9 | Phase J STEP-J4..J7 | Phase K |
| 10 | Phase L STEP-L1..L3 | Phase L STEP-L4..L6 |
| 11 | Phase M STEP-M1..M6 | Phase M STEP-M7..M13 |
| 12 | Phase N | Phase O |
| 13 | Phase P STEP-P1..P3 | Phase P STEP-P4..P7 |
| 14 | Phase P STEP-P8..P11 | Phase P STEP-P12..P13 |
| 15 | Buffer / polish | Buffer / polish |
| 16 | STEP-P14 release | STEP-P14 release |

---

# Status board template

Maintain a `STATUS.md` at the repo root with this table, one row per step:

| Step | Status | PR | Owner | ACs touched | Notes |
|---|---|---|---|---|---|
| STEP-A1 | ☐ todo / ⏳ in-progress / ✅ done | #N | name | — | — |
| … | | | | | |

This is the single source of truth for progress and unblocks daily standups without ceremony.

---

*End of step-by-step guide, ALFDF v0.1.0, 2026-05-22.*
