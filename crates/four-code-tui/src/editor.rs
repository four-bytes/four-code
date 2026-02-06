//! Editor widget for rendering the text buffer

use four_code_core::Editor;
use ratatui::{
    buffer::Buffer as RatatuiBuffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::Widget,
};

/// Widget for rendering the editor content
pub struct EditorWidget<'a> {
    editor: &'a Editor,
    line_number_width: usize,
}

impl<'a> EditorWidget<'a> {
    /// Create a new editor widget
    pub fn new(editor: &'a Editor) -> Self {
        let line_count = editor.buffer.len_lines();
        let line_number_width = line_count.to_string().len().max(3) + 1; // +1 for padding

        Self {
            editor,
            line_number_width,
        }
    }

    /// Check if a position is within selection
    fn is_selected(&self, line: usize, col: usize) -> bool {
        if let Some((start, end)) = self.editor.cursor.selection_range() {
            if line < start.line || line > end.line {
                return false;
            }
            if line == start.line && line == end.line {
                // Selection on single line
                col >= start.column && col < end.column
            } else if line == start.line {
                // First line of multi-line selection
                col >= start.column
            } else if line == end.line {
                // Last line of multi-line selection
                col < end.column
            } else {
                // Middle line of multi-line selection
                true
            }
        } else {
            false
        }
    }
}

impl Widget for EditorWidget<'_> {
    fn render(self, area: Rect, buf: &mut RatatuiBuffer) {
        let line_num_style = Style::default().fg(Color::DarkGray);
        let current_line_num_style = Style::default().fg(Color::Yellow);
        let text_style = Style::default().fg(Color::White);
        let selection_style = Style::default()
            .bg(Color::Blue)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD);

        let viewport = &self.editor.viewport;
        let cursor_line = self.editor.cursor.position.line;

        for (screen_row, y) in (area.y..area.y + area.height).enumerate() {
            let buffer_line = viewport.top_line + screen_row;

            if let Some(line) = self.editor.buffer.line(buffer_line) {
                // Line number - highlight current line
                let num_style = if buffer_line == cursor_line {
                    current_line_num_style
                } else {
                    line_num_style
                };
                let line_num = format!(
                    "{:>width$} ",
                    buffer_line + 1,
                    width = self.line_number_width - 1
                );
                buf.set_string(area.x, y, &line_num, num_style);

                // Line content with selection highlighting
                let content_x = area.x + self.line_number_width as u16;
                let available_width =
                    area.width.saturating_sub(self.line_number_width as u16) as usize;

                let line_str = line.to_string();
                for (col, ch) in line_str
                    .chars()
                    .take(available_width)
                    .filter(|c| *c != '\n' && *c != '\r')
                    .enumerate()
                {
                    let style = if self.is_selected(buffer_line, col) {
                        selection_style
                    } else {
                        text_style
                    };
                    let x = content_x + col as u16;
                    if x < area.x + area.width {
                        buf.set_string(x, y, ch.to_string(), style);
                    }
                }

                // If selection extends beyond line content, show it
                let line_len = line_str
                    .chars()
                    .filter(|c| *c != '\n' && *c != '\r')
                    .count();
                if self.is_selected(buffer_line, line_len) && line_len < available_width {
                    // Selection includes the newline - show a space with selection bg
                    let x = content_x + line_len as u16;
                    if x < area.x + area.width {
                        buf.set_string(x, y, " ", selection_style);
                    }
                }
            } else {
                // Empty line indicator (beyond end of file)
                let tilde = format!("{:>width$}~", "", width = self.line_number_width - 1);
                buf.set_string(area.x, y, &tilde, Style::default().fg(Color::DarkGray));
            }
        }
    }
}
