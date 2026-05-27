 Code-level Mini-Spec — Step 0.2.7 "Structural Recursion Checker"

This Step encodes **invariant I-1 (Totality)** statically. It is the foundation of every correctness guarantee downstream: the VM can trust termination, the fuzzer can trust convergence, the dedup engine's L3 layer can trust that "ran successfully" means "ran to completion." Get this wrong and the whole pipeline silently admits non-total functions.

The checker is also the **hardest Step to retrofit later**, because relaxing it would require re-validating every existing DB entry. We pin its semantics now.

---

## 0. Scope

Build `crates/alfdf-totality/` — given a typed `FnDecl` (or a mutually-recursive group of them), produce either a `TotalityCert` or a structured `TotalityError`.

Assumes upstream Steps already passed:
- **Step 0.2.6 (Exhaustiveness):** every `match` is exhaustive → no implicit partiality through missing patterns.
- **Step 0.2.x (Typecheck):** all references resolve, all types align.

**Not in this Step:** semantic / SMT proof, sized types, well-founded measures beyond structural subterms. Those are MVP2+ extensions.

---

## 1. What "Structural Recursion" Means in ABAL (locked definition)

A function `f` is **structurally recursive** if there exists at least one parameter — called the **decreasing argument** — such that on every recursive call to `f` (direct or indirect through a mutually-recursive group), the value passed at that position is a **strict structural subterm** of the value bound to that parameter at the call site.

"Strict structural subterm" of a pattern-bound variable `v` is defined inductively:

```
subterm(v) =  the set of variables bound in patterns that destructure v,
              plus their subterms transitively.
```

Examples:
- `match xs { Cons(h, t) => ... }` — both `h` and `t` are strict subterms of `xs`.
- `match t { Node(l, x, r) => ... }` — `l`, `x`, `r` are all strict subterms of `t`.
- `let y = xs in ...` — `y` is **not** a subterm of `xs` (binding equality, not destructuring).
- A constructor applied to a subterm is **not** a subterm (`Cons(h, t)` is not a subterm of `xs`, even if `h` and `t` are).

This rule is conservative — it rejects some total functions (e.g., the Ackermann-style total functions, McCarthy's 91 function). MVP0 accepts this trade for simplicity. A function rejected here can be reformulated with explicit accumulator + helper fns; the seed stdlib confirms this is workable for everything in scope.

---

## 2. Public Surface (`alfdf-totality/src/lib.rs`)

```rust
use alfdf_ast::{FnDecl, Expr, Ident, Pattern, ContentHash};
use serde::{Serialize, Deserialize};

/// Result of checking a single function or a mutually-recursive group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalityCert {
    pub schema:         &'static str,           // "alfdf/totality-cert/1.0.0"
    pub method:         CertMethod,
    pub fn_certs:       Vec<FnCert>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum CertMethod {
    StructuralRecursion,
    NonRecursive,                               // base case, trivially total
    MutualStructuralRecursion {
        group: Vec<Ident>,                      // names in the SCC
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FnCert {
    pub fn_name:                Ident,
    pub decreasing_arg_index:   Option<usize>,  // None for non-recursive
    pub decreasing_arg_name:    Option<Ident>,
    pub call_site_witnesses:    Vec<CallWitness>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallWitness {
    pub call_site_span:         Span,
    pub callee:                 Ident,
    pub passed_expr_summary:    String,         // e.g. "t (subterm of xs via Cons(h,t))"
    pub subterm_chain:          Vec<Ident>,     // [t, xs] — innermost to outermost
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalityError {
    pub schema:         &'static str,           // "alfdf/totality-error/1.0.0"
    pub kind:           TotalityErrorKind,
    pub offending_call: Option<Span>,
    pub fn_name:        Ident,
    pub hint:           Option<String>,         // structured hint, see §9
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "code")]
pub enum TotalityErrorKind {
    NoDecreasingArgument,
    NonStructuralRecursion {
        call_site:        Span,
        argument_passed:  String,
        reason:           NonStructuralReason,
    },
    MutualGroupUnranked {
        group:            Vec<Ident>,
        first_offender:   Span,
    },
    SelfApplication,                            // f f, f (g f), etc.
    HigherOrderCallback {
        callee:           Ident,
        cb_param:         Ident,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NonStructuralReason {
    NotASubterm,
    ReconstructedTerm,           // Cons(h, t) passed where subterm expected
    BindingNotDestructuring,     // let y = xs in f(y)
    UnknownProvenance,           // came from outside pattern matching
}

pub fn verify_module(
    module: &Module,
) -> Result<Vec<TotalityCert>, Vec<TotalityError>>;

pub fn verify_fn(fn_decl: &FnDecl) -> Result<TotalityCert, TotalityError>;
```

---

## 3. Algorithm (deterministic, four passes)

### Pass 1 — Build the call graph
- For the module, walk every `FnDecl` body and collect `(caller, callee, span)` triples.
- Restrict to intra-module edges (calls to stdlib / DB entries are leaves — already certified total by I-1).
- Compute strongly-connected components (Tarjan, O(V+E)).

### Pass 2 — Classify SCCs
For each SCC:
- **Size 1, no self-edge** → `CertMethod::NonRecursive`. Done.
- **Size 1, self-edge** → single-function structural-recursion check (§4).
- **Size > 1** → mutual structural-recursion check (§5).

### Pass 3 — Subterm provenance map per function
Build, for every variable in scope in the function body, a `Provenance`:

```rust
enum Provenance {
    Parameter(Ident, /* depth */ 0),
    Subterm { root_param: Ident, depth: u32, via_pattern: PatternId },
    Constructed,        // e.g. result of Cons(h, t) or f(...)
    LetBound(Ident),    // let y = e — provenance(y) := propagate(e), see below
    Opaque,             // anything else (lambda params, etc.)
}
```

Propagation rules:
- Parameters of the function: `Parameter(name, 0)`.
- Inside `match v { Cons(h, t) => body }`: `h` and `t` get `Subterm{ root_param: root_of(v), depth: depth_of(v) + 1, .. }` if `v` itself has parameter or subterm provenance; else `Opaque`.
- `let y = e in body`: `y` gets the propagated provenance of `e` (so `let y = t in f(y)` is fine when `t` was a subterm).
- Constructor applications, function applications, lambdas, literals → `Constructed`.

Provenance is **type-agnostic** in MVP0 — it tracks structural origin only.

### Pass 4 — Verify each recursive call
For each call to `f` (or any member of the same SCC) inside `f`'s body, locate the argument at the candidate decreasing index and check its `Provenance`:

```
ok iff provenance == Subterm { root_param == decreasing_arg, depth >= 1 }
```

For **single-function** SCCs, search candidate decreasing indices in this order:
1. The index annotated by the user via `@decreasing(arg_name)` (optional ABAL annotation, future-compatible).
2. Each parameter index in declaration order.

Accept the first index for which **every** recursive call passes the check. If none works → `NoDecreasingArgument` (with the most-promising index's failing call as the offender, for a useful error message).

---

## 4. Single-Function Check (worked example)

```
fn map<a, b>(f: (a -> b), xs: List<a>): List<b>
  ...
  = match xs {
      Nil        => Nil,
      Cons(h, t) => Cons(f(h), map(f, t))
    }
```

- Parameters: `f` at index 0, `xs` at index 1.
- Candidate index 0 (`f`): recursive call passes `f` itself at position 0 — `Provenance::Parameter("f", 0)`, depth 0 → fails (not strict subterm).
- Candidate index 1 (`xs`): recursive call passes `t` at position 1.
  - `t` is bound by `match xs { Cons(h, t) => ... }` → `Provenance::Subterm { root_param: "xs", depth: 1, .. }`. ✅
- Pick index 1. Emit `FnCert { decreasing_arg_index: Some(1), decreasing_arg_name: Some("xs"), call_site_witnesses: [...] }`.

---

## 5. Mutual Recursion Check

For an SCC `{ f, g }`, MVP0 requires **a common decreasing parameter position with the same root-parameter mapping across the group**.

Algorithm:
1. For each function in the SCC, determine its parameter count and types.
2. For each candidate position `i` (0-indexed, common to all members — requires equal arities **OR** an explicit `@decreasing` annotation per function):
   - For every recursive call `caller → callee` in the SCC, check that the argument passed at position `i` is a strict subterm of `caller`'s parameter at position `i`.
3. Accept the first index satisfying all calls. Otherwise emit `MutualGroupUnranked` with the first failing call.

MVP0 keeps the rule **conservative**: same decreasing index in every member. Heterogeneous indices via "lexicographic measures" are a known MVP2 extension.

Worked example — `even/odd` on naturals:
```
fn even(n: Nat): Bool = match n { Zero => True,  Succ(m) => odd(m) }
fn odd(n: Nat): Bool  = match n { Zero => False, Succ(m) => even(m) }
```
Index 0 (`n`): every recursive call passes `m`, bound by `Succ(m)`, subterm of `n`. ✅
Emit `CertMethod::MutualStructuralRecursion { group: ["even", "odd"] }` with both `FnCert`s pinned to index 0.

---

## 6. Special Cases (each gets a dedicated test)

| # | Case | Expected outcome |
|---|---|---|
| S1 | Non-recursive function | `CertMethod::NonRecursive` |
| S2 | Tail recursion with accumulator (`reverse` with helper) | Accept — accumulator at any non-decreasing index is fine if another index decreases |
| S3 | Recursive call inside a `let` binding scrutinee | Provenance propagates through `let` |
| S4 | Recursive call inside a lambda body returned as a value (HOF) | `HigherOrderCallback` if the lambda is called by callee; else OK if lambda is never invoked recursively |
| S5 | `f(Cons(h, t))` — reconstructed argument | `NonStructuralRecursion { reason: ReconstructedTerm }` |
| S6 | `let y = xs in f(y)` | OK — provenance flows through `let` |
| S7 | `let y = reverse(xs) in f(y)` | Rejected — `reverse(xs)` is `Constructed` |
| S8 | `f(f(t))` where `t` is subterm of `xs` | Accept — outer call's arg is `Constructed`? **Special rule**: if the inner call is itself certified recursive, treat its result as `Opaque`, not subterm. Outer call → fails (`NonStructuralRecursion`). |
| S9 | Self-application `f(f, t)` (higher-order) | `SelfApplication` |
| S10 | Mutual recursion, different arities | Reject unless every member has `@decreasing` annotation |
| S11 | Recursive call to a lambda-captured copy of `f` | Rejected (`SelfApplication` variant); ABAL disallows shadowing top-level fn names |
| S12 | Indirect recursion through a stdlib HOF: `List.foldr(f, init, xs)` where `f` calls into the recursive fn | Accept — `List.foldr` is total by I-1 over the DB, the recursive caller is not in the SCC at all |

S12 is important: it captures the **"if higher-order combinators are themselves certified total, we can use them freely"** principle that makes structural recursion practical.

---

## 7. Properties (enforced by `proptest`)

```
P1. verify_fn is pure and deterministic.
P2. Order-independence: verifying functions in different module orders yields
    identical certs/errors.
P3. Soundness sanity (offline check): for every fn that verifies, the VM (Step 0.3.2)
    evaluates it on 100 random inputs without OutOfFuel within standard fuel budget.
    Statistical sanity, not a proof — but a strong red flag if it fails.
P4. Rejection stability: minor syntactic edits that preserve structural recursion
    (e.g., let-renaming) do not change accept/reject outcome.
P5. Idempotence: verify(verify-accepted fn) === Accept; verify(verify-rejected fn) === Reject (same error).
```

---

## 8. File Layout

```
crates/alfdf-totality/
├── src/
│   ├── lib.rs
│   ├── call_graph.rs       # Pass 1
│   ├── scc.rs              # Tarjan
│   ├── provenance.rs       # Pass 3
│   ├── verify.rs           # Passes 2 + 4
│   └── error.rs            # error/hint formatting
└── tests/
    ├── single_fn/
    │   ├── s01_non_recursive.rs
    │   ├── s02_tail_recursion.rs
    │   ├── s03_let_scrutinee.rs
    │   ├── s05_reconstructed.rs
    │   ├── s06_let_through.rs
    │   ├── s07_let_blocks.rs
    │   ├── s08_nested_recursion.rs
    │   ├── s09_self_application.rs
    │   ├── s11_shadowing.rs
    │   └── s12_hof_total.rs
    ├── mutual/
    │   ├── even_odd.rs
    │   ├── tree_forest.rs
    │   └── unequal_arities.rs
    ├── property_pure.rs
    ├── property_order.rs
    ├── property_soundness_sanity.rs
    └── golden/
        ├── cert_map.json
        ├── cert_even_odd.json
        ├── err_no_decreasing.json
        └── err_non_structural.json
```

---

## 9. Error Messages → Structured Refactor Hints

Per invariant I-12, errors returned to the LLM are structured. But they should also carry actionable hints. Hint patterns (closed enum):

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "hint_kind")]
pub enum Hint {
    AddPatternMatch        { suggested_param: Ident },
    UseAccumulator         { example_fn_in_stdlib: ContentHash },
    SplitIntoHelper        { reason: String },
    ConvertToFoldr         { target_signature: String },
    SwapArgumentOrder      { from: usize, to: usize },
}
```

Examples:
- `NoDecreasingArgument` on a fn that takes a `List<a>` and calls itself with the same list → `Hint::AddPatternMatch { suggested_param: "xs" }`.
- `NonStructuralRecursion { reason: ReconstructedTerm }` → `Hint::UseAccumulator { example_fn_in_stdlib: hash_of(reverse_with_acc) }`.
- Mutual recursion with unequal arities → `Hint::SplitIntoHelper { reason: "Unify arities by introducing a wrapper" }`.

These hints are consumed by the pipeline's retry loop in Step 0.5.7. The LLM gets structured `Hint` objects, not prose — preserving I-6.

---

## 10. Benchmarks

| Bench | Target |
|---|---|
| `verify_simple_recursive_fn` (e.g., `map`) | p99 ≤ 200 µs |
| `verify_module_50_fns` | p99 ≤ 20 ms |
| `verify_mutual_group_10_fns` | p99 ≤ 5 ms |
| `verify_deep_match_nesting` (depth 10) | p99 ≤ 500 µs |

Saved to `bench/baselines/totality-v0.1.json`.

The checker is dominated by tree traversal; targets are loose because this stage runs once per submission and is not in any query hot path.

---

## 11. Refactor Smells to Watch

- **Recomputing provenance per call site.** Build the map once per fn body, then look up O(1).
- **Hard-coded "List" / "Cons" / "Nil".** The checker must be ADT-agnostic; it works off the typecheck-resolved constructor table.
- **String-based variable identity.** Use `Ident` (interned) throughout; never compare names by `String`.
- **Implicit dependency on parse order.** P2 forbids it; add a shuffle in tests.
- **Conflating "non-recursive" with "trivially terminating".** A function may be non-recursive yet call into a recursive callee — that's still `CertMethod::NonRecursive` because the callee carries its own cert. Don't propagate certs transitively; this is **local** verification with **global trust in DB entries**.

---

## 12. Failure Modes Outside the Algorithm

| Situation | Resolution |
|---|---|
| Typecheck failure upstream | Totality is not invoked; pipeline halts at Step [2] |
| Call to an entry that is itself uncertified (should never happen post-I-1) | Treat as `Opaque` — guarantees soundness even under bugs elsewhere; emit `error!` for ops |
| Cyclic module import (should be impossible — ABAL has no imports cycles in MVP0) | Reject at parse level; this Step never sees it |
| Recursive call inside a `prop`/`test` declaration body | Same rules apply; tests must themselves be total |

---

## 13. Observability

```
alfdf_totality_check_total{outcome="accept|reject"}
alfdf_totality_reject_reason_total{code="..."}
alfdf_totality_scc_sizes                 (histogram)
alfdf_totality_duration_seconds          (histogram)
alfdf_totality_hint_emitted_total{hint_kind="..."}
```

The `hint_emitted_total` series is especially useful to track which refactor hints the LLM most often acts on (correlate with subsequent successful re-submissions).

---

## 14. Done When

- All 12 single/mutual case tests (S1–S12 + 3 mutual cases) pass.
- All 5 property tests pass with 500 cases each.
- 4 golden JSON files committed and validated against `totality-cert-1.0.0.json` / `totality-error-1.0.0.json` schemas (the schemas themselves are sub-deliverables of this Step).
- Benchmarks meet §10 targets.
- Conformance test: every seed stdlib function verifies; deliberately broken variants (loop, ackermann-as-is, paramorphism that confuses subterm origin) are rejected with appropriate `Hint`s.
- `cargo clippy -p alfdf-totality -- -D warnings` clean; `cargo doc` warning-free.
- Grep test: this crate does not import the VM, the typecheck crate's internals (only its public AST-with-types), or storage.
- Spec invariant I-1 has its conformance test wired to this checker.

---

## Recap of mini-spec series so far

| Step | Status |
|---|---|
| 0.5.7 — Pipeline Orchestrator | ✅ specified |
| 0.5.6 — Dedup Decision Matrix | ✅ specified |
| 0.2.7 — Structural Recursion Checker | ✅ specified (this message) |

## Remaining mini-spec candidates

| Step | Why it still needs depth | Recommended priority |
|---|---|---|
| **0.4.5 — TypeIndex trie + signature normalization** | Sub-ms p99 lookup; normalization rules are subtle (currying, α-rename, arg-reorder); shared with L1 dedup | **High** — performance gate for queries |
| **0.5.8 — Type-directed synthesis** | Bounded BFS with type-guided pruning; budget accounting; consumed by both `synthesize` tool and query fallback | High |
| **0.4.6 — Embedding pipeline (candle + bge-small)** | Model loading, deterministic batching, ingest throughput AC | Medium |
| **0.5.16 — 20-task benchmark harness** | Defines AC-26..AC-30 measurement protocol; the bar for "MVP0 done" | High once code is flowing |

Say **"далее"** for the next one. My recommendation: **Step 0.4.5 (TypeIndex trie)** — it gates query latency (AC-15) and is the algorithmic core that makes ALFDF feel fast or sluggish to the LLM.