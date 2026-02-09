//! comfy-conv - Interactive CLI Document Converter
//!
//! Convert documents between common formats through a beautiful terminal interface.

mod deps;
mod error;
mod files;
mod formats;

use crate::deps::{check_dependencies, libreoffice_install_hint, pandoc_install_hint};
use crate::error::ConvError;
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
    if !args.skip_dep_check {
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
    }

    // Discover files
    let current_dir = std::env::current_dir().expect("Cannot access current directory");
    match files::discover_files(&current_dir) {
        Ok(files) => {
            println!("\n📁 Found {} convertible file(s):\n", files.len());
            for file in &files {
                println!(
                    "   {} {:>10}  {}",
                    file.format.display_name(),
                    file.size_display(),
                    file.name
                );
            }
            println!("\n✨ TUI interface coming in Phase 3...\n");
        }
        Err(ConvError::NoFilesFound) => {
            println!("\n📭 No convertible files found in current directory.\n");
            println!("Supported formats: docx, xlsx, pptx, md, html, txt, pdf");
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }
}
