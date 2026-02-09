//! comfy-conv - Interactive CLI Document Converter
//!
//! Convert documents between common formats through a beautiful terminal interface.

mod convert;
mod deps;
mod error;
mod files;
mod formats;
mod ui;

use crate::deps::{check_dependencies, libreoffice_install_hint, pandoc_install_hint};
use crate::error::ConvError;
use crate::ui::app::App;
use clap::Parser;
use std::path::PathBuf;

/// Interactive document converter for the terminal
#[derive(Parser, Debug)]
#[command(name = "comfy-conv")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file to convert (optional, will prompt if not provided)
    #[arg(value_name = "FILE")]
    input: Option<PathBuf>,

    /// Skip dependency check
    #[arg(long, hide = true)]
    skip_dep_check: bool,
}

fn main() {
    let args = Args::parse();

    // Check dependencies first
    let deps = if args.skip_dep_check {
        deps::DependencyStatus {
            libreoffice: Some("skipped".to_string()),
            pandoc: Some("skipped".to_string()),
        }
    } else {
        match check_dependencies() {
            Ok(status) => {
                if !status.has_libreoffice() {
                    eprintln!(
                        "⚠️  LibreOffice not found - Office document conversions disabled\n   {}",
                        libreoffice_install_hint()
                    );
                }
                if !status.has_pandoc() {
                    eprintln!(
                        "⚠️  Pandoc not found - Text format conversions disabled\n   {}",
                        pandoc_install_hint()
                    );
                }
                status
            }
            Err(ConvError::NoEnginesAvailable) => {
                eprintln!("❌ No conversion engines found!\n");
                eprintln!("Install at least one of:\n");
                eprintln!("LibreOffice (for Office documents):");
                eprintln!("   {}\n", libreoffice_install_hint());
                eprintln!("Pandoc (for text formats):");
                eprintln!("   {}", pandoc_install_hint());
                std::process::exit(1);
            }
            Err(e) => {
                eprintln!("❌ Dependency check failed: {}", e);
                std::process::exit(1);
            }
        }
    };

    // Discover files
    let current_dir = std::env::current_dir().expect("Cannot access current directory");
    let files = match files::discover_files(&current_dir) {
        Ok(files) => files,
        Err(ConvError::NoFilesFound) => {
            eprintln!("\n📭 No convertible files found in current directory.\n");
            eprintln!("Supported formats: docx, xlsx, pptx, md, html, txt, pdf");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    };

    // Run TUI
    let app = App::new(files, deps.clone());
    match app.run() {
        Ok(Some((file, conversion))) => {
            // Perform the conversion
            println!("\n⏳ Converting {} to {}...", file.name, conversion.to.extension());
            
            match convert::convert(&file, &conversion, &deps) {
                Ok(output_path) => {
                    let size = std::fs::metadata(&output_path)
                        .map(|m| m.len())
                        .unwrap_or(0);
                    let size_str = format_size(size);
                    
                    println!("\n✅ Conversion complete!");
                    println!("📕 {} ({})", output_path.file_name().unwrap_or_default().to_string_lossy(), size_str);
                    println!("📂 {}", output_path.display());
                }
                Err(e) => {
                    eprintln!("\n❌ Conversion failed: {}", e);
                    for suggestion in e.suggestions() {
                        eprintln!("   • {}", suggestion);
                    }
                    std::process::exit(1);
                }
            }
        }
        Ok(None) => {
            // User cancelled
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;

    if size >= MB {
        format!("{:.1} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{} KB", size / KB)
    } else {
        format!("{} B", size)
    }
}
