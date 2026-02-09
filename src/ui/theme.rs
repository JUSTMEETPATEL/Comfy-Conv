//! Color theme for comfy-conv UI
//!
//! Inspired by Next.js CLI aesthetics.

use ratatui::style::{Color, Modifier, Style};

// Color palette from PRD
pub const ACCENT: Color = Color::Rgb(0, 217, 255);      // #00D9FF Cyan
pub const SUCCESS: Color = Color::Rgb(0, 255, 0);       // #00FF00 Green
pub const ERROR: Color = Color::Rgb(255, 68, 68);       // #FF4444 Red
pub const MUTED: Color = Color::Rgb(136, 136, 136);     // #888888 Gray
pub const BORDER: Color = Color::Rgb(68, 68, 68);       // #444444 Dark Gray
pub const WHITE: Color = Color::White;

/// Style for selected/highlighted items
pub fn selected() -> Style {
    Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
}

/// Style for normal list items
pub fn normal() -> Style {
    Style::default().fg(WHITE)
}

/// Style for muted/secondary text
pub fn muted() -> Style {
    Style::default().fg(MUTED)
}

/// Style for success messages
pub fn success() -> Style {
    Style::default().fg(SUCCESS).add_modifier(Modifier::BOLD)
}

/// Style for error messages
pub fn error() -> Style {
    Style::default().fg(ERROR).add_modifier(Modifier::BOLD)
}

/// Style for borders
pub fn border() -> Style {
    Style::default().fg(BORDER)
}

/// Style for highlighted borders (when focused)
pub fn border_focused() -> Style {
    Style::default().fg(ACCENT)
}

/// Style for file names
pub fn filename() -> Style {
    Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
}

/// Style for hints/instructions
pub fn hint() -> Style {
    Style::default().fg(MUTED).add_modifier(Modifier::ITALIC)
}
