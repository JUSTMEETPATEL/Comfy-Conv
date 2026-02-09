//! comfy-conv error types
//!
//! Provides structured error handling with actionable messages.

use std::path::PathBuf;
use thiserror::Error;

/// Main error type for comfy-conv
#[derive(Error, Debug)]
pub enum ConvError {
    #[error("Dependency not found: {name}")]
    DependencyMissing {
        name: String,
        install_hint: String,
    },

    #[error("No conversion engines available")]
    NoEnginesAvailable,

    #[error("Unsupported format: {extension}")]
    UnsupportedFormat { extension: String },

    #[error("Cannot convert {from} to {to}")]
    IncompatibleConversion { from: String, to: String },

    #[error("Conversion failed: {message}")]
    ConversionFailed {
        message: String,
        stderr: Option<String>,
    },

    #[error("Conversion timed out after {seconds}s")]
    Timeout { seconds: u64 },

    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },

    #[error("Cannot write to {path}: {reason}")]
    WriteError { path: PathBuf, reason: String },

    #[error("No convertible files found in current directory")]
    NoFilesFound,

    #[error("User cancelled")]
    Cancelled,

    #[error("{0}")]
    Io(#[from] std::io::Error),
}

/// Result type alias for comfy-conv operations
pub type Result<T> = std::result::Result<T, ConvError>;

impl ConvError {
    /// Get actionable suggestions for this error
    pub fn suggestions(&self) -> Vec<String> {
        match self {
            ConvError::DependencyMissing { install_hint, .. } => {
                vec![install_hint.clone()]
            }
            ConvError::NoEnginesAvailable => {
                vec![
                    "Install LibreOffice for Office document conversions".into(),
                    "Install Pandoc for text format conversions".into(),
                ]
            }
            ConvError::UnsupportedFormat { extension } => {
                vec![format!(
                    "Supported formats: docx, xlsx, pptx, md, html, txt, pdf"
                )]
            }
            ConvError::ConversionFailed { stderr, message } => {
                let mut suggestions = Vec::new();
                
                // Check for LaTeX-related errors
                if message.contains("LaTeX") {
                    suggestions.push("Install LaTeX: brew install --cask mactex (macOS)".into());
                    suggestions.push("Or convert to HTML instead of PDF".into());
                    return suggestions;
                }
                
                // Parse stderr for hints
                if let Some(err) = stderr {
                    if err.contains("password") {
                        suggestions.push("Remove password protection from the file".into());
                    }
                    if err.contains("LaTeX") || err.contains("pdflatex") {
                        suggestions.push("Install LaTeX for PDF generation".into());
                        suggestions.push("Or convert to HTML instead".into());
                    }
                    // Include stderr lines that look like suggestions
                    for line in err.lines() {
                        if line.trim().starts_with("macOS:") || 
                           line.trim().starts_with("Linux:") ||
                           line.trim().starts_with("Windows:") {
                            suggestions.push(line.trim().to_string());
                        }
                    }
                }
                
                if suggestions.is_empty() {
                    suggestions.push("Check that the file is not corrupted".into());
                }
                suggestions
            }
            ConvError::Timeout { .. } => {
                vec![
                    "Try converting a smaller file".into(),
                    "Ensure LibreOffice is not already running".into(),
                ]
            }
            ConvError::WriteError { .. } => {
                vec![
                    "Check write permissions".into(),
                    "Ensure disk is not full".into(),
                ]
            }
            _ => vec![],
        }
    }
}
