//! Format definitions and compatibility matrix
//!
//! Defines supported file formats and valid conversion paths.

use std::path::Path;

/// Supported input/output formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Format {
    Docx,
    Xlsx,
    Pptx,
    Markdown,
    Html,
    Txt,
    Pdf,
    Csv,
}

impl Format {
    /// Get format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "docx" => Some(Format::Docx),
            "xlsx" => Some(Format::Xlsx),
            "pptx" => Some(Format::Pptx),
            "md" | "markdown" => Some(Format::Markdown),
            "html" | "htm" => Some(Format::Html),
            "txt" | "text" => Some(Format::Txt),
            "pdf" => Some(Format::Pdf),
            "csv" => Some(Format::Csv),
            _ => None,
        }
    }

    /// Get format from file path
    pub fn from_path(path: &Path) -> Option<Self> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(Self::from_extension)
    }

    /// Get file extension for this format
    pub fn extension(&self) -> &'static str {
        match self {
            Format::Docx => "docx",
            Format::Xlsx => "xlsx",
            Format::Pptx => "pptx",
            Format::Markdown => "md",
            Format::Html => "html",
            Format::Txt => "txt",
            Format::Pdf => "pdf",
            Format::Csv => "csv",
        }
    }

    /// Get icon for this format
    pub fn icon(&self) -> &'static str {
        match self {
            Format::Docx => "📄",
            Format::Xlsx => "📊",
            Format::Pptx => "📽️",
            Format::Markdown => "📝",
            Format::Html => "🌐",
            Format::Txt => "📋",
            Format::Pdf => "📕",
            Format::Csv => "📊",
        }
    }

    /// Get display name with icon
    pub fn display_name(&self) -> &'static str {
        match self {
            Format::Docx => "📄 DOCX",
            Format::Xlsx => "📊 XLSX",
            Format::Pptx => "📽️  PPTX",
            Format::Markdown => "📝 Markdown",
            Format::Html => "🌐 HTML",
            Format::Txt => "📋 TXT",
            Format::Pdf => "📕 PDF",
            Format::Csv => "📊 CSV",
        }
    }

    /// Get description for format selection
    pub fn description(&self) -> &'static str {
        match self {
            Format::Docx => "Microsoft Word Document",
            Format::Xlsx => "Microsoft Excel Spreadsheet",
            Format::Pptx => "Microsoft PowerPoint Presentation",
            Format::Markdown => "Markdown text format",
            Format::Html => "Web page format",
            Format::Txt => "Plain text",
            Format::Pdf => "Portable Document Format",
            Format::Csv => "Comma-separated values",
        }
    }

    /// Check if this is an Office format (requires LibreOffice)
    pub fn is_office_format(&self) -> bool {
        matches!(self, Format::Docx | Format::Xlsx | Format::Pptx)
    }

    /// All supported input formats
    pub fn supported_inputs() -> &'static [Format] {
        &[
            Format::Docx,
            Format::Xlsx,
            Format::Pptx,
            Format::Markdown,
            Format::Html,
            Format::Txt,
            Format::Pdf,
        ]
    }
}

/// Conversion engine to use
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Engine {
    LibreOffice,
    Pandoc,
}

impl Engine {
    pub fn display_name(&self) -> &'static str {
        match self {
            Engine::LibreOffice => "LibreOffice",
            Engine::Pandoc => "Pandoc",
        }
    }
}

/// A valid conversion path
#[derive(Debug, Clone)]
pub struct ConversionPath {
    pub from: Format,
    pub to: Format,
    pub engine: Engine,
    pub recommended: bool,
}

/// Get available output formats for a given input format
pub fn get_output_formats(input: Format) -> Vec<ConversionPath> {
    match input {
        Format::Docx => vec![
            ConversionPath { from: input, to: Format::Pdf, engine: Engine::LibreOffice, recommended: true },
            ConversionPath { from: input, to: Format::Html, engine: Engine::Pandoc, recommended: false },
            ConversionPath { from: input, to: Format::Txt, engine: Engine::Pandoc, recommended: false },
            ConversionPath { from: input, to: Format::Markdown, engine: Engine::Pandoc, recommended: false },
        ],
        Format::Xlsx => vec![
            ConversionPath { from: input, to: Format::Pdf, engine: Engine::LibreOffice, recommended: true },
            ConversionPath { from: input, to: Format::Html, engine: Engine::LibreOffice, recommended: false },
            ConversionPath { from: input, to: Format::Csv, engine: Engine::LibreOffice, recommended: false },
        ],
        Format::Pptx => vec![
            ConversionPath { from: input, to: Format::Pdf, engine: Engine::LibreOffice, recommended: true },
            ConversionPath { from: input, to: Format::Html, engine: Engine::LibreOffice, recommended: false },
        ],
        Format::Markdown => vec![
            ConversionPath { from: input, to: Format::Html, engine: Engine::Pandoc, recommended: true },
            ConversionPath { from: input, to: Format::Docx, engine: Engine::Pandoc, recommended: false },
            ConversionPath { from: input, to: Format::Txt, engine: Engine::Pandoc, recommended: false },
        ],
        Format::Html => vec![
            ConversionPath { from: input, to: Format::Markdown, engine: Engine::Pandoc, recommended: true },
            ConversionPath { from: input, to: Format::Docx, engine: Engine::Pandoc, recommended: false },
            ConversionPath { from: input, to: Format::Txt, engine: Engine::Pandoc, recommended: false },
        ],
        Format::Txt => vec![
            ConversionPath { from: input, to: Format::Html, engine: Engine::Pandoc, recommended: true },
            ConversionPath { from: input, to: Format::Docx, engine: Engine::Pandoc, recommended: false },
            ConversionPath { from: input, to: Format::Markdown, engine: Engine::Pandoc, recommended: false },
        ],
        Format::Pdf => vec![
            ConversionPath { from: input, to: Format::Txt, engine: Engine::Pandoc, recommended: true },
            ConversionPath { from: input, to: Format::Html, engine: Engine::Pandoc, recommended: false },
        ],
        Format::Csv => vec![], // CSV is output-only for MVP
    }
}

/// Filter conversion paths based on available engines
pub fn filter_by_available_engines(
    paths: Vec<ConversionPath>,
    has_libreoffice: bool,
    has_pandoc: bool,
) -> Vec<ConversionPath> {
    paths
        .into_iter()
        .filter(|p| match p.engine {
            Engine::LibreOffice => has_libreoffice,
            Engine::Pandoc => has_pandoc,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_from_extension() {
        assert_eq!(Format::from_extension("docx"), Some(Format::Docx));
        assert_eq!(Format::from_extension("DOCX"), Some(Format::Docx));
        assert_eq!(Format::from_extension("md"), Some(Format::Markdown));
        assert_eq!(Format::from_extension("markdown"), Some(Format::Markdown));
        assert_eq!(Format::from_extension("unknown"), None);
    }

    #[test]
    fn test_output_formats() {
        let docx_outputs = get_output_formats(Format::Docx);
        assert!(!docx_outputs.is_empty());
        assert!(docx_outputs.iter().any(|p| p.to == Format::Pdf && p.recommended));
    }
}
