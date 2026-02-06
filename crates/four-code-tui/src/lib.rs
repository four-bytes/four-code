//! four-code-tui: Terminal UI for four-code
//!
//! This crate provides the TUI layer using ratatui:
//! - Application state and event loop
//! - Panel rendering (editor, file tree, etc.)
//! - Keymap handling

mod app;
mod editor;

pub use app::App;
pub use editor::EditorWidget;
