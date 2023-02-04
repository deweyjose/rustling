use serde::Deserialize;
use serde::Serialize;

use crate::health::Health;

#[derive(Deserialize, Serialize)]
pub struct Pattern {
    pub name: String,
    pub matrix: Vec<Vec<Health>>,
}

#[derive(Deserialize, Serialize)]
pub struct PatternType {
    pub name: String,
    pub patterns: Vec<Pattern>,
}
