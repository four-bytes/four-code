# four-code History

## [Unreleased]

### 2026-02-06 - Syntax Highlighting with tree-sitter
- Added four-code-highlight crate with tree-sitter integration
- Upgraded tree-sitter from 0.24 to 0.26 (supports language version 15)
- Implemented highlighting for 11 languages:
  - PHP (primary), JavaScript, TypeScript, JSON, HTML, CSS
  - YAML, TOML, Bash, Markdown, Rust
- Language auto-detection from file extension and shebang
- Highlight caching per-line for performance
- One Dark inspired color scheme
- Integrated highlighting into EditorWidget
- 10 new tests for highlighting module
- Release binary size: 12MB (with all grammar parsers)

### 2026-02-06 - Working Editor Prototype
- Implemented Cargo workspace with 3 crates
- four-code-core: Buffer (ropey), Cursor with selection, 9 unit tests
- four-code-tui: Basic TUI with ratatui 0.30, welcome screen
- four-code: Main binary, Ctrl+Q to quit
- Release binary size: 2.2MB
- Learned from termide project (similar tech stack, 31 crates)
- Updated dependencies to match termide versions for compatibility:
  - ratatui 0.30, crossterm 0.28, ropey 1.6
  - Added: arboard, unicode-width, notify, similar

### 2026-02-06 - Project Initialization
- Created GitHub repository four-bytes/four-code
- Defined tech stack: Rust, ratatui, ropey, tree-sitter
- Documented architecture and roadmap
- Key decisions:
  - Own editor engine (not fork)
  - No plugin system in MVP
  - Configurable keymaps (VS Code/PhpStorm/mcedit)
  - AI: Chat with context (Mistral/Claude)
  - EU-first AI provider strategy
