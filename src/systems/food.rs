//! Food spawning and consumption systems.

use std::collections::HashSet;

use bevy::prelude::*;
use rand::Rng as _;

use crate::{
    components::{Food, GridPos, SnakeHead, SnakeSegment},
    config::{GRID_HEIGHT, GRID_WIDTH},
    resources::{FoodEatenEvent, SnakeBody, TailPosition},
};

/// Startup: spawns the initial food entity at a random unoccupied grid cell.
pub(crate) fn spawn_food(mut commands: Commands<'_, '_>, all_positions: Query<'_, '_, &GridPos>) {
    let occupied: HashSet<(i32, i32)> = all_positions.iter().map(|p| (p.x, p.y)).collect();
    let mut rng = rand::rng();
    loop {
        let x = rng.random_range(0..GRID_WIDTH);
        let y = rng.random_range(0..GRID_HEIGHT);
        if !occupied.contains(&(x, y)) {
            commands.spawn((Food, GridPos { x, y }));
            break;
        }
    }
}

/// Update (`GameSet::Food`): despawns food and emits [`FoodEatenEvent`] when the head enters the food cell.
pub(crate) fn check_food_eaten(
    head_query: Query<'_, '_, &GridPos, With<SnakeHead>>,
    food_query: Query<'_, '_, (Entity, &GridPos), With<Food>>,
    mut commands: Commands<'_, '_>,
    mut food_eaten: MessageWriter<'_, FoodEatenEvent>,
) {
    let Ok(head_pos) = head_query.single() else {
        return;
    };
    for (entity, food_pos) in &food_query {
        if head_pos == food_pos {
            commands.entity(entity).despawn();
            food_eaten.write_default();
            return;
        }
    }
}

/// Update (`GameSet::Food`): appends a new segment at the saved tail position on [`FoodEatenEvent`].
#[expect(
    clippy::needless_pass_by_value,
    reason = "Bevy system parameters must be owned; Res<T> is not Clone/Copy"
)]
pub(crate) fn grow_snake(
    mut commands: Commands<'_, '_>,
    mut body: ResMut<'_, SnakeBody>,
    tail_pos: Res<'_, TailPosition>,
    mut events: MessageReader<'_, '_, FoodEatenEvent>,
) {
    for _ in events.read() {
        if let Some(pos) = tail_pos.0 {
            let order = body.0.len();
            let new_seg = commands.spawn((SnakeSegment { order }, pos)).id();
            body.0.insert(0, new_seg);
        }
    }
}

/// Update (`GameSet::Score`): spawns replacement food at a random unoccupied cell after [`FoodEatenEvent`].
pub(crate) fn spawn_new_food(
    mut commands: Commands<'_, '_>,
    all_positions: Query<'_, '_, &GridPos>,
    mut events: MessageReader<'_, '_, FoodEatenEvent>,
) {
    for _ in events.read() {
        let occupied: HashSet<(i32, i32)> = all_positions.iter().map(|p| (p.x, p.y)).collect();
        let mut rng = rand::rng();
        loop {
            let x = rng.random_range(0..GRID_WIDTH);
            let y = rng.random_range(0..GRID_HEIGHT);
            if !occupied.contains(&(x, y)) {
                commands.spawn((Food, GridPos { x, y }));
                break;
            }
        }
    }
}
