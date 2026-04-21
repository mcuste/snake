//! Snake movement systems.

use bevy::prelude::*;

use crate::{
    components::{Direction, GridPos, SnakeHead, SnakeSegment},
    config::{GRID_CENTRE_X, GRID_CENTRE_Y, SPEED_INITIAL_MS},
    resources::{MoveTimer, PendingDirection, SnakeBody, TailPosition},
};

/// Startup: spawns the initial 3-entity snake at the grid centre.
///
/// Layout (tail-first in `SnakeBody`):
/// ```text
/// [seg_order_1, seg_order_0, head]   ← SnakeBody.0 order
/// (cx-2, cy)   (cx-1, cy)   (cx, cy) ← positions, moving Right
/// ```
pub(crate) fn spawn_initial_snake(mut commands: Commands<'_, '_>, mut body: ResMut<'_, SnakeBody>) {
    let head = commands
        .spawn((
            SnakeHead,
            GridPos {
                x: GRID_CENTRE_X,
                y: GRID_CENTRE_Y,
            },
            Direction::Right,
        ))
        .id();

    let seg0 = commands
        .spawn((
            SnakeSegment { order: 0 },
            GridPos {
                x: GRID_CENTRE_X - 1,
                y: GRID_CENTRE_Y,
            },
        ))
        .id();

    let seg1 = commands
        .spawn((
            SnakeSegment { order: 1 },
            GridPos {
                x: GRID_CENTRE_X - 2,
                y: GRID_CENTRE_Y,
            },
        ))
        .id();

    // Tail-first: furthest segment first, head last.
    body.0 = vec![seg1, seg0, head];
}

/// Startup: inserts the [`MoveTimer`] resource with the initial interval.
pub(crate) fn init_timer(mut commands: Commands<'_, '_>) {
    commands.insert_resource(MoveTimer::new(SPEED_INITIAL_MS));
}

/// Update (`GameSet::Tick`): advances the move timer by one frame delta.
#[expect(
    clippy::needless_pass_by_value,
    reason = "Bevy system parameters must be owned; Res<T> is not Clone/Copy"
)]
pub(crate) fn tick_move_timer(mut timer: ResMut<'_, MoveTimer>, time: Res<'_, Time>) {
    timer.0.tick(time.delta());
}

/// Update (`GameSet::Move`): cascades segment positions and advances the head.
#[expect(
    clippy::needless_pass_by_value,
    reason = "Bevy system parameters must be owned; Res<T> is not Clone/Copy"
)]
pub(crate) fn move_snake(
    timer: Res<'_, MoveTimer>,
    mut pending: ResMut<'_, PendingDirection>,
    mut tail_pos: ResMut<'_, TailPosition>,
    body: Res<'_, SnakeBody>,
    mut positions: Query<'_, '_, &mut GridPos>,
    mut head_dir: Query<'_, '_, &mut Direction, With<SnakeHead>>,
) {
    if !timer.0.just_finished() {
        return;
    }

    // Apply buffered direction; reversal guard is checked by the input system before buffering.
    if let Some(dir) = pending.0.take() {
        if let Ok(mut d) = head_dir.single_mut() {
            *d = dir;
        }
    }

    let dir = head_dir.single().copied().unwrap_or(Direction::Right);

    // Save old tail position before the cascade so grow_snake can append without a gap.
    if let Some(&tail_entity) = body.0.first() {
        tail_pos.0 = positions.get(tail_entity).ok().copied();
    }

    // Cascade positions tail-to-head: each entity takes the position of the next.
    let len = body.0.len();
    for i in 0..len.saturating_sub(1) {
        let next_pos = body
            .0
            .get(i + 1)
            .and_then(|&e| positions.get(e).ok().copied());
        if let (Some(next_pos), Some(&cur_entity)) = (next_pos, body.0.get(i)) {
            if let Ok(mut cur) = positions.get_mut(cur_entity) {
                *cur = next_pos;
            }
        }
    }

    // Advance head.
    if let Some(&head_entity) = body.0.last() {
        if let Ok(mut head_pos) = positions.get_mut(head_entity) {
            match dir {
                Direction::Up => head_pos.y -= 1,
                Direction::Down => head_pos.y += 1,
                Direction::Left => head_pos.x -= 1,
                Direction::Right => head_pos.x += 1,
            }
        }
    }
}
