# rs-claude-bar

> **Enhanced Claude Code usage tracker with 5-hour window monitoring** — lightning-fast Rust implementation with sub-100ms response times!

A high-performance Claude Code usage tracker written in Rust that analyzes your local JSONL files to provide fast status line integration and detailed usage reports. Perfect for monitoring your Claude usage within the 5-hour billing windows.

## ✨ Features

- ⚡ **Ultra-Fast Performance** - Sub-100ms response times (27-78ms typical) through intelligent caching
- 🚀 **Status Line Integration** - Seamless Claude Code status bar integration via `rs-claude-bar prompt`
- ⏰ **5-Hour Window Tracking** - Monitor usage within Claude's billing cycles with active block detection
- 💾 **Smart Caching** - Incremental JSONL parsing with automatic cache invalidation
- 📊 **Detailed Reports** - View usage blocks, limits, gaps, and comprehensive statistics
- 🎯 **Zero Dependencies** - Single binary with no external runtime requirements
- 🔧 **Configuration Management** - Automatic Claude data path detection with custom path support

## 🚀 Quick Start

### Installation

```bash
# Clone and build
git clone https://github.com/DevOpsBenjamin/rs-claude-bar.git
cd rs-claude-bar
cargo build --release

# Install to ~/.cargo/bin
cargo install --path .
```

### Basic Usage

```bash
# Show current status (default command)
rs-claude-bar

# Status line for Claude Code integration  
rs-claude-bar prompt

# View 5-hour usage blocks
rs-claude-bar blocks

# Configure Claude data path
rs-claude-bar config claude-path
```

## 📋 Commands

### Core Commands

- `rs-claude-bar info` - Show basic usage information (default)
- `rs-claude-bar prompt` - Generate status line for Claude Code integration
- `rs-claude-bar install` - Configure Claude settings integration
- `rs-claude-bar blocks` - Display recent 5-hour usage blocks

### Block Analysis

- `rs-claude-bar blocks all` - Show all usage blocks from cache
- `rs-claude-bar blocks limits` - Display all limit/unlock events
- `rs-claude-bar blocks gaps` - Show usage gaps between blocks

### Configuration

- `rs-claude-bar config claude-path` - Set Claude data directory path
- `rs-claude-bar config display` - Configure display settings

### Global Options

- `--no-cache` - Force bypass cache and reprocess all files
- `--no-save` - Don't save cache after processing
- `--help` - Show help information
- `--version` - Show version information

## 🏗️ Architecture

**rs-claude-bar** is built with performance in mind:

- **JSONL Processing** - Incremental parsing of Claude transcript files
- **Cache System** - HashMap-based O(1) lookups with persistent storage in `~/.claude-bar/`
- **5-Hour Windows** - Advanced analysis of Claude's billing cycles
- **Configuration Management** - Automatic detection and custom path support
- **Display Engine** - Formatted output with color coding and progress indicators

## 📊 Status Line Integration

Perfect for Claude Code status bar hooks! The `rs-claude-bar prompt` command provides:

- Current token usage with progress indicators
- 5-hour window progress and remaining time  
- Active model detection (Sonnet 4, Opus 4, etc.)
- Limit warnings and status indicators
- Sub-100ms response time for smooth integration

Example status line output:
```
[███░░░░░░░] 36.0% • 18.7K/52.0K • 💬 227 • 3h 23m remaining • 🤖 Sonnet 4
```

## ⚙️ Configuration

**rs-claude-bar** automatically detects your Claude data directory:

- Default: `~/.claude/projects/` 
- Fallback: `~/.config/claude/projects/`
- Custom: Configure via `rs-claude-bar config claude-path`

Cache is stored in `~/.claude-bar/` for persistent performance optimization.

## 🔧 Development

### Requirements

- Rust 1.70+ (2021 edition)
- Cargo for building and dependency management

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with cargo
cargo run -- prompt
```

### Dependencies

- `serde_json` - JSONL parsing and serialization
- `chrono` - Date/time handling with timezone support
- `clap` - CLI argument parsing with derive macros
- `tabled` - Table formatting and display
- `dirs` - Cross-platform directory detection
- `regex` - Pattern matching for file processing

## 📈 Performance

**rs-claude-bar** is designed for speed:

- **Sub-100ms Response** - Typical execution in 27-78ms
- **Intelligent Caching** - Only processes changed files
- **Incremental Parsing** - Line-by-line JSONL processing
- **Memory Efficient** - Optimized data structures for large datasets
- **Release Optimization** - LTO, single codegen unit, panic=abort

## 🎯 Use Cases

- **Claude Code Status Bar** - Primary integration via `prompt` command
- **Usage Monitoring** - Track token consumption and 5-hour windows
- **Limit Detection** - Early warning for approaching usage limits
- **Session Analysis** - Understand usage patterns across projects
- **Performance Optimization** - Lightning-fast alternative to existing tools

## 🔍 Compared to ccusage

While **ccusage** (TypeScript/Node.js) offers comprehensive reporting features, **rs-claude-bar** focuses on:

- **Performance First** - 10x+ faster execution times
- **Status Line Optimized** - Built specifically for Claude Code integration  
- **Native Binary** - No runtime dependencies or installation complexity
- **Caching Excellence** - Advanced cache invalidation and persistence
- **Windows Focus** - Specialized 5-hour billing window analysis

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

## 🤝 Contributing

Contributions welcome! Please see our development workflow:

1. Fork the repository
2. Create a feature branch
3. Make your changes with tests
4. Run `cargo test` and `cargo build --release`
5. Submit a pull request

## 🙏 Acknowledgments

Inspired by [ccusage](https://github.com/ryoppippi/ccusage) by @ryoppippi - the comprehensive Claude Code usage analysis tool. **rs-claude-bar** focuses on high-performance status line integration while ccusage provides extensive reporting capabilities.

---

*Built with ❤️ in Rust for the Claude Code community*