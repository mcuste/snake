//! Collision detection systems.

use bevy::prelude::*;

use crate::{
    components::{GridPos, SnakeHead, SnakeSegment},
    config::{GRID_HEIGHT, GRID_WIDTH},
    resources::GameOverEvent,
    state::GameState,
};

/// Update (`GameSet::Collision`): emits `GameOverEvent` when head leaves the grid.
pub(crate) fn check_wall_collision(
    head_query: Query<'_, '_, &GridPos, With<SnakeHead>>,
    mut game_over: MessageWriter<'_, GameOverEvent>,
) {
    if let Ok(pos) = head_query.single() {
        if pos.x < 0 || pos.x >= GRID_WIDTH || pos.y < 0 || pos.y >= GRID_HEIGHT {
            game_over.write_default();
        }
    }
}

/// Update (`GameSet::Collision`): emits `GameOverEvent` when head overlaps a segment.
pub(crate) fn check_self_collision(
    head_query: Query<'_, '_, &GridPos, With<SnakeHead>>,
    segment_query: Query<'_, '_, &GridPos, With<SnakeSegment>>,
    mut game_over: MessageWriter<'_, GameOverEvent>,
) {
    let Ok(head_pos) = head_query.single() else {
        return;
    };
    for seg_pos in &segment_query {
        if head_pos == seg_pos {
            game_over.write_default();
            return;
        }
    }
}

/// Update (always active): transitions to `GameState::GameOver` on `GameOverEvent`.
pub(crate) fn handle_game_over(
    mut events: MessageReader<'_, '_, GameOverEvent>,
    mut next_state: ResMut<'_, NextState<GameState>>,
) {
    for _ in events.read() {
        next_state.set(GameState::GameOver);
    }
}
