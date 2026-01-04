use std::io::{self, Stdout};
use std::time::{Duration, Instant};

use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, terminal};
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::Rect;
use ratatui::Terminal;

use crate::app::{App, AppMode, GalleryCursor};
use crate::commands::{Command, CommandHandler};
use crate::coordinates::Coordinates;
use crate::grid::Grid;
use crate::pattern::PatternType;
use crate::renderer::Renderer;
use crate::size::Size;
use crate::theme::Theme;
use crate::user_input;
use crate::viewport::Viewport;
use crate::widgets::pattern_gallery::compute_visible_nodes;

const PATTERN_GALLERY_WIDTH: u16 = 24;

fn init_grid_and_size(grid_multiplier: usize) -> io::Result<(Grid, Size)> {
    let (width, height) = terminal::size()?;

    let grid = Grid::new(Size {
        width: width as usize * grid_multiplier,
        height: height as usize * grid_multiplier,
    });

    let size = Size {
        width: width as usize,
        height: height as usize,
    };

    Ok((grid, size))
}

pub struct Orchestrator {
    app: App,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    theme: Theme,
    last_canvas_area: Option<Rect>,
    last_tick: Instant,
}

impl Orchestrator {
    pub fn init(configuration: Vec<PatternType>, grid_multiplier: usize) -> io::Result<Self> {
        let terminal = setup_terminal()?;
        let (grid, size) = init_grid_and_size(grid_multiplier)?;
        let viewport = Viewport::new(grid.get_size(), size.clone());
        let num_types = configuration.len();

        let app = App {
            grid,
            cursor: Coordinates { x: 0, y: 0 },
            viewport,
            viewport_size: size,
            running: true,
            configuration,
            current_pattern_type: 0,
            last_pattern: None,
            simulation_delay: 50,
            grid_multiplier,
            mode: AppMode::Normal,
            gallery_cursor: GalleryCursor::new(num_types),
        };

        Ok(Self {
            app,
            terminal,
            theme: Theme::default(),
            last_canvas_area: None,
            last_tick: Instant::now(),
        })
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.render()?;
        self.center_cursor();
        self.render()?;

        loop {
            let elapsed = self.last_tick.elapsed().as_millis() as u128;
            if self.app.mode != AppMode::Help
                && self.app.mode != AppMode::PatternGallery
                && self.app.running
                && elapsed > self.app.simulation_delay
            {
                self.app.grid.generate();
                self.last_tick = Instant::now();
                self.render()?;
            }

            if let Some(event) = user_input::poll_event(Duration::from_millis(25))? {
                let command = CommandHandler::event_to_command(&event, self.app.mode);
                if self.handle_command(command)? {
                    break;
                }
                self.render()?;
            }
        }

        Ok(())
    }

    fn render(&mut self) -> io::Result<()> {
        let mut canvas_area: Option<Rect> = None;
        let app = &self.app;
        let theme = &self.theme;
        let mut list_state = self.app.gallery_cursor.list_state.clone();

        self.terminal.draw(|frame| {
            let outcome =
                Renderer::render(frame, app, theme, PATTERN_GALLERY_WIDTH, &mut list_state);
            canvas_area = Some(outcome.canvas_area);
        })?;

        // Update the list state after render (for scroll position)
        self.app.gallery_cursor.list_state = list_state;

        if let Some(area) = canvas_area {
            self.app.viewport_size = Size {
                width: area.width as usize,
                height: area.height as usize,
            };
            self.app
                .viewport
                .update_size(self.app.viewport_size.clone(), self.app.grid.get_size());
            self.last_canvas_area = Some(area);
        }

        Ok(())
    }

    fn center_cursor(&mut self) {
        if self.app.viewport_size.width > 0 {
            self.app.cursor.x = (self.app.viewport_size.width - 1) / 2;
        } else {
            self.app.cursor.x = 0;
        }
        if self.app.viewport_size.height > 0 {
            self.app.cursor.y = (self.app.viewport_size.height - 1) / 2;
        } else {
            self.app.cursor.y = 0;
        }
    }

    fn clamp_cursor(&mut self) {
        if self.app.cursor.x >= self.app.viewport_size.width {
            if self.app.viewport_size.width > 0 {
                self.app.cursor.x = self.app.viewport_size.width - 1;
            } else {
                self.app.cursor.x = 0;
            }
        }
        if self.app.cursor.y >= self.app.viewport_size.height {
            if self.app.viewport_size.height > 0 {
                self.app.cursor.y = self.app.viewport_size.height - 1;
            } else {
                self.app.cursor.y = 0;
            }
        }
    }

    fn move_cur_left(&mut self) {
        if self.app.cursor.x > 0 {
            self.app.cursor.x -= 1;
        }
    }

    fn move_cur_left_by(&mut self, amount: usize) {
        self.app.cursor.x = self.app.cursor.x.saturating_sub(amount);
    }

    fn move_cur_up(&mut self) {
        if self.app.cursor.y > 0 {
            self.app.cursor.y -= 1;
        }
    }

    fn move_cur_right(&mut self) {
        if self.app.cursor.x + 1 < self.app.viewport_size.width {
            self.app.cursor.x += 1;
        }
    }

    fn move_cur_right_by(&mut self, amount: usize) {
        let max_x = self.app.viewport_size.width.saturating_sub(1);
        self.app.cursor.x = (self.app.cursor.x + amount).min(max_x);
    }

    fn move_cur_down(&mut self) {
        if self.app.cursor.y + 1 < self.app.viewport_size.height {
            self.app.cursor.y += 1;
        }
    }

    fn rotate_last_shape(&mut self) {
        if let Some(index) = self.app.last_pattern {
            if let Some(pattern) = self
                .app
                .configuration
                .get_mut(self.app.current_pattern_type)
                .and_then(|p| p.patterns.get_mut(index))
            {
                pattern.rotation_count = (pattern.rotation_count + 1) % 4;
                *pattern = pattern.rotate_90();
            }
        }
    }

    fn set_cursor_from_screen(&mut self, x: usize, y: usize) {
        if let Some(area) = self.last_canvas_area {
            let mouse_x = x as u16;
            let mouse_y = y as u16;

            if mouse_x >= area.x
                && mouse_x < area.x.saturating_add(area.width)
                && mouse_y >= area.y
                && mouse_y < area.y.saturating_add(area.height)
            {
                let view_x = (mouse_x - area.x) as usize;
                let view_y = (mouse_y - area.y) as usize;
                self.app.cursor.x = view_x.min(self.app.viewport_size.width.saturating_sub(1));
                self.app.cursor.y = view_y.min(self.app.viewport_size.height.saturating_sub(1));
            }
        }
    }

    // Gallery navigation helpers
    fn gallery_move_up(&mut self) {
        let nodes = compute_visible_nodes(&self.app);
        if nodes.is_empty() {
            return;
        }

        // Find current position in the flat list
        let current_idx = nodes.iter().position(|n| {
            n.type_idx == self.app.gallery_cursor.pattern_type_idx
                && n.pattern_idx == self.app.gallery_cursor.pattern_idx
        });

        if let Some(idx) = current_idx {
            if idx > 0 {
                let prev = &nodes[idx - 1];
                self.app.gallery_cursor.pattern_type_idx = prev.type_idx;
                self.app.gallery_cursor.pattern_idx = prev.pattern_idx;
            }
        }
    }

    fn gallery_move_down(&mut self) {
        let nodes = compute_visible_nodes(&self.app);
        if nodes.is_empty() {
            return;
        }

        let current_idx = nodes.iter().position(|n| {
            n.type_idx == self.app.gallery_cursor.pattern_type_idx
                && n.pattern_idx == self.app.gallery_cursor.pattern_idx
        });

        if let Some(idx) = current_idx {
            if idx + 1 < nodes.len() {
                let next = &nodes[idx + 1];
                self.app.gallery_cursor.pattern_type_idx = next.type_idx;
                self.app.gallery_cursor.pattern_idx = next.pattern_idx;
            }
        }
    }

    fn gallery_expand(&mut self) {
        let cursor = &self.app.gallery_cursor;
        let type_idx = cursor.pattern_type_idx;

        if cursor.pattern_idx.is_none() {
            // On a type node
            let already_expanded = self
                .app
                .gallery_cursor
                .expanded_types
                .get(type_idx)
                .copied()
                .unwrap_or(false);

            if !already_expanded {
                // Expand the type
                if type_idx < self.app.gallery_cursor.expanded_types.len() {
                    self.app.gallery_cursor.expanded_types[type_idx] = true;
                }
            } else {
                // Already expanded, move into first child
                if let Some(pt) = self.app.configuration.get(type_idx) {
                    if !pt.patterns.is_empty() {
                        self.app.gallery_cursor.pattern_idx = Some(0);
                    }
                }
            }
        }
    }

    fn gallery_collapse(&mut self) {
        let cursor = &self.app.gallery_cursor;
        let type_idx = cursor.pattern_type_idx;

        if cursor.pattern_idx.is_some() {
            // On a pattern, go back to parent type
            self.app.gallery_cursor.pattern_idx = None;
        } else {
            // On a type, collapse it
            if type_idx < self.app.gallery_cursor.expanded_types.len() {
                self.app.gallery_cursor.expanded_types[type_idx] = false;
            }
        }
    }

    fn gallery_select(&mut self) {
        let cursor = &self.app.gallery_cursor;

        if let Some(pat_idx) = cursor.pattern_idx {
            // Select pattern
            self.app.current_pattern_type = cursor.pattern_type_idx;
            self.app.last_pattern = Some(pat_idx);
            // Optionally exit gallery mode after selection
            self.app.mode = AppMode::Normal;
        } else {
            // On a type node, toggle expand/collapse
            let type_idx = cursor.pattern_type_idx;
            if type_idx < self.app.gallery_cursor.expanded_types.len() {
                self.app.gallery_cursor.expanded_types[type_idx] =
                    !self.app.gallery_cursor.expanded_types[type_idx];
            }
        }
    }

    fn handle_command(&mut self, command: Command) -> io::Result<bool> {
        let grid_position = self.app.grid_cursor();
        let mut should_quit = false;

        match command {
            Command::Quit => {
                should_quit = true;
            }
            Command::MoveCursorLeft => self.move_cur_left(),
            Command::MoveCursorRight => self.move_cur_right(),
            Command::MoveCursorUp => self.move_cur_up(),
            Command::MoveCursorDown => self.move_cur_down(),
            Command::MoveCursorLeftBy(amount) => self.move_cur_left_by(amount),
            Command::MoveCursorRightBy(amount) => self.move_cur_right_by(amount),
            Command::MoveCursorToStartOfLine => self.app.cursor.x = 0,
            Command::MoveCursorToEndOfLine => {
                if self.app.viewport_size.width > 0 {
                    self.app.cursor.x = self.app.viewport_size.width - 1;
                }
            }
            Command::ToggleCellAlive => {
                self.app.grid.resurrect(grid_position);
                self.move_cur_right();
            }
            Command::ToggleCellDead => {
                self.app.grid.kill(grid_position);
                self.move_cur_left();
            }
            Command::ClearGrid => {
                let (grid, size) = init_grid_and_size(self.app.grid_multiplier)?;
                self.app.grid = grid;
                self.app.viewport_size = size;
                self.app
                    .viewport
                    .update_size(self.app.viewport_size.clone(), self.app.grid.get_size());
                self.center_cursor();
            }
            Command::PlaceLastPattern => {
                if let Some(index) = self.app.last_pattern {
                    if let Some(patterns) = self
                        .app
                        .configuration
                        .get(self.app.current_pattern_type)
                        .map(|p| &p.patterns)
                    {
                        if let Some(pattern) = patterns.get(index) {
                            self.app.grid.shape(grid_position, &pattern.matrix);
                        }
                    }
                }
            }
            Command::CyclePatternType => {
                self.app.last_pattern = None;
                if self.app.current_pattern_type + 1 >= self.app.configuration.len() {
                    self.app.current_pattern_type = 0;
                } else {
                    self.app.current_pattern_type += 1;
                }
            }
            Command::RotateLastPattern => {
                self.rotate_last_shape();
            }
            Command::ToggleSimulation => {
                self.app.running = !self.app.running;
            }
            Command::StepSimulation => {
                self.app.grid.generate();
                self.last_tick = Instant::now();
            }
            Command::SpeedUp => {
                if let Some(val) = self.app.simulation_delay.checked_sub(10) {
                    self.app.simulation_delay = val;
                }
            }
            Command::SpeedDown => {
                if let Some(val) = self.app.simulation_delay.checked_add(10) {
                    self.app.simulation_delay = val;
                }
            }
            Command::PlacePattern(index) => {
                if let Some(pattern_type) =
                    self.app.configuration.get(self.app.current_pattern_type)
                {
                    if index < pattern_type.patterns.len() {
                        self.app.last_pattern = Some(index);
                        self.app
                            .grid
                            .shape(grid_position, &pattern_type.patterns[index].matrix);
                    }
                }
            }
            Command::ShowHelp => {
                self.app.mode = AppMode::Help;
            }
            Command::ExitHelp => {
                self.app.mode = AppMode::Normal;
            }
            Command::SetCursorPosition(x, y) => {
                self.set_cursor_from_screen(x, y);
            }
            // Gallery commands
            Command::EnterGalleryMode => {
                self.app.mode = AppMode::PatternGallery;
            }
            Command::ExitGalleryMode => {
                self.app.mode = AppMode::Normal;
            }
            Command::GalleryUp => {
                self.gallery_move_up();
            }
            Command::GalleryDown => {
                self.gallery_move_down();
            }
            Command::GalleryExpand => {
                self.gallery_expand();
            }
            Command::GalleryCollapse => {
                self.gallery_collapse();
            }
            Command::GallerySelect => {
                self.gallery_select();
            }
            Command::NoOp => {}
        }

        self.clamp_cursor();
        Ok(should_quit)
    }
}

impl Drop for Orchestrator {
    fn drop(&mut self) {
        let _ = restore_terminal(&mut self.terminal);
    }
}

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, event::EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
