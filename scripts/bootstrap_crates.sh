#!/usr/bin/env bash
# Generated scaffold for STEP-A1 — TDD exception (b).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

declare -A DOCS=(
    [alfdf-ast]="AST types, schema, and hashing. Spec §3."
    [alfdf-parser]="ABAL text to AST parser. Spec §3."
    [alfdf-typeck]="Bidirectional ABAL type checker. Spec §3."
    [alfdf-totality]="Recursion and exhaustiveness checks. Spec §3."
    [alfdf-vm]="Tree-walking VM with fuel. Spec §9."
    [alfdf-fuzz]="Property fuzzer and law registry. Spec §4."
    [alfdf-storage]="StorageAdapter and backends. Spec §8."
    [alfdf-embed]="Embedding generation for vectors. Spec §8."
    [alfdf-dedup]="L1-L5 deduplication engine. Spec §6."
    [alfdf-synthesize]="Pipeline synthesis helpers. Spec §5."
    [alfdf-pipeline]="Submission pipeline orchestrator. Spec §5."
    [alfdf-mcp]="MCP server over stdio. Spec §7."
    [alfdf-stdlib]="Seed stdlib ABAL corpus. Spec §10."
    [alfdf-metrics]="Benchmark and metrics harness. Spec §11."
)

for name in "${!DOCS[@]}"; do
    dir="crates/${name}"
    mkdir -p "${dir}/src"
    cat > "${dir}/Cargo.toml" <<EOF
[package]
name = "${name}"
description = "ALFDF MVP0 — ${name}"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
publish.workspace = true
license.workspace = true

[lints]
workspace = true
EOF
    doc="${DOCS[$name]}"
    cat > "${dir}/src/lib.rs" <<EOF
//! ${doc}
EOF
done

echo "bootstrap: ${#DOCS[@]} crates"
