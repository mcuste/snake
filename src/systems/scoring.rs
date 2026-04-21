//! Scoring and speed-recalculation system.

use bevy::prelude::*;

use crate::{
    config::{SPEED_DECREMENT_PER_FOOD, SPEED_INITIAL_MS, SPEED_MIN_MS},
    resources::{FoodEatenEvent, MoveTimer, Score},
};

/// Update (`GameSet::Score`): increments [`Score`] and recalculates [`MoveTimer`] speed on each [`FoodEatenEvent`].
pub(crate) fn update_score(
    mut score: ResMut<'_, Score>,
    mut timer: ResMut<'_, MoveTimer>,
    mut events: MessageReader<'_, '_, FoodEatenEvent>,
) {
    for _ in events.read() {
        score.0 += 1;
        let new_ms = SPEED_MIN_MS
            .max(SPEED_INITIAL_MS.saturating_sub(u64::from(score.0) * SPEED_DECREMENT_PER_FOOD));
        *timer = MoveTimer::new(new_ms);
    }
}
