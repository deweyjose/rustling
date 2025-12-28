use std::cmp::max;
use std::cmp::min;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use termion::event::Event;
use termion::event::Key::Esc;

use crate::commands::{Command, CommandHandler};
use crate::coordinates::Coordinates;
use crate::grid::Grid;
use crate::pattern;
use crate::renderer::{RenderState, Renderer};
use crate::size::Size;
use crate::user_input;
use crate::viewport::Viewport;

fn init_grid_and_size(grid_multiplier: usize) -> (Grid, Size) {
    let (width, height) = termion::terminal_size().unwrap_or((80, 24));

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

pub struct Orchestrator {
    grid: Grid,
    cur_pos: Coordinates,
    viewport: Viewport,
    viewport_size: Size,
    running: bool,
    configuration: Vec<pattern::PatternType>,
    current_pattern_type: usize,
    last_pattern: Option<usize>,
    rotation_count: usize, // For display only (0-3 = 0°, 90°, 180°, 270°)
    simulation_delay: u128,
    grid_multiplier: usize,
}

pub fn init(configuration: Vec<pattern::PatternType>, grid_multiplier: usize) -> Orchestrator {
    let (grid, size) = init_grid_and_size(grid_multiplier);
    let viewport = Viewport::new(grid.get_size(), size.clone());

    Orchestrator {
        grid,
        cur_pos: Coordinates { x: 1, y: 1 },
        viewport,
        viewport_size: size,
        running: true,
        configuration,
        current_pattern_type: 0,
        last_pattern: None,
        rotation_count: 0,
        simulation_delay: 50,
        grid_multiplier,
    }
}

impl Orchestrator {
    pub fn render(&mut self) {
        let old_pos = self.cur_pos.clone();
        let render_state = RenderState {
            cur_pos: &self.cur_pos,
            running: self.running,
            configuration: &self.configuration,
            current_pattern_type: self.current_pattern_type,
            last_pattern: self.last_pattern,
            rotation_count: self.rotation_count,
        };
        Renderer::render_all(&self.grid, &self.viewport, &self.viewport_size, &render_state);
        Renderer::set_cursor_pos(&old_pos);
    }

    fn render_help(&self) {
        Renderer::render_help(
            &self.viewport_size,
            self.grid.get_size().height,
            &self.configuration,
        );
    }

    fn set_pos(&mut self, x: usize, y: usize) {
        self.cur_pos.x = x;
        self.cur_pos.y = y;
        Renderer::set_cursor_pos(&self.cur_pos);
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
        self.cur_pos.x = min(self.cur_pos.x + 1, self.viewport_size.width);
    }

    fn move_cur_right_by(&mut self, amount: usize) {
        self.cur_pos.x = min(self.cur_pos.x + amount, self.viewport_size.width);
    }

    fn move_cur_down(&mut self) {
        self.cur_pos.y = min(self.cur_pos.y + 1, self.viewport_size.height);
    }

    fn view_to_grid_coordinates(&self) -> Coordinates {
        self.viewport.view_to_grid(self.cur_pos.clone())
    }

    fn rotate_last_shape(&mut self) {
        if let Some(index) = self.last_pattern {
            // Rotate the pattern in the configuration directly
            let pattern = &mut self.configuration[self.current_pattern_type].patterns[index];
            *pattern = pattern.rotate_90();
            
            // Update rotation count for display
            self.rotation_count = (self.rotation_count + 1) % 4;
        }
    }

    fn execute_command(&mut self, command: Command) -> bool {
        let grid_position = self.view_to_grid_coordinates();
        let mut should_quit = false;

        match command {
            Command::Quit => {
                self.set_pos(1, 1);
                should_quit = true;
            }
            Command::MoveCursorLeft => self.move_cur_left(),
            Command::MoveCursorRight => self.move_cur_right(),
            Command::MoveCursorUp => self.move_cur_up(),
            Command::MoveCursorDown => self.move_cur_down(),
            Command::MoveCursorLeftBy(amount) => self.move_cur_left_by(amount),
            Command::MoveCursorRightBy(amount) => self.move_cur_right_by(amount),
            Command::MoveCursorToStartOfLine => self.cur_pos.x = 1,
            Command::MoveCursorToEndOfLine => self.cur_pos.x = self.viewport_size.width,
            Command::ToggleCellAlive => {
                self.grid.resurrect(grid_position);
                self.move_cur_right();
            }
            Command::ToggleCellDead => {
                self.grid.kill(grid_position);
                self.move_cur_left();
            }
            Command::ClearGrid => {
                let (grid, size) = init_grid_and_size(self.grid_multiplier);
                self.grid = grid;
                self.viewport_size = size;
                self.viewport.update_size(self.viewport_size.clone(), self.grid.get_size());
            }
            Command::PlaceLastPattern => {
                if let Some(index) = self.last_pattern {
                    let matrix = &self.configuration[self.current_pattern_type].patterns[index].matrix;
                    self.grid.shape(grid_position, matrix);
                }
            }
            Command::CyclePatternType => {
                self.last_pattern = None;
                self.rotation_count = 0;
                match self.current_pattern_type {
                    x if x == self.configuration.len() - 1 => {
                        self.current_pattern_type = 0;
                    }
                    _ => {
                        self.current_pattern_type += 1;
                    }
                }
            }
            Command::RotateLastPattern => {
                self.rotate_last_shape();
            }
            Command::ToggleSimulation => {
                self.running = !self.running;
            }
            Command::StepSimulation => {
                self.grid.generate();
                self.render();
            }
            Command::SpeedUp => {
                if let Some(val) = self.simulation_delay.checked_sub(10) {
                    self.simulation_delay = val;
                }
            }
            Command::SpeedDown => {
                if let Some(val) = self.simulation_delay.checked_add(10) {
                    self.simulation_delay = val;
                }
            }
            Command::PlacePattern(index) => {
                let patterns = &self.configuration[self.current_pattern_type].patterns;
                if index < patterns.len() {
                    if self.last_pattern != Some(index) {
                        self.rotation_count = 0;
                    }
                    self.last_pattern = Some(index);
                    self.grid.shape(grid_position, &patterns[index].matrix);
                }
            }
            Command::ShowHelp => {
                self.render_help();
            }
            Command::ExitHelp => {
                self.render();
            }
            Command::SetCursorPosition(x, y) => {
                self.set_pos(x, y);
            }
            Command::NoOp => {}
        }

        should_quit
    }

    pub fn run(&mut self) {
        let (tx, rx): (Sender<Event>, Receiver<Event>) = mpsc::channel();

        let _ = thread::spawn(move || {
            user_input::capture_input(tx);
        });

        let mut wait_for_state: Vec<termion::event::Key> = Vec::new();
        let mut last_render = Instant::now();
        let mut in_help_mode = false;

        self.render();
        self.set_pos(self.viewport_size.width / 2, self.viewport_size.height / 2);

        loop {
            if wait_for_state.is_empty()
                && !in_help_mode
                && last_render.elapsed().as_millis() > self.simulation_delay
            {
                if self.running {
                    self.grid.generate();
                }
                last_render = Instant::now();
                self.render();
            }

            let result = rx.recv_timeout(Duration::from_millis(25));
            if let Ok(event) = result {
                // Handle help mode state
                if let Event::Key(key) = &event {
                    if !wait_for_state.is_empty() {
                        if wait_for_state.contains(key) {
                            wait_for_state.clear();
                            if key == &Esc {
                                in_help_mode = false;
                                self.render();
                            }
                        }
                        continue;
                    }
                    if key == &Esc && in_help_mode {
                        in_help_mode = false;
                        self.render();
                        continue;
                    }
                }

                let command = CommandHandler::event_to_command(&event, in_help_mode);

                match command {
                    Command::ShowHelp => {
                        in_help_mode = true;
                        wait_for_state.push(Esc);
                    }
                    Command::ExitHelp => {
                        in_help_mode = false;
                        self.render();
                        continue;
                    }
                    _ => {}
                }

                if self.execute_command(command) {
                    break;
                }

                if !in_help_mode {
                    self.render();
                }
            }
        }
    }
}

