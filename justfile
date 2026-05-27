# STEP-A2: full local verify mirrors CI (build, test, clippy, fmt, deny, jscpd).

build:
    cargo build --workspace

clippy:
    cargo clippy --workspace --all-targets -- -D warnings

fmt-check:
    cargo fmt --all -- --check

test:
    cargo nextest run --workspace --no-tests pass

deny:
    cargo deny check

jscpd:
    ./scripts/jscpd_gate.sh

verify: build clippy fmt-check test jscpd deny
