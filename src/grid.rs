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

    pub fn get_row(&self, row: usize) -> &Vec<Health> {
        &self.lines[row]
    }

    // resurrect a single cell
    pub fn resurrect(&mut self, position: Coordinates) {
        self.lines[position.y][position.x] = Alive;
    }

    // kill a single cell
    pub fn kill(&mut self, position: Coordinates) {
        self.lines[position.y][position.x] = Dead;
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

    // =============================
    // Positions relative a cell (C).
    //
    // E  - in front
    // SE - diagonal front below
    // S  - below
    // SW - diagonal behind below
    // W  - behind
    // NW - diagonal behind above
    // N - above
    // NE - diagonal front above
    //
    // =============================
    //
    //        NW     N    NE
    //           \   |   /
    //         W -   C   - E
    //           /   |   \
    //        SW     S    SE
    //
    // =============================

    fn compute_health(health: &Health, living_neighbors: usize) -> Health {
        match (health, living_neighbors) {
            (Alive, 2) => Alive,
            (Alive, 3) => Alive,
            (Dead, 3) => Alive,
            _ => Dead,
        }
    }

    fn count_living_neighbors(&self, row: usize, col: usize) -> usize {
        // Out-of-bounds neighbors are treated as dead.
        const OFFSETS: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let height = self.lines.len() as isize;
        let width = self.lines[0].len() as isize;

        OFFSETS
            .iter()
            .filter_map(|(dr, dc)| {
                let r = row as isize + dr;
                let c = col as isize + dc;
                if (0..height).contains(&r) && (0..width).contains(&c) {
                    Some(&self.lines[r as usize][c as usize])
                } else {
                    None
                }
            })
            .filter(|cell| matches!(cell, Alive))
            .count()
    }

    pub fn generate(&mut self) {
        let mut changed: Vec<(usize, usize, Health)> = Vec::new();
        let height = self.lines.len();
        let width = self.lines[0].len();

        for row in 0..height {
            for col in 0..width {
                let current = &self.lines[row][col];
                let living_neighbors = self.count_living_neighbors(row, col);
                let next = Self::compute_health(current, living_neighbors);

                if current != &next {
                    changed.push((row, col, next));
                }
            }
        }

        for (row, column, health) in changed {
            self.lines[row][column] = health;
        }
    }
}
