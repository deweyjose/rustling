use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::sync::mpsc::Sender;

use termion::event::Event;
use termion::event::Key::Char;
use termion::event::Key::Ctrl;
use termion::input::MouseTerminal;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

/// Captures user input from the terminal in a separate thread.
/// Reads keyboard and mouse events and sends them through the provided channel.
/// Exits on Ctrl+C or 'q' key press.
pub fn capture_input(tx: Sender<Event>) {
    let mut stdout = match stdout().into_raw_mode() {
        Ok(raw_stdout) => MouseTerminal::from(raw_stdout),
        Err(e) => {
            eprintln!("Failed to enter raw mode: {}", e);
            return;
        }
    };

    let stdin = stdin();
    for result in stdin.events() {
        let event = match result {
            Ok(event) => event,
            Err(e) => {
                eprintln!("Error reading input event: {}", e);
                continue;
            }
        };

        tx.send(event.clone()).expect("Failed to send input event");

        match event {
            Event::Key(Ctrl('c')) => break,
            Event::Key(Char('q')) => break,
            _ => {}
        }
        let _ = stdout.flush(); // Ignore flush errors as they're not critical
    }
}
