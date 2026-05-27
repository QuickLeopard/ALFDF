#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
exec npx --yes jscpd --config "${ROOT}/.jscpd.json" --reporters ai "${ROOT}/crates" "${ROOT}/scripts"
