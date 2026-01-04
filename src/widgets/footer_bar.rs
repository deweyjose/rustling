use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

use crate::app::App;
use crate::theme::Theme;

pub struct FooterBar<'a> {
    app: &'a App,
    theme: &'a Theme,
}

impl<'a> FooterBar<'a> {
    pub fn new(app: &'a App, theme: &'a Theme) -> Self {
        Self { app, theme }
    }
}

impl Widget for FooterBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let grid_size = self.app.grid.get_size();
        let cursor_grid = self.app.grid_cursor();
        let running_label = if self.app.running {
            "running"
        } else {
            "paused"
        };

        let (last_pattern, rotation_angle) = if let Some(last) = self.app.last_pattern {
            if let Some(pattern_type) = self.app.configuration.get(self.app.current_pattern_type) {
                if let Some(pattern) = pattern_type.patterns.get(last) {
                    (pattern.name.as_str(), self.app.rotation_degrees())
                } else {
                    ("none", 0)
                }
            } else {
                ("none", 0)
            }
        } else {
            ("none", 0)
        };

        let footer = format!(
            "grid {}, viewport {}, cursor {}, {}, pattern: {}, last: {}, rotation: {}°",
            grid_size,
            self.app.viewport_size,
            cursor_grid,
            running_label,
            self.app.current_pattern_type_name(),
            last_pattern,
            rotation_angle
        );

        let widget = Paragraph::new(footer)
            .block(Block::bordered())
            .style(self.theme.footer_style);

        widget.render(area, buf);
    }
}
