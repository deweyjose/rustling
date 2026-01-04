use ratatui::prelude::*;

use crate::app::App;
use crate::health::Health;
use crate::theme::Theme;

const DEAD_SYMBOL: &str = " ";
const ALIVE_SYMBOL: &str = "🚀";

// Box-drawing characters for grid boundary
const BOUNDARY_VERTICAL: &str = "│";
const BOUNDARY_HORIZONTAL: &str = "─";
const BOUNDARY_CORNER: &str = "┘";

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

        // Calculate where the grid ends in viewport coordinates
        let grid_end_x = grid_size.width.saturating_sub(self.app.viewport.x_offset());
        let grid_end_y = grid_size.height.saturating_sub(self.app.viewport.y_offset());

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

        // Draw grid boundary if viewport extends beyond grid
        let boundary_style = self.theme.grid_boundary;

        // Draw right edge (vertical line) if grid width < viewport width
        if grid_end_x < max_width as usize {
            let boundary_x = area.x + grid_end_x as u16;
            for y in 0..grid_end_y.min(max_height as usize) {
                if boundary_x < area.x + area.width {
                    let cell = &mut buf[(boundary_x, area.y + y as u16)];
                    cell.set_symbol(BOUNDARY_VERTICAL);
                    cell.set_style(boundary_style);
                }
            }
        }

        // Draw bottom edge (horizontal line) if grid height < viewport height
        if grid_end_y < max_height as usize {
            let boundary_y = area.y + grid_end_y as u16;
            for x in 0..grid_end_x.min(max_width as usize) {
                if boundary_y < area.y + area.height {
                    let cell = &mut buf[(area.x + x as u16, boundary_y)];
                    cell.set_symbol(BOUNDARY_HORIZONTAL);
                    cell.set_style(boundary_style);
                }
            }
        }

        // Draw corner if both edges are visible
        if grid_end_x < max_width as usize && grid_end_y < max_height as usize {
            let corner_x = area.x + grid_end_x as u16;
            let corner_y = area.y + grid_end_y as u16;
            if corner_x < area.x + area.width && corner_y < area.y + area.height {
                let cell = &mut buf[(corner_x, corner_y)];
                cell.set_symbol(BOUNDARY_CORNER);
                cell.set_style(boundary_style);
            }
        }
    }
}
