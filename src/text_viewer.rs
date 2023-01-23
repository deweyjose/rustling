use crate::text_viewer::Health::{Alive, Dead};
use std::cmp::{max, min};
use std::fmt;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::event::Key::{Char, Ctrl};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, style};

#[derive(Clone, Copy, PartialEq)]
enum Health {
    Alive,
    Dead,
}

impl fmt::Display for Health {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Alive => f.write_str("*"),
            Dead => f.write_str(" "),
        }
    }
}

struct Grid {
    lines: Vec<Vec<Health>>,
    columns: usize,
    rows: usize,
}

impl Grid {
    pub fn clear(&mut self) {
        let (lines, ..) = Grid::init_grid();
        self.lines = lines;
    }
    pub fn new() -> Self {
        let (lines, columns, rows) = Grid::init_grid();
        Grid {
            lines,
            columns,
            rows,
        }
    }

    pub fn init_grid() -> (Vec<Vec<Health>>, usize, usize) {
        let (columns, rows) = termion::terminal_size().unwrap();

        let mut line = Vec::new();

        for _ in 0..columns {
            line.push(Dead);
        }

        let mut lines = Vec::new();

        for _ in 0..(rows - 3) {
            lines.push(line.clone());
        }

        (lines, columns as usize, (rows - 3) as usize)
    }
}

#[derive(Debug)]
struct Coordinates {
    pub x: usize,
    pub y: usize,
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

pub struct GridViewer {
    grid: Grid,
    cur_pos: Coordinates,
    terminal_size: Coordinates,
}

pub fn init() -> GridViewer {
    let grid = Grid::new();
    let x = grid.columns;
    let y = grid.rows + 2;
    GridViewer {
        grid,
        cur_pos: Coordinates { x: 1, y: 1 },
        terminal_size: Coordinates { x, y },
    }
}

fn is_alive(cell: &Health) -> bool {
    match cell {
        Alive => true,
        Dead => false,
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

impl GridViewer {
    pub fn render(&mut self) {
        let pos = &self.cur_pos;
        let (old_x, old_y) = (pos.x, pos.y);
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

        println!(
            "{}{}Welcome to Game of Life Text Editor\r{}",
            color::Bg(color::Black),
            color::Fg(color::Red),
            style::Reset
        );

        for row in 0..self.grid.rows {
            let values: Vec<String> = self.grid.lines[row]
                .iter()
                .map(|e| format!("{}", e))
                .collect();
            println!(
                "{}{}{}\r",
                color::Bg(color::Green),
                color::Fg(color::Black),
                values.join("")
            );
        }

        println!(
            "{}",
            termion::cursor::Goto(0, (self.terminal_size.y - 1) as u16),
        );

        println!(
            "{}{}{}Terminal Size: {}, Cursor {} {}, Grid {}x{}{}",
            color::Bg(color::Black),
            color::Fg(color::Red),
            style::Bold,
            self.terminal_size,
            self.cur_pos.x,
            self.cur_pos.y,
            self.grid.rows,
            self.grid.columns,
            style::Reset
        );

        self.set_pos(old_x, old_y);
    }

    fn set_pos(&mut self, x: usize, y: usize) {
        self.cur_pos.x = x;
        self.cur_pos.y = y;
        println!(
            "{}",
            termion::cursor::Goto(self.cur_pos.x as u16, self.cur_pos.y as u16)
        );
    }

    fn dec_x(&mut self) {
        if self.cur_pos.x > 0 {
            self.cur_pos.x = self.cur_pos.x - 1;
        }
    }

    fn dec_y(&mut self) {
        self.cur_pos.y = max(self.cur_pos.y - 1, 1);
    }

    fn inc_x(&mut self) {
        self.cur_pos.x = min(self.cur_pos.x + 1, self.grid.columns);
    }

    fn inc_y(&mut self) {
        self.cur_pos.y = min(self.cur_pos.y + 1, self.grid.rows);
    }

    pub fn run(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let stdin = stdin();
        for c in stdin.keys() {
            match c.unwrap() {
                Ctrl('c') => break,
                Key::Left => {
                    self.dec_x();
                    self.render();
                }
                Key::Right => {
                    self.inc_x();
                    self.render();
                }
                Key::Up => {
                    self.dec_y();
                    self.render();
                }
                Key::Down => {
                    self.inc_y();
                    self.render();
                }
                Char('c') => {
                    self.grid.clear();
                    self.render();
                }
                Char('s') => {
                    self.step();
                    self.render();
                }
                Char('r') => {
                    self.resurrect();
                    self.inc_x();
                    self.render();
                }
                Char('k') => {
                    self.kill();
                    self.inc_x();
                    self.render();
                }
                Char('b') => {
                    self.cur_pos.x = 1;
                    self.render();
                }
                Char('e') => {
                    self.cur_pos.x = self.grid.columns;
                    self.render();
                }
                _ => {}
            }
            stdout.flush().unwrap();
        }
    }

    // =============================
    // Positions relative a cell (C).
    //
    // 1 - in front
    // 2 - diagonal front below
    // 3 - below
    // 4 - diagonal behind below
    // 5 - behind
    // 6 - diagonal behind above
    // 7 - above
    // 8 - diagonal front above
    //
    // =============================
    //
    //         6     7    8
    //           \   |   /
    //         5 -   C   - 1
    //           /   |   \
    //         4     3    2
    //
    // =============================

    // check above
    fn is_alive_8(&self, current_row: usize, current_column: usize) -> usize {
        match is_alive(&self.grid.lines[current_row - 1][current_column + 1]) {
            true => 1,
            false => 0,
        }
    }

    // check above
    fn is_alive_7(&self, current_row: usize, current_column: usize) -> usize {
        match is_alive(&self.grid.lines[current_row - 1][current_column]) {
            true => 1,
            false => 0,
        }
    }

    // check diagonal behind above
    fn is_alive_6(&self, current_row: usize, current_column: usize) -> usize {
        match is_alive(&self.grid.lines[current_row - 1][current_column - 1]) {
            true => 1,
            false => 0,
        }
    }

    // check behind
    fn is_alive_5(&self, current_row: usize, current_column: usize) -> usize {
        match is_alive(&self.grid.lines[current_row][current_column - 1]) {
            true => 1,
            false => 0,
        }
    }

    // check diagonal below behind
    fn is_alive_4(&self, current_row: usize, current_column: usize) -> usize {
        match is_alive(&self.grid.lines[current_row + 1][current_column - 1]) {
            true => 1,
            false => 0,
        }
    }

    // check below
    fn is_alive_3(&self, current_row: usize, current_column: usize) -> usize {
        match is_alive(&self.grid.lines[current_row + 1][current_column]) {
            true => 1,
            false => 0,
        }
    }

    // check diagonal below in front
    fn is_alive_2(&self, current_row: usize, current_column: usize) -> usize {
        match is_alive(&self.grid.lines[current_row + 1][current_column + 1]) {
            true => 1,
            false => 0,
        }
    }

    // check in front
    fn is_alive_1(&self, current_row: usize, current_column: usize) -> usize {
        match is_alive(&self.grid.lines[current_row][current_column + 1]) {
            true => 1,
            false => 0,
        }
    }

    fn resurrect(&mut self) {
        self.grid.lines[self.cur_pos.y - 1][self.cur_pos.x - 1] = Alive;
    }

    fn kill(&mut self) {
        self.grid.lines[self.cur_pos.y - 1][self.cur_pos.x - 1] = Dead;
    }

    fn step(&mut self) {
        let mut changed: Vec<(usize, usize, Health)> = Vec::new();
        for row in self.grid.lines.iter().enumerate() {
            for column in row.1.iter().enumerate() {
                match row.0 {
                    0 => {
                        // nothing above me
                        match column.0 {
                            0 => {
                                // nothing behind me
                                // check:
                                //   1. in front
                                //   2. diagonal front down
                                //   3. directly below
                                let mut living_neighbors = 0;

                                living_neighbors += self.is_alive_1(row.0, column.0);
                                living_neighbors += self.is_alive_2(row.0, column.0);
                                living_neighbors += self.is_alive_3(row.0, column.0);

                                let health = compute_health(column.1, living_neighbors);

                                if column.1.ne(&health) {
                                    changed.push((row.0, column.0, health));
                                }
                            }
                            x if x == row.1.len() - 1 => {
                                // nothing in front of me
                                // check:
                                //   1. directly below
                                //   2. diagonal behind back
                                //   3. behind
                                let mut living_neighbors = 0;

                                living_neighbors += self.is_alive_3(row.0, column.0);
                                living_neighbors += self.is_alive_4(row.0, column.0);
                                living_neighbors += self.is_alive_5(row.0, column.0);

                                let health = compute_health(column.1, living_neighbors);

                                if column.1.ne(&health) {
                                    changed.push((row.0, column.0, health));
                                }
                            }
                            _ => {
                                // in front and behind me
                                // check
                                //   1. in front
                                //   2. diagonal front down
                                //   3. directly below
                                //   4. diagonal behind back
                                //   5. behind

                                let mut living_neighbors = 0;

                                living_neighbors += self.is_alive_1(row.0, column.0);
                                living_neighbors += self.is_alive_2(row.0, column.0);
                                living_neighbors += self.is_alive_3(row.0, column.0);
                                living_neighbors += self.is_alive_4(row.0, column.0);
                                living_neighbors += self.is_alive_5(row.0, column.0);

                                let health = compute_health(column.1, living_neighbors);

                                if column.1.ne(&health) {
                                    changed.push((row.0, column.0, health));
                                }
                            }
                        }
                    }
                    x if x == self.grid.lines.len() - 1 => {
                        // nothing below me
                        match column.0 {
                            0 => {
                                // nothing behind me
                                // check:
                                //   1. in front
                                //   7. directly above
                                //   8. diagonal front up
                                let mut living_neighbors = 0;

                                living_neighbors += self.is_alive_1(row.0, column.0);
                                living_neighbors += self.is_alive_7(row.0, column.0);
                                living_neighbors += self.is_alive_8(row.0, column.0);

                                let health = compute_health(column.1, living_neighbors);

                                if column.1.ne(&health) {
                                    changed.push((row.0, column.0, health));
                                }
                            }
                            x if x == row.1.len() - 1 => {
                                // nothing in front of me
                                // check:
                                //   5. behind
                                //   6. diagonal behind up
                                //   7. directly above
                                let mut living_neighbors = 0;

                                living_neighbors += self.is_alive_5(row.0, column.0);
                                living_neighbors += self.is_alive_6(row.0, column.0);
                                living_neighbors += self.is_alive_7(row.0, column.0);

                                let health = compute_health(column.1, living_neighbors);

                                if column.1.ne(&health) {
                                    changed.push((row.0, column.0, health));
                                }
                            }
                            _ => {
                                // in front and behind me
                                // check
                                //   1. in front
                                //   5. behind
                                //   6. diagonal behind up
                                //   7. directly above
                                //   8. diagonal front up
                                let mut living_neighbors = 0;

                                living_neighbors += self.is_alive_1(row.0, column.0);
                                living_neighbors += self.is_alive_5(row.0, column.0);
                                living_neighbors += self.is_alive_6(row.0, column.0);
                                living_neighbors += self.is_alive_7(row.0, column.0);
                                living_neighbors += self.is_alive_8(row.0, column.0);

                                let health = compute_health(column.1, living_neighbors);

                                if column.1.ne(&health) {
                                    changed.push((row.0, column.0, health));
                                }
                            }
                        }
                    }
                    _ => {
                        // above and below me
                        match column.0 {
                            0 => {
                                // nothing behind me
                                // check:
                                //   1. in front
                                //   2. diagonal front down
                                //   3. directly below
                                //   7. directly above
                                //   8. diagonal front up
                                let mut living_neighbors = 0;

                                living_neighbors += self.is_alive_1(row.0, column.0);
                                living_neighbors += self.is_alive_2(row.0, column.0);
                                living_neighbors += self.is_alive_3(row.0, column.0);
                                living_neighbors += self.is_alive_7(row.0, column.0);
                                living_neighbors += self.is_alive_8(row.0, column.0);

                                let health = compute_health(column.1, living_neighbors);

                                if column.1.ne(&health) {
                                    changed.push((row.0, column.0, health));
                                }
                            }
                            x if x == row.1.len() - 1 => {
                                // nothing in front of me
                                // check:
                                //   3. directly below
                                //   4. diagonal behind back
                                //   5. behind
                                //   6. diagonal behind up
                                //   7. directly above
                                let mut living_neighbors = 0;

                                living_neighbors += self.is_alive_3(row.0, column.0);
                                living_neighbors += self.is_alive_4(row.0, column.0);
                                living_neighbors += self.is_alive_5(row.0, column.0);
                                living_neighbors += self.is_alive_6(row.0, column.0);
                                living_neighbors += self.is_alive_7(row.0, column.0);

                                let health = compute_health(column.1, living_neighbors);

                                if column.1.ne(&health) {
                                    changed.push((row.0, column.0, health));
                                }
                            }
                            _ => {
                                // in front and behind me
                                // check
                                //   1. in front
                                //   2. diagonal front down
                                //   3. directly below
                                //   4. diagonal behind back
                                //   5. behind
                                //   6. diagonal behind up
                                //   7. directly above
                                //   8. diagonal front up
                                let mut living_neighbors = 0;

                                living_neighbors += self.is_alive_1(row.0, column.0);
                                living_neighbors += self.is_alive_2(row.0, column.0);
                                living_neighbors += self.is_alive_3(row.0, column.0);
                                living_neighbors += self.is_alive_4(row.0, column.0);
                                living_neighbors += self.is_alive_5(row.0, column.0);
                                living_neighbors += self.is_alive_6(row.0, column.0);
                                living_neighbors += self.is_alive_7(row.0, column.0);
                                living_neighbors += self.is_alive_8(row.0, column.0);

                                let health = compute_health(column.1, living_neighbors);

                                if column.1.ne(&health) {
                                    changed.push((row.0, column.0, health));
                                }
                            }
                        }
                    }
                }
            }
        }

        for (row, column, health) in changed {
            self.grid.lines[row][column] = health;
        }
    }
}
