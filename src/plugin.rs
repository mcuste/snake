//! [`GamePlugin`] and [`GameSet`] — system wiring for the snake game.

use bevy::prelude::*;
use bevy_ratatui::crossterm::terminal;
use bevy_ratatui::event::InputSet;

use crate::{
    resources::{
        FoodEatenEvent, GameOverEvent, PendingDirection, Score, SnakeBody, TailPosition,
        TerminalSize,
    },
    state::GameState,
    systems::{
        collision::{check_self_collision, check_wall_collision, handle_game_over},
        food::{check_food_eaten, grow_snake, spawn_food, spawn_new_food},
        input::{handle_resize, read_input},
        lifecycle::{handle_quit, handle_restart},
        movement::{init_timer, move_snake, spawn_initial_snake, tick_move_timer},
        render::{game_over_render, render},
        scoring::update_score,
    },
};

/// System sets that impose ordering within the `Update` schedule.
///
/// The chain is: `Tick → Move → Collision → Food → Score → Render`.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum GameSet {
    /// Timer tick — runs every frame regardless of game state.
    Tick,
    /// Snake movement — gated to [`GameState::Playing`].
    Move,
    /// Wall and self-collision checks — gated to [`GameState::Playing`].
    Collision,
    /// Food consumption and snake growth — gated to [`GameState::Playing`].
    Food,
    /// Scoring and speed recalculation — gated to [`GameState::Playing`].
    Score,
    /// Terminal rendering — split by game state.
    Render,
}

/// Bevy plugin that registers all game systems, resources, messages, and states.
#[derive(Debug, Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let (cols, rows) = terminal::size().unwrap_or((80, 24));

        app
            // ── Resources ────────────────────────────────────────────────
            .insert_resource(SnakeBody::default())
            .insert_resource(PendingDirection::default())
            .insert_resource(Score::default())
            .insert_resource(TailPosition::default())
            .insert_resource(TerminalSize { cols, rows })
            // ── Messages ─────────────────────────────────────────────────
            .add_message::<GameOverEvent>()
            .add_message::<FoodEatenEvent>()
            // ── State ─────────────────────────────────────────────────────
            .init_state::<GameState>()
            // ── System set ordering ───────────────────────────────────────
            .configure_sets(
                Update,
                (
                    GameSet::Tick,
                    GameSet::Move,
                    GameSet::Collision,
                    GameSet::Food,
                    GameSet::Score,
                    GameSet::Render,
                )
                    .chain(),
            )
            // ── Startup ───────────────────────────────────────────────────
            .add_systems(
                Startup,
                (spawn_initial_snake, spawn_food, init_timer).chain(),
            )
            // ── PreUpdate ─────────────────────────────────────────────────
            .add_systems(PreUpdate, (read_input, handle_resize).after(InputSet::Post))
            // ── Update: timer tick (always) ───────────────────────────────
            .add_systems(Update, tick_move_timer.in_set(GameSet::Tick))
            // ── Update: playing-only systems ─────────────────────────────
            .add_systems(
                Update,
                move_snake
                    .in_set(GameSet::Move)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (check_wall_collision, check_self_collision)
                    .in_set(GameSet::Collision)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (check_food_eaten, grow_snake)
                    .in_set(GameSet::Food)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (update_score, spawn_new_food)
                    .in_set(GameSet::Score)
                    .run_if(in_state(GameState::Playing)),
            )
            // ── Update: render (state-split) ──────────────────────────────
            .add_systems(
                Update,
                render
                    .in_set(GameSet::Render)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                game_over_render
                    .in_set(GameSet::Render)
                    .run_if(in_state(GameState::GameOver)),
            )
            // ── Update: always-active ─────────────────────────────────────
            .add_systems(Update, (handle_game_over, handle_quit))
            // ── Update: game-over only ────────────────────────────────────
            .add_systems(Update, handle_restart.run_if(in_state(GameState::GameOver)));
    }
}
