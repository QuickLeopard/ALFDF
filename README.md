# ALFDF

Abstract LLM-friendly Functional Decomposition Framework — MVP0 Rust workspace.

## Documentation

- [Project specification](.DOCS/ALFDF-MVP0-Project-Spec-v0.1.md)
- [Step-by-step implementation guide](.DOCS/ALFDF-MVP0-Stepbystep-Guide-v0.md)
- [Build guide](.DOCS/ALFDF-Build-Guide-v0.1.md)

## Prerequisites

- [Rust 1.95.0](https://www.rust-lang.org/) (`rust-toolchain.toml` pins the toolchain)
- [just](https://github.com/casey/just) task runner

## Development

```bash
just verify          # build + clippy + fmt (STEP-A1 subset)
cargo build --workspace
```

STEP-A2 will extend `just verify` with `cargo test`, `cargo deny`, and nextest.

## Workspace crates

Fourteen library crates under `crates/alfdf-*` — see the step guide Phase B–O for responsibilities.
