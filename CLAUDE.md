# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

**Build and Test:**
- `cargo build` - Build the project
- `cargo build --release` - Build optimized release version
- `cargo test` - Run tests
- `cargo check` - Fast compile-time checks without building binaries

**Development:**
- `cargo run` - Run the debug build (same as `./target/debug/rs-claude-bar`)
- `cargo run --release` - Run the release build
- `./target/release/rs-claude-bar` - Run compiled release binary directly

**Installation:**
- `cargo install --path .` - Install to ~/.cargo/bin/rs-claude-bar
- `/home/vscode/.cargo/bin/rs-claude-bar` - Run installed version

**CLI Usage:**
- `rs-claude-bar info` - Show basic usage information (default command)
- `rs-claude-bar prompt` - Show status line for Claude Code integration
- `rs-claude-bar install` - Configure Claude settings integration
- `rs-claude-bar config claude-path` - Configure Claude data directory path
- `rs-claude-bar config display` - Configure display settings
- `rs-claude-bar blocks` - Show 5-hour usage blocks (default: recent blocks)
- `rs-claude-bar blocks all` - Show all usage blocks from cache
- `rs-claude-bar blocks limits` - Show all limit events from cache
- `rs-claude-bar blocks gaps` - Show usage gaps between blocks

**Global Flags:**
- `--no-cache` - Force bypass cache and reprocess all files
- `--no-save` - Don't save cache after processing

**Debugging:**
- `RUST_BACKTRACE=1 ./target/debug/rs-claude-bar debug --blocks` - Debug with backtrace

## Architecture Overview

This is a Rust CLI tool that analyzes Claude Code usage data from local JSONL files and provides fast status line integration for Claude Code. The architecture follows a modular design:

**Core Components:**

1. **JSONL Parsing** (`cache/`) - Processes Claude transcript files from `~/.claude/projects/` directory
2. **Cache System** (`cache/`) - HashMap-based O(1) lookup storage with persistence to `~/.claude_bar/`
3. **Analysis Engine** (`analyze/`) - Calculates 5-hour usage windows and token statistics
4. **CLI Interface** (`cli.rs`, `commands/`) - Command-line interface using clap
5. **Display System** (`display/`, `table/`) - Formatted output with color coding and tables

**Key Data Flow:**

1. **Configuration Loading** (`config/`) - Manages user settings and Claude data path detection
2. **Cache Management** (`cache/cache_manager.rs`) - Loads/saves parsed JSONL data with automatic invalidation
3. **File Processing** (`cache/utils/parse.rs`) - Incremental JSONL parsing with boundary detection
4. **Analysis** (`analyze/analyzer.rs`) - Aggregates usage into 5-hour billing windows
5. **Display** (`display/prompt.rs`) - Generates status line output for Claude Code integration

**Data Structures:**

- **PerHourBlock** - Tracks token usage, model types, and timestamps for each hour
- **BlockLine** - Records limit/unlock events and session boundaries
- **Cache System** - Persistent storage with automatic file change detection
- **5-Hour Windows** - Groups usage data by Claude's billing cycles

**Performance Features:**

- **Sub-100ms Response Time** - Achieved through intelligent caching (27-78ms typical)
- **Incremental Processing** - Only parses new JSONL entries since last run
- **Cache Persistence** - Avoids reprocessing unchanged files
- **Zero Filesystem Access** - Debug commands operate entirely from cache

**Integration Points:**

- **Claude Code Status Line** - Primary use case via `rs-claude-bar prompt`
- **Cache Directory** - `~/.claude_bar/` for persistent data storage
- **Claude Data Path** - Automatically detects `~/.claude/projects/` or custom paths

## Code Organization

**Module Structure:**
- `src/main.rs` - Entry point with timing instrumentation
- `src/cli.rs` - Command-line argument parsing with clap
- `src/config/` - Configuration management and Claude data path detection
- `src/cache/` - JSONL parsing, caching, and file management
- `src/analyze/` - Usage analysis and 5-hour window calculations
- `src/display/` - Output formatting and status line generation
- `src/table/` - Table formatting utilities with color coding
- `src/common/` - Shared utilities (colors, duration formatting)
- `src/commands/` - Individual CLI command implementations

**Key Files:**
- `src/cache/cache_manager.rs` - Core caching logic and persistence
- `src/cache/utils/parse.rs` - JSONL parsing with error handling
- `src/analyze/analyzer.rs` - Usage analysis and window detection
- `src/display/prompt.rs` - Status line generation for Claude Code integration

**Dependencies:**
- `serde_json` - JSONL parsing and serialization
- `chrono` - Date/time handling with timezone support
- `clap` - CLI argument parsing with derive macros
- `tabled` - Table formatting and display
- `dirs` - Cross-platform directory detection
- `regex` - Pattern matching for file processing

**Performance Optimizations:**
- Release profile uses LTO, single codegen unit, and panic=abort
- Cache invalidation only on file modification times
- Incremental JSONL parsing with line-by-line processing
- Memory-efficient data structures for large usage datasets