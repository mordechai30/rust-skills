#!/bin/bash
# Wrapper script to ensure correct CWD and environment for rust-guardian

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

# Set the working directory to the project root (where the script is)
cd "$SCRIPT_DIR" || exit 1

# Build in the plugin data directory when the client provides one.
if [ -n "${PLUGIN_DATA:-}" ]; then
    mkdir -p "$PLUGIN_DATA/target"
    export CARGO_TARGET_DIR="$PLUGIN_DATA/target"
fi
TARGET_DIR="${CARGO_TARGET_DIR:-$SCRIPT_DIR/target}"
BINARY="$TARGET_DIR/release/rust-agentic-skills"
if ! command -v cargo &> /dev/null; then
    if [ -f "$HOME/.cargo/bin/cargo" ]; then
        export PATH="$HOME/.cargo/bin:$PATH"
    else
        echo "Error: cargo is required to start rust-guardian." >&2
        exit 1
    fi
fi
if ! cargo build --release --bin rust-agentic-skills --quiet >&2; then
    echo "Error: Failed to build rust-guardian." >&2
    exit 1
fi

# Exec the binary, passing all arguments
# We use exec to replace the shell process with the binary
exec "$BINARY" "$@"
