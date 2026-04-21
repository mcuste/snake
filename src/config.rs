//! Compile-time game configuration constants.

/// Number of columns in the playfield grid.
pub(crate) const GRID_WIDTH: i32 = 40;

/// Number of rows in the playfield grid.
pub(crate) const GRID_HEIGHT: i32 = 20;

/// Horizontal grid coordinate of the spawn centre (column index): `GRID_WIDTH / 2 = 30`.
pub(crate) const GRID_CENTRE_X: i32 = GRID_WIDTH / 2;

/// Vertical grid coordinate of the spawn centre (row index): `GRID_HEIGHT / 2`.
pub(crate) const GRID_CENTRE_Y: i32 = GRID_HEIGHT / 2;

/// Initial move interval in milliseconds.
pub(crate) const SPEED_INITIAL_MS: u64 = 150;

/// Fastest allowed move interval in milliseconds (speed floor).
pub(crate) const SPEED_MIN_MS: u64 = 70;

/// Milliseconds to shave off the move interval for each food eaten.
pub(crate) const SPEED_DECREMENT_PER_FOOD: u64 = 4;

/// Grid width as [`u16`] for terminal coordinate arithmetic (avoids `as` casts).
pub(crate) const GRID_WIDTH_U16: u16 = GRID_WIDTH as u16;

/// Grid height as [`u16`] for terminal coordinate arithmetic (avoids `as` casts).
pub(crate) const GRID_HEIGHT_U16: u16 = GRID_HEIGHT as u16;
