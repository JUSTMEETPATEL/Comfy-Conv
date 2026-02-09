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
use crate::files::generate_alternative_name;
use crate::ui::app::App;
use crate::ui::collision_dialog::{CollisionChoice, CollisionDialog};
use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};
use std::io::stdout;
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

    // Run TUI to get file and format selection
    let app = App::new(files, deps.clone());
    let selection = match app.run() {
        Ok(Some(sel)) => sel,
        Ok(None) => return, // User cancelled
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    };

    let (file, conversion) = selection;
    
    // Calculate output path
    let output_dir = file.path.parent().unwrap_or(std::path::Path::new("."));
    let stem = file.path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let output_path = output_dir.join(format!("{}.{}", stem, conversion.to.extension()));

    // Check for collision and handle it
    let final_output_path = if output_path.exists() {
        match run_collision_dialog(&output_path) {
            Some(path) => path,
            None => {
                println!("Conversion cancelled.");
                return;
            }
        }
    } else {
        output_path.clone()
    };

    // Perform the conversion
    println!("\n⏳ Converting {} to {}...", file.name, conversion.to.extension());
    
    // If renaming, we need to convert to original path then rename
    // For simplicity, we convert directly - the engine handles output naming
    match convert::convert(&file, &conversion, &deps) {
        Ok(converted_path) => {
            // If user chose rename, move the file
            let final_path = if final_output_path != converted_path && final_output_path != output_path {
                if let Err(e) = std::fs::rename(&converted_path, &final_output_path) {
                    eprintln!("⚠️  Could not rename to {}: {}", final_output_path.display(), e);
                    converted_path
                } else {
                    final_output_path
                }
            } else {
                converted_path
            };

            let size = std::fs::metadata(&final_path)
                .map(|m| m.len())
                .unwrap_or(0);
            let size_str = format_size(size);
            
            println!("\n✅ Conversion complete!");
            println!("📕 {} ({})", final_path.file_name().unwrap_or_default().to_string_lossy(), size_str);
            println!("📂 {}", final_path.display());
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

/// Run collision dialog in TUI and return chosen path
fn run_collision_dialog(existing_path: &PathBuf) -> Option<PathBuf> {
    // Setup terminal
    enable_raw_mode().ok()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen).ok()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).ok()?;

    let mut dialog = CollisionDialog::new(existing_path.clone());
    let result;

    loop {
        terminal.draw(|f| {
            let area = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(8), Constraint::Length(2)])
                .split(area);
            
            dialog.render(f, chunks[0]);
            
            // Render hints
            let hint = ratatui::widgets::Paragraph::new(
                "  ↑↓/jk: navigate  │  Enter: select  │  Esc: cancel"
            ).style(crate::ui::theme::hint());
            f.render_widget(hint, chunks[1]);
        }).ok()?;

        if let Event::Key(key) = event::read().ok()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Up | KeyCode::Char('k') => dialog.previous(),
                KeyCode::Down | KeyCode::Char('j') => dialog.next(),
                KeyCode::Enter => {
                    result = dialog.output_path();
                    break;
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    result = None;
                    break;
                }
                _ => {}
            }
        }
    }

    // Restore terminal
    disable_raw_mode().ok()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen).ok()?;
    terminal.show_cursor().ok()?;

    result
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
