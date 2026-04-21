use std::time::Duration;

use bevy::prelude::*;
use bevy::time::{Timer, TimerMode};

use crate::components::{Direction, GridPos};

// Ordered tail-first: [furthest_segment, ..., segment_adjacent_to_head, head].
#[derive(Resource, Default)]
pub(crate) struct SnakeBody(pub(crate) Vec<Entity>);

#[derive(Resource, Default)]
pub(crate) struct PendingDirection(pub(crate) Option<Direction>);

#[derive(Resource)]
pub(crate) struct MoveTimer(pub(crate) Timer);

impl MoveTimer {
    pub(crate) fn new(ms: u64) -> Self {
        Self(Timer::new(Duration::from_millis(ms), TimerMode::Repeating))
    }
}

#[derive(Resource, Default)]
pub(crate) struct Score(pub(crate) u32);

#[derive(Resource)]
pub(crate) struct TerminalSize {
    pub(crate) cols: u16,
    pub(crate) rows: u16,
}

impl Default for TerminalSize {
    fn default() -> Self {
        Self { cols: 80, rows: 24 }
    }
}

// Snapshot of the tail's GridPos taken before the cascade in move_snake;
// consumed by grow_snake to place the new segment without a gap.
#[derive(Resource, Default)]
pub(crate) struct TailPosition(pub(crate) Option<GridPos>);

#[derive(Message, Default)]
pub(crate) struct GameOverEvent;

#[derive(Message, Default)]
pub(crate) struct FoodEatenEvent;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_timer_new_sets_duration_millis() {
        let sut = MoveTimer::new(150);
        assert_eq!(sut.0.duration().as_millis(), 150);
    }

    #[test]
    fn terminal_size_default_is_80x24() {
        let sut = TerminalSize::default();
        assert_eq!((sut.cols, sut.rows), (80, 24));
    }
}
