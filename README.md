# four-code

Minimalist terminal IDE for PHP developers with AI integration.

> **Status:** Early Development - Not yet functional

## Vision

A lightweight, terminal-native IDE that brings JetBrains-level productivity to the command line:

- **~8MB binary** - No bloat, instant startup (~3ms)
- **AI-first** - Chat with context, EU-compliant providers (Mistral, local models)
- **PHP-focused** - First-class support for Symfony, Laravel, Shopware
- **Portable** - Single binary, runs anywhere (WSL2, servers, Docker)

## Why?

| Problem | four-code Solution |
|---------|-------------------|
| PhpStorm uses 2GB+ RAM | ~30MB RAM |
| VS Code Remote is laggy | Native terminal, zero latency |
| Neovim config takes weeks | Works out of the box |
| AI tools send code to US | EU-first providers, local models |

## Planned Features (MVP)

- [ ] Editor core (ropey + tree-sitter)
- [ ] Syntax highlighting (PHP, JS, JSON, YAML, Markdown)
- [ ] File tree with fuzzy finder
- [ ] LSP integration (Intelephense/Phpactor)
- [ ] Git integration (status, diff, stage, commit)
- [ ] AI chat with code context
- [ ] Configurable keymaps (VS Code / PhpStorm / mcedit)

## Tech Stack

- **Language:** Rust
- **TUI:** ratatui
- **Editor:** ropey (rope data structure)
- **Syntax:** tree-sitter
- **LSP:** tower-lsp client
- **Git:** git2-rs
- **Config:** TOML

## Installation

```bash
# Not yet available - coming soon
curl -L https://four-code.dev/install | sh
```

## Development

```bash
# Prerequisites: Rust toolchain
cargo build --release
./target/release/four-code .
```

## Keymaps

four-code supports multiple keymap presets:

| Preset | Style | Target Users |
|--------|-------|--------------|
| `vscode` | Ctrl+S, Ctrl+P, Ctrl+Shift+F | VS Code users |
| `phpstorm` | Ctrl+Shift+N, Alt+Enter | JetBrains users |
| `mcedit` | F2 save, F10 quit | Midnight Commander users |

Configure in `~/.config/four-code/config.toml`:

```toml
[keymap]
preset = "vscode"  # or "phpstorm", "mcedit"
```

## AI Integration

EU-first approach with multiple provider support:

```toml
[ai]
# EU Providers (GDPR-compliant)
provider = "mistral"  # or "ionos", "local"

# US Providers (optional)
# provider = "claude"
# provider = "openai"

# Local (maximum privacy)
# provider = "ollama"
# model = "codellama:13b"
```

## License

MIT

## Links

- [Architecture](docs/ARCHITECTURE.md)
- [Roadmap](docs/ROADMAP.md)
- [Contributing](CONTRIBUTING.md)
