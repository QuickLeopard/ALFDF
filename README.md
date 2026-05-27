# ALFDF

Abstract LLM-friendly Functional Decomposition Framework — MVP0 Rust workspace.

## Documentation

- [Project specification](.DOCS/ALFDF-MVP0-Project-Spec-v0.1.md)
- [Step-by-step implementation guide](.DOCS/ALFDF-MVP0-Stepbystep-Guide-v0.md)
- [Build guide](.DOCS/ALFDF-Build-Guide-v0.1.md)

## Prerequisites

- [Rust 1.95.0](https://www.rust-lang.org/) (`rust-toolchain.toml` pins the toolchain)
- [just](https://github.com/casey/just) task runner
- [cargo-nextest](https://nexte.st/) (`cargo install --locked cargo-nextest`)
- [cargo-deny](https://embarkstudios.github.io/cargo-deny/) (`cargo install --locked cargo-deny`)
- Node.js 20+ (`npx` for jscpd via `just jscpd`)

## Development

```bash
just verify          # build, clippy, fmt, test, jscpd, deny
cargo build --workspace
```

Individual targets: `just build`, `just test`, `just deny`, `just jscpd`.

## Workspace crates

Fourteen library crates under `crates/alfdf-*` — see the step guide Phase B–O for responsibilities.
