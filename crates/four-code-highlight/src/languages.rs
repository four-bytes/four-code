//! Language detection and supported languages

use std::path::Path;

/// Supported languages for syntax highlighting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Php,
    JavaScript,
    TypeScript,
    Tsx,
    Json,
    Html,
    Css,
    Yaml,
    Toml,
    Bash,
    Markdown,
    Rust,
}

impl Language {
    /// Get the language name as used by tree-sitter
    pub fn name(&self) -> &'static str {
        match self {
            Language::Php => "php",
            Language::JavaScript => "javascript",
            Language::TypeScript => "typescript",
            Language::Tsx => "tsx",
            Language::Json => "json",
            Language::Html => "html",
            Language::Css => "css",
            Language::Yaml => "yaml",
            Language::Toml => "toml",
            Language::Bash => "bash",
            Language::Markdown => "markdown",
            Language::Rust => "rust",
        }
    }
}

/// List of all supported languages
pub const SUPPORTED_LANGUAGES: &[Language] = &[
    Language::Php,
    Language::JavaScript,
    Language::TypeScript,
    Language::Tsx,
    Language::Json,
    Language::Html,
    Language::Css,
    Language::Yaml,
    Language::Toml,
    Language::Bash,
    Language::Markdown,
    Language::Rust,
];

/// Detect language from file path/extension
pub fn detect_language(path: &Path) -> Option<Language> {
    let ext = path.extension()?.to_str()?.to_lowercase();

    match ext.as_str() {
        // PHP
        "php" | "phtml" | "php3" | "php4" | "php5" | "phps" => Some(Language::Php),

        // JavaScript
        "js" | "mjs" | "cjs" | "jsx" => Some(Language::JavaScript),

        // TypeScript
        "ts" | "mts" | "cts" => Some(Language::TypeScript),
        "tsx" => Some(Language::Tsx),

        // Web
        "json" => Some(Language::Json),
        "html" | "htm" | "xhtml" => Some(Language::Html),
        "css" | "scss" | "sass" | "less" => Some(Language::Css),

        // Config
        "yaml" | "yml" => Some(Language::Yaml),
        "toml" => Some(Language::Toml),

        // Shell
        "sh" | "bash" | "zsh" | "fish" => Some(Language::Bash),

        // Documentation
        "md" | "markdown" => Some(Language::Markdown),

        // Rust (for editing four-code itself)
        "rs" => Some(Language::Rust),

        _ => None,
    }
}

/// Detect language from shebang line
#[allow(dead_code)] // Will be used when opening files without extension
pub fn detect_from_shebang(first_line: &str) -> Option<Language> {
    if !first_line.starts_with("#!") {
        return None;
    }

    let line = first_line.to_lowercase();

    if line.contains("php") {
        Some(Language::Php)
    } else if line.contains("node") || line.contains("deno") || line.contains("bun") {
        Some(Language::JavaScript)
    } else if line.contains("bash") || line.contains("/sh") {
        Some(Language::Bash)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_php_extensions() {
        assert_eq!(detect_language(Path::new("index.php")), Some(Language::Php));
        assert_eq!(
            detect_language(Path::new("template.phtml")),
            Some(Language::Php)
        );
    }

    #[test]
    fn test_shebang_php() {
        assert_eq!(
            detect_from_shebang("#!/usr/bin/env php"),
            Some(Language::Php)
        );
        assert_eq!(detect_from_shebang("#!/usr/bin/php"), Some(Language::Php));
    }

    #[test]
    fn test_shebang_node() {
        assert_eq!(
            detect_from_shebang("#!/usr/bin/env node"),
            Some(Language::JavaScript)
        );
    }
}
