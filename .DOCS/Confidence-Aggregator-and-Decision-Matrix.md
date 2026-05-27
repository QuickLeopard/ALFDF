# Code-level Mini-Spec — Step 0.5.6 "Confidence Aggregator + Decision Matrix"

Continuing the mini-spec series. This Step is the **unique value-add of ALFDF**: the place where layered evidence from L1–L5 collapses into one of four outcomes, enforcing invariant **I-4** ("no two entries are L2/L3/L4-equivalent in the DB").

I'm choosing this Step next because:
- Every dedup layer (Steps 0.5.1–0.5.5) feeds into it; getting this wrong silently corrupts the DB forever.
- The decision semantics (Reject / MergeAlias / StoreAlternative / Accept) have subtle edge cases that the spec sketches but doesn't pin down.
- It's the integration test for goals **(b) reduced duplication** and **(e) architectural consistency**.

---

## 0. Scope

Build the decision module inside `crates/alfdf-dedup/` that takes the raw L1–L5 evidence for a candidate submission and produces a structured `DedupDecision`. The orchestrator (Step 0.5.7) consumes this decision to route the submission.

**Not in this Step:** the L1–L5 layer implementations themselves; the orchestrator's wiring; the replacement-review UI.

---

## 1. Public Surface (`alfdf-dedup/src/decision.rs`)

```rust
use alfdf_ast::{ContentHash, Decl};
use serde::{Serialize, Deserialize};

/// Raw evidence from one peer candidate detected by L1.
#[derive(Debug, Clone)]
pub struct PeerEvidence {
    pub peer_hash:         ContentHash,
    pub peer_entity_id:    String,
    pub l1_signature_iso:  IsoKind,              // Exact | Curried | ArgReordered
    pub l2_beta_eta_equal: bool,
    pub l3_outputs_equal:  Option<L3Result>,     // None = skipped
    pub l4_cross_tests:    Option<L4Result>,     // None = skipped
    pub l5_cosine_sim:     f32,                  // [0.0, 1.0]
    pub peer_metrics:      PeerMetrics,
}

#[derive(Debug, Clone)]
pub struct L3Result {
    pub corpus_size:    usize,    // = 1000 by default
    pub agreements:     usize,
    pub disagreements:  usize,
    pub eval_failures:  usize,    // OutOfFuel counted as disagreement
}

#[derive(Debug, Clone)]
pub struct L4Result {
    pub peer_tests_total:     usize,
    pub peer_tests_passing:   usize,
    pub cand_tests_total:     usize,
    pub cand_tests_passing:   usize,    // run against peer
}

#[derive(Debug, Clone)]
pub struct PeerMetrics {
    pub ast_node_count:        u32,
    pub declared_time_bigo:    BigO,
    pub declared_space_bigo:   BigO,
    pub measured_p50_ns:       Option<u64>,
    pub measured_p99_ns:       Option<u64>,
    pub direct_dep_count:      u32,
    pub usage_count:           u32,
    pub age_days:              u32,
}

#[derive(Debug, Clone)]
pub struct CandidateMetrics { /* same shape, for the new submission */ }

/// Input to the aggregator.
pub struct DedupInput<'a> {
    pub candidate:         &'a Decl,
    pub candidate_metrics: CandidateMetrics,
    pub peers:             Vec<PeerEvidence>,    // all L1-matched peers
}

/// Output: one of four outcomes, with structured rationale.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum DedupDecision {
    /// No high-confidence duplicate; submission proceeds to indexing.
    Accept,

    /// High-confidence duplicate, candidate not better → reject.
    Reject {
        canonical_peer:    ContentHash,
        evidence:          ConfirmedEquivalence,
    },

    /// High-confidence duplicate, candidate strictly better → store as alias,
    /// enqueue for replacement review.
    MergeAsAlias {
        canonical_peer:    ContentHash,
        evidence:          ConfirmedEquivalence,
        improvements:      ImprovementDeltas,
        review_required:   bool,    // always true in MVP0
    },

    /// High-confidence duplicate with mixed trade-offs → store as alternative,
    /// link via equivalent_to edge.
    StoreAsAlternative {
        peers:             Vec<ContentHash>,
        evidence:          Vec<ConfirmedEquivalence>,
        trade_offs:        Vec<TradeOff>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmedEquivalence {
    pub layer_decisive:    DecisiveLayer,    // L2 | L3_AND_L4
    pub l3_agreement_rate: Option<f32>,
    pub l4_mutual_pass:    Option<bool>,
    pub l5_cosine_sim:     f32,              // informational only
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecisiveLayer { L2, L3AndL4 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementDeltas {
    pub better:   Vec<MetricDelta>,    // dimensions where candidate wins
    pub equal:    Vec<&'static str>,
    pub worse:    Vec<MetricDelta>,    // MUST be empty for MergeAsAlias
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDelta {
    pub dimension: MetricDimension,    // enum below
    pub peer:      f64,
    pub candidate: f64,
    pub pct_diff:  f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MetricDimension {
    AstNodeCount,
    BenchP50,
    BenchP99,
    DeclaredTimeBigO,
    DeclaredSpaceBigO,
    DirectDepCount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOff {
    pub peer:        ContentHash,
    pub wins:        Vec<MetricDimension>,
    pub losses:      Vec<MetricDimension>,
}

pub fn decide(input: DedupInput) -> DedupDecision;
```

---

## 2. Confidence Aggregation Rules (locked at MVP0)

Per-peer confidence classification, evaluated for **each peer independently** before the cross-peer decision step:

```
confidence(peer) =
  if l2_beta_eta_equal              → CONFIRMED_EQUIVALENT (decisive_layer = L2)
  else if l3_outputs_equal.is_some()
       and l3.agreements == l3.corpus_size
       and l3.eval_failures == 0
       and l4_cross_tests.is_some()
       and l4.peer_tests_passing == l4.peer_tests_total
       and l4.cand_tests_passing == l4.cand_tests_total
                                    → CONFIRMED_EQUIVALENT (decisive_layer = L3_AND_L4)
  else if l3_outputs_equal.is_some()
       and l3.agreements == l3.corpus_size
       and l3.eval_failures == 0
       and l4_cross_tests.is_none() (peer has no tests — should not happen post-MVP0 I-2)
                                    → SUSPECTED  (do not act on alone)
  else                              → DISTINCT
```

**Rationale:**
- L2 alone is **sufficient and decisive** — β-η-equality is mathematical.
- L3 alone is **not sufficient** because the fuzz corpus, however large, is finite. Requiring L4 in addition ("each function passes the other's hand-written tests") closes the gap with developer-curated cases.
- L5 (embedding) is **never decisive**; it only scores ordering and surfaces SUSPECTED candidates for the next-tier review queue (out of MVP0 scope).
- An L3 partial-disagreement (even 1 case out of 1000) classifies the peer as DISTINCT. We do **not** accept near-equivalence at MVP0.

---

## 3. Decision Matrix (deterministic algorithm)

```
let confirmed = peers.iter().filter(|p| confidence(p) == CONFIRMED_EQUIVALENT).collect();

if confirmed.is_empty() {
    return DedupDecision::Accept;
}

// Compute deltas of candidate vs each confirmed peer.
let deltas: Vec<(peer, ImprovementDeltas)> =
    confirmed.iter().map(|p| (p, compute_deltas(candidate_metrics, p.metrics))).collect();

// Classify candidate vs each confirmed peer.
let mut strictly_better = vec![];
let mut strictly_worse_or_equal = vec![];
let mut mixed = vec![];

for (peer, d) in &deltas {
    match (d.better.is_empty(), d.worse.is_empty()) {
        (false, true)  => strictly_better.push((peer, d)),       // wins ≥1 dim, loses 0
        (true,  _)     => strictly_worse_or_equal.push(peer),    // wins 0
        (false, false) => mixed.push((peer, d)),                 // wins ≥1, loses ≥1
        (true,  true)  => strictly_worse_or_equal.push(peer),    // exact tie everywhere
    }
}

// Apply matrix in priority order.
if !strictly_worse_or_equal.is_empty() {
    // At least one peer dominates or ties → REJECT.
    let canonical = pick_canonical(&strictly_worse_or_equal);
    return DedupDecision::Reject { canonical_peer: canonical, evidence: ... };
}

if !strictly_better.is_empty() && mixed.is_empty() {
    // Candidate strictly beats every confirmed peer → MERGE AS ALIAS.
    // Pick the most-used peer as canonical to maximize alias hit rate.
    let canonical = strictly_better.iter().max_by_key(|(p, _)| p.metrics.usage_count).unwrap().0;
    let improvements = aggregate_improvements(&strictly_better);
    return DedupDecision::MergeAsAlias {
        canonical_peer: canonical.peer_hash.clone(),
        evidence: ...,
        improvements,
        review_required: true,
    };
}

// Otherwise: mixed trade-offs exist → STORE AS ALTERNATIVE.
let peers_list = confirmed.iter().map(|p| p.peer_hash.clone()).collect();
let trade_offs = mixed.iter().map(|(p, d)| build_trade_off(p, d)).collect();
return DedupDecision::StoreAsAlternative { peers: peers_list, evidence: ..., trade_offs };
```

### Priority order — why this sequence

1. **Reject first.** If *any* confirmed peer dominates or ties, the candidate adds no value. Even if other peers are worse than the candidate, the existence of one dominant peer means the DB already has something at-least-as-good — rejecting prevents tree-of-aliases sprawl.
2. **MergeAsAlias second.** Only when the candidate beats *every* confirmed peer on at least one dimension and loses on none. This is the rare but valuable "strict improvement."
3. **StoreAsAlternative last.** The "I can't decide" branch. Mixed trade-offs (e.g., faster but uses more memory) genuinely deserve coexistence with explicit linkage.

### Tie-breaking in `pick_canonical`

```
key = (usage_count DESC, age_days ASC, content_hash ASC)
```
Usage-weighted, then prefer older (stable) entries, then lexicographic for determinism.

---

## 4. Metric Comparison Rules

A candidate is "better" on a dimension iff it wins per the table below.

| Dimension | "Better" means | Threshold |
|---|---|---|
| `AstNodeCount` | Smaller | ≥ 10% fewer nodes |
| `BenchP50` | Faster | ≥ 10% lower p50 ns |
| `BenchP99` | Faster | ≥ 10% lower p99 ns |
| `DeclaredTimeBigO` | Strictly lower complexity class | exact class comparison (O(1) < O(log n) < O(n) < O(n log n) < O(n²) …) |
| `DeclaredSpaceBigO` | Strictly lower class | same |
| `DirectDepCount` | Fewer dependencies | ≥ 1 fewer |

**Threshold rationale:** 10% guards against benchmark noise. Below threshold = `equal`, not `better` and not `worse`.

`BigO` comparison uses a fixed total order from a closed enum:
```rust
pub enum BigO { O1, OLogN, OSqrtN, ON, ONLogN, ON2, ON3, OExpN, Other(String) }
```
`Other(_)` is incomparable; treat as `equal` on that dimension.

---

## 5. Edge Cases (must each have a dedicated test)

| # | Scenario | Expected outcome |
|---|---|---|
| E1 | No peers from L1 | `Accept` |
| E2 | One peer, L2-equal, identical metrics | `Reject` (peer dominates by tie) |
| E3 | One peer, L2-equal, candidate strictly smaller AST | `MergeAsAlias` |
| E4 | One peer, L3+L4 confirmed, candidate strictly faster p99 | `MergeAsAlias` |
| E5 | Two peers: one L2-equal, one only L5-similar | `Reject` (only L2 peer counts; L5 ignored) |
| E6 | One peer, L3 99.9% agreement, L4 mutual pass | `Accept` (L3 not unanimous → DISTINCT) |
| E7 | One peer, L3 unanimous, L4 mutual pass, but L3 had eval_failures > 0 | `Accept` (treat OutOfFuel as disagreement) |
| E8 | One peer, L2-equal, candidate faster but uses one more dependency | `StoreAsAlternative` (mixed) |
| E9 | One peer, L2-equal, candidate faster AND smaller AST | `MergeAsAlias` |
| E10 | Two peers, both confirmed; one dominated by candidate, one ties | `Reject` (tie counts as dominant) |
| E11 | One peer, L3 unanimous + L4 mutual pass, candidate has BigO O(log n) vs peer O(n) | `MergeAsAlias` (strict class improvement) |
| E12 | Candidate has no benchmarks recorded yet (`measured_p50_ns = None`) | Bench dimensions skipped; decision uses available dims only |
| E13 | Confirmed peer is itself an alias of another entry | Resolve to canonical first, compare against canonical |

---

## 6. Idempotence and Determinism Properties

These are **enforced by property tests**:

```
P1. decide(input) is a pure function of input. (no I/O, no clock, no RNG)
P2. decide(input) is deterministic across architectures and Rust versions.
P3. If a peer is later removed from peers and the decision recomputed, the new outcome is one of:
       Accept (if it was the only confirmed peer)
       same decision (if other confirmed peers remain)
       weaker decision (Reject → MergeAsAlias, or MergeAsAlias → Accept) — never stronger.
P4. Permutation invariance: decide(input) is invariant under permutation of input.peers.
P5. Stability under alias resolution: pre-resolving aliased peers before calling decide
    gives identical output to post-resolution.
```

---

## 7. File Layout

```
crates/alfdf-dedup/
├── src/
│   ├── lib.rs              # re-exports
│   ├── layers/
│   │   ├── l1_iso.rs       # (Step 0.5.1)
│   │   ├── l2_beta_eta.rs  # (Step 0.5.2)
│   │   ├── l3_fuzz.rs      # (Step 0.5.3)
│   │   ├── l4_xtests.rs    # (Step 0.5.4)
│   │   └── l5_embed.rs     # (Step 0.5.5)
│   ├── confidence.rs       # §2
│   ├── metrics.rs          # §4 metric comparison, BigO ordering
│   ├── decision.rs         # §1 + §3 (this Step)
│   └── canonicalize.rs     # alias resolution (E13)
└── tests/
    ├── decision_matrix.rs        # E1–E13, one test each
    ├── property_pure.rs          # P1
    ├── property_permutation.rs   # P4
    ├── property_monotone.rs      # P3
    └── golden/
        ├── decision_reject.json
        ├── decision_merge_alias.json
        ├── decision_alternative.json
        └── decision_accept.json
```

---

## 8. Test Specifications

Each table-row test follows this template:

```rust
#[test]
fn e04_l3_l4_confirmed_candidate_faster_p99() {
    let peer = peer_evidence()
        .l2(false)
        .l3_unanimous(1000)
        .l4_mutual_pass(true)
        .ast_node_count(50)
        .bench_p99_ns(1_000)
        .build();
    let candidate = candidate_metrics()
        .ast_node_count(50)
        .bench_p99_ns(800)   // 20% faster — above 10% threshold
        .build();

    let decision = decide(DedupInput {
        candidate: &fake_decl(),
        candidate_metrics: candidate,
        peers: vec![peer],
    });

    assert_matches!(decision, DedupDecision::MergeAsAlias {
        improvements: ImprovementDeltas { better, worse, .. }, ..
    } if better.len() == 1
       && better[0].dimension == MetricDimension::BenchP99
       && worse.is_empty());
}
```

**Property test sketch (P3 — monotonicity):**

```rust
proptest! {
    #[test]
    fn p3_monotone_under_peer_removal(input in arb_dedup_input()) {
        let full = decide(input.clone());
        for i in 0..input.peers.len() {
            let mut shrunk = input.clone();
            shrunk.peers.remove(i);
            let reduced = decide(shrunk);
            prop_assert!(is_weaker_or_equal(&reduced, &full));
        }
    }
}
```

`is_weaker_or_equal` is the partial order:
`Accept < MergeAsAlias < StoreAsAlternative < Reject`
(reading "<" as "weaker"; rejecting is the strongest stance).

---

## 9. Benchmarks

| Bench | Target |
|---|---|
| `decide_no_peers` | p99 ≤ 50 µs |
| `decide_one_peer_l2` | p99 ≤ 200 µs |
| `decide_ten_peers_full_evidence` | p99 ≤ 2 ms |
| `decide_100_peers_all_l1_no_l2` (worst-case L3/L4 already done upstream) | p99 ≤ 10 ms |

The decision module itself does no heavy work — it consumes evidence. Bench numbers exclude L3/L4 execution costs (those are Steps 0.5.3 / 0.5.4 territory).

Stored in `bench/baselines/dedup-decision-v0.1.json`.

---

## 10. Refactor Smells to Watch

- **Magic numbers in match arms.** The 10% threshold, the L3 unanimity rule — all must live in `const`s at the top of `metrics.rs` with doc comments.
- **`BigO::Other(_)` leaking into comparison.** Always treat as `equal` and log at `debug!` so we notice if it becomes frequent.
- **L5 sneaking into confidence.** If `confidence()` ever reads `l5_cosine_sim`, that's a bug — L5 is scoring-only.
- **Async signatures.** `decide` is sync; no `async` here. If you find yourself wanting `async`, the I/O belongs upstream in the layer implementations.
- **Cloning `Decl` for comparison.** Decisions consume references; deep clones only when serializing for the `evidence` payload.

---

## 11. Failure Modes and Their Handling

| Failure | Source | Handling |
|---|---|---|
| L3 evaluation `OutOfFuel` on candidate | VM fuel exhausted | Count as disagreement → candidate likely DISTINCT, may pass dedup but pipeline stage 6 (Tests) might still reject it |
| Peer load fails (missing from store) | Stale L1 index | Skip that peer with `warn!`; recompute L1 cache on background sweeper |
| BigO declared incomparably (`Other`) | User-supplied annotation | Skip dimension; warn once per submission |
| Candidate has no tests yet | Should not happen (caught at stage 6 before dedup, but defensive) | Caller must guarantee non-empty tests before invoking decide |
| All peers L5-similar but none L2/L3/L4 | Embedding false positive | Returns `Accept`; metric `dedup_l5_only_total` increments for observability |

---

## 12. Observability

Emit metrics:
```
alfdf_dedup_decisions_total{outcome="accept|reject|merge_alias|alternative"}
alfdf_dedup_layer_decisive_total{layer="L2|L3_AND_L4"}
alfdf_dedup_l3_unanimous_rate            (histogram)
alfdf_dedup_peers_per_submission         (histogram)
alfdf_dedup_decision_duration_seconds    (histogram)
alfdf_dedup_l5_only_suspected_total      (counter — informational)
```

---

## 13. Done When

- All 13 edge-case tests (E1–E13) pass.
- All 5 property tests (P1–P5) pass with 1000 cases each.
- 4 golden JSON files validate against `mcp-submit-1.0.0.json` `failure.details` or `result.dedup_evidence` (whichever applies).
- 4 benchmarks meet targets in §9.
- `cargo clippy -p alfdf-dedup -- -D warnings` clean.
- `cargo doc -p alfdf-dedup` produces no warnings; every public item documented.
- Conformance test inserts a known duplicate twice and observes invariant I-4 holds.
- Grep test confirms `decision.rs` does not import storage or VM crates directly (it only consumes evidence structs).

---

## Remaining mini-spec candidates

Three Steps still warrant the same depth before serious coding starts:

| Step | Why it's tricky |
|---|---|
| **0.2.7 — Structural recursion checker** | Encodes invariant I-1; subtle interaction with pattern matching and let-bindings; mutual recursion ranking is non-obvious |
| **0.4.5 — TypeIndex trie + signature normalization** | Performance-critical (sub-ms p99); normalization rules (curry / α-rename / arg-reorder) interact with L1 dedup |
| **0.5.8 — Type-directed synthesis** | Bounded search with type-guided pruning; needs careful budget accounting to avoid combinatorial blow-up |

Say **"далее"** again to get the next one (my recommendation: **0.2.7 Structural recursion checker** — it's the foundation of I-1 and the hardest one to retrofit later). Or name a specific Step.