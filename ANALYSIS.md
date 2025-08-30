# rs-claude-bar – Repository Analysis (Aug 2025)

## Overview

- Purpose: Fast, CLI-friendly tracker for Claude Code usage with 5‑hour window analysis and a prompt-friendly status line.
- Language: Rust 2021. Binaries: `rs-claude-bar` (`src/main.rs`).
- CI: GitHub Actions builds and tests on Linux/macOS/Windows (`.github/workflows/ci.yml`).

## High‑Level Architecture

- `src/analyzer/` (helpers & orchestration)
  - Loads transcript JSONL entries (full scan or since timestamp), parses reset times, computes unlocks, constructs GuessBlocks and CurrentBlocks, and aggregates tokens into blocks.
- `src/cache/`
  - CacheManager for filesystem cache; provides cached folder/file views for quick debug and table output.
- `src/claudebar_types/`
  - Core types used across commands: `SimpleBlock`, `StatsFile`, `GuessBlock`, `CurrentBlock`, `AssistantInfo`, `UserInfo`, and usage entry structures.
- `src/commands/`
  - Subcommands: `info`, `help`, `prompt`, `install`, `config`, `blocks`, `debug`, `table`, `history`, `update`, `display-config`, `resets`.
- `src/common/`, `src/display/`, `src/utils/`
  - Colors, formatting, display items and status line composition.
- `src/config_manager/`
  - App dir, stats/config load/save, migration/defaults.

## Data Flow

- Input: Claude Code JSONL transcript files at `~/.claude/projects/*/*.jsonl`.
- Parse: `analyzer::load_entries_since` reads files, deserializes `TranscriptEntry`, converts to `ClaudeBarUsageEntry`.
- Detect 5h Windows:
  - Limit messages → `parse_reset_time` → `calculate_unlock_time`.
  - Each “limit hit” produces a completed 5h block: `[end-5h, end]`.
  - A projected block is created from the latest end → `end+5h` for “current/next window”.
- Aggregate:
  - `build_current_blocks_from_guess` creates a timeline: first gap, real blocks (+ gaps where needed), last gap. No gap is inserted when successive blocks touch (`start == prev.end`).
  - `aggregate_events_into_blocks` sums assistant/user content and all token types into blocks; tracks min/max timestamps per block.
- Persist:
  - `StatsFile { past, current, last_processed }` saved under `~/.claude-bar`.

## Commands Snapshot

- `info`, `help`: usage and onboarding.
- `prompt`: generates compact status line (emojis, tokens, block status).
- `install`: writes Claude Code settings for status line integration.
- `config`: manage data path and display settings.
- `blocks`: human summary of 5h windows; now reuses analyzer helpers for reset parsing/unlock.
- `resets`: builds GuessBlocks/CurrentBlocks, filters, and prints debug structures; also prints compact SimpleBlocks and slot stats.
- `debug`, `table`, `history`, `update`, `display-config`: developer and advanced views; several leverage cache for speed.

## Resets Analysis (Current Behavior)

- Produces three debug outputs:
  - GuessBlocks: Vec<`GuessBlock`> including one projected block (latest end → +5h).
  - CurrentBlocks: Vec<`CurrentBlock`> with aggregated counts and min/max timestamps; zero-activity gap blocks are elided in debug output.
  - SimpleBlocks: last 10 real blocks, chronological order (old → new): `{ start, end, assistant_output_tokens, assistant_input_tokens, projected }`; non‑projected blocks with zero in+out tokens are dropped.
- Slot Stats (UTC bands): 0–6, 6–12, 12–18, 18–24.
  - If a 5h block crosses a boundary, it contributes to both bands.
  - For each band, prints Input/Output/Total metrics: `count`, `mean`, `min`, `max`.

## Caching & Performance

- CacheManager scans and persists a normalized view of the transcript directory; powers fast debug/table listings without re‑reading disk.
- `main.rs` records phase timings (config, cache load, file refresh, analyze, exec, save) and writes the last execution report under `~/.claude-bar/last_exec`.

## CI/CD

- Workflow: `.github/workflows/ci.yml`.
  - Matrix: Ubuntu, macOS, Windows.
  - Toolchain: stable (dtolnay/rust-toolchain).
  - Cache: `Swatinem/rust-cache`.
  - Steps: debug+release builds, full tests with `--all-features --all-targets`.

## Quality Notes

- The analyzer is now a shared module (`src/analyzer`) and is reused by `blocks` and `resets`. This removes earlier duplication and ensures consistent limit parsing/unlock calculation.
- `TODO.md` and `NEW_TODO.md` document an ambitious roadmap; several items (status line completeness, session tracking, history/stats UIs) are planned or partially implemented.

## Gaps & Opportunities

- Tests: limited coverage. Add unit tests for:
  - `parse_reset_time`, `calculate_unlock_time` (edge cases, 12am/pm, minutes).
  - Block construction/adjoining logic (no middle gap when touching).
  - Aggregation correctness from synthetic entries.
- Stats Source of Truth:
  - Ensure all user‑facing commands read from `StatsFile` when possible; migrate remaining ad‑hoc scans to analyzer/cache.
- Display: finalize `prompt` to always reflect `StatsFile.current` and compute remaining time/tokens consistently.
- Windows/Timezones: current logic assumes UTC; confirm alignment with Claude timestamps and display expectations.

## Suggested Next Steps

- Consolidate analyzer APIs: expose a thin facade returning `{ guess_blocks, current_blocks, simple_blocks, slot_stats }` to reduce per‑command code.
- Add targeted tests under `tests/` for helpers and block math.
- Tighten warnings: remove unused imports; prefer explicit error types where feasible.
- Extend CI with `cargo fmt --check` and `clippy` (warn‑only) once code stabilizes.

## Build & Run

- Build: `cargo build --release`
- Default help: `rs-claude-bar`
- Status line (simple): `rs-claude-bar prompt`
- Resets analysis (debug): `rs-claude-bar resets`
- Table (cached): `rs-claude-bar table`

---

This report reflects the current repository state at the time of analysis and highlights architecture, command coverage, analyzer integration, and practical follow‑ups to reach a production‑ready v0.3.

