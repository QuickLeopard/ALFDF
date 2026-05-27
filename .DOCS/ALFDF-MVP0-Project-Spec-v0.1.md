# ALFDF — Abstract LLM-friendly Functional Decomposition Framework
## MVP0 Project Specification

| Field | Value |
|-------|-------|
| **Document version** | 0.1.0 |
| **Document status** | Draft — locked for MVP0 implementation |
| **Date** | 2026-05-22 |
| **Spec lifecycle** | MVP0 (this doc) → MVP1 (CLI) → MVP2 (IDE/LSP) |
| **Implementation language** | Rust (stable toolchain, edition 2024+) |
| **Target language built by ALFDF** | ABAL — AST-Based AI-friendly Language |
| **Primary consumer** | LLM agents via MCP; humans via rendered views |

---

## 0. Glossary

| Term | Definition |
|------|------------|
| **ALFDF** | The framework/tool described by this document. |
| **ABAL** | AST-Based AI-friendly Language. A total, pure, strongly-typed functional language designed for LLM emission and machine reasoning. |
| **Entry / Entity** | A first-class DB record: function, data type, law, test, or property. |
| **Logical entity** | A named entity (e.g., `List.map`) whose implementation evolves across versions. |
| **Version** | A specific content-addressed AST for a logical entity. |
| **AST** | Canonical Abstract Syntax Tree, represented in JSON, content-addressed by BLAKE3 hash. |
| **Pipeline** | The fixed sequence of gates a submission passes before becoming a DB entry. |
| **Hard gate (⊗)** | A pipeline stage whose failure rejects the submission (subject to refactor retries). |
| **Soft gate (⊘)** | A pipeline stage that records data but does not reject. |
| **Capability tag** | A declarable property of a data type (`Eq`, `Functor`, `Monoid`, …). |
| **Law tag** | A declarable algebraic property of a function (`Associative`, `Functor.Identity`, …). |
| **Totality certificate** | A machine-checked witness that a function terminates on every input. |
| **MCP** | Model Context Protocol — the JSON-RPC surface exposed to LLM agents. |
| **Truth store** | The canonical, authoritative storage layer (SQLite). |
| **L1–L5** | The five layers of the dedup engine, ordered cheapest-first. |

---

## 1. Vision and Goals

ALFDF is a knowledge base + verification pipeline + query surface that lets an LLM build software by composing **typed, total, tested, deduplicated** units of an AST-based language (ABAL) instead of emitting unstructured source text. The DB stores ASTs, not text. Queries are type-driven, law-driven, and semantic. Every entry is guaranteed correct by construction.

### Success criteria for MVP0 (mapped to user-stated goals a–e)

| # | Goal | Concrete acceptance metric |
|---|------|-----------------------------|
| a | **Token savings** | LLM can reference any DB entry by `entity_id` (≤ 64 bytes) instead of re-emitting its body. The MCP server logs `tokens_referenced_vs_emitted` per session. Acceptance: on the bundled benchmark task set, agents using ALFDF emit ≥ 30% fewer tokens than a baseline agent solving the same tasks without ALFDF. |
| b | **Reduced duplication** | DB invariant: no two stored entries are L2-, L3-, or L4-equivalent (definitions §6). Acceptance: a 10 000-submission fuzz benchmark with intentional duplicates yields a dedup recall ≥ 0.98 and precision ≥ 0.99 on a labeled test set. |
| c | **Higher correctness** | Every DB entry carries a `totality_certificate = Verified`, ≥ 1 passing test, and all declared laws fuzz-verified. Acceptance: invariant holds on every read; periodic re-verification job confirms it nightly. |
| d | **Composability discovery** | `abal.synthesize` returns ≥ 1 valid composition for ≥ 80% of "no exact match" queries where a composition of depth ≤ 3 exists in the DB (measured on the bundled synthesis benchmark). |
| e | **Architectural consistency** | A fixed capability and law registry, plus a fixed set of primitive types and stdlib seed, are the only construction primitives. Acceptance: 100% of DB entries reference only the registered tags and seeded primitives (lint job, hard fail). |

### Non-goals for MVP0

- Effect tracking (`IO`, `State`, `Async`) — deferred to MVP1+.
- General (non-structural) recursion — deferred.
- FFI / native calls — deferred.
- Formal proof backends (SMT, refinement types) — property fuzzing only.
- Multi-tenant hosted service — single-user, per-project DB only.
- IDE integration — MVP2.
- CLI tool — MVP1.

---

## 2. Top-Level Architecture

```
┌──────────────────────────────────────────────────────────────────────┐
│                       LLM Agent (external)                            │
└────────────────────────────┬─────────────────────────────────────────┘
                             │  MCP / JSON-RPC over stdio
┌────────────────────────────▼─────────────────────────────────────────┐
│                       ALFDF MCP Server                                │
│  ┌────────────────────────────────────────────────────────────────┐  │
│  │  Tool Surface (9 tools, §7)                                    │  │
│  └────────────────────────────────────────────────────────────────┘  │
│  ┌────────────────────────────────────────────────────────────────┐  │
│  │  Submission Pipeline (§5)                                      │  │
│  │  Parse → Type → Total → Determinism → Dedup → Tests → Laws →   │  │
│  │  Benchmark → Index                                             │  │
│  └────────────────────────────────────────────────────────────────┘  │
│  ┌──────────────┬──────────────┬──────────────┬─────────────────┐    │
│  │ ABAL Parser/ │ Type Checker │ Totality     │ Tree-walking VM │    │
│  │ Pretty-Print │              │ Checker      │ + Fuel + Fuzzer │    │
│  └──────────────┴──────────────┴──────────────┴─────────────────┘    │
│  ┌────────────────────────────────────────────────────────────────┐  │
│  │  Dedup Engine (L1–L5, §6)                                      │  │
│  └────────────────────────────────────────────────────────────────┘  │
│  ┌────────────────────────────────────────────────────────────────┐  │
│  │  Storage Adapter (trait, §8)                                   │  │
│  │   TruthStore  │ TypeIndex │ GraphIndex │ VectorIndex            │  │
│  └─────┬──────────────┬────────────┬────────────┬─────────────────┘  │
└────────┼──────────────┼────────────┼────────────┼────────────────────┘
         ▼              ▼            ▼            ▼
      SQLite        SQLite       SQLite        LanceDB
      (WAL)         (trie blob)  (edges)       (embeddings)
```

### Crate layout (Cargo workspace)

```
alfdf/
├── Cargo.toml                  # workspace
├── crates/
│   ├── abal-ast/               # AST types, JSON schema, content hashing
│   ├── abal-parser/            # text ↔ AST (bijective)
│   ├── abal-typeck/            # bidirectional type checker
│   ├── abal-totality/          # structural recursion + exhaustiveness
│   ├── abal-vm/                # tree-walking interpreter + fuel
│   ├── abal-fuzzer/            # property-based tester, law-template registry
│   ├── alfdf-dedup/            # L1–L5 layered dedup engine
│   ├── alfdf-storage/          # StorageAdapter trait + concrete backends
│   ├── alfdf-pipeline/         # orchestrates submission pipeline
│   ├── alfdf-mcp/              # MCP server (JSON-RPC over stdio)
│   ├── alfdf-bench/            # benchmark harness
│   └── alfdf-stdlib/           # seed stdlib in ABAL source
└── docs/
```

---

## 3. ABAL Language Specification

### 3.1 Design invariants

- **I-LANG-1 — Total**: every function terminates on every well-typed input.
- **I-LANG-2 — Pure**: no side effects, no I/O, no FFI in MVP0.
- **I-LANG-3 — Explicit**: no type inference; every binding, parameter, return type, and field type is annotated.
- **I-LANG-4 — Exhaustive**: every `match` covers every constructor.
- **I-LANG-5 — Bijective forms**: parse(pretty_print(ast)) = ast; pretty_print(parse(text)) = canonical(text).
- **I-LANG-6 — Stable hashing**: two ASTs are content-equal iff their canonical JSON serializations are byte-equal.

### 3.2 Primitive types (MVP0)

`Int` (64-bit signed), `Bool`, `String` (UTF-8), `List<a>`, `Tuple<a, b, …>` (arity 2–8), `Option<a>`, `Result<e, a>`.

### 3.3 Grammar (EBNF)

```
Module       ::= (DataDecl | FnDecl | LawDecl | TestDecl | PropDecl)*

DataDecl     ::= "data" Name TypeParams? "=" Constructor ("|" Constructor)*
                 ("supports" "{" CapabilityTag ("," CapabilityTag)* "}")?
Constructor  ::= Name ("(" Field ("," Field)* ")")?
Field        ::= Name ":" Type

FnDecl       ::= "fn" Name TypeParams? "(" Param ("," Param)* ")" ":" Type
                 ("complexity" "{" "time" ":" BigO "," "space" ":" BigO "}")
                 ("laws" "{" LawRef ("," LawRef)* "}")?
                 "tests" "{" TestRef ("," TestRef)* "}"
                 "=" Expr
Param        ::= Name ":" Type

LawDecl      ::= "law" Name TypeParams? "(" Param ("," Param)* ")" ":" "Bool" "=" Expr
PropDecl     ::= "prop" Name TypeParams? "(" Param ("," Param)* ")" ":" "Bool" "=" Expr
TestDecl     ::= "test" Name "=" Expr                  -- Expr must evaluate to Bool == true

Type         ::= TypeVar
               | TypeCon ("<" Type ("," Type)* ">")?
               | "(" Type ("," Type)+ ")"              -- tuple
               | "(" Type "->" Type ")"
TypeParams   ::= "<" TypeVar ("," TypeVar)* ">"

Expr         ::= Literal | Var | App | Lambda | Let | Match | Constructor
Lambda       ::= "\\" "(" Param ("," Param)* ")" "->" Type "=>" Expr
Let          ::= "let" Name ":" Type "=" Expr "in" Expr
Match        ::= "match" Expr "{" (Pattern "=>" Expr)+ "}"
App          ::= Expr "(" Expr ("," Expr)* ")"

BigO         ::= "O(1)" | "O(log n)" | "O(n)" | "O(n log n)" | "O(n^2)" | "O(n^3)" | "O(2^n)"
CapabilityTag::= "Eq" | "Ord" | "Show" | "Functor" | "Foldable" | "Monoid" | ...   (closed registry §4.1)
LawRef       ::= NamedLawTag | LawName                 -- tag from registry or custom law
```

### 3.4 Type system

- Hindley-Milner skeleton **without inference**: every position annotated; checker is purely a verifier.
- Parametric polymorphism (rank-1).
- No type classes (capability tags are metadata, not constraints) in MVP0.
- No higher-kinded types.

**Acceptance:** The type checker accepts a fixed corpus of 200 hand-written valid ABAL modules and rejects a fixed corpus of 200 invalid ones, each with a specific structured error code.

### 3.5 Totality rules

A function is total if **all** hold:
1. Every `match` is exhaustive over the scrutinee's type.
2. Every recursive call decreases a structurally smaller subterm of an inductive argument (the **decreasing argument** is declared in the totality certificate).
3. No call to a non-total function (vacuous in MVP0 — none exist).

**Acceptance:** Totality checker accepts the seed stdlib (~30–50 functions) and rejects a fixed corpus of 50 obviously non-terminating examples.

### 3.6 What ABAL forbids (MVP0)

No `error`/`panic`/`undefined`, no exceptions, no division by zero (use `div : Int -> Int -> Option<Int>`), no `head : List<a> -> a` (use `Option`), no global mutable state, no I/O.

---

## 4. Capability & Law Registry

### 4.1 Capability tags (closed set for MVP0)

`Eq`, `Ord`, `Show`, `Functor`, `Foldable`, `Monoid`, `Semigroup`, `Traversable`. A capability declaration on a data type is a promise: the type must have witnesses (functions registered as instances) for the corresponding operations, or the declaration is rejected.

### 4.2 Law tags (closed set for MVP0)

`Reflexive`, `Symmetric`, `Transitive`, `Antisymmetric`, `Associative`, `Commutative`, `Identity.Left`, `Identity.Right`, `Idempotent`, `Distributive.Left`, `Distributive.Right`, `Involutive`, `Functor.Identity`, `Functor.Composition`, `Monoid.Identity`, `Monoid.Associativity`, `Homomorphism`.

Each tag has a **law template** parameterized by holes filled by the candidate function (and, where needed, by `Eq` for the result type).

### 4.3 Custom properties

A `prop`-prefixed total function returning `Bool` is fuzzed by the VM. Declared in `laws { … }` block as `custom(prop_name)`.

**Acceptance:** The registry is exposed via an `abal.registry` MCP tool that returns the full set with template signatures; tests verify the templates compile against representative candidates.

---

## 5. Submission Pipeline

The pipeline is a fixed sequence. Each stage is either a **hard gate (⊗)** or **soft gate (⊘)**.

| # | Stage | Gate | Action on fail |
|---|-------|------|----------------|
| 1 | Parse + AST schema validation | ⊗ | Refactor request |
| 2 | Bidirectional type check | ⊗ | Refactor request |
| 3 | Totality / exhaustiveness check | ⊗ | Refactor request |
| 4 | Determinism whitelist check | ⊗ | Refactor request |
| 5 | Dedup engine (§6) | ⊗ | Reject / alias / alternative per §6.4 |
| 6 | Tests exist (≥ 1) and pass | ⊗ | Refactor request |
| 7 | Law fuzzing (1000 cases default) | ⊗ | Refactor request with counterexample |
| 8 | Benchmark vs alternatives | ⊘ | Record metrics only |
| 9 | Index into DB | ⊗ | Atomic; rollback on partial failure |

### 5.1 Refactor loop

- Default `N = 3` refactor attempts per submission.
- On each failure, the MCP server returns a structured rejection report (schema §7.10) including the failed stage, error code, counterexample (if any), and the original AST.
- On final failure: write a record to the `unsuccessful_attempts` log with TTL = 30 days. The LLM or a human may tag an entry as `interesting` to exempt it from rolling deletion.

### 5.2 Tunable parameters

| Parameter | Default | Range |
|-----------|---------|-------|
| `refactor_retries_N` | 3 | 1–10 |
| `fuzz_cases` | 1000 | 100–100 000 |
| `vm_fuel` | 10⁷ reductions | 10⁵–10⁹ |
| `rolling_log_ttl_days` | 30 | 1–365 |
| `dedup_l3_corpus_size` | 1000 | 100–10 000 |

**Acceptance:** Pipeline integration test runs 200 prepared submissions covering every gate's pass and fail paths; all error codes are emitted with the documented schema.

---

## 6. Dedup Engine

### 6.1 Layers (ordered cheapest-first, short-circuit on high confidence)

| Layer | Method | Cost | Verdict on match |
|-------|--------|------|------------------|
| **L1** | Type-signature pre-filter (normalized type hash; isomorphic-or-subtype match) | O(1) | Gate, not a match |
| **L2** | β-η-normalized AST hash (BLAKE3 over canonical AST after β-reduction and η-conversion) | cheap | HIGH → duplicate |
| **L3** | Shared fuzz-corpus output equality (1000 inputs per type, deterministic seed) | medium | HIGH → behaviorally equivalent |
| **L4** | Cross-test execution (run candidate against existing entry's tests, and vice versa; both pass ⇒ equivalent) | medium | HIGH → behaviorally equivalent |
| **L5** | Embedding similarity over (AST-structure ⊕ name ⊕ doc) | medium | SCORING only — flagged for review |

### 6.2 Confidence thresholds

- L2 match ⇒ confidence 1.0 (exact equivalence up to α/β/η).
- L3 match ⇒ confidence 0.99 (probabilistic).
- L4 match ⇒ confidence 0.95.
- L5 similarity ⇒ scoring, never an automatic verdict.

Any layer at confidence ≥ 0.95 triggers the decision matrix.

### 6.3 Better-variant detection

When a duplicate is found, compare candidate vs existing on:

| Metric | Comparator |
|--------|------------|
| Complexity class (declared) | smaller class is better |
| Benchmark p50 (measured) | lower is better |
| AST node count | lower is better (size proxy) |
| Test coverage | higher is better |

A candidate is **strictly better** if it dominates on ≥ 1 metric and ties on the rest.

### 6.4 Decision matrix

| Candidate vs existing | Action |
|-----------------------|--------|
| Equivalent and equal-or-worse on all metrics | **REJECT**; return reference to existing entity |
| Equivalent and strictly better | **MERGE AS ALIAS**; record `improvement = { reason, deltas }`; enqueue in `replacement_review` queue |
| Equivalent and mixed trade-offs | **STORE AS ALTERNATIVE**; link via `equivalent_to` edge; tag `needs_human_review = true` |

**Acceptance:**
- On the labeled dedup benchmark (10 000 submissions, ~30% intentional duplicates): recall ≥ 0.98, precision ≥ 0.99.
- Better-variant detection recall ≥ 0.95 on a curated set of 200 (existing, improved) pairs.
- Decision matrix produces deterministic verdicts (same input → same output) verified by replay test.

---

## 7. MCP Tool Surface

All tools use JSON-RPC 2.0 over stdio. Every response carries:
```json
{ "schema": "alfdf/<tool>/<semver>", "data": { ... } }
```

### 7.0 Binding invariants

- **I-MCP-1**: No tool consumed by an LLM returns free-form prose. Only `abal.explain` returns Markdown, and it is for humans.
- **I-MCP-2**: `abal.explain(id, opts) ≡ render_markdown(abal.describe(id, opts))`. `explain` reads no source except `describe`'s output.
- **I-MCP-3**: Every response includes a `schema` field. Schema versions are SemVer; breaking changes bump major.
- **I-MCP-4**: All `include`/`depth`/filter options accepted by `describe` are accepted by `explain` with identical semantics.

### 7.1 Tool surface (9 tools)

| # | Tool | Consumer | Output |
|---|------|----------|--------|
| 1 | `abal.query_by_type`   | LLM, tools | Structured JSON |
| 2 | `abal.query_by_data`   | LLM, tools | Structured JSON |
| 3 | `abal.synthesize`      | LLM, tools | Structured JSON |
| 4 | `abal.submit_fn`       | LLM, tools | Pipeline report (JSON) |
| 5 | `abal.submit_data`     | LLM, tools | Pipeline report (JSON) |
| 6 | `abal.diff_versions`   | LLM, tools | Structured JSON |
| 7 | `abal.describe`        | LLM, tools | Structured JSON |
| 8 | `abal.explain`         | Humans     | Markdown |
| 9 | `abal.dependencies`    | LLM, tools | Structured JSON |

Plus one introspection endpoint:
| 10 | `abal.registry`       | LLM, tools | Structured JSON — capabilities + laws |

### 7.2 `abal.query_by_type`

**Request:**
```json
{
  "signature": "(a -> b) -> List<a> -> List<b>",
  "laws_required":   ["Functor.Identity", "Functor.Composition"],
  "laws_forbidden":  [],
  "complexity":      { "time_max": "O(n)", "space_max": "O(n)" },
  "capabilities_in_scope": ["Eq", "Ord"],
  "free_text":       "map over list preserving order",
  "k": 10
}
```

**Response:** ranked array of `{ entity_id, signature, score, score_breakdown, ... }`.

**Ranking function:**
```
score = w1·type_distance + w2·law_match_ratio + w3·complexity_fit
      + w4·embedding_similarity + w5·log(1+usage_count)
      + w6·test_coverage + w7·benchmark_percentile - w8·age_penalty
```
Default weights documented in code; tunable via config.

**Fallback:** if top hit `type_distance > τ` (default 0.4), the response includes a `synthesis_candidates` array from `abal.synthesize`.

### 7.3 `abal.describe`

Returns the full structured record (schema in §7.11).

**Request:**
```json
{
  "entity_id": "fn:list.map@v3",
  "include": {
    "ast": true,
    "tests": false,
    "dependencies_transitive": false,
    "embedding_neighbors": 5,
    "version_history": false
  },
  "depth": 1
}
```

### 7.4 `abal.explain`

Same request shape as `describe`. Returns Markdown rendered purely from `describe`'s output.

### 7.5 `abal.synthesize`

**Request:** target type + optional law/complexity constraints + max composition depth (default 3).

**Response:** array of candidate compositions, each a constructed AST that type-checks against the target, with provenance (which DB entities were used).

### 7.6 `abal.submit_fn` / `abal.submit_data`

**Request:** AST (JSON) + optional metadata.

**Response:** pipeline report — each stage's verdict, structured error codes on failure, final `entity_id` on success.

### 7.7 `abal.diff_versions`

**Request:** logical entity name (e.g., `list.map`).
**Response:** ordered version list with AST diffs (structured tree-diff, not text diff) and metric deltas.

### 7.8 `abal.dependencies`

**Request:** `entity_id`, `direction ∈ {"depends_on", "used_by"}`, `depth`.
**Response:** subgraph with nodes and edges.

### 7.9 `abal.registry`

Returns the closed capability and law registries with template signatures.

### 7.10 Pipeline rejection report schema

```json
{
  "schema": "alfdf/submission_report/0.1.0",
  "data": {
    "accepted": false,
    "failed_stage": "totality",
    "error_code": "TOT_E007_NON_DECREASING_RECURSION",
    "message_structured": {
      "fn_name": "loop",
      "offending_call_path": ["loop", "loop"],
      "non_decreasing_arg": "n"
    },
    "counterexample": null,
    "refactor_attempts_remaining": 2,
    "original_ast_hash": "blake3:..."
  }
}
```

### 7.11 `describe` schema (canonical record)

Full schema as locked in earlier in the design conversation, exposed as `alfdf/describe/0.1.0`. Top-level fields:
`entity_id`, `content_hash`, `kind`, `version`, `signature`, `totality`, `laws`, `complexity`, `tests`, `dependencies`, `equivalence`, `ast`, `capabilities_used`, `purity`, `determinism`, `usage`, `provenance`.

**Acceptance:** Every tool has a JSON Schema file checked into `crates/alfdf-mcp/schemas/`. A schema-conformance test runs every tool on a corpus and validates responses against schemas. Coverage 100%.

---

## 8. Storage Layer

### 8.1 Storage adapter invariant

> **I-STORE-1**: All persistent-state access goes through the `StorageAdapter` interface exposing four sub-stores: `TruthStore`, `TypeIndex`, `GraphIndex`, `VectorIndex`. No pipeline stage, no MCP tool, and no checker imports a concrete storage driver directly. Backend swaps must be possible without touching any code outside `crates/alfdf-storage/`.

### 8.2 Backends (MVP0)

| Sub-store | Backend | Rationale |
|-----------|---------|-----------|
| TruthStore | **SQLite (WAL mode)** | Transactional, embedded, scales to hundreds of GB. |
| TypeIndex | In-process trie, persisted as a SQLite blob | Custom algorithm regardless of storage. |
| GraphIndex | SQLite tables (`edges`) with recursive CTEs | Sufficient for MVP0 closure depths. |
| VectorIndex | **LanceDB** (embedded) | Pre-emptive choice; scales 100M+ vectors; most painful to migrate later. |

### 8.3 Schema sketch (Truth store)

```sql
CREATE TABLE entities (
  entity_id     TEXT PRIMARY KEY,        -- "fn:list.map@v3"
  logical_name  TEXT NOT NULL,           -- "list.map"
  kind          TEXT NOT NULL,           -- function|data|law|test|prop
  version       INTEGER NOT NULL,
  content_hash  TEXT NOT NULL UNIQUE,    -- "blake3:..."
  ast_json      BLOB NOT NULL,
  signature_json BLOB NOT NULL,
  type_hash     TEXT NOT NULL,           -- normalized type hash (L1)
  beta_eta_hash TEXT NOT NULL,           -- (L2)
  status        TEXT NOT NULL,           -- draft|stable|deprecated|alias
  totality_cert TEXT NOT NULL,           -- verified|unverified
  laws_json     BLOB,
  complexity_json BLOB,
  tests_json    BLOB,
  metrics_json  BLOB,
  provenance_json BLOB,
  created_at    INTEGER NOT NULL,
  CHECK (totality_cert = 'verified')     -- DB-wide invariant
);

CREATE INDEX idx_entities_logical ON entities(logical_name, version);
CREATE INDEX idx_entities_type_hash ON entities(type_hash);
CREATE INDEX idx_entities_beta_eta ON entities(beta_eta_hash);

CREATE TABLE edges (
  from_id  TEXT NOT NULL,
  to_id    TEXT NOT NULL,
  kind     TEXT NOT NULL,    -- depends_on|equivalent_to|alias_of|replaces|superseded_by|instance_of_law
  meta_json BLOB,
  PRIMARY KEY (from_id, to_id, kind)
);

CREATE TABLE unsuccessful_attempts (
  id            INTEGER PRIMARY KEY,
  submitted_at  INTEGER NOT NULL,
  expires_at    INTEGER NOT NULL,
  ast_json      BLOB NOT NULL,
  failure_log   BLOB NOT NULL,
  interesting   BOOLEAN NOT NULL DEFAULT 0
);
```

### 8.4 Versioning

- AST content-addressed via BLAKE3 over canonical JSON.
- Logical entity → version chain. `v1 → v2 → v3`, edges `superseded_by`.
- Signature change ⇒ new logical entity, old marked `superseded_by` the new with metadata explaining the break.
- Default queries return `latest && status = stable`; pinning supported via `@vN`.

### 8.5 Acceptance

- Storage trait has ≥ 95% line coverage in unit tests.
- All four sub-stores have an in-memory test implementation for fast tests.
- Backend swap test: replace SQLite truth store with the in-memory test impl; full integration suite passes unchanged.
- DB invariant enforcer: every write goes through a constraint that rejects any entity with `totality_cert != 'verified'` or `tests_json` empty.

---

## 9. VM Specification

- Tree-walking interpreter over canonical AST (no compilation).
- Fuel-bounded: every reduction decrements fuel; running out is a structural error (should never happen for total functions; serves as last-resort safety net).
- Deterministic execution: closed whitelist of primitive operations; no clock, RNG (except via PRNG seeded by the fuzzer), file/network I/O, or environment reads.
- Fuzzer: PRNG with explicit seed, shrinking on counterexamples.

**Acceptance:** VM passes a conformance suite of 500 programs with declared expected outputs; fuzzer produces minimized counterexamples on a labeled set of buggy candidates.

---

## 10. Bootstrapping stdlib

A seed set of 30–50 ABAL entries (~split: 60% functions, 30% data types, 10% laws). Provides initial vocabulary for the LLM and seed data for embeddings.

Coverage targets:
- `List`: `map`, `filter`, `fold_left`, `fold_right`, `length`, `reverse`, `concat`, `zip`, `unzip`, `take`, `drop`, `at` (returns `Option`).
- `Option`: `map`, `and_then`, `or_else`, `get_or`, `is_some`, `is_none`.
- `Result`: `map`, `map_err`, `and_then`, `or_else`, `is_ok`, `is_err`.
- `Tuple`: `fst`, `snd`, `swap`, `map_fst`, `map_snd`.
- `Int`: `add`, `sub`, `mul`, `div` (`Option<Int>`), `mod` (`Option<Int>`), `eq`, `lt`, `lte`, `abs`, `neg`.
- `Bool`: `and`, `or`, `not`, `xor`.
- `String`: `length`, `concat`, `parse_int` (`Result<ParseError, Int>`).
- Data: `List<a>`, `Option<a>`, `Result<e, a>`, `Tuple<a, b>`, `BinaryTree<a>` (with `Functor`, `Foldable` capabilities).

**Acceptance:** All seeds pass the pipeline cleanly on a fresh DB; the seed run is reproducible from a single command (`cargo run --bin alfdf-stdlib-seed`).

---

## 11. Benchmarks (acceptance harness)

A reproducible benchmark suite lives in `crates/alfdf-bench/`. It contains:

1. **Token-savings benchmark** — 50 small coding tasks; measure tokens emitted with and without ALFDF. Target: ≥ 30% reduction.
2. **Dedup benchmark** — 10 000 labeled submissions. Target: recall ≥ 0.98, precision ≥ 0.99.
3. **Synthesis benchmark** — 200 "no-exact-match" queries with known depth-≤-3 compositions. Target: ≥ 80% recovered.
4. **Pipeline conformance** — 200 prepared submissions (positive and negative) exercising every gate.
5. **VM conformance** — 500 programs with expected outputs.
6. **Schema conformance** — every MCP response validates against its schema; 100%.
7. **Storage swap** — full integration suite runs identically against SQLite-backed and in-memory `StorageAdapter`s.

Pass criterion for MVP0 release: all seven benchmarks meet their targets in CI on the reference machine spec.

---

## 12. Global Project Invariants

| ID | Invariant |
|----|-----------|
| I-LANG-1..6 | ABAL: total, pure, explicit, exhaustive, bijective forms, stable hashing (§3.1). |
| I-DB-1 | Every DB entry has `totality_cert = verified`, ≥ 1 passing test, all declared laws fuzz-passing. |
| I-DB-2 | No two DB entries are L2-, L3-, or L4-equivalent. |
| I-DB-3 | AST nodes are content-addressed; identical ASTs share storage. |
| I-MCP-1..4 | MCP tool contract (§7.0). |
| I-STORE-1 | All persistence behind `StorageAdapter` (§8.1). |
| I-OUTPUT-1 | No tool consumed by an LLM returns prose. |
| I-VERSION-1 | Signature change creates a new logical entity, never mutates an existing one. |

A nightly job re-checks I-DB-1 on every entry; failure pages the maintainer.

---

## 13. Configuration

Single TOML config file at `~/.alfdf/config.toml` (or `$ALFDF_CONFIG`). Schema:

```toml
[pipeline]
refactor_retries_N    = 3
fuzz_cases            = 1000
vm_fuel               = 10000000

[dedup]
l3_corpus_size        = 1000
l5_min_similarity     = 0.85

[storage]
truth_db_path         = "~/.alfdf/db/truth.sqlite"
vector_db_path        = "~/.alfdf/db/vectors.lance"

[logs]
rolling_ttl_days      = 30

[ranking.weights]
type_distance         = 1.0
law_match_ratio       = 0.6
complexity_fit        = 0.4
embedding_similarity  = 0.5
usage                 = 0.2
test_coverage         = 0.3
benchmark_percentile  = 0.3
age_penalty           = 0.05
```

---

## 14. Out-of-scope (explicit)

- Effects, async, I/O
- General recursion
- HKT, type classes, dependent types
- Multi-user / hosted deployment
- IDE, LSP, CLI (MVP1/2)
- Formal proofs (SMT, refinement types)
- Languages other than ABAL

---

## 15. Deliverables Checklist (MVP0)

| # | Deliverable | Acceptance reference |
|---|-------------|----------------------|
| D1 | ABAL spec doc (this document §3) + JSON AST schema file | §3.4, §3.5 acceptance |
| D2 | `abal-parser` crate (bijective text↔AST) | Round-trip test on 500 samples |
| D3 | `abal-typeck` crate | §3.4 acceptance |
| D4 | `abal-totality` crate | §3.5 acceptance |
| D5 | `abal-vm` crate (tree-walking + fuel) | §9 acceptance |
| D6 | `abal-fuzzer` crate + law-template registry | §4 acceptance |
| D7 | `alfdf-dedup` crate (L1–L5) | §6.5 acceptance |
| D8 | `alfdf-storage` crate (trait + SQLite + LanceDB + in-memory) | §8.5 acceptance |
| D9 | `alfdf-pipeline` crate orchestrating §5 | §5 acceptance |
| D10 | `alfdf-mcp` crate (9 + 1 tools, JSON Schemas) | §7 acceptance |
| D11 | `alfdf-stdlib` seed (~30–50 entries) | §10 acceptance |
| D12 | `alfdf-bench` harness | §11 — all seven benchmarks pass on CI |
| D13 | Operator runbook (Markdown): install, seed, configure, backup, restore | Manual review |
| D14 | Reference MCP integration example with at least one LLM agent (e.g., a Claude/Aider script) | Smoke test executes end-to-end |

---

## 16. MVP0 Release Criteria

ALFDF MVP0 is **released** when, on the reference CI environment, **all** of the following hold simultaneously:

1. All 14 deliverables (D1–D14) are present and reviewed.
2. All seven benchmarks (§11) meet their targets.
3. All eight categories of global invariants (§12) hold and are enforced by automated checks.
4. The reference LLM integration (D14) completes the bundled "build a small library" scenario end-to-end without manual intervention.
5. A clean install on a blank machine, following the runbook, reaches "seed completed" within 10 minutes on the reference hardware (8-core, 16 GB RAM, NVMe SSD).
6. Documentation is complete: this spec, JSON Schemas, runbook, and a short README per crate.

Once these are met, the version tag is `alfdf-mvp0/0.1.0` and the project advances to MVP1 (CLI) planning.

---

## 17. Open items deferred to MVP1 planning

- CLI surface and command set (`alfdf` binary).
- Multi-project workspace support.
- Effect-tag system design (`IO`, `State`, `Async`, `Partial`).
- Plan for migrating GraphIndex to a dedicated graph engine if traversal latency degrades.
- Embedding-model choice review (MVP0 default: a small local model over canonical S-expression form of AST).

---

*End of document. Spec is locked for MVP0 implementation as of 2026-05-22, version 0.1.0.*
