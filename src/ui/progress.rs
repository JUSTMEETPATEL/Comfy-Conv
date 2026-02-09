//! Progress screen widget
//!
//! Shows conversion progress with spinner animation.

use crate::formats::{ConversionPath, Format};
use crate::ui::theme;
use ratatui::{
    layout::Rect,
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Spinner animation frames
const SPINNER_FRAMES: &[char] = &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

/// Progress screen state
pub struct Progress {
    pub source_name: String,
    pub source_format: Format,
    pub target: ConversionPath,
    pub status: String,
    pub spinner_idx: usize,
}

impl Progress {
    pub fn new(source_name: String, source_format: Format, target: ConversionPath) -> Self {
        let engine_name = target.engine.display_name();
        Self {
            source_name,
            source_format,
            target,
            status: format!("Starting {}...", engine_name),
            spinner_idx: 0,
        }
    }

    /// Advance spinner animation
    pub fn tick(&mut self) {
        self.spinner_idx = (self.spinner_idx + 1) % SPINNER_FRAMES.len();
    }

    /// Update status message
    pub fn set_status(&mut self, status: &str) {
        self.status = status.to_string();
    }

    /// Get current spinner character
    fn spinner(&self) -> char {
        SPINNER_FRAMES[self.spinner_idx]
    }

    /// Render the progress screen
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let source_icon = self.source_format.display_name();
        let target_icon = self.target.to.display_name();
        let output_name = format!(
            "{}.{}",
            std::path::Path::new(&self.source_name)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("output"),
            self.target.to.extension()
        );

        let lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled(source_icon, theme::muted()),
                Span::raw(" "),
                Span::styled(&self.source_name, theme::filename()),
                Span::styled("  →  ", theme::muted()),
                Span::styled(target_icon, theme::muted()),
                Span::raw(" "),
                Span::styled(output_name, theme::filename()),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    format!("{} ", self.spinner()),
                    theme::selected(),
                ),
                Span::styled(&self.status, theme::muted()),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "  This may take a few seconds",
                theme::hint(),
            )),
        ];

        let block = Block::default()
            .title(" Converting ")
            .title_style(theme::selected().add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_style(theme::border_focused());

        let paragraph = Paragraph::new(lines).block(block);
        frame.render_widget(paragraph, area);
    }
}
