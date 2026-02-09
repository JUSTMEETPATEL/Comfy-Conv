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
            ConvError::ConversionFailed { stderr, .. } => {
                let mut suggestions = vec!["Check that the file is not corrupted".into()];
                if let Some(err) = stderr {
                    if err.contains("password") {
                        suggestions.insert(0, "Remove password protection from the file".into());
                    }
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
