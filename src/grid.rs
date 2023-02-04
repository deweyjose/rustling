use crate::coordinates::Coordinates;
use crate::grid::Health::Alive;
use crate::grid::Health::Dead;
use crate::health::Health;
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

    // clear the grid
    pub fn clear(&mut self) {
        self.lines = Grid::init_grid(&self.size);
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

    fn is_alive(cell: &Health) -> bool {
        match cell {
            Alive => true,
            Dead => false,
        }
    }

    // check to the north east
    fn is_alive_ne(&self, current_row: usize, current_column: usize) -> usize {
        match Self::is_alive(&self.lines[current_row - 1][current_column + 1]) {
            true => 1,
            false => 0,
        }
    }

    // check to the north
    fn is_alive_n(&self, current_row: usize, current_column: usize) -> usize {
        match Self::is_alive(&self.lines[current_row - 1][current_column]) {
            true => 1,
            false => 0,
        }
    }

    // check to the north west
    fn is_alive_nw(&self, current_row: usize, current_column: usize) -> usize {
        match Self::is_alive(&self.lines[current_row - 1][current_column - 1]) {
            true => 1,
            false => 0,
        }
    }

    // check to the west
    fn is_alive_w(&self, current_row: usize, current_column: usize) -> usize {
        match Self::is_alive(&self.lines[current_row][current_column - 1]) {
            true => 1,
            false => 0,
        }
    }

    // check to the south west
    fn is_alive_sw(&self, current_row: usize, current_column: usize) -> usize {
        match Self::is_alive(&self.lines[current_row + 1][current_column - 1]) {
            true => 1,
            false => 0,
        }
    }

    // check south
    fn is_alive_s(&self, current_row: usize, current_column: usize) -> usize {
        match Self::is_alive(&self.lines[current_row + 1][current_column]) {
            true => 1,
            false => 0,
        }
    }

    // check south east
    fn is_alive_se(&self, current_row: usize, current_column: usize) -> usize {
        match Self::is_alive(&self.lines[current_row + 1][current_column + 1]) {
            true => 1,
            false => 0,
        }
    }

    // check east
    fn is_alive_e(&self, current_row: usize, current_column: usize) -> usize {
        match Self::is_alive(&self.lines[current_row][current_column + 1]) {
            true => 1,
            false => 0,
        }
    }

    pub fn generate(&mut self) {
        let mut changed: Vec<(usize, usize, Health)> = Vec::new();
        for row in self.lines.iter().enumerate() {
            for column in row.1.iter().enumerate() {
                let mut living_neighbors = 0;
                match row.0 {
                    0 => {
                        // nothing to the n
                        match column.0 {
                            0 => {
                                // nothing behind me, check: e, se, s
                                living_neighbors += self.is_alive_e(row.0, column.0);
                                living_neighbors += self.is_alive_se(row.0, column.0);
                                living_neighbors += self.is_alive_s(row.0, column.0);
                            }
                            x if x == row.1.len() - 1 => {
                                // nothing in front of me, check: s, sw, w
                                living_neighbors += self.is_alive_s(row.0, column.0);
                                living_neighbors += self.is_alive_sw(row.0, column.0);
                                living_neighbors += self.is_alive_w(row.0, column.0);
                            }
                            _ => {
                                // in front and behind me, check: e, se, s, sw, w
                                living_neighbors += self.is_alive_e(row.0, column.0);
                                living_neighbors += self.is_alive_se(row.0, column.0);
                                living_neighbors += self.is_alive_s(row.0, column.0);
                                living_neighbors += self.is_alive_sw(row.0, column.0);
                                living_neighbors += self.is_alive_w(row.0, column.0);
                            }
                        }
                    }
                    x if x == self.lines.len() - 1 => {
                        // nothing to the s
                        match column.0 {
                            0 => {
                                // nothing behind me, check: e, n, ne
                                living_neighbors += self.is_alive_e(row.0, column.0);
                                living_neighbors += self.is_alive_n(row.0, column.0);
                                living_neighbors += self.is_alive_ne(row.0, column.0);
                            }
                            x if x == row.1.len() - 1 => {
                                // nothing in front of me, check: w, nw, n
                                living_neighbors += self.is_alive_w(row.0, column.0);
                                living_neighbors += self.is_alive_nw(row.0, column.0);
                                living_neighbors += self.is_alive_n(row.0, column.0);
                            }
                            _ => {
                                // in front and behind me, check: e, w, nw, n, ne
                                living_neighbors += self.is_alive_e(row.0, column.0);
                                living_neighbors += self.is_alive_w(row.0, column.0);
                                living_neighbors += self.is_alive_nw(row.0, column.0);
                                living_neighbors += self.is_alive_n(row.0, column.0);
                                living_neighbors += self.is_alive_ne(row.0, column.0);
                            }
                        }
                    }
                    _ => {
                        // things to the n and s
                        match column.0 {
                            0 => {
                                // nothing behind me, check: e, se, s, n, ne
                                living_neighbors += self.is_alive_e(row.0, column.0);
                                living_neighbors += self.is_alive_se(row.0, column.0);
                                living_neighbors += self.is_alive_s(row.0, column.0);
                                living_neighbors += self.is_alive_n(row.0, column.0);
                                living_neighbors += self.is_alive_ne(row.0, column.0);
                            }
                            x if x == row.1.len() - 1 => {
                                // nothing in front of me, check: e, sw, w, nw, n
                                living_neighbors += self.is_alive_s(row.0, column.0);
                                living_neighbors += self.is_alive_sw(row.0, column.0);
                                living_neighbors += self.is_alive_w(row.0, column.0);
                                living_neighbors += self.is_alive_nw(row.0, column.0);
                                living_neighbors += self.is_alive_n(row.0, column.0);
                            }
                            _ => {
                                // in front and behind me, check: e, se, s, sw, w, nw, n, ne
                                living_neighbors += self.is_alive_e(row.0, column.0);
                                living_neighbors += self.is_alive_se(row.0, column.0);
                                living_neighbors += self.is_alive_s(row.0, column.0);
                                living_neighbors += self.is_alive_sw(row.0, column.0);
                                living_neighbors += self.is_alive_w(row.0, column.0);
                                living_neighbors += self.is_alive_nw(row.0, column.0);
                                living_neighbors += self.is_alive_n(row.0, column.0);
                                living_neighbors += self.is_alive_ne(row.0, column.0);
                            }
                        }
                    }
                }

                let health = Self::compute_health(column.1, living_neighbors);

                if column.1.ne(&health) {
                    changed.push((row.0, column.0, health));
                }
            }
        }

        for (row, column, health) in changed {
            self.lines[row][column] = health;
        }
    }
}
