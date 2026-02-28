//! File picker widget
//!
//! Interactive list for selecting files to convert.

use crate::files::FileInfo;
use crate::ui::theme;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

/// File picker state
pub struct FilePicker {
    pub all_files: Vec<FileInfo>,
    pub filtered_files: Vec<FileInfo>,
    pub state: ListState,
    pub search_query: String,
    pub search_mode: bool,
}

impl FilePicker {
    pub fn new(files: Vec<FileInfo>) -> Self {
        let mut state = ListState::default();
        if !files.is_empty() {
            state.select(Some(0));
        }
        Self { 
            all_files: files.clone(),
            filtered_files: files, 
            state,
            search_query: String::new(),
            search_mode: false,
        }
    }

    /// Move selection up
    pub fn previous(&mut self) {
        if self.filtered_files.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.filtered_files.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    /// Move selection down
    pub fn next(&mut self) {
        if self.filtered_files.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.filtered_files.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    /// Get currently selected file
    pub fn selected(&self) -> Option<&FileInfo> {
        self.state.selected().and_then(|i| self.filtered_files.get(i))
    }

    /// Update filtered list based on search query
    pub fn update_filter(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_files = self.all_files.clone();
        } else {
            let query = self.search_query.to_lowercase();
            self.filtered_files = self.all_files
                .iter()
                .filter(|f| f.name.to_lowercase().contains(&query))
                .cloned()
                .collect();
        }
        
        // Reset selection when filtering
        if self.filtered_files.is_empty() {
            self.state.select(None);
        } else {
            self.state.select(Some(0));
        }
    }

    /// Render the file picker
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .filtered_files
            .iter()
            .map(|file| {
                let icon = file.format.icon();
                let name = &file.name;
                let size = file.size_display();
                let modified = file.modified_display();
                
                let line = Line::from(vec![
                    Span::raw("  "),
                    Span::styled(
                        format!("{} ", icon),
                        theme::muted(),
                    ),
                    Span::styled(
                        format!("{:<30}", name),
                        Style::default().fg(theme::WHITE),
                    ),
                    Span::styled(
                        format!("{:>8}", size),
                        theme::muted(),
                    ),
                    Span::raw("  "),
                    Span::styled(
                        modified,
                        theme::muted(),
                    ),
                ]);
                
                ListItem::new(line)
            })
            .collect();

        let title = if self.search_mode || !self.search_query.is_empty() {
            format!(" Select a file (Search: {}{}) ", self.search_query, if self.search_mode { "_" } else { "" })
        } else {
            " Select a file to convert ".to_string()
        };

        let list = List::new(items)
            .block(
                Block::default()
                    .title(title)
                    .title_style(Style::default().fg(theme::WHITE).add_modifier(Modifier::BOLD))
                    .borders(Borders::ALL)
                    .border_style(if self.search_mode { theme::selected() } else { theme::border_focused() }),
            )
            .highlight_style(theme::selected())
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, area, &mut self.state);
    }
}
