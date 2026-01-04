use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

use crate::app::{App, AppMode};
use crate::theme::Theme;

pub struct HeaderBar<'a> {
    app: &'a App,
    theme: &'a Theme,
}

impl<'a> HeaderBar<'a> {
    pub fn new(app: &'a App, theme: &'a Theme) -> Self {
        Self { app, theme }
    }
}

impl Widget for HeaderBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mode_label = match self.app.mode {
            AppMode::Normal => "Normal",
            AppMode::Help => "Help",
            AppMode::PatternGallery => "Pattern Gallery",
        };

        let header_text = format!("rustmaton - Game of Life (mode: {mode_label})");

        let widget = Paragraph::new(header_text)
            .block(Block::bordered())
            .style(self.theme.header_style);

        widget.render(area, buf);
    }
}
