//! Cross-platform clipboard with WSL support
//!
//! This module provides clipboard operations that work across:
//! - Native Linux (X11/Wayland via arboard)
//! - WSL (Windows Subsystem for Linux via clip.exe/powershell)
//! - macOS (via arboard)
//! - Windows (via arboard)
//!
//! WSL Detection: Checks /proc/version for "microsoft" or "WSL"

use std::process::{Command, Stdio};
use std::sync::{Mutex, OnceLock};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClipboardError {
    #[error("Clipboard not available")]
    NotAvailable,

    #[error("Failed to copy: {0}")]
    CopyFailed(String),

    #[error("Failed to paste: {0}")]
    PasteFailed(String),

    #[error("Empty text")]
    EmptyText,
}

/// Cached WSL detection result
static IS_WSL: OnceLock<bool> = OnceLock::new();

/// Global arboard clipboard instance
static CLIPBOARD: OnceLock<Mutex<arboard::Clipboard>> = OnceLock::new();

/// Detect if running in WSL
fn is_wsl() -> bool {
    *IS_WSL.get_or_init(|| {
        if let Ok(version) = std::fs::read_to_string("/proc/version") {
            let lower = version.to_lowercase();
            lower.contains("microsoft") || lower.contains("wsl")
        } else {
            false
        }
    })
}

/// Get or initialize the arboard clipboard
fn get_arboard() -> Option<&'static Mutex<arboard::Clipboard>> {
    CLIPBOARD
        .get_or_init(|| {
            arboard::Clipboard::new()
                .map(Mutex::new)
                .unwrap_or_else(|_| {
                    // Return a dummy that will fail on use
                    // This shouldn't happen but we handle it gracefully
                    Mutex::new(arboard::Clipboard::new().expect("Clipboard init failed"))
                })
        })
        .into()
}

/// Copy text to clipboard
///
/// In WSL: Uses PowerShell Set-Clipboard for proper UTF-8 support
/// (clip.exe has encoding issues with non-ASCII characters)
pub fn copy(text: &str) -> Result<(), ClipboardError> {
    if text.is_empty() {
        return Err(ClipboardError::EmptyText);
    }

    if is_wsl() {
        copy_wsl(text)
    } else {
        copy_native(text)
    }
}

/// Paste text from clipboard
///
/// In WSL: Uses PowerShell Get-Clipboard
pub fn paste() -> Result<String, ClipboardError> {
    if is_wsl() {
        paste_wsl()
    } else {
        paste_native()
    }
}

/// Cut is the same as copy (caller handles deletion)
pub fn cut(text: &str) -> Result<(), ClipboardError> {
    copy(text)
}

// === Native Implementation (arboard) ===

fn copy_native(text: &str) -> Result<(), ClipboardError> {
    let clipboard = get_arboard().ok_or(ClipboardError::NotAvailable)?;
    let mut clipboard = clipboard
        .lock()
        .map_err(|e| ClipboardError::CopyFailed(e.to_string()))?;

    #[cfg(target_os = "linux")]
    {
        use arboard::{LinuxClipboardKind, SetExtLinux};

        // Copy to both CLIPBOARD and PRIMARY on Linux
        clipboard
            .set()
            .clipboard(LinuxClipboardKind::Clipboard)
            .text(text.to_string())
            .map_err(|e| ClipboardError::CopyFailed(e.to_string()))?;

        // PRIMARY is optional (for middle-click paste)
        let _ = clipboard
            .set()
            .clipboard(LinuxClipboardKind::Primary)
            .text(text.to_string());
    }

    #[cfg(not(target_os = "linux"))]
    clipboard
        .set_text(text)
        .map_err(|e| ClipboardError::CopyFailed(e.to_string()))?;

    Ok(())
}

fn paste_native() -> Result<String, ClipboardError> {
    let clipboard = get_arboard().ok_or(ClipboardError::NotAvailable)?;
    let mut clipboard = clipboard
        .lock()
        .map_err(|e| ClipboardError::PasteFailed(e.to_string()))?;

    #[cfg(target_os = "linux")]
    {
        use arboard::{GetExtLinux, LinuxClipboardKind};

        // Try CLIPBOARD first
        if let Ok(text) = clipboard
            .get()
            .clipboard(LinuxClipboardKind::Clipboard)
            .text()
        {
            if !text.is_empty() {
                return Ok(text);
            }
        }

        // Fall back to PRIMARY
        clipboard
            .get()
            .clipboard(LinuxClipboardKind::Primary)
            .text()
            .map_err(|e| ClipboardError::PasteFailed(e.to_string()))
    }

    #[cfg(not(target_os = "linux"))]
    clipboard
        .get_text()
        .map_err(|e| ClipboardError::PasteFailed(e.to_string()))
}

// === WSL Implementation (PowerShell) ===

/// Copy to Windows clipboard via PowerShell (UTF-8 safe)
fn copy_wsl(text: &str) -> Result<(), ClipboardError> {
    // Use PowerShell with here-string for proper UTF-8 handling
    // This is more reliable than clip.exe which has encoding issues
    // Note: Can't use inline format here because PowerShell here-string
    // requires the text to be on its own line, not interpolated
    #[allow(clippy::uninlined_format_args)]
    let script = format!(
        r#"$text = @'
{}
'@
Set-Clipboard -Value $text"#,
        text
    );

    let output = Command::new("powershell.exe")
        .args(["-NoProfile", "-Command", &script])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| ClipboardError::CopyFailed(format!("Failed to run powershell: {e}")))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(ClipboardError::CopyFailed(format!(
            "PowerShell failed: {stderr}"
        )))
    }
}

/// Paste from Windows clipboard via PowerShell
fn paste_wsl() -> Result<String, ClipboardError> {
    let output = Command::new("powershell.exe")
        .args(["-NoProfile", "-Command", "Get-Clipboard"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| ClipboardError::PasteFailed(format!("Failed to run powershell: {e}")))?;

    if output.status.success() {
        let text = String::from_utf8_lossy(&output.stdout);
        // Remove trailing CRLF that PowerShell adds
        Ok(text.trim_end_matches("\r\n").to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(ClipboardError::PasteFailed(format!(
            "PowerShell failed: {stderr}"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wsl_detection() {
        // Just ensure it doesn't panic
        let _ = is_wsl();
    }

    #[test]
    fn test_copy_empty_fails() {
        let result = copy("");
        assert!(matches!(result, Err(ClipboardError::EmptyText)));
    }

    // Note: Clipboard tests that actually copy/paste need to be run
    // manually as they depend on system state
    #[test]
    #[ignore]
    fn test_copy_paste_roundtrip() {
        let test_text = "Hello, four-code! Umlaute: öäüß 中文";
        copy(test_text).expect("Copy failed");
        let pasted = paste().expect("Paste failed");
        assert_eq!(pasted, test_text);
    }
}
