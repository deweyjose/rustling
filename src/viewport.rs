use crate::coordinates::Coordinates;
use crate::size::Size;

pub struct Viewport {
    x_offset: usize,
    y_offset: usize,
    viewport_size: Size,
}

impl Viewport {
    /// Create a new viewport, initially centered on the grid
    pub fn new(grid_size: &Size, viewport_size: Size) -> Self {
        let x_offset = if grid_size.width > viewport_size.width {
            (grid_size.width - viewport_size.width) / 2
        } else {
            0
        };

        let y_offset = if grid_size.height > viewport_size.height {
            (grid_size.height - viewport_size.height) / 2
        } else {
            0
        };

        Self {
            x_offset,
            y_offset,
            viewport_size,
        }
    }

    /// Convert viewport coordinates to grid coordinates
    pub fn view_to_grid(&self, view_coord: Coordinates) -> Coordinates {
        Coordinates {
            x: self.x_offset + view_coord.x.saturating_sub(1),
            y: self.y_offset + view_coord.y.saturating_sub(2),
        }
    }

    /// Convert grid coordinates to viewport coordinates
    #[allow(dead_code)]
    pub fn grid_to_view(&self, grid_coord: Coordinates) -> Option<Coordinates> {
        if grid_coord.x >= self.x_offset
            && grid_coord.x < self.x_offset + self.viewport_size.width
            && grid_coord.y >= self.y_offset
            && grid_coord.y < self.y_offset + self.viewport_size.height
        {
            Some(Coordinates {
                x: grid_coord.x - self.x_offset + 1,
                y: grid_coord.y - self.y_offset + 2,
            })
        } else {
            None
        }
    }

    /// Get the current x offset
    pub fn x_offset(&self) -> usize {
        self.x_offset
    }

    /// Get the current y offset
    pub fn y_offset(&self) -> usize {
        self.y_offset
    }

    /// Pan the viewport left
    #[allow(dead_code)]
    pub fn pan_left(&mut self, amount: usize, _grid_width: usize) {
        if self.x_offset >= amount {
            self.x_offset -= amount;
        } else {
            self.x_offset = 0;
        }
    }

    /// Pan the viewport right
    #[allow(dead_code)]
    pub fn pan_right(&mut self, amount: usize, grid_width: usize) {
        let max_offset = grid_width.saturating_sub(self.viewport_size.width);
        let new_offset = self.x_offset + amount;
        self.x_offset = if new_offset <= max_offset {
            new_offset
        } else {
            max_offset
        };
    }

    /// Pan the viewport up
    #[allow(dead_code)]
    pub fn pan_up(&mut self, amount: usize, _grid_height: usize) {
        if self.y_offset >= amount {
            self.y_offset -= amount;
        } else {
            self.y_offset = 0;
        }
    }

    /// Pan the viewport down
    #[allow(dead_code)]
    pub fn pan_down(&mut self, amount: usize, grid_height: usize) {
        let max_offset = grid_height.saturating_sub(self.viewport_size.height);
        let new_offset = self.y_offset + amount;
        self.y_offset = if new_offset <= max_offset {
            new_offset
        } else {
            max_offset
        };
    }

    /// Update viewport size (e.g., on terminal resize)
    pub fn update_size(&mut self, new_size: Size, grid_size: &Size) {
        // Maintain relative position, but ensure we stay within bounds
        self.viewport_size = new_size;

        let max_x_offset = grid_size.width.saturating_sub(self.viewport_size.width);
        if self.x_offset > max_x_offset {
            self.x_offset = max_x_offset;
        }

        let max_y_offset = grid_size.height.saturating_sub(self.viewport_size.height);
        if self.y_offset > max_y_offset {
            self.y_offset = max_y_offset;
        }
    }
}
