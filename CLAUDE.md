# four-code - Claude Code Configuration

## Project Overview

**four-code** is a minimalist terminal IDE for PHP developers with AI integration.

- **Language:** Rust
- **Status:** Early development
- **Org:** four-bytes (Four\ namespace)

## Tech Stack

- **TUI:** ratatui
- **Editor:** ropey (rope data structure)
- **Syntax:** tree-sitter
- **LSP:** tower-lsp client
- **Git:** git2-rs
- **Config:** TOML
- **AI:** reqwest for API calls

## Architecture

Cargo workspace with multiple crates:

```
crates/
├── four-code/          # Main binary
├── four-code-core/     # Editor core (buffer, cursor, selection)
├── four-code-tui/      # Terminal UI
├── four-code-lsp/      # LSP client
├── four-code-ai/       # AI providers
└── four-code-git/      # Git operations
```

## Key Design Decisions

1. **ropey + tree-sitter** - Own editor implementation, not a fork
2. **No plugins in MVP** - Everything compiled-in for simplicity
3. **Keymap presets** - VS Code, PhpStorm, mcedit (user-selectable)
4. **AI: Chat + Context** - No inline completion in MVP
5. **EU-First AI** - Mistral as default, local models supported

## Development Commands

```bash
cargo build                    # Debug build
cargo build --release          # Release build
cargo test                     # Run tests
cargo clippy                   # Lint
cargo fmt                      # Format
```

## Files to Know

- `docs/ARCHITECTURE.md` - System architecture
- `docs/ROADMAP.md` - Feature roadmap
- `crates/four-code/src/main.rs` - Entry point

## Conventions

- Use `thiserror` for error types
- Async with `tokio`
- Logging with `tracing`
- Tests next to implementation (`#[cfg(test)]`)
