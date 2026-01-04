use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    pub header_style: Style,
    pub footer_style: Style,
    pub cell_alive: Style,
    pub cell_dead: Style,
    pub cursor: Style,
    pub border: Style,
    pub grid_boundary: Style,
    pub gallery_focus: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            header_style: Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            footer_style: Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            cell_alive: Style::default().fg(Color::Green),
            cell_dead: Style::default(),
            cursor: Style::default().bg(Color::DarkGray),
            border: Style::default().fg(Color::Gray),
            grid_boundary: Style::default().fg(Color::DarkGray),
            gallery_focus: Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        }
    }
}
