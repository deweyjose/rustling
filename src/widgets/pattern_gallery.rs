use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

use crate::app::App;
use crate::theme::Theme;

pub struct PatternGallery<'a> {
    app: &'a App,
    theme: &'a Theme,
}

impl<'a> PatternGallery<'a> {
    pub fn new(app: &'a App, theme: &'a Theme) -> Self {
        Self { app, theme }
    }
}

impl Widget for PatternGallery<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut lines: Vec<String> = Vec::new();

        for (type_idx, pattern_type) in self.app.configuration.iter().enumerate() {
            let selected = type_idx == self.app.current_pattern_type;
            lines.push(format!(
                "{} {}",
                if selected { ">" } else { " " },
                pattern_type.name
            ));

            let patterns = pattern_type
                .patterns
                .iter()
                .enumerate()
                .map(|(idx, p)| {
                    let num = idx + 1;
                    let marker = if Some(idx) == self.app.last_pattern && selected {
                        "*"
                    } else {
                        " "
                    };
                    format!("{marker}({num}) {}", p.name)
                })
                .collect::<Vec<String>>()
                .join(", ");

            lines.push(patterns);
            lines.push(String::new());
        }

        let text = lines.join("\n");

        let widget = Paragraph::new(text)
            .block(
                Block::bordered()
                    .title(" Patterns ")
                    .border_style(self.theme.border),
            )
            .style(self.theme.cell_dead);

        widget.render(area, buf);
    }
}
