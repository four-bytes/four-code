//! Editor widget for rendering the text buffer with syntax highlighting

use four_code_core::Editor;
use four_code_highlight::HighlightCache;
use ratatui::{
    buffer::Buffer as RatatuiBuffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::Widget,
};

/// Widget for rendering the editor content
pub struct EditorWidget<'a> {
    editor: &'a Editor,
    highlight_cache: &'a mut HighlightCache,
    line_number_width: usize,
}

impl<'a> EditorWidget<'a> {
    /// Create a new editor widget
    pub fn new(editor: &'a Editor, highlight_cache: &'a mut HighlightCache) -> Self {
        let line_count = editor.buffer.len_lines();
        let line_number_width = line_count.to_string().len().max(3) + 1; // +1 for padding

        Self {
            editor,
            highlight_cache,
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
        let selection_style = Style::default()
            .bg(Color::Rgb(68, 71, 90)) // Subtle blue-gray selection
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

                // Line content with syntax highlighting
                let content_x = area.x + self.line_number_width as u16;
                let available_width =
                    area.width.saturating_sub(self.line_number_width as u16) as usize;

                let line_str = line.to_string();
                let line_text: String = line_str
                    .chars()
                    .filter(|c| *c != '\n' && *c != '\r')
                    .collect();

                // Get highlighted segments for this line
                // SAFETY: We need to get a mutable reference, but the borrow checker
                // doesn't know that self is consumed by render()
                let highlight_cache =
                    unsafe { &mut *(self.highlight_cache as *const _ as *mut HighlightCache) };
                let segments = highlight_cache.get_line(buffer_line, &line_text);

                let mut col = 0;
                for segment in segments {
                    for ch in segment.text.chars() {
                        if col >= available_width {
                            break;
                        }

                        let x = content_x + col as u16;
                        if x >= area.x + area.width {
                            break;
                        }

                        // Apply selection style if selected, otherwise use syntax style
                        let style = if self.is_selected(buffer_line, col) {
                            // Merge selection background with syntax foreground
                            selection_style.fg(segment.style.fg.unwrap_or(Color::White))
                        } else {
                            segment.style
                        };

                        buf.set_string(x, y, ch.to_string(), style);
                        col += 1;
                    }
                }

                // If selection extends beyond line content, show it
                let line_len = line_text.len();
                if self.is_selected(buffer_line, line_len) && line_len < available_width {
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
