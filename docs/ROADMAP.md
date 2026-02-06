# Roadmap

## Phase 1: MVP (v0.1) - ~6 Weeks

**Goal:** Usable editor for PHP development with basic AI chat.

### Week 1-2: Editor Core
- [ ] Project setup (Cargo workspace)
- [ ] ropey buffer implementation
- [ ] Cursor and selection
- [ ] Basic editing (insert, delete, undo/redo)
- [ ] File open/save

### Week 2-3: TUI & Syntax
- [ ] ratatui setup
- [ ] Editor widget with line numbers
- [ ] tree-sitter integration
- [ ] PHP syntax highlighting
- [ ] JS/JSON/YAML/Markdown highlighting

### Week 3-4: Navigation
- [ ] File tree sidebar
- [ ] Fuzzy file finder (Ctrl+P)
- [ ] Go to line
- [ ] Search in file
- [ ] Search in project

### Week 4-5: Intelligence
- [ ] LSP client setup
- [ ] Intelephense integration
- [ ] Completion popup
- [ ] Go to definition
- [ ] Hover documentation
- [ ] Diagnostics display

### Week 5-6: AI & Polish
- [ ] AI chat panel
- [ ] Mistral/Claude provider
- [ ] Context sending (current file, selection)
- [ ] Keymap presets (VS Code, PhpStorm, mcedit)
- [ ] Config file support
- [ ] Error handling & stability

### MVP Deliverable
- Single binary for Linux (x86_64, aarch64)
- Basic PHP editing with LSP
- AI chat with code context
- Three keymap presets

---

## Phase 2: Enhanced Editing (v0.2) - ~4 Weeks

**Goal:** Multi-file workflow, more languages.

- [ ] Split views (horizontal/vertical)
- [ ] Tabs / buffer list
- [ ] Multiple cursors
- [ ] More AI providers (Ollama, OpenAI)
- [ ] AI inline suggestions (basic)
- [ ] More languages (Python, Rust, Go)
- [ ] Snippet support
- [ ] Bracket matching & auto-pairs

---

## Phase 3: GUI & Plugins (v0.3) - ~6 Weeks

**Goal:** Optional GUI, extensibility.

- [ ] Tauri-based GUI (Windows/macOS/Linux)
- [ ] WASM plugin system design
- [ ] Plugin API specification
- [ ] First plugins: additional themes, languages
- [ ] Settings UI
- [ ] Minimap

---

## Phase 4: Advanced Features (v0.4) - ~6 Weeks

**Goal:** Full IDE experience.

- [ ] Database browser (MySQL, PostgreSQL)
- [ ] GitHub integration (issues, PRs)
- [ ] Debugger integration (Xdebug for PHP)
- [ ] Terminal multiplexer (built-in tmux-like)
- [ ] Project templates (Symfony, Laravel, Shopware)

---

## Phase 5: Enterprise & Polish (v0.5+)

**Goal:** Production-ready for teams.

- [ ] Remote development (SSH)
- [ ] Collaboration features
- [ ] On-premise AI (for enterprises)
- [ ] Performance profiling tools
- [ ] Documentation generation
- [ ] Audit logging (compliance)

---

## Version Milestones

| Version | Target | Key Features |
|---------|--------|--------------|
| v0.1 | Q1 2026 | MVP - Basic editing, LSP, AI chat |
| v0.2 | Q2 2026 | Multi-file, more languages |
| v0.3 | Q3 2026 | GUI, plugin system |
| v0.4 | Q4 2026 | Database, GitHub, debugger |
| v1.0 | 2027 | Production-ready |

---

## Non-Goals (Intentionally Out of Scope)

- **Full Vim emulation** - We offer Helix-inspired modal editing, not Vim
- **Electron-based** - Terminal-first, Tauri for GUI
- **Plugin marketplace** - Curated plugins only (quality over quantity)
- **Visual designers** - No drag-and-drop UI builders
- **Notebook interface** - We're an editor, not Jupyter
