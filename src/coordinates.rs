use std::fmt;

#[derive(Debug)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}
