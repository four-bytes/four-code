# four-code - Claude Code Configuration

## Project Overview

**four-code** is a minimalist terminal IDE for PHP developers with AI integration.

- **Language:** Rust
- **Status:** Early development (working editor prototype)
- **Org:** four-bytes (Four\ namespace)

## Tech Stack

- **TUI:** ratatui 0.30
- **Editor:** ropey (rope data structure)
- **Syntax:** tree-sitter (planned)
- **LSP:** tower-lsp client (planned)
- **Git:** git2-rs (planned)
- **Config:** TOML
- **AI:** reqwest for API calls (planned)

## Architecture

Cargo workspace with multiple crates:

```
crates/
├── four-code/          # Main binary (entry point)
├── four-code-core/     # Editor core (Buffer, Cursor, Editor, Viewport)
├── four-code-tui/      # Terminal UI (App, EditorWidget)
├── four-code-lsp/      # LSP client (planned)
├── four-code-ai/       # AI providers (planned)
└── four-code-git/      # Git operations (planned)
```

## Key Design Decisions

1. **ropey + tree-sitter** - Own editor implementation, not a fork
2. **No plugins in MVP** - Everything compiled-in for simplicity
3. **Keymap presets** - VS Code, PhpStorm, mcedit (user-selectable)
4. **AI: Chat + Context** - No inline completion in MVP
5. **EU-First AI** - Mistral as default, local models supported

## Development Workflow

### Quick Commands (Makefile)

```bash
make              # Full check: fmt + lint + test
make check        # Quick syntax check (fastest)
make test         # Run all tests
make lint         # Clippy (like PHPStan max level)
make fmt          # Format code (like PSR-12)
make run          # Run the editor
make release      # Build optimized binary
make ci           # Full CI check
```

### Rust Tools (PHP Equivalents)

| PHP Tool | Rust Equivalent | Command |
|----------|-----------------|---------|
| PHPStan/Psalm | clippy | `cargo clippy -- -D warnings` |
| PSR-12/php-cs-fixer | rustfmt | `cargo fmt` |
| PHPUnit | cargo test | `cargo test` |
| Composer | cargo | `cargo build` |

### IMPORTANT: Always run before committing

```bash
make ci   # or: cargo fmt && cargo clippy -- -D warnings && cargo test
```

## Files to Know

- `docs/ARCHITECTURE.md` - System architecture
- `docs/ROADMAP.md` - Feature roadmap
- `crates/four-code/src/main.rs` - Entry point
- `crates/four-code-core/src/editor.rs` - Main editor logic
- `rustfmt.toml` - Formatting config
- `clippy.toml` - Linter config

## Conventions

### Code Style
- Use `thiserror` for error types
- Async with `tokio`
- Logging with `tracing`
- Tests next to implementation (`#[cfg(test)]`)
- All clippy warnings must pass (`-D warnings`)

### Naming
- `Buffer::with_content()` not `from_str()` (clippy rule)
- Use inline format strings: `format!("{x}")` not `format!("{}", x)`

### Testing
- Write tests for all public functions
- Tests go in `#[cfg(test)] mod tests { }` at bottom of file
- Use descriptive test names: `test_backspace_join_lines`

## Current State

- **Working:** Basic text editing, cursor movement, viewport scrolling
- **Tests:** 16 passing
- **Binary size:** 2.2MB (release)
