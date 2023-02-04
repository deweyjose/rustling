use std::fmt;

#[derive(Debug)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}x{})", self.width, self.height)
    }
}
