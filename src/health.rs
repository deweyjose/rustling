use std::fmt;

use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum Health {
    Alive = 1,
    Dead = 0,
}

impl fmt::Display for Health {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Health::Dead => f.write_str(" "),
            Health::Alive => f.write_str("@"),
        }
    }
}
