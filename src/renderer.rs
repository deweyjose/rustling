use ratatui::prelude::*;

use crate::app::{App, AppMode};
use crate::theme::Theme;
use crate::widgets::{
    footer_bar::FooterBar, game_canvas::GameCanvas, header_bar::HeaderBar, help_popup::HelpPopup,
    pattern_gallery::PatternGallery,
};

pub struct RenderOutcome {
    pub canvas_area: Rect,
}

pub struct Renderer;

impl Renderer {
    pub fn render(
        frame: &mut Frame,
        app: &App,
        theme: &Theme,
        gallery_width: u16,
    ) -> RenderOutcome {
        let layout = Layout::vertical([
            Constraint::Length(3), // Header (border + content + border)
            Constraint::Fill(1),   // Body
            Constraint::Length(3), // Footer
        ])
        .split(frame.area());

        let body_layout = Layout::horizontal([
            Constraint::Fill(1),               // Canvas
            Constraint::Length(gallery_width), // Pattern Gallery
        ])
        .split(layout[1]);

        let canvas_area = body_layout[0];

        frame.render_widget(HeaderBar::new(app, theme), layout[0]);
        frame.render_widget(GameCanvas::new(app, theme), canvas_area);
        frame.render_widget(PatternGallery::new(app, theme), body_layout[1]);
        frame.render_widget(FooterBar::new(app, theme), layout[2]);

        if app.mode == AppMode::Help {
            frame.render_widget(HelpPopup::new(app, theme), frame.area());
        }

        RenderOutcome { canvas_area }
    }
}
