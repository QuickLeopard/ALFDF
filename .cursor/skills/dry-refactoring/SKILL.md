---
name: dry-refactoring
description: Eliminates copy-paste duplication in ALFDF Rust code after jscpd reports clones. Use when jscpd finds duplicates, during refactor sweeps, or when reducing LOC in crates/ or scripts/.
---

# dry-refactoring (ALFDF)

Guided workflow to eliminate copy-paste duplication in Rust source. Use **after** running [jscpd](../jscpd/SKILL.md).

## Prerequisites

```bash
just jscpd
```

Do not run bare `npx jscpd` without `--config .jscpd.json` — thresholds would not match [.cursor/rules/21-dry.mdc](../../rules/21-dry.mdc).

## Workflow

1. Run `just jscpd` on the workspace (or the paths you changed).
2. Parse each clone line: file + line range for both sides.
3. Read both fragments from `crates/` or `scripts/`.
4. Confirm the duplication is **real** (not coincidental similarity per [23-yagni-kiss](../../rules/23-yagni-kiss.mdc)).
5. Choose a Rust refactoring (see strategies below).
6. Apply — update **all** call sites, not only the two jscpd locations.
7. Re-run `just jscpd` until clean.
8. Run `just verify` (build, clippy, fmt). Run `cargo test` when tests exist for touched crates.

## Rust refactoring strategies

**Extract `fn`** — duplicate logic block in one or more modules:

```rust
// Before: same block in two fns
// After: shared fn called from both
```

**Extract module** — shared logic across files in the same crate:

```rust
// crates/alfdf-foo/src/util.rs — import from both call sites
```

**Extract test helper** — duplicate setup/assertions in tests ([60-test-policy](../../rules/60-test-policy.mdc)):

```rust
// crates/<c>/tests/support/mod.rs
```

**Extract constant / type alias** — repeated literals or type expressions.

**Shared trait** — repeated method sets (prefer concrete extraction first; no speculative traits per YAGNI).

## Always ensure

- All call sites updated, not just the two reported clones
- Tests still pass after refactoring
- Extracted names are clear and crate-local unless truly workspace-wide
- No new `pub` without a caller ([23-yagni-kiss](../../rules/23-yagni-kiss.mdc))

## Commit guidance

Per [.cursor/system.md](../../system.md):

```text
refactor(<crate>): STEP-<id> dedupe <area>
```

## Tips

- Start with highest line-count clones
- Clones between `tests/*.rs` → check `tests/support/` first
- Clones across unrelated crates may need a shared dependency — justify in PR body if adding one
- Phase-end sweeps: see [refactor-sweep](../refactor-sweep/SKILL.md)

## Related

- [jscpd](../jscpd/SKILL.md) — detection
- [refactor-sweep](../refactor-sweep/SKILL.md) — scheduled sweep
- [21-dry.mdc](../../rules/21-dry.mdc) — policy
