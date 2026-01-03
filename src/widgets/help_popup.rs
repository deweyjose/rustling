use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap};

use crate::app::App;
use crate::theme::Theme;

const HELP_TEXT: &str = r#"
# command keys:
a       - toggle cursor point alive
b       - move cursor to the beginning of the current line
c       - clear the screen
d       - toggle cursor point dead
e       - move cursor to the end of the current line
h       - display help, or exit help if currently rendered
l       - print the previous pattern again
p       - cycle through the pattern classes defined in patterns.json
q       - quit
r       - rotate the current shape 90 degrees
s       - toggle the simulation run loop
' '     - step the simulation forward
+       - speed up the simulation
-       - slow down the simulation
[esc]   - exit help
ctrl+c  - quit
"#;

pub struct HelpPopup<'a> {
    #[allow(dead_code)]
    app: &'a App,
    theme: &'a Theme,
}

impl<'a> HelpPopup<'a> {
    pub fn new(app: &'a App, theme: &'a Theme) -> Self {
        Self { app, theme }
    }
}

impl Widget for HelpPopup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Center the popup at 60% width and 80% height of the current area.
        let popup_area = centered_rect(60, 80, area);

        let block = Block::default()
            .title(" Help ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(self.theme.border);

        let help_text = Paragraph::new(HELP_TEXT)
            .block(block)
            .wrap(Wrap { trim: false })
            .style(self.theme.cell_alive);

        Clear.render(popup_area, buf);
        help_text.render(popup_area, buf);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    let vertical = popup_layout[1];

    let horizontal_layout = Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(vertical);

    horizontal_layout[1]
}
