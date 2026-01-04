use ratatui::widgets::ListState;

use crate::coordinates::Coordinates;
use crate::grid::Grid;
use crate::pattern::PatternType;
use crate::size::Size;
use crate::viewport::Viewport;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Normal,
    Help,
    PatternGallery,
}

/// Tracks the gallery navigation state
pub struct GalleryCursor {
    pub pattern_type_idx: usize,
    pub pattern_idx: Option<usize>,
    pub expanded_types: Vec<bool>,
    pub list_state: ListState,
}

impl GalleryCursor {
    pub fn new(num_types: usize) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            pattern_type_idx: 0,
            pattern_idx: None,
            expanded_types: vec![true; num_types],
            list_state,
        }
    }
}

pub struct App {
    pub grid: Grid,
    pub cursor: Coordinates,
    pub viewport: Viewport,
    pub viewport_size: Size,
    pub running: bool,
    pub configuration: Vec<PatternType>,
    pub current_pattern_type: usize,
    pub last_pattern: Option<usize>,
    pub simulation_delay: u128,
    pub mode: AppMode,
    pub gallery_cursor: GalleryCursor,
}

impl App {
    pub fn rotation_count(&self) -> usize {
        self.last_pattern
            .and_then(|idx| {
                self.configuration
                    .get(self.current_pattern_type)
                    .and_then(|pt| pt.patterns.get(idx))
            })
            .map(|p| p.rotation_count)
            .unwrap_or(0)
    }

    pub fn rotation_degrees(&self) -> usize {
        self.rotation_count() * 90
    }

    pub fn current_pattern_type_name(&self) -> &str {
        self.configuration
            .get(self.current_pattern_type)
            .map(|p| p.name.as_str())
            .unwrap_or("unknown")
    }

    pub fn grid_cursor(&self) -> Coordinates {
        self.viewport.view_to_grid(self.cursor.clone())
    }
}
