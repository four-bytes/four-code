//! Main application state and event loop

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use four_code_core::Buffer;
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
    /// Current buffer
    buffer: Buffer,

    /// Whether the app should quit
    should_quit: bool,

    /// Status message
    status: String,
}

impl App {
    /// Create a new app instance
    pub fn new() -> Self {
        Self {
            buffer: Buffer::from_str(
                "Welcome to four-code!\n\nPress Ctrl+Q to quit.\nPress Ctrl+O to open a file.",
            ),
            should_quit: false,
            status: String::from("four-code v0.1.0 | Ctrl+Q: Quit"),
        }
    }

    /// Create app with a file
    pub fn with_file(path: &str) -> Result<Self, AppError> {
        let buffer = Buffer::from_file(path).map_err(|e| AppError::Terminal(e.to_string()))?;
        let status = format!("Opened: {}", path);
        Ok(Self {
            buffer,
            should_quit: false,
            status,
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
    fn render(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),    // Editor
                Constraint::Length(1), // Status bar
            ])
            .split(frame.area());

        // Editor area
        let editor_block = Block::default().borders(Borders::ALL).title(" four-code ");

        // Get inner area before rendering the block
        let inner = editor_block.inner(chunks[0]);
        frame.render_widget(editor_block, chunks[0]);

        // Render editor content inside the block
        let editor = EditorWidget::new(&self.buffer);
        frame.render_widget(editor, inner);

        // Status bar
        let status = Paragraph::new(self.status.as_str())
            .style(Style::default().fg(Color::White).bg(Color::DarkGray));
        frame.render_widget(status, chunks[1]);
    }

    /// Handle input events
    fn handle_events(&mut self) -> Result<(), AppError> {
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            }
        }
        Ok(())
    }

    /// Handle a key event
    fn handle_key(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            // Quit
            (KeyModifiers::CONTROL, KeyCode::Char('q')) => {
                self.should_quit = true;
            }

            // Save
            (KeyModifiers::CONTROL, KeyCode::Char('s')) => {
                if let Err(e) = self.buffer.save() {
                    self.status = format!("Error saving: {}", e);
                } else if let Some(path) = self.buffer.path() {
                    self.status = format!("Saved: {}", path.display());
                } else {
                    self.status = String::from("No file path set. Use Ctrl+Shift+S for Save As.");
                }
            }

            // TODO: Add more keybindings
            _ => {}
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
