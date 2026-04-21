//! Game lifecycle systems (quit and restart).

use std::collections::HashSet;

use bevy::prelude::*;
use bevy_ratatui::event::KeyMessage;
use rand::Rng as _;
use ratatui::crossterm::event::{KeyCode, KeyEventKind};

use crate::{
    components::{Direction, Food, GridPos, SnakeHead, SnakeSegment},
    config::{GRID_CENTRE_X, GRID_CENTRE_Y, GRID_HEIGHT, GRID_WIDTH, SPEED_INITIAL_MS},
    resources::{MoveTimer, PendingDirection, Score, SnakeBody, TailPosition},
    state::GameState,
};

/// Update (always active): writes `AppExit::Success` on `q` or `Esc` keypress.
pub(crate) fn handle_quit(
    mut messages: MessageReader<'_, '_, KeyMessage>,
    mut app_exit: MessageWriter<'_, AppExit>,
) {
    for msg in messages.read() {
        if msg.kind != KeyEventKind::Press {
            continue;
        }
        if matches!(msg.code, KeyCode::Char('q') | KeyCode::Esc) {
            app_exit.write(AppExit::Success);
            return;
        }
    }
}

/// Update (`GameState::GameOver` only): resets game state on `r` keypress.
#[expect(
    clippy::too_many_arguments,
    reason = "Bevy system function; all parameters are required ECS resources and queries for the restart logic"
)]
pub(crate) fn handle_restart(
    mut messages: MessageReader<'_, '_, KeyMessage>,
    mut commands: Commands<'_, '_>,
    mut body: ResMut<'_, SnakeBody>,
    mut pending: ResMut<'_, PendingDirection>,
    mut score: ResMut<'_, Score>,
    mut timer: ResMut<'_, MoveTimer>,
    mut tail_pos: ResMut<'_, TailPosition>,
    mut next_state: ResMut<'_, NextState<GameState>>,
    mut head_pos_query: Query<'_, '_, &mut GridPos, With<SnakeHead>>,
    mut head_dir_query: Query<'_, '_, &mut Direction, With<SnakeHead>>,
    segment_query: Query<'_, '_, Entity, With<SnakeSegment>>,
    food_query: Query<'_, '_, Entity, With<Food>>,
) {
    let pressed_r = messages
        .read()
        .any(|msg| msg.kind == KeyEventKind::Press && msg.code == KeyCode::Char('r'));
    if !pressed_r {
        return;
    }

    // Despawn all segments.
    for entity in &segment_query {
        commands.entity(entity).despawn();
    }

    // Extract head entity (last in body list) before clearing.
    let head_entity = body.0.last().copied();
    body.0.clear();

    // Reset head position and direction.
    if let Ok(mut pos) = head_pos_query.single_mut() {
        pos.x = GRID_CENTRE_X;
        pos.y = GRID_CENTRE_Y;
    }
    if let Ok(mut dir) = head_dir_query.single_mut() {
        *dir = Direction::Right;
    }

    // Spawn 2 initial segments.
    let seg1 = commands
        .spawn((
            SnakeSegment { order: 0 },
            GridPos {
                x: GRID_CENTRE_X - 1,
                y: GRID_CENTRE_Y,
            },
        ))
        .id();
    let seg2 = commands
        .spawn((
            SnakeSegment { order: 1 },
            GridPos {
                x: GRID_CENTRE_X - 2,
                y: GRID_CENTRE_Y,
            },
        ))
        .id();

    // Rebuild body: [seg2, seg1, head].
    if let Some(head) = head_entity {
        body.0 = vec![seg2, seg1, head];
    }

    // Reset resources.
    score.0 = 0;
    pending.0 = None;
    tail_pos.0 = None;
    *timer = MoveTimer::new(SPEED_INITIAL_MS);

    // Despawn all food.
    for entity in &food_query {
        commands.entity(entity).despawn();
    }

    // Spawn new food at random unoccupied cell.
    // After restart the snake occupies exactly these 3 cells; compute the set directly
    // to avoid a Query<&GridPos> that would conflict with the mutable head-pos query above.
    let occupied: HashSet<(i32, i32)> = [
        (GRID_CENTRE_X, GRID_CENTRE_Y),
        (GRID_CENTRE_X - 1, GRID_CENTRE_Y),
        (GRID_CENTRE_X - 2, GRID_CENTRE_Y),
    ]
    .into_iter()
    .collect();
    let mut rng = rand::rng();
    loop {
        let x = rng.random_range(0..GRID_WIDTH);
        let y = rng.random_range(0..GRID_HEIGHT);
        if !occupied.contains(&(x, y)) {
            commands.spawn((Food, GridPos { x, y }));
            break;
        }
    }

    next_state.set(GameState::Playing);
}
