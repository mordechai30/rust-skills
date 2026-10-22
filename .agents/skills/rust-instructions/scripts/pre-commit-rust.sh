#!/usr/bin/env bash
# Check workspace formatting and lints before committing.
# Invoke from the Cargo root through the existing hook manager.
set -euo pipefail

if [[ ! -f Cargo.toml ]]; then
    printf '%s\n' 'Run this hook from the Cargo workspace root.' >&2
    exit 2
fi
export RUSTUP_AUTO_INSTALL=0
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
