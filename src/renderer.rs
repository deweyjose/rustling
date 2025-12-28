use std::io::stdout;
use std::io::Write;

use termion::cursor;
use termion::style;
use termion::{clear, color};

use crate::coordinates::Coordinates;
use crate::grid::Grid;
use crate::pattern;
use crate::size::Size;
use crate::viewport::Viewport;

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

pub struct RenderState<'a> {
    pub cur_pos: &'a Coordinates,
    pub running: bool,
    pub configuration: &'a [pattern::PatternType],
    pub current_pattern_type: usize,
    pub last_pattern: Option<usize>,
    pub last_pattern_rotation: &'a std::collections::HashMap<usize, usize>,
}

pub struct Renderer;

impl Renderer {
    pub fn render_header(width: usize) {
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

    pub fn render_grid(grid: &Grid, viewport: &Viewport, viewport_size: &Size) {
        let x_offset = viewport.x_offset();
        let y_offset = viewport.y_offset();

        for row in 2..viewport_size.height {
            let grid_row = y_offset + row - 2;
            if grid_row < grid.get_size().height {
                let health = grid.get_row(grid_row);

                print!("{}{}", cursor::Goto(1, row as u16), color::Fg(color::Green));
                stdout().flush().unwrap();

                for column in 0..viewport_size.width {
                    let grid_col = column + x_offset;
                    if grid_col < grid.get_size().width {
                        print!("{}", health[grid_col]);
                    } else {
                        print!(" ");
                    }
                }

                stdout().flush().unwrap();
            } else {
                // Clear remaining lines if grid is shorter than viewport
                print!("{}{}", cursor::Goto(1, row as u16), clear::CurrentLine);
            }
        }
    }

    pub fn render_footer(
        grid_size: &Size,
        viewport_size: &Size,
        render_state: &RenderState,
    ) {
        let (last_pattern, last_rotation) = if let Some(last) = render_state.last_pattern {
            let pattern = render_state.configuration[render_state.current_pattern_type].patterns[last]
                .name
                .as_str();

            let rotation = render_state
                .last_pattern_rotation
                .get(&last)
                .unwrap_or(&0);

            (pattern, rotation)
        } else {
            ("none", &0)
        };

        let width = viewport_size.width;
        let footer = format!(
            "grid {}, view-port {}, cursor {}, (s){}, (p)attern class: {}, (l)ast pattern {}, (r)otation {} degrees",
            grid_size,
            viewport_size,
            render_state.cur_pos,
            if render_state.running {
                "top"
            } else {
                "tart"
            },
            render_state.configuration[render_state.current_pattern_type].name,
            last_pattern,
            last_rotation
        );

        print!(
            "{}{}{}{:width$}{}",
            cursor::Goto(1, (viewport_size.height + 1) as u16),
            color::Fg(color::Red),
            style::Bold,
            footer,
            style::Reset
        );
        stdout().flush().unwrap();
    }

    pub fn render_help(
        viewport_size: &Size,
        grid_height: usize,
        configuration: &[pattern::PatternType],
    ) {
        Self::render_header(viewport_size.width);

        let width = viewport_size.width;
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

        for pattern_type in configuration {
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

        for line in current_line..grid_height as u16 {
            print!("{}{}", cursor::Goto(1, line), clear::CurrentLine);
        }
    }

    pub fn render_all(
        grid: &Grid,
        viewport: &Viewport,
        viewport_size: &Size,
        render_state: &RenderState,
    ) {
        Self::render_header(viewport_size.width);
        Self::render_grid(grid, viewport, viewport_size);
        Self::render_footer(grid.get_size(), viewport_size, render_state);
    }

    pub fn set_cursor_pos(pos: &Coordinates) {
        print!("{}", cursor::Goto(pos.x as u16, pos.y as u16));
        stdout().flush().unwrap();
    }
}

