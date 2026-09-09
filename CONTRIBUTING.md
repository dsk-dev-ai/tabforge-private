# Contributing

Thanks for your interest in contributing to TabForge.

## Setup

Requires [Rust](https://rustup.rs/) (stable) and the Tauri system dependencies
for your OS (see [Tauri prerequisites](https://tauri.app/start/prerequisites/)).

```bash
git clone https://github.com/dsk-dev-ai/tabforge-private.git
cd tabforge-private
cd desktop-app/src-tauri
cargo build
```

## Quality gates

```bash
cd desktop-app/src-tauri
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

CI runs `cargo test` on every push/PR.

## Process

1. Branch from `main`: `feat/my-feature` or `fix/my-bug`.
2. Keep the pure logic (IPC/bridge, encoder) unit-testable.
3. Run the quality gates above.
4. Commit with a Conventional Commit message and open a PR.

## PR checklist

- [ ] `cargo fmt --check` passes
- [ ] `cargo clippy -- -D warnings` passes
- [ ] Tests added/updated and passing
- [ ] README updated if behavior changed