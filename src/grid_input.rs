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

// run off the main thread ... read a key and transmit it
// if ctrl+c or q break
pub fn grid_input(tx: Sender<Event>) {
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    //    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();
    for result in stdin.events() {
        let event = result.unwrap();
        tx.send(event.clone()).expect("TODO: panic message");

        match event {
            Event::Key(Ctrl('c')) => break,
            Event::Key(Char('q')) => break,
            _ => {}
        }
        stdout.flush().unwrap();
    }
}
