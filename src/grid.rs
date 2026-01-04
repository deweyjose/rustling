use crate::coordinates::Coordinates;
use crate::health::Health;
use crate::health::Health::{Alive, Dead};
use crate::size::Size;

#[allow(dead_code)]
pub struct Grid {
    lines: Vec<Vec<Health>>,
    size: Size,
}

impl Grid {
    pub fn new(size: Size) -> Self {
        Self {
            lines: Grid::init_grid(&size),
            size,
        }
    }

    fn init_grid(size: &Size) -> Vec<Vec<Health>> {
        let mut line = Vec::new();

        for _ in 0..size.width {
            line.push(Dead);
        }

        let mut lines = Vec::new();

        for _ in 0..size.height {
            lines.push(line.clone());
        }

        lines
    }

    pub fn get_size(&self) -> &Size {
        &self.size
    }

    pub fn get_cell(&self, position: Coordinates) -> Option<Health> {
        if self.is_valid_position(&position) {
            Some(self.lines[position.y][position.x])
        } else {
            None
        }
    }

    fn is_valid_position(&self, position: &Coordinates) -> bool {
        position.y < self.size.height && position.x < self.size.width
    }

    // resurrect a single cell
    pub fn resurrect(&mut self, position: Coordinates) {
        if self.is_valid_position(&position) {
            self.lines[position.y][position.x] = Alive;
        }
    }

    // kill a single cell
    pub fn kill(&mut self, position: Coordinates) {
        if self.is_valid_position(&position) {
            self.lines[position.y][position.x] = Dead;
        }
    }

    pub fn shape(&mut self, position: Coordinates, shape: &[Vec<Health>]) {
        for row in shape.iter().enumerate() {
            let grid_row = position.y + row.0;
            if grid_row < self.size.height {
                for column in row.1.iter().enumerate() {
                    let column_row = position.x + column.0;
                    if column_row < self.size.width {
                        self.lines[grid_row][column_row] = shape[row.0][column.0];
                    }
                }
            } else {
                break;
            }
        }
    }

    fn compute_health(health: &Health, living_neighbors: usize) -> Health {
        match (health, living_neighbors) {
            (Alive, 2) => Alive,
            (Alive, 3) => Alive,
            (Dead, 3) => Alive,
            _ => Dead,
        }
    }

    fn is_alive(cell: &Health) -> bool {
        matches!(cell, Alive)
    }

    fn count_living_neighbors(&self, row: usize, col: usize) -> usize {
        let mut count = 0;
        let height = self.size.height;
        let width = self.size.width;

        // Define the 8 directions: NW, N, NE, W, E, SW, S, SE
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for (dr, dc) in directions {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;

            // Check bounds and if cell is alive
            if new_row >= 0
                && new_row < height as i32
                && new_col >= 0
                && new_col < width as i32
                && Self::is_alive(&self.lines[new_row as usize][new_col as usize])
            {
                count += 1;
            }
        }

        count
    }

    pub fn generate(&mut self) {
        let mut changed: Vec<(usize, usize, Health)> = Vec::new();

        for (row_idx, row) in self.lines.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                let living_neighbors = self.count_living_neighbors(row_idx, col_idx);
                let new_health = Self::compute_health(cell, living_neighbors);

                if cell != &new_health {
                    changed.push((row_idx, col_idx, new_health));
                }
            }
        }

        // Apply all changes
        for (row, col, health) in changed {
            self.lines[row][col] = health;
        }
    }
}
