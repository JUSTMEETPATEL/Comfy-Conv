//! Dependency detection for LibreOffice and Pandoc
//!
//! Checks for required external tools and provides installation guidance.

use crate::error::{ConvError, Result};
use std::env::consts::OS;

/// Status of external dependencies
#[derive(Debug, Clone)]
pub struct DependencyStatus {
    pub libreoffice: Option<String>,
    pub pandoc: Option<String>,
}

impl DependencyStatus {
    /// Check if LibreOffice is available
    pub fn has_libreoffice(&self) -> bool {
        self.libreoffice.is_some()
    }

    /// Check if Pandoc is available
    pub fn has_pandoc(&self) -> bool {
        self.pandoc.is_some()
    }

    /// Check if any conversion engine is available
    pub fn has_any(&self) -> bool {
        self.has_libreoffice() || self.has_pandoc()
    }
}

/// Detect available conversion tools
pub fn check_dependencies() -> Result<DependencyStatus> {
    let libreoffice = find_libreoffice();
    let pandoc = find_pandoc();

    let status = DependencyStatus {
        libreoffice,
        pandoc,
    };

    if !status.has_any() {
        return Err(ConvError::NoEnginesAvailable);
    }

    Ok(status)
}

/// Find LibreOffice executable
fn find_libreoffice() -> Option<String> {
    // Try common names
    let names = if cfg!(target_os = "windows") {
        vec!["soffice.exe", "soffice"]
    } else {
        vec!["soffice", "libreoffice"]
    };

    for name in names {
        if let Ok(path) = which::which(name) {
            return Some(path.to_string_lossy().to_string());
        }
    }

    // Check common installation paths on macOS
    #[cfg(target_os = "macos")]
    {
        let mac_path = "/Applications/LibreOffice.app/Contents/MacOS/soffice";
        if std::path::Path::new(mac_path).exists() {
            return Some(mac_path.to_string());
        }
    }

    None
}

/// Find Pandoc executable
fn find_pandoc() -> Option<String> {
    which::which("pandoc")
        .ok()
        .map(|p| p.to_string_lossy().to_string())
}

/// Get OS-specific installation instructions for LibreOffice
pub fn libreoffice_install_hint() -> String {
    match OS {
        "macos" => "Install with: brew install --cask libreoffice".to_string(),
        "linux" => {
            "Install with:\n  Ubuntu/Debian: sudo apt install libreoffice\n  Fedora: sudo dnf install libreoffice".to_string()
        }
        "windows" => "Download from: https://www.libreoffice.org/download/".to_string(),
        _ => "Install LibreOffice from https://www.libreoffice.org/".to_string(),
    }
}

/// Get OS-specific installation instructions for Pandoc
pub fn pandoc_install_hint() -> String {
    match OS {
        "macos" => "Install with: brew install pandoc".to_string(),
        "linux" => {
            "Install with:\n  Ubuntu/Debian: sudo apt install pandoc\n  Fedora: sudo dnf install pandoc".to_string()
        }
        "windows" => "Install with: choco install pandoc\nOr download from: https://pandoc.org/installing.html".to_string(),
        _ => "Install Pandoc from https://pandoc.org/".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install_hints_not_empty() {
        assert!(!libreoffice_install_hint().is_empty());
        assert!(!pandoc_install_hint().is_empty());
    }
}
