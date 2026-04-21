use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct GridPos {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub(crate) fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Component, Default)]
pub(crate) struct SnakeHead;

#[derive(Component, Clone, Debug)]
pub(crate) struct SnakeSegment {
    /// Spawn index; 0 = segment adjacent to head, increasing toward tail.
    #[expect(
        dead_code,
        reason = "metadata field; not needed for position cascade but kept for tracing/debug"
    )]
    pub(crate) order: usize,
}

#[derive(Component, Default)]
pub(crate) struct Food;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opposite_up_is_down() {
        assert_eq!(Direction::Up.opposite(), Direction::Down);
    }

    #[test]
    fn opposite_down_is_up() {
        assert_eq!(Direction::Down.opposite(), Direction::Up);
    }

    #[test]
    fn opposite_left_is_right() {
        assert_eq!(Direction::Left.opposite(), Direction::Right);
    }

    #[test]
    fn opposite_right_is_left() {
        assert_eq!(Direction::Right.opposite(), Direction::Left);
    }
}
