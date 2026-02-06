//! Main application state and event loop

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use four_code_core::Editor;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;
use thiserror::Error;

use crate::EditorWidget;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Terminal error: {0}")]
    Terminal(String),
}

/// Application state
pub struct App {
    /// Editor instance
    editor: Editor,

    /// Whether the app should quit
    should_quit: bool,

    /// Status message
    status: String,

    /// Last terminal size
    last_size: (u16, u16),
}

impl App {
    /// Create a new app instance
    pub fn new() -> Self {
        Self {
            editor: Editor::with_content(
                "Welcome to four-code!\n\n\
                 A minimalist terminal IDE for PHP developers.\n\n\
                 Keybindings:\n\
                 - Arrow keys: Move cursor\n\
                 - Ctrl+Q: Quit\n\
                 - Ctrl+S: Save\n\
                 - Home/End: Start/End of line\n\
                 - Ctrl+Home/End: Start/End of document\n\
                 - Page Up/Down: Scroll\n\n\
                 Start typing to edit...\n",
            ),
            should_quit: false,
            status: String::from("four-code v0.1.0 | Ctrl+Q: Quit | Ctrl+S: Save"),
            last_size: (0, 0),
        }
    }

    /// Create app with a file
    pub fn with_file(path: &str) -> Result<Self, AppError> {
        let editor = Editor::open(path).map_err(|e| AppError::Terminal(e.to_string()))?;
        let status = format!("Opened: {path}");
        Ok(Self {
            editor,
            should_quit: false,
            status,
            last_size: (0, 0),
        })
    }

    /// Run the application
    pub fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    ) -> Result<(), AppError> {
        while !self.should_quit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// Render the UI
    fn render(&mut self, frame: &mut Frame) {
        let size = frame.area();

        // Update viewport size if terminal resized
        if (size.width, size.height) != self.last_size {
            self.last_size = (size.width, size.height);
            // Account for borders and status bar
            let editor_height = size.height.saturating_sub(3) as usize;
            let editor_width = size.width.saturating_sub(2) as usize;
            self.editor.set_viewport_size(editor_height, editor_width);
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),    // Editor
                Constraint::Length(1), // Status bar
            ])
            .split(size);

        // Editor title with filename and modified indicator
        let title = if self.editor.is_modified() {
            format!(" {} [+] ", self.editor.filename())
        } else {
            format!(" {} ", self.editor.filename())
        };

        let editor_block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().fg(Color::Cyan));

        // Get inner area before rendering the block
        let inner = editor_block.inner(chunks[0]);
        frame.render_widget(editor_block, chunks[0]);

        // Render editor content
        let editor_widget = EditorWidget::new(&self.editor);
        frame.render_widget(editor_widget, inner);

        // Set cursor position
        let cursor_x = inner.x + self.editor.cursor.position.column as u16 + 4; // +4 for line numbers
        let cursor_y =
            inner.y + (self.editor.cursor.position.line - self.editor.viewport.top_line) as u16;

        if cursor_y >= inner.y && cursor_y < inner.y + inner.height {
            frame.set_cursor_position((cursor_x.min(inner.x + inner.width - 1), cursor_y));
        }

        // Status bar with position info
        let pos_info = format!(
            "Ln {}, Col {} | {}",
            self.editor.cursor.position.line + 1,
            self.editor.cursor.position.column + 1,
            &self.status
        );
        let status =
            Paragraph::new(pos_info).style(Style::default().fg(Color::White).bg(Color::DarkGray));
        frame.render_widget(status, chunks[1]);
    }

    /// Handle input events
    fn handle_events(&mut self) -> Result<(), AppError> {
        if event::poll(std::time::Duration::from_millis(16))? {
            // ~60 FPS
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            }
        }
        Ok(())
    }

    /// Handle a key event
    fn handle_key(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            // === Application Commands ===

            // Quit
            (KeyModifiers::CONTROL, KeyCode::Char('q')) => {
                self.should_quit = true;
            }

            // Save
            (KeyModifiers::CONTROL, KeyCode::Char('s')) => match self.editor.save() {
                Ok(()) => {
                    if let Some(path) = self.editor.path() {
                        self.status = format!("Saved: {}", path.display());
                    } else {
                        self.status = String::from("No file path. Use :w <path> to save.");
                    }
                }
                Err(e) => {
                    self.status = format!("Error: {e}");
                }
            },

            // === Cursor Movement ===

            // Arrow keys
            (KeyModifiers::NONE, KeyCode::Up) => self.editor.move_up(),
            (KeyModifiers::NONE, KeyCode::Down) => self.editor.move_down(),
            (KeyModifiers::NONE, KeyCode::Left) => self.editor.move_left(),
            (KeyModifiers::NONE, KeyCode::Right) => self.editor.move_right(),

            // Home/End
            (KeyModifiers::NONE, KeyCode::Home) => self.editor.move_to_line_start(),
            (KeyModifiers::NONE, KeyCode::End) => self.editor.move_to_line_end(),

            // Ctrl+Home/End - document start/end
            (KeyModifiers::CONTROL, KeyCode::Home) => self.editor.move_to_start(),
            (KeyModifiers::CONTROL, KeyCode::End) => self.editor.move_to_end(),

            // Page Up/Down
            (KeyModifiers::NONE, KeyCode::PageUp) => self.editor.page_up(),
            (KeyModifiers::NONE, KeyCode::PageDown) => self.editor.page_down(),

            // === Text Editing ===

            // Enter
            (KeyModifiers::NONE, KeyCode::Enter) => self.editor.insert_newline(),

            // Backspace
            (KeyModifiers::NONE, KeyCode::Backspace) => self.editor.backspace(),

            // Delete
            (KeyModifiers::NONE, KeyCode::Delete) => self.editor.delete(),

            // Tab
            (KeyModifiers::NONE, KeyCode::Tab) => {
                // Insert 4 spaces (configurable later)
                self.editor.insert_str("    ");
            }

            // Regular character input
            (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(c)) => {
                self.editor.insert_char(c);
            }

            _ => {}
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
