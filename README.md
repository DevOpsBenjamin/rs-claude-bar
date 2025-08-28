# rs-claude-bar

Enhanced Claude Code usage tracker with 5-hour window monitoring, written in Rust.

A fast, lightweight alternative to CCUsage for tracking Claude Code token usage, session windows, and providing real-time status information in your Claude Code status bar.

Run `rs-claude-bar` with no arguments to view available commands.
Use `rs-claude-bar install` to configure Claude settings and `rs-claude-bar prompt` to display the status line.

## Features

- 🚀 **Fast**: Written in Rust, optimized for performance
- 🧠 **Smart**: Tracks token usage across 5-hour billing windows
- ⏱️ **Real-time**: Shows current session progress and time remaining
- 📊 **Detailed**: Provides comprehensive usage statistics
- 🛠️ **Zero Dependencies**: Single binary, no runtime requirements

## Installation

### Option 1: Cargo (for Rust developers)
```bash
cargo install rs-claude-bar
```

### Option 2: Binary Download (for everyone else)
```bash
# Install script (coming soon)
curl -fsSL https://install.rs-claude-bar.com | sh

# Or download from GitHub Releases
# https://github.com/your-username/rs-claude-bar/releases
```

## Configuration

Update your Claude Code settings (`~/.claude/settings.json`):

```json
{
  "statusLine": {
    "type": "command",
    "command": "rs-claude-bar prompt",
    "padding": 0
  }
}
```

Or run the built-in installer:

```bash
rs-claude-bar install
```

## Output Format

```
🧠 15,234 tokens (53.6%) 🟡 | 💬 124 | ⏱️ 2h15m | ⏰ 2h45m left | 🤖 Sonnet 4
```

- 🧠 **Tokens**: Current session token count with percentage of limit
- 💬 **Messages**: Number of messages in current session
- ⏱️ **Elapsed**: Time elapsed in current 5-hour window
- ⏰ **Remaining**: Time remaining in current window
- 🤖 **Model**: Current Claude model in use

## Status Indicators

- 🟢 **Green**: < 50% token usage
- 🟡 **Yellow**: 50-80% token usage  
- 🔴 **Red**: > 80% token usage

## Development

```bash
# Clone the repository
git clone https://github.com/your-username/rs-claude-bar
cd rs-claude-bar

# Build
cargo build --release

# Install locally
cargo install --path .

# Run tests
cargo test
```

## How It Works

`rs-claude-bar` reads Claude Code's JSONL usage files from:
- `~/.claude/projects/` (legacy location)
- `~/.config/claude/projects/` (new location)

It calculates 5-hour billing windows, tracks token usage, and maintains a cache in `~/.claude_bar/` for fast status line updates.

## Comparison with CCUsage

| Feature | rs-claude-bar | CCUsage |
|---------|---------------|---------|
| **Language** | Rust | TypeScript |
| **Speed** | ⚡ Very fast | Fast |
| **Dependencies** | None | Node.js |
| **Binary Size** | ~2MB | ~50MB+ (with Node) |
| **Memory Usage** | ~1-5MB | ~50MB+ |
| **Installation** | `cargo install` | `npm install -g` |

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [CCUsage](https://github.com/configurable-and-comprehensible/ccusage)
- Built for the Claude Code community