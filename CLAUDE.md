# CLAUDE.md

This file provides guidance to Claude Code when working with this Rust project.

## Project Overview

`rs-claude-bar` is a fast, lightweight Claude Code usage tracker written in Rust. It replaces Python/Node.js solutions with a zero-dependency binary that provides real-time status information for Claude Code sessions.

## Architecture

### Core Components

1. **Main Binary** (`src/main.rs`)
   - Entry point for the CLI tool
   - Handles basic JSONL file discovery and parsing
   - Generates status line output

2. **Future Modules** (planned)
   - `src/parser.rs` - JSONL parsing and data extraction
   - `src/windows.rs` - 5-hour window calculations  
   - `src/cache.rs` - Caching system for performance
   - `src/models.rs` - Data structures and types

### Data Flow

1. **Discovery**: Find Claude data directories (`~/.claude/projects/`, `~/.config/claude/projects/`)
2. **Parsing**: Read JSONL files from project subdirectories
3. **Analysis**: Calculate tokens, windows, and session state
4. **Cache**: Store processed data in `~/.claude_bar/`
5. **Output**: Generate formatted status line

## Claude Code Integration

The tool integrates with Claude Code via the status line feature in `~/.claude/settings.json`:

```json
{
  "statusLine": {
    "type": "command", 
    "command": "/path/to/rs-claude-bar",
    "padding": 0
  }
}
```

## JSONL File Format

Claude Code stores usage data in JSONL files with this structure:
```json
{
  "timestamp": "2025-08-27T18:00:00.000Z",
  "sessionId": "session-id-here",
  "model": {
    "id": "claude-sonnet-4-20250514",
    "display_name": "Claude 3.5 Sonnet"
  },
  "message": {
    "usage": {
      "input_tokens": 1234,
      "output_tokens": 567,
      "cache_creation_input_tokens": 890,
      "cache_read_input_tokens": 123
    }
  },
  "costUSD": 0.015
}
```

## Development Guidelines

### Code Style
- Use `rustfmt` for formatting: `cargo fmt`
- Use `clippy` for linting: `cargo clippy`
- Follow Rust naming conventions (snake_case for functions, PascalCase for types)

### Dependencies
Keep dependencies minimal:
- **serde_json**: JSON parsing
- **chrono**: Date/time handling
- **std library**: File I/O, path manipulation

Avoid heavy dependencies like:
- tokio (unless async is required)
- clap (for minimal version)
- anyhow (use std::error::Error)

### Performance
- Optimize for status line responsiveness (< 100ms)
- Use caching to avoid re-parsing JSONL files
- Consider memory usage (should be < 10MB)

### Error Handling
- Use `Result<T, Box<dyn std::error::Error>>` for simple error handling
- Fail gracefully with helpful error messages
- Never panic in production code

### Testing
```bash
# Run tests
cargo test

# Build release
cargo build --release

# Test installation
cargo install --path . --force
```

## 5-Hour Window Logic

Based on CCUsage implementation:
1. Group usage entries by 5-hour periods from first entry
2. Calculate total tokens per window
3. Track active window (current 5-hour period)
4. Show progress within current window

## Cache Strategy

Store in `~/.claude_bar/`:
- `usage_cache.json` - Processed windows and session data
- Invalidate when JSONL files are modified
- Update every 30 seconds for real-time status

## Output Format Design

Target format (like CCUsage blocks):
```
🧠 15,234 (53.6%) 🟡 | 💬 124 | ⏱️ 2h15m | ⏰ 2h45m left | 🤖 Sonnet 4
```

Components:
- Token count with percentage and color indicator
- Message count in current session  
- Elapsed time in current window
- Time remaining in current window
- Current model name (shortened)

## Installation & Distribution

### Development
```bash
cargo install --path .
```

### Production
1. GitHub Actions build for multiple platforms
2. GitHub Releases with binaries
3. Cargo registry publication
4. Optional: Package managers (brew, chocolatey)

## Future Enhancements

See [TODO.md](TODO.md) for planned features and improvements.

## Reference Materials

- CCUsage source code in `../REFERENCE/` (excluded from git)
- Claude Code JSONL format documentation
- Rust performance best practices