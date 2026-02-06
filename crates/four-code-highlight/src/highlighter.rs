//! Tree-sitter based syntax highlighter with caching

use crate::{style_for_highlight, Language, HIGHLIGHT_NAMES};
use ratatui::style::{Color, Style};
use std::collections::HashMap;
use std::path::Path;
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter as TsHighlighter};

/// Maximum number of cached lines
const MAX_CACHE_SIZE: usize = 2000;

/// Syntax highlighter using tree-sitter
pub struct Highlighter {
    /// Language configurations
    configs: HashMap<Language, HighlightConfiguration>,
    /// Highlight names for mapping
    highlight_names: Vec<String>,
}

impl Highlighter {
    /// Create a new highlighter with all supported languages
    pub fn new() -> Self {
        let highlight_names: Vec<String> = HIGHLIGHT_NAMES.iter().map(|s| s.to_string()).collect();
        let mut configs = HashMap::new();

        // PHP (primary focus)
        Self::load_config(
            &mut configs,
            Language::Php,
            tree_sitter_php::LANGUAGE_PHP.into(),
            tree_sitter_php::HIGHLIGHTS_QUERY,
            tree_sitter_php::INJECTIONS_QUERY,
            &highlight_names,
        );

        // JavaScript
        Self::load_config(
            &mut configs,
            Language::JavaScript,
            tree_sitter_javascript::LANGUAGE.into(),
            tree_sitter_javascript::HIGHLIGHT_QUERY,
            tree_sitter_javascript::INJECTIONS_QUERY,
            &highlight_names,
        );

        // TypeScript
        Self::load_config(
            &mut configs,
            Language::TypeScript,
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            tree_sitter_typescript::HIGHLIGHTS_QUERY,
            "",
            &highlight_names,
        );

        // TSX
        Self::load_config(
            &mut configs,
            Language::Tsx,
            tree_sitter_typescript::LANGUAGE_TSX.into(),
            tree_sitter_typescript::HIGHLIGHTS_QUERY,
            "",
            &highlight_names,
        );

        // JSON
        Self::load_config(
            &mut configs,
            Language::Json,
            tree_sitter_json::LANGUAGE.into(),
            tree_sitter_json::HIGHLIGHTS_QUERY,
            "",
            &highlight_names,
        );

        // HTML
        Self::load_config(
            &mut configs,
            Language::Html,
            tree_sitter_html::LANGUAGE.into(),
            tree_sitter_html::HIGHLIGHTS_QUERY,
            tree_sitter_html::INJECTIONS_QUERY,
            &highlight_names,
        );

        // CSS
        Self::load_config(
            &mut configs,
            Language::Css,
            tree_sitter_css::LANGUAGE.into(),
            tree_sitter_css::HIGHLIGHTS_QUERY,
            "",
            &highlight_names,
        );

        // YAML
        Self::load_config(
            &mut configs,
            Language::Yaml,
            tree_sitter_yaml::LANGUAGE.into(),
            tree_sitter_yaml::HIGHLIGHTS_QUERY,
            "",
            &highlight_names,
        );

        // TOML
        Self::load_config(
            &mut configs,
            Language::Toml,
            tree_sitter_toml_ng::LANGUAGE.into(),
            tree_sitter_toml_ng::HIGHLIGHTS_QUERY,
            "",
            &highlight_names,
        );

        // Bash
        Self::load_config(
            &mut configs,
            Language::Bash,
            tree_sitter_bash::LANGUAGE.into(),
            tree_sitter_bash::HIGHLIGHT_QUERY,
            "",
            &highlight_names,
        );

        // Markdown
        Self::load_config(
            &mut configs,
            Language::Markdown,
            tree_sitter_md::LANGUAGE.into(),
            tree_sitter_md::HIGHLIGHT_QUERY_BLOCK,
            tree_sitter_md::INJECTION_QUERY_BLOCK,
            &highlight_names,
        );

        // Rust
        Self::load_config(
            &mut configs,
            Language::Rust,
            tree_sitter_rust::LANGUAGE.into(),
            tree_sitter_rust::HIGHLIGHTS_QUERY,
            "",
            &highlight_names,
        );

        Self {
            configs,
            highlight_names,
        }
    }

    /// Load a language configuration
    fn load_config(
        configs: &mut HashMap<Language, HighlightConfiguration>,
        language: Language,
        ts_language: tree_sitter::Language,
        highlights_query: &str,
        injections_query: &str,
        highlight_names: &[String],
    ) {
        match HighlightConfiguration::new(
            ts_language,
            language.name(),
            highlights_query,
            injections_query,
            "",
        ) {
            Ok(mut config) => {
                config.configure(highlight_names);
                configs.insert(language, config);
            }
            Err(e) => {
                eprintln!(
                    "Warning: Failed to load {} highlighting: {}",
                    language.name(),
                    e
                );
            }
        }
    }

    /// Check if a language is supported
    pub fn supports_language(&self, language: Language) -> bool {
        self.configs.contains_key(&language)
    }

    /// Get configuration for a language
    pub fn get_config(&self, language: Language) -> Option<&HighlightConfiguration> {
        self.configs.get(&language)
    }

    /// Get highlight names
    pub fn highlight_names(&self) -> &[String] {
        &self.highlight_names
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}

/// A highlighted text segment
#[derive(Debug, Clone)]
pub struct Segment {
    pub text: String,
    pub style: Style,
}

/// Line-based highlight cache for efficient rendering
pub struct HighlightCache {
    /// Cached highlighted lines
    cache: HashMap<usize, Vec<Segment>>,
    /// Current language
    language: Option<Language>,
    /// Reference to global highlighter
    highlighter: &'static Highlighter,
    /// Default style for unhighlighted text
    default_style: Style,
    /// Access counter for LRU eviction
    access_counter: u64,
    /// Access times for each line
    access_times: HashMap<usize, u64>,
}

impl HighlightCache {
    /// Create a new highlight cache
    pub fn new(highlighter: &'static Highlighter) -> Self {
        Self {
            cache: HashMap::new(),
            language: None,
            highlighter,
            default_style: Style::default().fg(Color::White),
            access_counter: 0,
            access_times: HashMap::new(),
        }
    }

    /// Set the language for highlighting
    pub fn set_language(&mut self, language: Option<Language>) {
        if self.language != language {
            self.language = language;
            self.invalidate_all();
        }
    }

    /// Set language from file path
    pub fn set_language_from_path(&mut self, path: &Path) {
        self.set_language(crate::detect_language(path));
    }

    /// Set the default style for unhighlighted text
    pub fn set_default_style(&mut self, style: Style) {
        if self.default_style != style {
            self.default_style = style;
            self.invalidate_all();
        }
    }

    /// Get highlighted segments for a line
    pub fn get_line(&mut self, line_idx: usize, line_text: &str) -> &[Segment] {
        self.access_counter += 1;

        // Update access time if cached
        if self.cache.contains_key(&line_idx) {
            self.access_times.insert(line_idx, self.access_counter);
        } else {
            // Compute and cache
            let segments = self.highlight_line(line_text);

            // Evict if cache is too large
            if self.cache.len() >= MAX_CACHE_SIZE {
                self.evict_lru();
            }

            self.cache.insert(line_idx, segments);
            self.access_times.insert(line_idx, self.access_counter);
        }

        self.cache.get(&line_idx).expect("just inserted")
    }

    /// Highlight a single line
    fn highlight_line(&self, line_text: &str) -> Vec<Segment> {
        let Some(language) = self.language else {
            return vec![Segment {
                text: line_text.to_string(),
                style: self.default_style,
            }];
        };

        let Some(config) = self.highlighter.get_config(language) else {
            return vec![Segment {
                text: line_text.to_string(),
                style: self.default_style,
            }];
        };

        let mut ts_highlighter = TsHighlighter::new();
        let source = line_text.as_bytes();

        let highlights = match ts_highlighter.highlight(config, source, None, |_| None) {
            Ok(h) => h,
            Err(_) => {
                return vec![Segment {
                    text: line_text.to_string(),
                    style: self.default_style,
                }]
            }
        };

        let mut segments = Vec::new();
        let mut current_style = self.default_style;
        let mut current_text = String::new();
        let highlight_names = self.highlighter.highlight_names();

        for event in highlights {
            match event {
                Ok(HighlightEvent::Source { start, end }) => {
                    if let Ok(text) = std::str::from_utf8(&source[start..end]) {
                        current_text.push_str(text);
                    }
                }
                Ok(HighlightEvent::HighlightStart(highlight)) => {
                    if !current_text.is_empty() {
                        segments.push(Segment {
                            text: std::mem::take(&mut current_text),
                            style: current_style,
                        });
                    }
                    // Get highlight name and style
                    let name = highlight_names
                        .get(highlight.0)
                        .map(|s| s.as_str())
                        .unwrap_or("");
                    current_style = style_for_highlight(name);
                }
                Ok(HighlightEvent::HighlightEnd) => {
                    if !current_text.is_empty() {
                        segments.push(Segment {
                            text: std::mem::take(&mut current_text),
                            style: current_style,
                        });
                    }
                    current_style = self.default_style;
                }
                Err(_) => {
                    return vec![Segment {
                        text: line_text.to_string(),
                        style: self.default_style,
                    }]
                }
            }
        }

        // Don't forget remaining text
        if !current_text.is_empty() {
            segments.push(Segment {
                text: current_text,
                style: current_style,
            });
        }

        if segments.is_empty() {
            vec![Segment {
                text: line_text.to_string(),
                style: self.default_style,
            }]
        } else {
            segments
        }
    }

    /// Evict least recently used entries
    fn evict_lru(&mut self) {
        let evict_count = MAX_CACHE_SIZE / 5;

        let mut entries: Vec<(usize, u64)> = self
            .access_times
            .iter()
            .map(|(&line, &time)| (line, time))
            .collect();

        if entries.len() <= evict_count {
            return;
        }

        // Partial sort for O(n) performance
        entries.select_nth_unstable_by_key(evict_count, |&(_, time)| time);

        for (line, _) in entries.iter().take(evict_count) {
            self.cache.remove(line);
            self.access_times.remove(line);
        }
    }

    /// Invalidate a specific line
    pub fn invalidate_line(&mut self, line_idx: usize) {
        self.cache.remove(&line_idx);
        self.access_times.remove(&line_idx);
    }

    /// Invalidate lines from a starting point
    pub fn invalidate_from(&mut self, start_line: usize) {
        let to_remove: Vec<usize> = self
            .cache
            .keys()
            .filter(|&&line| line >= start_line)
            .copied()
            .collect();

        for line in to_remove {
            self.cache.remove(&line);
            self.access_times.remove(&line);
        }
    }

    /// Invalidate entire cache
    pub fn invalidate_all(&mut self) {
        self.cache.clear();
        self.access_times.clear();
    }

    /// Check if highlighting is active
    pub fn has_highlighting(&self) -> bool {
        self.language.is_some()
    }

    /// Get current language
    pub fn current_language(&self) -> Option<Language> {
        self.language
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlighter_creation() {
        let hl = Highlighter::new();
        assert!(hl.supports_language(Language::Php));
        assert!(hl.supports_language(Language::JavaScript));
        assert!(hl.supports_language(Language::Rust));
    }

    #[test]
    fn test_highlight_cache() {
        let hl = crate::global_highlighter();
        let mut cache = HighlightCache::new(hl);

        // Without language set
        let segments = cache.get_line(0, "hello world");
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].text, "hello world");

        // With PHP
        cache.set_language(Some(Language::Php));
        let segments = cache.get_line(0, "<?php echo 'Hello';");
        assert!(segments.len() > 1, "PHP should be highlighted");
    }

    #[test]
    fn test_cache_invalidation() {
        let hl = crate::global_highlighter();
        let mut cache = HighlightCache::new(hl);

        cache.get_line(0, "line 0");
        cache.get_line(1, "line 1");
        cache.get_line(2, "line 2");

        cache.invalidate_from(1);

        // Line 0 should still be cached, but we can't check that directly
        // Just ensure no panic
        cache.get_line(0, "line 0");
        cache.get_line(1, "new line 1");
    }
}
