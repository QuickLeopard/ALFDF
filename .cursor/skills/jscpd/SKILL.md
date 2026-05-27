---
name: jscpd
description: Runs jscpd copy-paste detection for the ALFDF Rust workspace using project thresholds (30 LOC / 60 tokens). Use when fixing duplication, before refactor sweeps, when jscpd/DRY CI fails, or when the 21-dry rule applies.
---

# jscpd (ALFDF)

Copy-paste detector for the ALFDF Rust workspace. Enforces [.cursor/rules/21-dry.mdc](../../rules/21-dry.mdc): no duplicated block ≥ **30 LOC** or **60 tokens**.

## Commands (prefer these)

```bash
just jscpd
./scripts/jscpd_gate.sh
```

Ad-hoc (same config as the gate):

```bash
npx jscpd --config .jscpd.json --reporters ai crates/ scripts/
```

Configuration: [`.jscpd.json`](../../../.jscpd.json) at repo root.

**Note:** STEP-A1 `just verify` is build + clippy + fmt only. `just verify` will include jscpd in **STEP-A2** (see [docs/step-reviews/STEP-A1/README.md](../../../docs/step-reviews/STEP-A1/README.md)).

## AI reporter output

```
Clones:
crates/alfdf-ast/src/lib.rs:10-25 ~ crates/alfdf-parser/src/lib.rs:42-57
---
0 clones · 0% duplication
```

Each line is one clone pair:

- **Same file:** `path/file.rs 10-25 ~ 45-60`
- **Different paths:** `crates/a/src/lib.rs:10-25 ~ crates/b/src/lib.rs:42-57`

## Project thresholds

| Setting | Value | Source |
|---------|-------|--------|
| `minLines` | 30 | 21-dry LOC gate |
| `minTokens` | 60 | 21-dry token gate |
| `threshold` | 0 | Gate fails on any reported clone |
| `format` | `rust` | Primary scan target |
| `pattern` | `crates/**`, `scripts/**` | Production Rust + scripts |

Ignored: `target/`, `node_modules/`, `docs/step-reviews/`, `Cargo.lock`, `.gitignore` paths.

## Useful jscpd options

| Option | Description |
|--------|-------------|
| `--reporters ai` | Compact clone list for agents (used by gate) |
| `--config .jscpd.json` | Project thresholds and ignores |
| `--min-lines N` | Override minimum lines (default 30 in config) |
| `--min-tokens N` | Override minimum tokens (default 60 in config) |
| `--gitignore` | Respect `.gitignore` (enabled in config) |

## After clones are found

Use **[dry-refactoring](../dry-refactoring/SKILL.md)** for a guided workflow to eliminate duplication.

Related: [refactor-sweep](../refactor-sweep/SKILL.md) (phase-end sweep).
