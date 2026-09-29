#!/bin/bash
# Initialize a robust Rust project

if [ -f "Cargo.toml" ]; then
    echo "Cargo projects already exists."
else
    cargo init
fi

echo "Rust project initialized. Add dependencies required by the task."
