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
    pub files: Vec<FileInfo>,
    pub state: ListState,
}

impl FilePicker {
    pub fn new(files: Vec<FileInfo>) -> Self {
        let mut state = ListState::default();
        if !files.is_empty() {
            state.select(Some(0));
        }
        Self { files, state }
    }

    /// Move selection up
    pub fn previous(&mut self) {
        if self.files.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.files.len() - 1
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
        if self.files.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.files.len() - 1 {
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
        self.state.selected().and_then(|i| self.files.get(i))
    }

    /// Render the file picker
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .files
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

        let list = List::new(items)
            .block(
                Block::default()
                    .title(" Select a file to convert ")
                    .title_style(Style::default().fg(theme::WHITE).add_modifier(Modifier::BOLD))
                    .borders(Borders::ALL)
                    .border_style(theme::border_focused()),
            )
            .highlight_style(theme::selected())
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, area, &mut self.state);
    }
}
