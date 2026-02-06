//! Editor widget for rendering the text buffer

use four_code_core::Buffer;
use ratatui::{
    buffer::Buffer as RatatuiBuffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

/// Widget for rendering the editor content
pub struct EditorWidget<'a> {
    buffer: &'a Buffer,
    line_number_width: usize,
}

impl<'a> EditorWidget<'a> {
    /// Create a new editor widget
    pub fn new(buffer: &'a Buffer) -> Self {
        let line_count = buffer.len_lines();
        let line_number_width = line_count.to_string().len().max(3) + 1; // +1 for padding

        Self {
            buffer,
            line_number_width,
        }
    }
}

impl Widget for EditorWidget<'_> {
    fn render(self, area: Rect, buf: &mut RatatuiBuffer) {
        let line_num_style = Style::default().fg(Color::DarkGray);
        let text_style = Style::default().fg(Color::White);

        for (i, y) in (area.y..area.y + area.height).enumerate() {
            if let Some(line) = self.buffer.line(i) {
                // Line number
                let line_num = format!("{:>width$} ", i + 1, width = self.line_number_width - 1);
                buf.set_string(area.x, y, &line_num, line_num_style);

                // Line content
                let content_x = area.x + self.line_number_width as u16;
                let available_width = area.width.saturating_sub(self.line_number_width as u16);

                let line_str = line.to_string();
                let display_str: String = line_str
                    .chars()
                    .take(available_width as usize)
                    .filter(|c| *c != '\n' && *c != '\r')
                    .collect();

                buf.set_string(content_x, y, &display_str, text_style);
            } else {
                // Empty line indicator
                let tilde = format!("{:>width$}~", "", width = self.line_number_width - 1);
                buf.set_string(area.x, y, &tilde, Style::default().fg(Color::DarkGray));
            }
        }
    }
}
