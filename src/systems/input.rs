//! Input handling systems.

use bevy::prelude::*;
use bevy_ratatui::event::{KeyMessage, ResizeMessage};
use ratatui::crossterm::event::{KeyCode, KeyEventKind};

use crate::{
    components::{Direction, SnakeHead},
    resources::{PendingDirection, TerminalSize},
};

/// `PreUpdate` (after `InputSet::Post`): maps key events to `PendingDirection`.
///
/// Handles all three control schemes: arrow keys, WASD, and vim hjkl.
/// Ignores presses that would reverse the snake's current direction.
pub(crate) fn read_input(
    mut messages: MessageReader<'_, '_, KeyMessage>,
    mut pending: ResMut<'_, PendingDirection>,
    head_dir: Query<'_, '_, &Direction, With<SnakeHead>>,
) {
    let current = head_dir.single().copied().ok();

    for msg in messages.read() {
        if msg.kind != KeyEventKind::Press {
            continue;
        }
        let candidate = match msg.code {
            KeyCode::Up | KeyCode::Char('w' | 'W' | 'k') => Direction::Up,
            KeyCode::Down | KeyCode::Char('s' | 'S' | 'j') => Direction::Down,
            KeyCode::Left | KeyCode::Char('a' | 'A' | 'h') => Direction::Left,
            KeyCode::Right | KeyCode::Char('d' | 'D' | 'l') => Direction::Right,
            _ => continue,
        };
        if current.is_some_and(|d| candidate == d.opposite()) {
            continue;
        }
        pending.0 = Some(candidate);
    }
}

/// `PreUpdate` (after `InputSet::Post`): updates `TerminalSize` on resize events.
pub(crate) fn handle_resize(
    mut messages: MessageReader<'_, '_, ResizeMessage>,
    mut terminal_size: ResMut<'_, TerminalSize>,
) {
    for msg in messages.read() {
        terminal_size.cols = msg.width;
        terminal_size.rows = msg.height;
    }
}
