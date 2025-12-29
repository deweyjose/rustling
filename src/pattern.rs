use serde::Deserialize;
use serde::Serialize;

use crate::health::Health;

#[derive(Deserialize, Serialize, Clone)]
pub struct Pattern {
    pub name: String,
    pub matrix: Vec<Vec<Health>>,
    #[serde(default)]
    pub rotation_count: usize, // 0-3 = 0°, 90°, 180°, 270°
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PatternType {
    pub name: String,
    pub patterns: Vec<Pattern>,
}

impl Pattern {
    /// Rotate the pattern 90 degrees clockwise, returning a new Pattern
    /// Handles both square and rectangular matrices
    pub fn rotate_90(&self) -> Pattern {
        if self.matrix.is_empty() || self.matrix[0].is_empty() {
            return self.clone();
        }

        let rows = self.matrix.len();
        let cols = self.matrix[0].len();

        // Create new matrix with dimensions swapped
        let mut rotated = Vec::with_capacity(cols);
        for _ in 0..cols {
            rotated.push(Vec::with_capacity(rows));
        }

        // Rotate by transposing and reversing rows
        for row in self.matrix.iter() {
            for (col_idx, cell) in row.iter().enumerate() {
                rotated[col_idx].push(*cell);
            }
        }

        // Reverse each row to complete 90-degree clockwise rotation
        for row in rotated.iter_mut() {
            row.reverse();
        }

        Pattern {
            name: self.name.clone(),
            matrix: rotated,
            rotation_count: self.rotation_count,
        }
    }
}
