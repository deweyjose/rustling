use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use termion::color;
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
p       - cycle through the pattern classes defined in shapes.json
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
}

pub fn init(configuration: Vec<pattern::PatternType>) -> GridViewer {
    let (width, height) = termion::terminal_size().unwrap();

    let grid = Grid::new(Size {
        width: width as usize,
        height: (height - 3) as usize,
    });

    let size = Size {
        width: width as usize,
        height: (height - 1) as usize,
    };

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
    }
}

impl GridViewer {
    pub fn render_help(&self) {
        self.render_header();

        let width = self.grid.get_size().width;
        let mut line_count = 0;

        for line in HELP.lines() {
            println!(
                "{}{}{:width$}\r",
                color::Bg(color::Black),
                color::Fg(color::Green),
                line
            );
            line_count += 1;
        }

        for pattern_type in &self.configuration {
            let values: Vec<String> = pattern_type
                .patterns
                .iter()
                .enumerate()
                .map(|e| format!("({}) {}", e.0 + 1, e.1.name))
                .collect();

            println!("{:width$}\r", pattern_type.name);
            println!("{:width$}\r", values.join(", "));
            println!("{:width$}\r", " ");
            line_count += 3;
        }

        for _ in 0..self.grid.get_size().height - line_count {
            println!("{:width$}\r", " ");
        }

        self.render_footer();
    }

    fn render_header(&self) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        let width = self.grid.get_size().width;
        let header = "Welcome to Game of Life Text Editor. (h)elp";
        println!(
            "{}{}{}{:width$}\r{}",
            color::Bg(color::Black),
            color::Fg(color::Red),
            style::Bold,
            header,
            style::Reset
        );
    }

    fn render_grid(&self) {
        for row in 0..self.grid.get_size().height {
            let values: Vec<String> = self
                .grid
                .get_row(row)
                .iter()
                .map(|e| format!("{e}"))
                .collect();
            println!(
                "{}{}{}\r",
                color::Bg(color::Black),
                color::Fg(color::Green),
                values.join("")
            );
        }
    }

    fn render_footer(&self) {
        println!(
            "{}",
            termion::cursor::Goto(0, (self.size.height - 1) as u16),
        );

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

        let width = self.grid.get_size().width;
        let footer = format!(
            "cursor {}, (s){}, (p)attern class: {}, (l)ast pattern {}, (r)otation {} degrees",
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

        println!(
            "{}{}{}{:width$}{}",
            color::Bg(color::Black),
            color::Fg(color::Red),
            style::Bold,
            footer,
            style::Reset
        );
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
        println!(
            "{}",
            termion::cursor::Goto(self.cur_pos.x as u16, self.cur_pos.y as u16)
        );
    }

    fn dec_x(&mut self) {
        if self.cur_pos.x > 0 {
            self.cur_pos.x -= 1;
        }
    }

    fn sub_x(&mut self, amount: usize) {
        match self.cur_pos.x.checked_sub(amount) {
            Some(s) => self.cur_pos.x = s,
            _ => self.cur_pos.x = 0,
        };
    }

    fn dec_y(&mut self) {
        self.cur_pos.y = max(self.cur_pos.y - 1, 1);
    }

    fn inc_x(&mut self) {
        self.cur_pos.x = min(self.cur_pos.x + 1, self.grid.get_size().width);
    }

    fn add_x(&mut self, amount: usize) {
        self.cur_pos.x = min(self.cur_pos.x + amount, self.grid.get_size().width);
    }

    fn inc_y(&mut self) {
        self.cur_pos.y = min(self.cur_pos.y + 1, self.grid.get_size().height);
    }

    fn normalize_position(&self) -> Coordinates {
        let x = if self.cur_pos.x == 0 {
            0
        } else {
            self.cur_pos.x - 1
        };

        let y = if self.cur_pos.y == 0 {
            0
        } else {
            self.cur_pos.y - 1
        };

        Coordinates { x, y }
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

    pub fn run(&mut self) {
        let (tx, rx): (Sender<Event>, Receiver<Event>) = mpsc::channel();

        let _ = thread::spawn(move || {
            grid_input::grid_input(tx);
        });

        let mut wait_for_state: Vec<termion::event::Key> = Vec::new();
        let mut last_render = Instant::now();

        loop {
            if wait_for_state.is_empty()
                && last_render.elapsed().as_millis() > self.simulation_delay
            {
                if self.running {
                    self.grid.generate();
                }
                last_render = Instant::now();
                self.render();
            }

            let result = rx.recv_timeout(Duration::from_millis(25));
            let grid_position = self.normalize_position();
            if let Ok(event) = result {
                match event {
                    Key(key) => match wait_for_state.is_empty() {
                        false => {
                            if wait_for_state.contains(&key) {
                                wait_for_state.clear();
                            }
                        }
                        _ => match key {
                            Ctrl('c') => break,
                            Char('q') => break,
                            Left => self.dec_x(),
                            Right => self.inc_x(),
                            Up => self.dec_y(),
                            Down => self.inc_y(),
                            Backspace => {
                                self.grid.kill(grid_position);
                                self.dec_x();
                            }
                            BackTab => self.sub_x(4),
                            Char('\t') => {
                                self.add_x(4);
                            }
                            Char('a') => {
                                self.grid.resurrect(grid_position);
                                self.inc_x();
                            }
                            Char('b') => {
                                self.cur_pos.x = 1;
                            }
                            Char('c') => {
                                let (width, height) = termion::terminal_size().unwrap();

                                let grid = Grid::new(Size {
                                    width: width as usize,
                                    height: (height - 3) as usize,
                                });

                                let size = Size {
                                    width: width as usize,
                                    height: (height - 1) as usize,
                                };

                                self.grid = grid;
                                self.size = size;
                            }
                            Char('d') => {
                                self.grid.kill(grid_position);
                                self.dec_x();
                            }
                            Char('e') => {
                                self.cur_pos.x = self.grid.get_size().width;
                            }
                            Char('h') => {
                                self.render_help();
                                wait_for_state.push(key);
                                wait_for_state.push(Esc);
                            }
                            Char('l') => {
                                if let Some(index) = self.last_pattern {
                                    self.grid.shape(
                                        grid_position,
                                        &self.configuration[self.current_pattern_type].patterns
                                            [index]
                                            .matrix,
                                    );
                                }
                            }
                            Char('p') => {
                                self.last_pattern = None;
                                match self.current_pattern_type {
                                    x if x == self.configuration.len() - 1 => {
                                        self.current_pattern_type = 0;
                                    }
                                    _ => {
                                        self.current_pattern_type += 1;
                                    }
                                }
                            }
                            Char('r') => {
                                self.rotate_last_shape();
                            }
                            Char('s') => {
                                self.running = !self.running;
                            }
                            Char(' ') => {
                                self.grid.generate();
                                self.render();
                            }
                            Char('+') => {
                                if let Some(val) = self.simulation_delay.checked_sub(10) {
                                    self.simulation_delay = val;
                                }
                            }
                            Char('-') => {
                                if let Some(val) = self.simulation_delay.checked_add(10) {
                                    self.simulation_delay = val;
                                }
                            }
                            Char(c) if c.is_ascii_digit() => {
                                let mut index: usize = c.to_digit(10).unwrap() as usize;
                                if index > 0 {
                                    index -= 1;
                                    let patterns =
                                        &self.configuration[self.current_pattern_type].patterns;
                                    if index < patterns.len() {
                                        self.last_pattern = Some(index);
                                        self.grid.shape(grid_position, &patterns[index].matrix);
                                    }
                                }
                            }
                            _ => {}
                        },
                    },
                    Event::Mouse(MouseEvent::Press(_, x, y)) => {
                        self.set_pos(x as usize, y as usize)
                    }
                    _ => {}
                };
            }
        }
    }
}
