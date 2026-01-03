use ratatui::prelude::*;

use crate::app::App;
use crate::health::Health;
use crate::theme::Theme;

const DEAD_SYMBOL: &str = " ";
const ALIVE_SYMBOL: &str = "@";

pub struct GameCanvas<'a> {
    app: &'a App,
    theme: &'a Theme,
}

impl<'a> GameCanvas<'a> {
    pub fn new(app: &'a App, theme: &'a Theme) -> Self {
        Self { app, theme }
    }
}

impl Widget for GameCanvas<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let grid_size = self.app.grid.get_size();

        let max_height = area.height.min(self.app.viewport_size.height as u16);
        let max_width = area.width.min(self.app.viewport_size.width as u16);

        for y in 0..max_height {
            let grid_y = self.app.viewport.y_offset() + y as usize;
            if grid_y >= grid_size.height {
                continue;
            }

            for x in 0..max_width {
                let grid_x = self.app.viewport.x_offset() + x as usize;
                if grid_x >= grid_size.width {
                    continue;
                }

                let coord = crate::coordinates::Coordinates {
                    x: grid_x,
                    y: grid_y,
                };
                let health = self.app.grid.get_cell(coord).unwrap_or(Health::Dead);

                let mut style = match health {
                    Health::Alive => self.theme.cell_alive,
                    Health::Dead => self.theme.cell_dead,
                };

                if self.app.cursor.x == x as usize && self.app.cursor.y == y as usize {
                    style = style.patch(self.theme.cursor);
                }

                let symbol = match health {
                    Health::Alive => ALIVE_SYMBOL,
                    Health::Dead => DEAD_SYMBOL,
                };

                let cell = &mut buf[(area.x + x, area.y + y)];
                cell.set_symbol(symbol);
                cell.set_style(style);
            }
        }
    }
}
