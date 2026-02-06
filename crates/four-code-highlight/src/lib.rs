//! Syntax highlighting for four-code using tree-sitter
//!
//! Provides syntax highlighting for PHP and common web development languages.
//! Designed to be lightweight and fast with line-based caching.

mod highlighter;
mod languages;

pub use highlighter::{HighlightCache, Highlighter};
pub use languages::{detect_language, Language, SUPPORTED_LANGUAGES};

use ratatui::style::{Color, Modifier, Style};
use std::sync::OnceLock;

/// Global highlighter instance (lazily initialized)
static GLOBAL_HIGHLIGHTER: OnceLock<Highlighter> = OnceLock::new();

/// Get the global highlighter instance
pub fn global_highlighter() -> &'static Highlighter {
    GLOBAL_HIGHLIGHTER.get_or_init(Highlighter::new)
}

/// Standard highlight categories used by tree-sitter
pub const HIGHLIGHT_NAMES: &[&str] = &[
    "attribute",
    "comment",
    "constant",
    "constant.builtin",
    "constructor",
    "function",
    "function.builtin",
    "function.method",
    "keyword",
    "label",
    "number",
    "operator",
    "property",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "punctuation.special",
    "string",
    "string.special",
    "tag",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.parameter",
    "escape",
    "embedded",
];

/// Get style for a highlight category (One Dark theme)
pub fn style_for_highlight(name: &str) -> Style {
    let (color, bold, italic) = match name {
        "comment" => (Color::Rgb(92, 99, 112), false, true),
        "keyword" => (Color::Rgb(198, 120, 221), true, false), // Purple
        "function" | "function.builtin" | "function.method" => {
            (Color::Rgb(97, 175, 239), false, false) // Blue
        }
        "string" | "string.special" => (Color::Rgb(152, 195, 121), false, false), // Green
        "number" => (Color::Rgb(209, 154, 102), false, false),                    // Orange
        "constant" | "constant.builtin" => (Color::Rgb(209, 154, 102), false, false),
        "type" | "type.builtin" => (Color::Rgb(229, 192, 123), false, false), // Yellow
        "variable" => (Color::Rgb(224, 108, 117), false, false),              // Red
        "variable.builtin" => (Color::Rgb(224, 108, 117), false, true),
        "variable.parameter" => (Color::Rgb(171, 178, 191), false, true),
        "property" => (Color::Rgb(224, 108, 117), false, false),
        "operator" => (Color::Rgb(171, 178, 191), false, false),
        "punctuation" | "punctuation.bracket" | "punctuation.delimiter" => {
            (Color::Rgb(171, 178, 191), false, false)
        }
        "punctuation.special" => (Color::Rgb(198, 120, 221), false, false),
        "constructor" => (Color::Rgb(229, 192, 123), true, false),
        "tag" => (Color::Rgb(224, 108, 117), false, false),
        "attribute" => (Color::Rgb(209, 154, 102), false, false),
        "escape" => (Color::Rgb(86, 182, 194), false, false), // Cyan
        "embedded" => (Color::Rgb(198, 120, 221), false, false),
        _ => (Color::Rgb(171, 178, 191), false, false), // Default gray
    };

    let mut style = Style::default().fg(color);
    if bold {
        style = style.add_modifier(Modifier::BOLD);
    }
    if italic {
        style = style.add_modifier(Modifier::ITALIC);
    }
    style
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_php() {
        assert_eq!(
            detect_language(std::path::Path::new("test.php")),
            Some(Language::Php)
        );
    }

    #[test]
    fn test_detect_javascript() {
        assert_eq!(
            detect_language(std::path::Path::new("app.js")),
            Some(Language::JavaScript)
        );
        assert_eq!(
            detect_language(std::path::Path::new("app.mjs")),
            Some(Language::JavaScript)
        );
    }

    #[test]
    fn test_detect_unknown() {
        assert_eq!(detect_language(std::path::Path::new("file.xyz")), None);
    }

    #[test]
    fn test_global_highlighter() {
        let hl = global_highlighter();
        assert!(hl.supports_language(Language::Php));
        assert!(hl.supports_language(Language::JavaScript));
    }
}
