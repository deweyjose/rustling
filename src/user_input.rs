use std::io;
use std::time::Duration;

use crossterm::event::{self, Event};

/// Poll for a single crossterm event with a timeout.
/// Returns Ok(None) when no event is available within the timeout.
pub fn poll_event(timeout: Duration) -> io::Result<Option<Event>> {
    if event::poll(timeout).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? {
        Ok(Some(
            event::read().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?,
        ))
    } else {
        Ok(None)
    }
}
