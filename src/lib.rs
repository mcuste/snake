//! Terminal snake game built with Bevy ECS and ratatui.

/// ECS components: [`GridPos`], [`Direction`], [`SnakeHead`], [`SnakeSegment`], [`Food`].
///
/// [`GridPos`]: components::GridPos
/// [`Direction`]: components::Direction
/// [`SnakeHead`]: components::SnakeHead
/// [`SnakeSegment`]: components::SnakeSegment
/// [`Food`]: components::Food
pub(crate) mod components;

/// Compile-time game configuration constants (grid dimensions, speed tuning).
pub(crate) mod config;

/// ECS resources and message types shared across systems.
pub(crate) mod resources;

/// [`GameState`] enum driving state-gated system scheduling.
///
/// [`GameState`]: state::GameState
pub(crate) mod state;

/// [`GamePlugin`] that wires all systems, resources, and state into the Bevy [`App`].
///
/// [`GamePlugin`]: plugin::GamePlugin
/// [`App`]: bevy::prelude::App
pub mod plugin;

/// All game systems, grouped by concern into submodules.
pub(crate) mod systems;
