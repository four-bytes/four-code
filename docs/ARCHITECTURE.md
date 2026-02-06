# Architecture

## Overview

```
┌──────────────────────────────────────────────────────────────┐
│  four-code                                                    │
├──────────────────────────────────────────────────────────────┤
│                         UI Layer                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐        │
│  │   TUI        │  │  Keymaps     │  │  Themes      │        │
│  │  (ratatui)   │  │  (presets)   │  │  (colors)    │        │
│  └──────────────┘  └──────────────┘  └──────────────┘        │
├──────────────────────────────────────────────────────────────┤
│                       Editor Layer                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐        │
│  │  Buffer      │  │  Cursor      │  │  Selection   │        │
│  │  (ropey)     │  │  (multi)     │  │  (ranges)    │        │
│  └──────────────┘  └──────────────┘  └──────────────┘        │
├──────────────────────────────────────────────────────────────┤
│                     Intelligence Layer                        │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐        │
│  │  tree-sitter │  │  LSP Client  │  │  AI Client   │        │
│  │  (syntax)    │  │  (tower-lsp) │  │  (providers) │        │
│  └──────────────┘  └──────────────┘  └──────────────┘        │
├──────────────────────────────────────────────────────────────┤
│                      Foundation Layer                         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐        │
│  │  File System │  │  Git         │  │  Config      │        │
│  │  (async)     │  │  (git2-rs)   │  │  (toml)      │        │
│  └──────────────┘  └──────────────┘  └──────────────┘        │
└──────────────────────────────────────────────────────────────┘
```

## Crate Structure

```
four-code/
├── Cargo.toml              # Workspace
├── crates/
│   ├── four-code/          # Main binary
│   ├── four-code-core/     # Editor core (buffer, cursor, selection)
│   ├── four-code-tui/      # Terminal UI (ratatui widgets)
│   ├── four-code-lsp/      # LSP client
│   ├── four-code-ai/       # AI provider abstraction
│   └── four-code-git/      # Git operations
└── docs/
```

## Key Design Decisions

### 1. Editor Engine: ropey

We use [ropey](https://github.com/cessen/ropey) for the text buffer:

- **Rope data structure** - O(log n) for inserts/deletes anywhere
- **Unicode-aware** - Proper grapheme cluster handling
- **Line indexing** - Fast line-based operations
- **Memory efficient** - Handles large files (100MB+)

### 2. Syntax Highlighting: tree-sitter

[tree-sitter](https://tree-sitter.github.io/tree-sitter/) for parsing:

- **Incremental parsing** - Only re-parse changed regions
- **Error recovery** - Works with incomplete/invalid code
- **Query-based highlighting** - Consistent across languages
- **Industry standard** - Same as Helix, Zed, Neovim

Supported languages (MVP):
- PHP (+ Blade, Twig via injections)
- JavaScript/TypeScript
- JSON, YAML, TOML
- Markdown
- HTML, CSS

### 3. LSP Integration

Standard Language Server Protocol:

```
four-code <--JSON-RPC--> Language Server (Intelephense)
```

Features:
- Completion
- Go to definition
- Find references
- Hover documentation
- Diagnostics (errors/warnings)

### 4. Keymaps: Preset-based

Rather than full customization, we offer curated presets:

```rust
enum KeymapPreset {
    VSCode,     // Familiar to most developers
    PhpStorm,   // JetBrains users
    Mcedit,     // Terminal purists
}
```

Each preset maps logical actions to keys:

```rust
enum Action {
    Save,
    Find,
    FindInFiles,
    GoToFile,
    GoToDefinition,
    // ...
}
```

### 5. AI Architecture

Provider-agnostic design:

```rust
trait AiProvider {
    async fn chat(&self, messages: Vec<Message>, context: CodeContext) -> Result<String>;
    async fn complete(&self, prompt: &str, context: CodeContext) -> Result<String>;
}

struct CodeContext {
    file_path: PathBuf,
    file_content: String,
    selection: Option<Range>,
    language: String,
    project_files: Vec<PathBuf>,  // For broader context
}
```

Providers (MVP):
- `MistralProvider` - EU, Codestral model
- `ClaudeProvider` - Anthropic API
- `OllamaProvider` - Local models

## Data Flow

### Keystroke → Action

```
1. Terminal event (crossterm)
2. Keymap lookup (preset-based)
3. Action dispatch
4. State mutation
5. UI re-render (ratatui)
```

### File Open → Syntax Highlighted

```
1. Read file (tokio::fs)
2. Create ropey buffer
3. Parse with tree-sitter
4. Apply highlight queries
5. Render with colors
```

### LSP Request

```
1. User action (e.g., hover)
2. Build LSP request
3. Send via JSON-RPC
4. Await response
5. Display in UI (popup/panel)
```

## Performance Targets

| Metric | Target | Rationale |
|--------|--------|-----------|
| Startup time | <50ms | Instant feel |
| Keystroke latency | <16ms | 60fps responsiveness |
| Memory (empty) | <20MB | Lightweight |
| Memory (large project) | <100MB | Reasonable for IDE |
| Binary size | <10MB | Easy distribution |

## Future: Plugin System

Post-MVP, we plan WASM-based plugins:

```
┌─────────────────┐     ┌─────────────────┐
│  four-code      │     │  Plugin (WASM)  │
│  (host)         │────▶│  (sandboxed)    │
└─────────────────┘     └─────────────────┘
        │
        ▼
  Plugin API (wit-bindgen)
```

Benefits:
- Language-agnostic (Rust, Go, JS can compile to WASM)
- Sandboxed execution
- No ABI compatibility issues
