use termion::event::Event;
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

#[derive(Debug, Clone)]
pub enum Command {
    Quit,
    MoveCursorLeft,
    MoveCursorRight,
    MoveCursorUp,
    MoveCursorDown,
    MoveCursorLeftBy(usize),
    MoveCursorRightBy(usize),
    MoveCursorToStartOfLine,
    MoveCursorToEndOfLine,
    ToggleCellAlive,
    ToggleCellDead,
    ClearGrid,
    PlaceLastPattern,
    CyclePatternType,
    RotateLastPattern,
    ToggleSimulation,
    StepSimulation,
    SpeedUp,
    SpeedDown,
    PlacePattern(usize),
    ShowHelp,
    ExitHelp,
    SetCursorPosition(usize, usize),
    NoOp,
}

pub struct CommandHandler;

impl CommandHandler {
    /// Convert a termion event into a command
    pub fn event_to_command(event: &Event, in_help_mode: bool) -> Command {
        if in_help_mode {
            if let Event::Key(key) = event {
                if *key == Esc || *key == Char('h') {
                    return Command::ExitHelp;
                }
            }
            return Command::NoOp;
        }

        match event {
            Event::Key(key) => Self::key_to_command(key),
            Event::Mouse(MouseEvent::Press(_, x, y)) => {
                Command::SetCursorPosition(*x as usize, *y as usize)
            }
            _ => Command::NoOp,
        }
    }

    fn key_to_command(key: &termion::event::Key) -> Command {
        match key {
            Ctrl('c') => Command::Quit,
            Char('q') => Command::Quit,
            Left => Command::MoveCursorLeft,
            Right => Command::MoveCursorRight,
            Up => Command::MoveCursorUp,
            Down => Command::MoveCursorDown,
            Backspace => Command::ToggleCellDead,
            BackTab => Command::MoveCursorLeftBy(4),
            Char('\t') => Command::MoveCursorRightBy(4),
            Char('a') => Command::ToggleCellAlive,
            Char('b') => Command::MoveCursorToStartOfLine,
            Char('c') => Command::ClearGrid,
            Char('d') => Command::ToggleCellDead,
            Char('e') => Command::MoveCursorToEndOfLine,
            Char('h') => Command::ShowHelp,
            Char('l') => Command::PlaceLastPattern,
            Char('p') => Command::CyclePatternType,
            Char('r') => Command::RotateLastPattern,
            Char('s') => Command::ToggleSimulation,
            Char(' ') => Command::StepSimulation,
            Char('+') => Command::SpeedUp,
            Char('-') => Command::SpeedDown,
            Char(c) if c.is_ascii_digit() => {
                if let Some(digit) = c.to_digit(10) {
                    let mut index = digit as usize;
                    if index > 0 {
                        index -= 1;
                        Command::PlacePattern(index)
                    } else {
                        Command::NoOp
                    }
                } else {
                    Command::NoOp
                }
            }
            _ => Command::NoOp,
        }
    }
}
