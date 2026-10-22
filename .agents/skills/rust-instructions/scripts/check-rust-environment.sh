#!/usr/bin/env bash
# Check Rust tools without installing or changing them.
# Use Python for structured Cargo, TOML, and release data.
set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
python_bin="${PYTHON:-python3}"
if ! command -v "$python_bin" >/dev/null 2>&1 ||
    ! "$python_bin" -c 'import sys; sys.exit(sys.version_info < (3, 11))' 2>/dev/null; then
    printf '%s\n' 'UNKNOWN: Full inventory requires Python 3.11 or newer; set PYTHON to its executable.'
    for tool in rustup rustc cargo rustfmt cargo-clippy cargo-audit bacon cargo-nextest \
        cargo-flamegraph hyperfine tokio-console oha; do
        if command -v "$tool" >/dev/null 2>&1; then
            printf 'PRESENT: %s (usability and version not checked)\n' "$tool"
        else
            printf 'MISSING: %s\n' "$tool"
        fi
    done
    exit 1
fi
exec "$python_bin" "$script_dir/check_rust_environment.py" "$@"
