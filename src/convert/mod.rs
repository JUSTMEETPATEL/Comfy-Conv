//! Document conversion module
//!
//! Handles conversion using LibreOffice and Pandoc engines.

pub mod libreoffice;
pub mod pandoc;

use crate::deps::DependencyStatus;
use crate::error::{ConvError, Result};
use crate::files::FileInfo;
use crate::formats::{ConversionPath, Engine};
use std::path::PathBuf;
use std::time::Duration;

/// Default timeout for LibreOffice conversions (30 seconds)
pub const LIBREOFFICE_TIMEOUT: Duration = Duration::from_secs(30);

/// Default timeout for Pandoc conversions (10 seconds)
pub const PANDOC_TIMEOUT: Duration = Duration::from_secs(10);

/// Convert a file to the specified format
pub fn convert(
    file: &FileInfo,
    target: &ConversionPath,
    deps: &DependencyStatus,
) -> Result<PathBuf> {
    // Determine output path
    let output_dir = file.path.parent().unwrap_or(std::path::Path::new("."));
    let stem = file.path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let output_path = output_dir.join(format!("{}.{}", stem, target.to.extension()));

    match target.engine {
        Engine::LibreOffice => {
            let soffice_path = deps.libreoffice.as_ref()
                .ok_or_else(|| ConvError::DependencyMissing {
                    name: "LibreOffice".to_string(),
                    install_hint: crate::deps::libreoffice_install_hint(),
                })?;
            
            libreoffice::convert(
                soffice_path,
                &file.path,
                target.to.extension(),
                output_dir,
                LIBREOFFICE_TIMEOUT,
            )?;
            
            // LibreOffice creates output with same basename
            Ok(output_path)
        }
        Engine::Pandoc => {
            let pandoc_path = deps.pandoc.as_ref()
                .ok_or_else(|| ConvError::DependencyMissing {
                    name: "Pandoc".to_string(),
                    install_hint: crate::deps::pandoc_install_hint(),
                })?;
            
            pandoc::convert(
                pandoc_path,
                &file.path,
                &output_path,
                PANDOC_TIMEOUT,
            )?;
            
            Ok(output_path)
        }
    }
}
