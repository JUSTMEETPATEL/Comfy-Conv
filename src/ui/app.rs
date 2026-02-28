//! Main TUI application
//!
//! Manages application state and event loop.

use crate::deps::DependencyStatus;
use crate::error::{ConvError, Result};
use crate::files::FileInfo;
use crate::formats::ConversionPath;
use crate::ui::file_picker::FilePicker;
use crate::ui::format_picker::FormatPicker;
use crate::ui::theme;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io::{self, stdout};
use std::path::PathBuf;

/// Application state
pub enum AppState {
    SelectFile(FilePicker),
    SelectFormat(FormatPicker, FileInfo),
    Converting {
        source: FileInfo,
        target: ConversionPath,
    },
    Success {
        output_path: PathBuf,
        size: u64,
    },
    Error {
        error: ConvError,
    },
    Exit,
}

/// Main application
pub struct App {
    pub state: AppState,
    pub deps: DependencyStatus,
}

impl App {
    pub fn new(files: Vec<FileInfo>, deps: DependencyStatus) -> Self {
        Self {
            state: AppState::SelectFile(FilePicker::new(files)),
            deps,
        }
    }

    /// Run the TUI application
    pub fn run(mut self) -> Result<Option<(FileInfo, ConversionPath)>> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = self.event_loop(&mut terminal);

        // Restore terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        result
    }

    fn event_loop(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    ) -> Result<Option<(FileInfo, ConversionPath)>> {
        loop {
            terminal.draw(|f| self.render(f))?;

            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match &mut self.state {
                    AppState::SelectFile(picker) => {
                        if picker.search_mode {
                            match key.code {
                                KeyCode::Char(c) => {
                                    picker.search_query.push(c);
                                    picker.update_filter();
                                }
                                KeyCode::Backspace => {
                                    picker.search_query.pop();
                                    picker.update_filter();
                                }
                                KeyCode::Up => picker.previous(),
                                KeyCode::Down => picker.next(),
                                KeyCode::Enter => {
                                    if let Some(file) = picker.selected().cloned() {
                                        let format_picker = FormatPicker::new(&file.path, file.format);
                                        self.state = AppState::SelectFormat(format_picker, file);
                                    }
                                }
                                KeyCode::Esc => {
                                    picker.search_mode = false;
                                }
                                _ => {}
                            }
                        } else {
                            match key.code {
                                KeyCode::Char('/') => {
                                    picker.search_mode = true;
                                }
                                KeyCode::Up | KeyCode::Char('k') => picker.previous(),
                                KeyCode::Down | KeyCode::Char('j') => picker.next(),
                                KeyCode::Enter => {
                                    if let Some(file) = picker.selected().cloned() {
                                        let format_picker = FormatPicker::new(&file.path, file.format);
                                        self.state = AppState::SelectFormat(format_picker, file);
                                    }
                                }
                                KeyCode::Esc => {
                                    if !picker.search_query.is_empty() {
                                        picker.search_query.clear();
                                        picker.update_filter();
                                    } else {
                                        self.state = AppState::Exit;
                                    }
                                }
                                KeyCode::Char('q') => {
                                    self.state = AppState::Exit;
                                }
                                _ => {}
                            }
                        }
                    },
                    AppState::SelectFormat(picker, file) => match key.code {
                        KeyCode::Up | KeyCode::Char('k') => picker.previous(),
                        KeyCode::Down | KeyCode::Char('j') => picker.next(),
                        KeyCode::Enter => {
                            if let Some(conversion) = picker.selected().cloned() {
                                // Return selection for conversion
                                return Ok(Some((file.clone(), conversion)));
                            }
                        }
                        KeyCode::Esc => {
                            // Go back to file selection
                            if let AppState::SelectFormat(_, _) = &self.state {
                                // Recreate file picker - we need to get files again
                                // For now, just exit
                                self.state = AppState::Exit;
                            }
                        }
                        KeyCode::Char('q') => {
                            self.state = AppState::Exit;
                        }
                        _ => {}
                    },
                    AppState::Exit => {
                        return Ok(None);
                    }
                    _ => {}
                }

                if matches!(self.state, AppState::Exit) {
                    return Ok(None);
                }
            }
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.size();
        
        // Create layout with main area and hint bar
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(5), Constraint::Length(2)])
            .split(area);

        // Handle Success/Error states first (they don't need mutable state)
        match &self.state {
            AppState::Success { output_path, size } => {
                let output_path = output_path.clone();
                let size = *size;
                render_success(frame, chunks[0], &output_path, size);
                render_hints(frame, chunks[1], &["Enter: convert another", "q: quit"]);
                return;
            }
            AppState::Error { error } => {
                let suggestions = error.suggestions();
                let error_msg = error.to_string();
                render_error(frame, chunks[0], &error_msg, &suggestions);
                render_hints(frame, chunks[1], &["Enter: try again", "q: quit"]);
                return;
            }
            _ => {}
        }

        // Handle states that need mutable access to pickers
        match &mut self.state {
            AppState::SelectFile(picker) => {
                picker.render(frame, chunks[0]);
                if picker.search_mode {
                    render_hints(frame, chunks[1], &["Type to search", "Enter: select", "Esc: exit search"]);
                } else {
                    render_hints(frame, chunks[1], &["↑↓/jk: navigate", "/: search", "Enter: select", "Esc/q: quit"]);
                }
            }
            AppState::SelectFormat(picker, _) => {
                picker.render(frame, chunks[0]);
                render_hints(frame, chunks[1], &["↑↓/jk: navigate", "Enter: convert", "Esc: back", "q: quit"]);
            }
            _ => {}
        }
    }
}

fn render_hints(frame: &mut Frame, area: Rect, hints: &[&str]) {
    let hint_text = hints.join("  │  ");
    let hint = Paragraph::new(Line::from(vec![
        Span::styled("  ", theme::muted()),
        Span::styled(hint_text, theme::hint()),
    ]));
    frame.render_widget(hint, area);
}

fn render_success(frame: &mut Frame, area: Rect, output_path: &PathBuf, size: u64) {
    let size_str = format_size(size);
    let path_str = output_path.to_string_lossy();
    
    let text = vec![
        Line::from(""),
        Line::from(Span::styled("  ✓ Conversion complete!", theme::success())),
        Line::from(""),
        Line::from(vec![
            Span::raw("  📕 "),
            Span::styled(
                output_path.file_name().unwrap_or_default().to_string_lossy().to_string(),
                theme::filename(),
            ),
            Span::styled(format!(" ({})", size_str), theme::muted()),
        ]),
        Line::from(vec![
            Span::styled("  📂 ", theme::muted()),
            Span::styled(path_str.to_string(), theme::muted()),
        ]),
    ];

    let block = Block::default()
        .title(" Success ")
        .title_style(theme::success().add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(theme::border_focused());

    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

fn render_error(frame: &mut Frame, area: Rect, error_msg: &str, suggestions: &[String]) {
    let mut lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  ✗ ", theme::error()),
            Span::styled(error_msg.to_string(), theme::error()),
        ]),
        Line::from(""),
    ];

    if !suggestions.is_empty() {
        lines.push(Line::from(Span::styled("  Suggestions:", theme::muted())));
        for suggestion in suggestions {
            lines.push(Line::from(vec![
                Span::raw("  • "),
                Span::styled(suggestion.clone(), theme::muted()),
            ]));
        }
    }

    let block = Block::default()
        .title(" Error ")
        .title_style(theme::error().add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(theme::border());

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
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
