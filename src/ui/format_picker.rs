//! Format picker widget
//!
//! Interactive list for selecting output format.

use crate::formats::{get_output_formats, ConversionPath, Format};
use crate::ui::theme;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use std::path::Path;

/// Format picker state
pub struct FormatPicker {
    pub source_file: String,
    pub source_format: Format,
    pub formats: Vec<ConversionPath>,
    pub state: ListState,
}

impl FormatPicker {
    pub fn new(source_path: &Path, source_format: Format) -> Self {
        let formats = get_output_formats(source_format);
        let mut state = ListState::default();
        
        // Select recommended format by default, or first if none
        let recommended_idx = formats.iter().position(|f| f.recommended).unwrap_or(0);
        if !formats.is_empty() {
            state.select(Some(recommended_idx));
        }
        
        let source_file = source_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file")
            .to_string();
        
        Self {
            source_file,
            source_format,
            formats,
            state,
        }
    }

    /// Move selection up
    pub fn previous(&mut self) {
        if self.formats.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.formats.len() - 1
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
        if self.formats.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.formats.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    /// Get currently selected conversion path
    pub fn selected(&self) -> Option<&ConversionPath> {
        self.state.selected().and_then(|i| self.formats.get(i))
    }

    /// Get output filename for current selection
    pub fn output_filename(&self) -> Option<String> {
        self.selected().map(|path| {
            let stem = Path::new(&self.source_file)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("output");
            format!("{}.{}", stem, path.to.extension())
        })
    }

    /// Render the format picker
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .formats
            .iter()
            .map(|path| {
                let icon = path.to.display_name();
                let desc = path.to.description();
                let engine = path.engine.display_name();
                
                let line = Line::from(vec![
                    Span::raw("  "),
                    Span::styled(
                        format!("{:<12}", icon),
                        Style::default().fg(theme::WHITE),
                    ),
                    Span::styled(
                        format!("{:<30}", desc),
                        theme::muted(),
                    ),
                    Span::styled(
                        format!("({})", engine),
                        theme::muted(),
                    ),
                ]);
                
                ListItem::new(line)
            })
            .collect();

        let title = format!(" Convert {} to: ", self.source_file);
        
        let list = List::new(items)
            .block(
                Block::default()
                    .title(title)
                    .title_style(Style::default().fg(theme::WHITE).add_modifier(Modifier::BOLD))
                    .borders(Borders::ALL)
                    .border_style(theme::border_focused()),
            )
            .highlight_style(theme::selected())
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, area, &mut self.state);

        // Render output filename preview at bottom
        if let Some(output) = self.output_filename() {
            let preview_area = Rect {
                x: area.x + 2,
                y: area.y + area.height.saturating_sub(2),
                width: area.width.saturating_sub(4),
                height: 1,
            };
            let preview = Paragraph::new(Line::from(vec![
                Span::styled("Output: ", theme::muted()),
                Span::styled(output, theme::filename()),
            ]));
            frame.render_widget(preview, preview_area);
        }
    }
}
