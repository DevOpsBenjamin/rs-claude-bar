# Repository Guidelines

## Project Structure & Module Organization
- `src/`: Rust source code (entry: `src/main.rs`).
- `Cargo.toml`: Package metadata, dependencies, and build config.
- `README.md`: Project overview and quickstart.
- `TODO.md`: Open tasks and follow-ups.
- `CLAUDE.md`: Agent-specific notes for this repo.
- `LICENSE`: Licensing terms.
- Tests live alongside modules in `#[cfg(test)] mod tests` or as integration tests under `tests/`.

## Build, Test, and Development Commands
- `cargo build`: Compile in debug mode.
- `cargo run -- <args>`: Build and run the binary locally.
- `cargo test`: Run unit/integration tests.
- `cargo fmt --all`: Format code with rustfmt.
- `cargo clippy --all-targets --all-features -D warnings`: Lint and deny warnings.

## Coding Style & Naming Conventions
- Use rustfmt defaults; 4-space indentation.
- Naming: `snake_case` for functions/modules, `CamelCase` for types, `SCREAMING_SNAKE_CASE` for constants.
- Keep functions small and focused; prefer explicit over clever.
- Document non-obvious logic with concise comments and `///` doc comments for public items.

## Testing Guidelines
- Write unit tests in `#[cfg(test)]` modules within the same file; use integration tests in `tests/` for public APIs.
- Name tests descriptively (e.g., `handles_empty_input`, `parses_valid_config`).
- Prefer deterministic tests; avoid external network or time dependencies without fakes.
- Aim for coverage of core flows and error cases before adding features.

## Commit & Pull Request Guidelines
- Commits: imperative, concise subject (≤ 50 chars), with context in the body when helpful. Example: `fix: handle empty args in parser`.
- Reference issues with `Closes #123` when applicable.
- Before PR: run `cargo fmt`, `cargo clippy -D warnings`, and `cargo test`.
- PRs should include: purpose, summary of changes, testing steps, and any logs or screenshots relevant to behavior.

## Security & Configuration Tips
- Do not commit secrets or tokens. Use environment variables and `.env` entries only locally; never commit `.env`.
- Validate and sanitize any user-provided input paths or flags in the CLI.

