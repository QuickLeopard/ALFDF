#!/usr/bin/env bash
# Wire STEP-A4 workspace dependencies into every member crate (TDD exception b).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DEPS_BLOCK='
[dependencies]
serde = { workspace = true }
serde_json = { workspace = true }
thiserror = { workspace = true }
anyhow = { workspace = true }
tracing = { workspace = true }
blake3 = { workspace = true }

[dev-dependencies]
proptest = { workspace = true }
insta = { workspace = true }
criterion = { workspace = true }
'

for manifest in "$ROOT"/crates/*/Cargo.toml; do
    if grep -q '^\[dependencies\]' "$manifest"; then
        echo "skip (already wired): $manifest"
        continue
    fi
    printf '%s\n' "$DEPS_BLOCK" >> "$manifest"
    echo "wired: $manifest"
done
