use ratatui::prelude::*;
use ratatui::widgets::{Block, List, ListItem, ListState};

use crate::app::{App, AppMode};
use crate::theme::Theme;

/// A tree node in the flattened list
#[derive(Clone)]
pub struct GalleryNode {
    pub text: String,
    pub type_idx: usize,
    pub pattern_idx: Option<usize>,
    pub is_selected_last: bool,
}

pub struct PatternGallery<'a> {
    app: &'a App,
    theme: &'a Theme,
}

impl<'a> PatternGallery<'a> {
    pub fn new(app: &'a App, theme: &'a Theme) -> Self {
        Self { app, theme }
    }

    /// Build a flat list of nodes from the tree structure
    pub fn build_nodes(&self) -> Vec<GalleryNode> {
        let mut nodes = Vec::new();

        for (type_idx, pattern_type) in self.app.configuration.iter().enumerate() {
            let expanded = self
                .app
                .gallery_cursor
                .expanded_types
                .get(type_idx)
                .copied()
                .unwrap_or(true);

            let indicator = if expanded { "▼" } else { "▶" };
            nodes.push(GalleryNode {
                text: format!("{} {}", indicator, pattern_type.name),
                type_idx,
                pattern_idx: None,
                is_selected_last: false,
            });

            if expanded {
                for (pat_idx, pattern) in pattern_type.patterns.iter().enumerate() {
                    let is_selected_last = self.app.last_pattern == Some(pat_idx)
                        && self.app.current_pattern_type == type_idx;

                    let marker = if is_selected_last { "*" } else { " " };
                    nodes.push(GalleryNode {
                        text: format!("  {} {}", marker, pattern.name),
                        type_idx,
                        pattern_idx: Some(pat_idx),
                        is_selected_last,
                    });
                }
            }
        }

        nodes
    }

    /// Find the list index for the current gallery cursor position
    pub fn cursor_to_list_index(&self, nodes: &[GalleryNode]) -> Option<usize> {
        let cursor = &self.app.gallery_cursor;
        nodes.iter().position(|node| {
            node.type_idx == cursor.pattern_type_idx && node.pattern_idx == cursor.pattern_idx
        })
    }

    /// Build list items from nodes
    fn build_list_items<'b>(&self, nodes: &'b [GalleryNode]) -> Vec<ListItem<'b>> {
        nodes
            .iter()
            .map(|node| {
                let mut style = Style::default();
                if node.is_selected_last {
                    style = style.fg(Color::Yellow);
                }
                ListItem::new(Line::from(node.text.clone()).style(style))
            })
            .collect()
    }
}

impl StatefulWidget for PatternGallery<'_> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let nodes = self.build_nodes();
        let is_gallery_mode = self.app.mode == AppMode::PatternGallery;

        // Sync list state with gallery cursor when in gallery mode
        if is_gallery_mode {
            if let Some(idx) = self.cursor_to_list_index(&nodes) {
                state.select(Some(idx));
            }
        } else {
            state.select(None);
        }

        let items = self.build_list_items(&nodes);

        let highlight_style = if is_gallery_mode {
            self.theme.gallery_focus
        } else {
            Style::default()
        };

        let list = List::new(items)
            .block(
                Block::bordered()
                    .title(" Patterns ")
                    .border_style(self.theme.border),
            )
            .highlight_style(highlight_style)
            .highlight_symbol("► ");

        StatefulWidget::render(list, area, buf, state);
    }
}

/// Helper to compute visible nodes for navigation
pub fn compute_visible_nodes(app: &App) -> Vec<GalleryNode> {
    let mut nodes = Vec::new();

    for (type_idx, pattern_type) in app.configuration.iter().enumerate() {
        let expanded = app
            .gallery_cursor
            .expanded_types
            .get(type_idx)
            .copied()
            .unwrap_or(true);

        let indicator = if expanded { "▼" } else { "▶" };
        nodes.push(GalleryNode {
            text: format!("{} {}", indicator, pattern_type.name),
            type_idx,
            pattern_idx: None,
            is_selected_last: false,
        });

        if expanded {
            for (pat_idx, pattern) in pattern_type.patterns.iter().enumerate() {
                let is_selected_last =
                    app.last_pattern == Some(pat_idx) && app.current_pattern_type == type_idx;

                nodes.push(GalleryNode {
                    text: format!("    {}", pattern.name),
                    type_idx,
                    pattern_idx: Some(pat_idx),
                    is_selected_last,
                });
            }
        }
    }

    nodes
}
