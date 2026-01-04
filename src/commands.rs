use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;
use crossterm::event::KeyModifiers;
use crossterm::event::MouseButton;
use crossterm::event::MouseEvent;
use crossterm::event::MouseEventKind;

use crate::app::AppMode;

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
    // Gallery commands
    EnterGalleryMode,
    ExitGalleryMode,
    GalleryUp,
    GalleryDown,
    GalleryExpand,
    GalleryCollapse,
    GallerySelect,
    NoOp,
}

pub struct CommandHandler;

impl CommandHandler {
    /// Convert a crossterm event into a command based on current mode
    pub fn event_to_command(event: &Event, mode: AppMode) -> Command {
        match mode {
            AppMode::Help => Self::help_event_to_command(event),
            AppMode::PatternGallery => Self::gallery_event_to_command(event),
            AppMode::Normal => Self::normal_event_to_command(event),
        }
    }

    fn help_event_to_command(event: &Event) -> Command {
        if let Event::Key(key) = event {
            if (key.code == KeyCode::Esc || key.code == KeyCode::Char('h'))
                && key.kind == KeyEventKind::Press
            {
                return Command::ExitHelp;
            }
        }
        Command::NoOp
    }

    fn gallery_event_to_command(event: &Event) -> Command {
        if let Event::Key(key) = event {
            if key.kind != KeyEventKind::Press {
                return Command::NoOp;
            }
            return match key.code {
                KeyCode::Char('g') | KeyCode::Esc => Command::ExitGalleryMode,
                KeyCode::Up => Command::GalleryUp,
                KeyCode::Down => Command::GalleryDown,
                KeyCode::Left => Command::GalleryCollapse,
                KeyCode::Right => Command::GalleryExpand,
                KeyCode::Enter => Command::GallerySelect,
                KeyCode::Char('q') => Command::Quit,
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    Command::Quit
                }
                _ => Command::NoOp,
            };
        }
        Command::NoOp
    }

    fn normal_event_to_command(event: &Event) -> Command {
        match event {
            Event::Key(key) => Self::key_to_command(key),
            Event::Mouse(mouse) => Self::mouse_to_command(mouse),
            _ => Command::NoOp,
        }
    }

    fn mouse_to_command(event: &MouseEvent) -> Command {
        match event.kind {
            MouseEventKind::Down(MouseButton::Left) | MouseEventKind::Drag(MouseButton::Left) => {
                Command::SetCursorPosition(event.column as usize, event.row as usize)
            }
            _ => Command::NoOp,
        }
    }

    fn key_to_command(key: &KeyEvent) -> Command {
        if key.kind != KeyEventKind::Press {
            return Command::NoOp;
        }

        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Command::Quit,
            KeyCode::Char('q') => Command::Quit,
            KeyCode::Left => Command::MoveCursorLeft,
            KeyCode::Right => Command::MoveCursorRight,
            KeyCode::Up => Command::MoveCursorUp,
            KeyCode::Down => Command::MoveCursorDown,
            KeyCode::Backspace => Command::ToggleCellDead,
            KeyCode::BackTab => Command::MoveCursorLeftBy(4),
            KeyCode::Tab => Command::MoveCursorRightBy(4),
            KeyCode::Char('a') => Command::ToggleCellAlive,
            KeyCode::Char('b') => Command::MoveCursorToStartOfLine,
            KeyCode::Char('c') => Command::ClearGrid,
            KeyCode::Char('d') => Command::ToggleCellDead,
            KeyCode::Char('e') => Command::MoveCursorToEndOfLine,
            KeyCode::Char('g') => Command::EnterGalleryMode,
            KeyCode::Char('h') => Command::ShowHelp,
            KeyCode::Char('l') => Command::PlaceLastPattern,
            KeyCode::Char('p') => Command::CyclePatternType,
            KeyCode::Char('r') => Command::RotateLastPattern,
            KeyCode::Char('s') => Command::ToggleSimulation,
            KeyCode::Char(' ') => Command::StepSimulation,
            KeyCode::Char('+') => Command::SpeedUp,
            KeyCode::Char('-') => Command::SpeedDown,
            KeyCode::Char(c) if c.is_ascii_digit() => {
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
