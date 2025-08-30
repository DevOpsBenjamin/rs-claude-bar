# TODO: rs-claude-bar Development Roadmap

## ✅ Completed (v0.1.0)

- [x] **Project Setup**
  - [x] Initialize Rust project with Cargo
  - [x] Set up basic dependencies
  - [x] Create project structure

- [x] **Minimal Working Version**
  - [x] Basic JSONL file discovery
  - [x] Simple entry counting
  - [x] Basic status line output
  - [x] Claude Code integration

- [x] **Installation & Distribution**
  - [x] Cargo build configuration
  - [x] Local installation working
  - [x] Claude settings integration

- [x] **Documentation**
  - [x] README.md
  - [x] CLAUDE.md (development guide)
  - [x] .gitignore
  - [x] This TODO.md

## 🚧 In Progress (v0.2.0)

### Core Features
- [x] **JSONL Parsing Enhancement** 
  - [x] Parse actual token counts from usage data
  - [x] Extract model information
  - [x] Handle timestamps properly
  - [x] Error handling for malformed JSONL
  - [x] Incremental parsing with boundary detection

- [x] **Cache System Architecture**
  - [x] HashMap-based storage for O(1) lookups
  - [x] PerHourBlock structures with comprehensive statistics
  - [x] BlockLine tracking for limit/unlock events
  - [x] Cache invalidation logic (Fresh, NeedsRefresh, NotInCache)

- [ ] **5-Hour Window Calculation**
  - [ ] Implement window detection logic (like CCUsage)
  - [ ] Calculate token usage per window
  - [ ] Track active vs completed windows
  - [ ] Handle overlapping sessions

### Status Line Improvements
- [ ] **Enhanced Output Format**
  - [ ] Show actual token counts (not just entry counts)
  - [ ] Display usage percentage with color coding
  - [ ] Calculate and show elapsed/remaining time
  - [ ] Format model names properly (Sonnet 4, Opus 4, etc.)

- [ ] **Visual Indicators**
  - [ ] 🟢 Green: < 50% usage
  - [ ] 🟡 Yellow: 50-80% usage  
  - [ ] 🔴 Red: > 80% usage

### Performance & Reliability
- [x] **Caching System**
  - [x] Implement ~/.claude_bar/ cache directory
  - [x] Cache parsed JSONL data with persistence
  - [x] Invalidate cache when files change
  - [x] Sub-100ms response time for status line (27-78ms achieved)
  - [x] 3x+ performance improvement from caching

### Debug & Development Tools
- [x] **Cache-Only Debug Commands**
  - [x] --limits: Show all limit events from cache
  - [x] --blocks: Show per-hour usage blocks from cache
  - [x] --gaps: Show usage gaps between blocks
  - [x] Zero filesystem access for debug operations

## 📋 Planned (v0.3.0)

### Advanced Features
- [ ] **Session Management**
  - [ ] Detect session boundaries
  - [ ] Track multiple concurrent projects
  - [ ] Session history and statistics

- [ ] **Cost Tracking**
  - [ ] Parse cost data from JSONL
  - [ ] Calculate running costs
  - [ ] Daily/weekly cost summaries

- [ ] **Configuration**
  - [ ] Config file support (~/.claude_bar/config.toml)
  - [ ] Customizable token limits
  - [ ] Output format options
  - [ ] Custom emoji/text preferences

### CLI Commands
- [ ] **Subcommands**
  - [ ] `rs-claude-bar status` (default)
  - [ ] `rs-claude-bar update` (force cache refresh)
  - [ ] `rs-claude-bar history` (show recent windows)
  - [ ] `rs-claude-bar stats` (detailed statistics)

## 🔮 Future (v0.4.0+)

### Distribution & Packaging
- [ ] **Multi-Platform Releases**
  - [ ] GitHub Actions CI/CD
  - [ ] Windows, macOS, Linux binaries
  - [ ] ARM64 support (Apple Silicon, etc.)

- [ ] **Package Managers**
  - [ ] Homebrew formula
  - [ ] Chocolatey package (Windows)
  - [ ] Scoop manifest (Windows)
  - [ ] AUR package (Arch Linux)

- [ ] **Install Script**
  - [ ] One-line installer: `curl -fsSL install.rs-claude-bar.com | sh`
  - [ ] Platform detection and binary download
  - [ ] Automatic PATH setup

### Advanced Analytics
- [ ] **Detailed Reporting**
  - [ ] HTML reports (like CCUsage)
  - [ ] JSON export for external tools
  - [ ] Usage trends and patterns
  - [ ] Model usage breakdown

- [ ] **Live Dashboard**
  - [ ] Web interface for usage monitoring
  - [ ] Real-time token burn rate
  - [ ] Session progress visualization

### Integration Features
- [ ] **Claude Desktop Integration**
  - [ ] Native plugin support (if available)
  - [ ] Menu bar integration (macOS)
  - [ ] System tray integration (Windows/Linux)

- [ ] **API & Webhooks**
  - [ ] REST API for usage data
  - [ ] Webhook notifications (approaching limits)
  - [ ] Integration with monitoring tools

## 🐛 Known Issues

- [ ] **Path Handling**
  - [ ] Better cross-platform path handling
  - [ ] Handle special characters in paths
  - [ ] Symlink resolution

- [ ] **Error Recovery**
  - [ ] Graceful handling of corrupted JSONL files
  - [ ] Recovery from cache corruption
  - [ ] Better error messages

## 🔧 Technical Debt

- [ ] **Code Organization**
  - [ ] Split main.rs into modules
  - [ ] Proper error types (vs Box<dyn Error>)
  - [ ] Unit tests for core functions

- [ ] **Documentation**
  - [ ] API documentation (rustdoc)
  - [ ] Usage examples
  - [ ] Performance benchmarks

## 💡 Ideas & Research

- [ ] **Performance Optimizations**
  - [ ] Incremental JSONL parsing (only new entries)
  - [ ] Memory-mapped files for large JSONL files
  - [ ] Background cache updates

- [ ] **Alternative Installations**
  - [ ] Docker container
  - [ ] WebAssembly version
  - [ ] Cloud function deployment

- [ ] **Compatibility**
  - [ ] Support for other AI usage tracking
  - [ ] Export to CCUsage format
  - [ ] Import from CCUsage cache

---

## Priority Legend
- 🚨 **Critical**: Blocking issues or essential features
- ⚡ **High**: Important for next release  
- 📊 **Medium**: Nice to have, good user experience
- 🔧 **Low**: Technical debt, optimizations
- 💡 **Research**: Ideas to explore

## Version Planning
- **v0.1.0**: ✅ Minimal working version
- **v0.2.0**: 🚧 Core features (JSONL parsing, 5h windows, enhanced output)
- **v0.3.0**: 📋 Advanced features (CLI, config, caching)
- **v0.4.0**: 🔮 Distribution & packaging
- **v1.0.0**: 🎯 Feature complete, production ready