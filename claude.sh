#!/bin/bash

# Claude Code Status Line - Rust Wrapper
# This bash script wraps the Rust binary to ensure proper ANSI color handling

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Execute the Rust binary and pass through all arguments
exec "${SCRIPT_DIR}/target/release/rs-claude-bar" "$@"