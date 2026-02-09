//! File collision dialog
//!
//! Interactive dialog for handling file name conflicts.

use crate::files::generate_alternative_name;
use crate::ui::theme;
use ratatui::{
    layout::Rect,
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use std::path::PathBuf;

/// User's choice for handling file collision
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CollisionChoice {
    Overwrite,
    Rename,
    Cancel,
}

/// Collision dialog state
pub struct CollisionDialog {
    pub existing_path: PathBuf,
    pub existing_size: u64,
    pub alternative_path: PathBuf,
    pub state: ListState,
    choices: Vec<CollisionChoice>,
}

impl CollisionDialog {
    pub fn new(existing_path: PathBuf) -> Self {
        let existing_size = std::fs::metadata(&existing_path)
            .map(|m| m.len())
            .unwrap_or(0);
        
        let alternative_path = generate_alternative_name(&existing_path);
        
        let mut state = ListState::default();
        state.select(Some(0)); // Default to Overwrite
        
        Self {
            existing_path,
            existing_size,
            alternative_path,
            state,
            choices: vec![
                CollisionChoice::Overwrite,
                CollisionChoice::Rename,
                CollisionChoice::Cancel,
            ],
        }
    }

    /// Move selection up
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.choices.len() - 1
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
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.choices.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    /// Get currently selected choice
    pub fn selected(&self) -> Option<CollisionChoice> {
        self.state.selected().map(|i| self.choices[i])
    }

    /// Get the final output path based on current selection
    pub fn output_path(&self) -> Option<PathBuf> {
        match self.selected()? {
            CollisionChoice::Overwrite => Some(self.existing_path.clone()),
            CollisionChoice::Rename => Some(self.alternative_path.clone()),
            CollisionChoice::Cancel => None,
        }
    }

    /// Render the collision dialog
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let filename = self.existing_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file");
        let alt_filename = self.alternative_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file");
        let size_str = format_size(self.existing_size);

        // Create warning header
        let header_lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  ⚠️  ", theme::selected()),
                Span::styled(filename, theme::filename()),
                Span::styled(format!(" already exists ({})", size_str), theme::muted()),
            ]),
            Line::from(""),
        ];

        // Render header area
        let header_height = 4;
        let header_area = Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: header_height.min(area.height),
        };
        
        let block = Block::default()
            .title(" File Already Exists ")
            .title_style(theme::selected().add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_style(theme::border_focused());

        let header = Paragraph::new(header_lines).block(block);
        frame.render_widget(header, area);

        // Render choices as list inside the block
        let list_area = Rect {
            x: area.x + 2,
            y: area.y + header_height,
            width: area.width.saturating_sub(4),
            height: area.height.saturating_sub(header_height + 1),
        };

        let items: Vec<ListItem> = self.choices.iter().map(|choice| {
            let text = match choice {
                CollisionChoice::Overwrite => "Overwrite".to_string(),
                CollisionChoice::Rename => format!("Rename to {}", alt_filename),
                CollisionChoice::Cancel => "Cancel".to_string(),
            };
            ListItem::new(Line::from(Span::raw(format!("  {}", text))))
        }).collect();

        let list = List::new(items)
            .highlight_style(theme::selected())
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, list_area, &mut self.state);
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
