use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::io::stdout;
use std::io::Write;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use termion::cursor;
use termion::event::Event;
use termion::event::Event::Key;
use termion::event::Key::BackTab;
use termion::event::Key::Backspace;
use termion::event::Key::Char;
use termion::event::Key::Ctrl;
use termion::event::Key::Down;
use termion::event::Key::Esc;
use termion::event::Key::Left;
use termion::event::Key::Right;
use termion::event::Key::Up;
use termion::event::MouseEvent;
use termion::style;
use termion::{clear, color};

use crate::coordinates::Coordinates;
use crate::grid::Grid;
use crate::grid_input;
use crate::pattern;
use crate::size::Size;

const DEFAULT_ZERO: usize = 0_usize;

const HELP: &str = r#"
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

# pattern classes
Select a different pattern class using the p key
Print a shape using the number in () to the left of the name

"#;

pub struct GridViewer {
    grid: Grid,
    cur_pos: Coordinates,
    size: Size,
    running: bool,
    configuration: Vec<pattern::PatternType>,
    current_pattern_type: usize,
    last_pattern: Option<usize>,
    last_pattern_rotation: HashMap<usize, usize>,
    simulation_delay: u128,
    grid_multiplier: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UiMode {
    Normal,
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LoopControl {
    Continue,
    Quit,
}

fn init_grid_and_size(grid_multiplier: usize) -> (Grid, Size) {
    let (width, height) = termion::terminal_size().unwrap();

    let grid = Grid::new(Size {
        width: width as usize * grid_multiplier,
        height: height as usize * grid_multiplier,
    });

    let size = Size {
        width: width as usize,
        height: height as usize,
    };

    (grid, size)
}

pub fn init(configuration: Vec<pattern::PatternType>, grid_multiplier: usize) -> GridViewer {
    let (grid, size) = init_grid_and_size(grid_multiplier);

    GridViewer {
        grid,
        cur_pos: Coordinates { x: 1, y: 1 },
        size,
        running: true,
        configuration,
        current_pattern_type: 0,
        last_pattern: None,
        last_pattern_rotation: HashMap::new(),
        simulation_delay: 50,
        grid_multiplier,
    }
}

impl GridViewer {
    pub fn render_help(&self) {
        self.render_header();

        let width = self.size.width;
        let mut current_line = 2;

        for line in HELP.lines() {
            print!(
                "{}{}{}{:width$}\r",
                cursor::Goto(1, current_line),
                clear::CurrentLine,
                color::Fg(color::Green),
                line
            );
            current_line += 1;
        }

        for pattern_type in &self.configuration {
            let values: Vec<String> = pattern_type
                .patterns
                .iter()
                .enumerate()
                .map(|e| format!("({}) {}", e.0 + 1, e.1.name))
                .collect();

            print!(
                "{}{}{:width$}\r",
                cursor::Goto(1, current_line),
                clear::CurrentLine,
                pattern_type.name
            );
            current_line += 1;
            print!(
                "{}{}{:width$}\r",
                cursor::Goto(1, current_line),
                clear::CurrentLine,
                values.join(", ")
            );
            current_line += 1;
            print!(
                "{}{}{:width$}\r",
                cursor::Goto(1, current_line),
                clear::CurrentLine,
                " "
            );
            current_line += 1;
        }

        for line in current_line..self.grid.get_size().height as u16 {
            print!("{}{}", cursor::Goto(1, line), clear::CurrentLine);
        }

        self.render_footer();
    }

    fn render_header(&self) {
        let width = self.size.width;
        let header = "Welcome to Game of Life Text Editor. (h)elp";
        print!(
            "{}{}{}{:width$}\r{}",
            cursor::Goto(1, 1),
            color::Fg(color::Red),
            style::Bold,
            header,
            style::Reset
        );
        stdout().flush().unwrap();
    }

    fn render_grid(&self) {
        let x_offset = (self.grid.get_size().width - self.size.width) / 2;
        let y_offset = (self.grid.get_size().height - self.size.height) / 2;

        for row in 2..self.size.height {
            let health = self.grid.get_row(y_offset + row - 2);

            print!("{}{}", cursor::Goto(1, row as u16), color::Fg(color::Green));

            stdout().flush().unwrap();

            for column in 0..self.size.width {
                print!("{}", health[column + x_offset]);
            }

            stdout().flush().unwrap();
        }
    }

    fn render_footer(&self) {
        let (last_pattern, last_rotation) = if let Some(last) = self.last_pattern {
            let pattern = self.configuration[self.current_pattern_type].patterns[last]
                .name
                .as_str();

            let rotation = self
                .last_pattern_rotation
                .get(&last)
                .unwrap_or(&DEFAULT_ZERO);

            (pattern, rotation)
        } else {
            ("none", &0)
        };

        let width = self.size.width;
        let footer = format!(
            "grid {}, view-port {}, cursor {}, (s){}, (p)attern class: {}, (l)ast pattern {}, (r)otation {} degrees",
            self.grid.get_size(),
            self.size,
            self.cur_pos,
            if self.running {
                "top".to_string()
            } else {
                "tart".to_string()
            },
            self.configuration[self.current_pattern_type].name,
            last_pattern,
            last_rotation
        );

        print!(
            "{}{}{}{:width$}{}",
            cursor::Goto(1, (self.size.height + 1) as u16),
            color::Fg(color::Red),
            style::Bold,
            footer,
            style::Reset
        );
        stdout().flush().unwrap();
    }

    pub fn render(&mut self) {
        let pos = &self.cur_pos;
        let (old_x, old_y) = (pos.x, pos.y);
        self.render_header();
        self.render_grid();
        self.render_footer();
        self.set_pos(old_x, old_y);
    }

    fn set_pos(&mut self, x: usize, y: usize) {
        self.cur_pos.x = x;
        self.cur_pos.y = y;
        print!(
            "{}",
            cursor::Goto(self.cur_pos.x as u16, self.cur_pos.y as u16)
        );
        stdout().flush().unwrap();
    }

    fn move_cur_left(&mut self) {
        if self.cur_pos.x > 0 {
            self.cur_pos.x -= 1;
        }
    }

    fn move_cur_left_by(&mut self, amount: usize) {
        match self.cur_pos.x.checked_sub(amount) {
            Some(s) => self.cur_pos.x = s,
            _ => self.cur_pos.x = 0,
        };
    }

    fn move_cur_up(&mut self) {
        self.cur_pos.y = max(self.cur_pos.y - 1, 2);
    }

    fn move_cur_right(&mut self) {
        self.cur_pos.x = min(self.cur_pos.x + 1, self.size.width);
    }

    fn move_cur_right_by(&mut self, amount: usize) {
        self.cur_pos.x = min(self.cur_pos.x + amount, self.size.width);
    }

    fn move_cur_down(&mut self) {
        self.cur_pos.y = min(self.cur_pos.y + 1, self.size.height);
    }

    fn view_to_grid_coordinates(&self) -> Coordinates {
        let x_offset = (self.grid.get_size().width - self.size.width) / 2;
        let y_offset = (self.grid.get_size().height - self.size.height) / 2;

        Coordinates {
            x: x_offset + self.cur_pos.x - 1,
            y: y_offset + self.cur_pos.y - 2,
        }
    }

    fn rotate_last_shape(&mut self) {
        if let Some(index) = self.last_pattern {
            let matrix = &mut self.configuration[self.current_pattern_type].patterns[index].matrix;
            let n = matrix.len();
            for i in 0..n / 2 {
                for j in i..n - i - 1 {
                    let temp = matrix[i][j];
                    matrix[i][j] = matrix[n - j - 1][i];
                    matrix[n - j - 1][i] = matrix[n - i - 1][n - j - 1];
                    matrix[n - i - 1][n - j - 1] = matrix[j][n - i - 1];
                    matrix[j][n - i - 1] = temp;
                }
            }

            let mut x = self
                .last_pattern_rotation
                .get(&index)
                .unwrap_or(&DEFAULT_ZERO)
                + 90;

            if x == 360 {
                x = 0;
            }

            self.last_pattern_rotation.insert(index, x);
        }
    }

    fn maybe_tick(&mut self, mode: UiMode, last_render: &mut Instant) {
        if mode != UiMode::Normal {
            return;
        }

        if last_render.elapsed().as_millis() <= self.simulation_delay {
            return;
        }

        if self.running {
            self.grid.generate();
        }
        *last_render = Instant::now();
        self.render();
    }

    fn cycle_pattern_class(&mut self) {
        self.last_pattern = None;
        if self.configuration.is_empty() {
            self.current_pattern_type = 0;
            return;
        }
        self.current_pattern_type = (self.current_pattern_type + 1) % self.configuration.len();
    }

    fn handle_key(
        &mut self,
        key: termion::event::Key,
        grid_position: Coordinates,
        mode: &mut UiMode,
    ) -> LoopControl {
        // When help is open, only allow exiting it.
        if *mode == UiMode::Help {
            if matches!(key, Esc | Char('h')) {
                *mode = UiMode::Normal;
                // Restore the normal view immediately.
                self.render();
            }
            return LoopControl::Continue;
        }

        match key {
            Ctrl('c') | Char('q') => {
                self.set_pos(1, 1);
                LoopControl::Quit
            }
            Left => {
                self.move_cur_left();
                LoopControl::Continue
            }
            Right => {
                self.move_cur_right();
                LoopControl::Continue
            }
            Up => {
                self.move_cur_up();
                LoopControl::Continue
            }
            Down => {
                self.move_cur_down();
                LoopControl::Continue
            }
            Backspace => {
                self.grid.kill(grid_position);
                self.move_cur_left();
                LoopControl::Continue
            }
            BackTab => {
                self.move_cur_left_by(4);
                LoopControl::Continue
            }
            Char('\t') => {
                self.move_cur_right_by(4);
                LoopControl::Continue
            }
            Char('a') => {
                self.grid.resurrect(grid_position);
                self.move_cur_right();
                LoopControl::Continue
            }
            Char('b') => {
                self.cur_pos.x = 1;
                LoopControl::Continue
            }
            Char('c') => {
                let (grid, size) = init_grid_and_size(self.grid_multiplier);
                self.grid = grid;
                self.size = size;
                LoopControl::Continue
            }
            Char('d') => {
                self.grid.kill(grid_position);
                self.move_cur_left();
                LoopControl::Continue
            }
            Char('e') => {
                self.cur_pos.x = self.size.width;
                LoopControl::Continue
            }
            Char('h') => {
                self.render_help();
                *mode = UiMode::Help;
                LoopControl::Continue
            }
            Char('l') => {
                if let Some(index) = self.last_pattern {
                    self.grid.shape(
                        grid_position,
                        &self.configuration[self.current_pattern_type].patterns[index].matrix,
                    );
                }
                LoopControl::Continue
            }
            Char('p') => {
                self.cycle_pattern_class();
                LoopControl::Continue
            }
            Char('r') => {
                self.rotate_last_shape();
                LoopControl::Continue
            }
            Char('s') => {
                self.running = !self.running;
                LoopControl::Continue
            }
            Char(' ') => {
                self.grid.generate();
                self.render();
                LoopControl::Continue
            }
            Char('+') => {
                if let Some(val) = self.simulation_delay.checked_sub(10) {
                    self.simulation_delay = val;
                }
                LoopControl::Continue
            }
            Char('-') => {
                if let Some(val) = self.simulation_delay.checked_add(10) {
                    self.simulation_delay = val;
                }
                LoopControl::Continue
            }
            Char(c) if c.is_ascii_digit() => {
                let mut index: usize = c.to_digit(10).unwrap() as usize;
                if index > 0 {
                    index -= 1;
                    let patterns = &self.configuration[self.current_pattern_type].patterns;
                    if index < patterns.len() {
                        self.last_pattern = Some(index);
                        self.grid.shape(grid_position, &patterns[index].matrix);
                    }
                }
                LoopControl::Continue
            }
            _ => LoopControl::Continue,
        }
    }

    pub fn run(&mut self) {
        let (tx, rx): (Sender<Event>, Receiver<Event>) = mpsc::channel();

        let _ = thread::spawn(move || {
            grid_input::grid_input(tx);
        });

        let mut mode = UiMode::Normal;
        let mut last_render = Instant::now();

        self.render();
        self.set_pos(self.size.width / 2, self.size.height / 2);

        loop {
            self.maybe_tick(mode, &mut last_render);

            let result = rx.recv_timeout(Duration::from_millis(25));
            let grid_position = self.view_to_grid_coordinates();
            if let Ok(event) = result {
                match event {
                    Key(key) => {
                        if self.handle_key(key, grid_position, &mut mode) == LoopControl::Quit {
                            break;
                        }
                    }
                    Event::Mouse(MouseEvent::Press(_, x, y)) => {
                        self.set_pos(x as usize, y as usize)
                    }
                    _ => {}
                };
            }
        }
    }
}
