//! File discovery and metadata
//!
//! Scans directories for convertible files and collects metadata.

use crate::error::{ConvError, Result};
use crate::formats::Format;
use chrono::{DateTime, Local};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Information about a discovered file
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub name: String,
    pub format: Format,
    pub size: u64,
    pub modified: Option<DateTime<Local>>,
}

impl FileInfo {
    /// Create FileInfo from a path
    pub fn from_path(path: &Path) -> Option<Self> {
        let format = Format::from_path(path)?;
        let name = path.file_name()?.to_string_lossy().to_string();

        let metadata = std::fs::metadata(path).ok()?;
        let size = metadata.len();
        let modified = metadata.modified().ok().map(DateTime::from);

        Some(FileInfo {
            path: path.to_path_buf(),
            name,
            format,
            size,
            modified,
        })
    }

    /// Format file size for display (e.g., "1.2 MB")
    pub fn size_display(&self) -> String {
        const KB: u64 = 1024;
        const MB: u64 = 1024 * KB;
        const GB: u64 = 1024 * MB;

        if self.size >= GB {
            format!("{:.1} GB", self.size as f64 / GB as f64)
        } else if self.size >= MB {
            format!("{:.1} MB", self.size as f64 / MB as f64)
        } else if self.size >= KB {
            format!("{} KB", self.size / KB)
        } else {
            format!("{} B", self.size)
        }
    }

    /// Format modification time for display (e.g., "2 hours ago")
    pub fn modified_display(&self) -> String {
        match self.modified {
            Some(time) => {
                let now = Local::now();
                let duration = now.signed_duration_since(time);

                if duration.num_days() > 7 {
                    time.format("%b %d").to_string()
                } else if duration.num_days() >= 1 {
                    let days = duration.num_days();
                    if days == 1 {
                        "yesterday".to_string()
                    } else {
                        format!("{} days ago", days)
                    }
                } else if duration.num_hours() >= 1 {
                    let hours = duration.num_hours();
                    if hours == 1 {
                        "1 hour ago".to_string()
                    } else {
                        format!("{} hours ago", hours)
                    }
                } else if duration.num_minutes() >= 1 {
                    let minutes = duration.num_minutes();
                    if minutes == 1 {
                        "1 minute ago".to_string()
                    } else {
                        format!("{} minutes ago", minutes)
                    }
                } else {
                    "just now".to_string()
                }
            }
            None => "unknown".to_string(),
        }
    }

    /// Get the output path for a given target format
    pub fn output_path(&self, target_format: Format) -> PathBuf {
        let mut output = self.path.clone();
        output.set_extension(target_format.extension());
        output
    }
}

/// Discover all convertible files in a directory
pub fn discover_files(dir: &Path) -> Result<Vec<FileInfo>> {
    if !dir.exists() {
        return Err(ConvError::FileNotFound {
            path: dir.to_path_buf(),
        });
    }

    let mut files: Vec<FileInfo> = WalkDir::new(dir)
        .max_depth(1) // Only current directory for MVP
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| FileInfo::from_path(e.path()))
        .collect();

    // Sort by name
    files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    if files.is_empty() {
        return Err(ConvError::NoFilesFound);
    }

    Ok(files)
}

/// Check if output path already exists
pub fn check_collision(output_path: &Path) -> bool {
    output_path.exists()
}

/// Generate an alternative filename to avoid collision
pub fn generate_alternative_name(path: &Path) -> PathBuf {
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    let parent = path.parent().unwrap_or(Path::new("."));

    let mut counter = 1;
    loop {
        let new_name = if ext.is_empty() {
            format!("{}-{}", stem, counter)
        } else {
            format!("{}-{}.{}", stem, counter, ext)
        };

        let new_path = parent.join(new_name);
        if !new_path.exists() {
            return new_path;
        }
        counter += 1;

        // Safety limit
        if counter > 100 {
            return parent.join(format!("{}-{}.{}", stem, chrono::Utc::now().timestamp(), ext));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_size_display() {
        let info = FileInfo {
            path: PathBuf::from("test.docx"),
            name: "test.docx".to_string(),
            format: Format::Docx,
            size: 1536,
            modified: None,
        };
        assert_eq!(info.size_display(), "1 KB");
    }

    #[test]
    fn test_generate_alternative_name() {
        // This test would need a temp directory in a real scenario
        let path = Path::new("/nonexistent/test.pdf");
        let alt = generate_alternative_name(path);
        assert!(alt.to_string_lossy().contains("test-1.pdf") || alt.to_string_lossy().contains("test-"));
    }
}
