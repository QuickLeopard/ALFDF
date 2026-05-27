# STEP-A1: build, clippy, fmt only. Full verify (deny, nextest) lands in STEP-A2.

build:
    cargo build --workspace

clippy:
    cargo clippy --workspace --all-targets -- -D warnings

fmt-check:
    cargo fmt --all -- --check

verify: build clippy fmt-check
