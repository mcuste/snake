# PLAN-RFC-001: Terminal Snake — Implementation Plan

**RFC**: [RFC-001](../rfc/RFC-001-terminal-snake-architecture.md)
**Created**: 2026-04-19
**Status**: Draft

---

## File Map

All files are new (greenfield). Every milestone references this map.

| File | Milestone(s) | Purpose |
|------|-------------|---------|
| `Cargo.toml` | M01 | Dependencies: bevy, bevy_ratatui, ratatui, rand |
| `src/main.rs` | M01, M08 | Entry point: panic hook + Bevy app setup |
| `src/lib.rs` | M01–M07 | Module declarations, re-exports; updated each milestone |
| `src/config.rs` | M02 | 5 named constants for grid size and speed curve |
| `src/state.rs` | M03 | `GameState` enum |
| `src/components.rs` | M04 | `GridPos`, `Direction`, `SnakeHead`, `SnakeSegment`, `Food` |
| `src/resources.rs` | M05 | `SnakeBody`, `PendingDirection`, `MoveTimer`, `Score`, `TerminalSize`, `TailPosition`, `GameOverEvent`, `FoodEatenEvent` |
| `src/plugin.rs` | M07 | `GamePlugin`, `GameSet` enum, system wiring |
| `src/systems/mod.rs` | M06 | System submodule declarations |
| `src/systems/movement.rs` | M06 stub, M09–M11 | `spawn_initial_snake`, `init_timer`, `tick_move_timer`, `move_snake` |
| `src/systems/render.rs` | M06 stub, M12, M16, M21, M25, M28, M29 | `render` (Playing), `game_over_render` (GameOver) |
| `src/systems/input.rs` | M06 stub, M13, M23, M27 | `read_input`, `handle_resize` |
| `src/systems/collision.rs` | M06 stub, M14, M22 | `check_wall_collision`, `check_self_collision`, `handle_game_over` |
| `src/systems/food.rs` | M06 stub, M15, M17–M19 | `spawn_food`, `check_food_eaten`, `grow_snake`, `spawn_new_food` |
| `src/systems/scoring.rs` | M06 stub, M20 | `update_score` |
| `src/systems/lifecycle.rs` | M06 stub, M24, M26 | `handle_quit`, `handle_restart` |

---

## Phase 1 — Core

Walking skeleton: snake renders, moves, accepts directional input from all three control schemes, dies on wall collision. No food, no scoring, no restart, no self-collision.

---

### M01 — Project scaffold

**Status**: Pending
**Files**: `Cargo.toml`, `src/main.rs`, `src/lib.rs`

Create `Cargo.toml`:

```toml
[package]
name = "snake"
edition = "2021"

[dependencies]
bevy = { version = "0.18", default-features = false, features = ["bevy_state", "bevy_time"] }
bevy_ratatui = "0.11"
ratatui = "0.30"
rand = "0.9"
```

Verify bevy and bevy_ratatui are compatible per ADR-003. Disable `bevy/default-features` to exclude `bevy_render`, `bevy_winit`, `bevy_log`. Create `src/main.rs` with `fn main() {}` (empty body). Create `src/lib.rs` as empty file.

**AC**:

1. `cargo check` exits 0 with no errors.
2. `cargo tree` shows no `bevy_render` or `bevy_winit` in the dependency graph.

---

### M02 — config.rs: named constants

**Status**: Pending
**Files**: `src/config.rs`, `src/lib.rs`

Create `src/config.rs`:

```rust
pub(crate) const GRID_WIDTH: i32 = 60;
pub(crate) const GRID_HEIGHT: i32 = 60;
pub(crate) const SPEED_INITIAL_MS: u64 = 150;
pub(crate) const SPEED_MIN_MS: u64 = 70;
pub(crate) const SPEED_DECREMENT_PER_FOOD: u64 = 4;
```

Add to `src/lib.rs`:

```rust
pub(crate) mod config;
pub(crate) use config::*;
```

**AC**:

1. `cargo check` passes.
2. All 5 constants accessible from any `crate::` path without qualification.

---

### M03 — state.rs: GameState enum

**Status**: Pending
**Files**: `src/state.rs`, `src/lib.rs`

Create `src/state.rs`:

```rust
use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(crate) enum GameState {
    #[default]
    Playing,
    GameOver,
}
```

Add to `src/lib.rs`:

```rust
pub(crate) mod state;
```

**AC**:

1. `cargo check` passes.
2. `GameState::default()` is `GameState::Playing`.

---

### M04 — components.rs: all 5 component types

**Status**: Pending
**Files**: `src/components.rs`, `src/lib.rs`

Create `src/components.rs` with these types, all deriving `Component`:

- `GridPos { x: i32, y: i32 }` — also derive `Copy, Clone, Debug, PartialEq, Eq`
- `Direction` enum — variants `Up, Down, Left, Right` — derive `Component, Copy, Clone, Debug, PartialEq, Eq` — implement `fn opposite(&self) -> Direction` (Up↔Down, Left↔Right)
- `SnakeHead` — unit struct marker, derive `Component, Default`
- `SnakeSegment { order: usize }` — derive `Component, Clone, Debug`
- `Food` — unit struct marker, derive `Component, Default`

Add to `src/lib.rs`:

```rust
pub(crate) mod components;
pub(crate) use components::*;
```

**AC**:

1. `cargo check` passes.
2. `Direction::Up.opposite()` returns `Direction::Down`; `Direction::Left.opposite()` returns `Direction::Right`.

---

### M05 — resources.rs: all resources and events

**Status**: Pending
**Files**: `src/resources.rs`, `src/lib.rs`

Create `src/resources.rs` with:

- `SnakeBody(pub(crate) Vec<Entity>)` — derive `Resource, Default`
- `PendingDirection(pub(crate) Option<Direction>)` — derive `Resource, Default`
- `MoveTimer(pub(crate) Timer)` — derive `Resource` — implement `fn new(ms: u64) -> Self` creating a `Timer::new(Duration::from_millis(ms), TimerMode::Repeating)`
- `Score(pub(crate) u32)` — derive `Resource, Default`
- `TerminalSize { pub(crate) cols: u16, pub(crate) rows: u16 }` — derive `Resource` — `Default` impl returns `(80, 24)`
- `TailPosition(pub(crate) Option<GridPos>)` — derive `Resource, Default` — saves old tail position before each move for `grow_snake`
- `GameOverEvent` — derive `Event, Default`
- `FoodEatenEvent` — derive `Event, Default`

Add to `src/lib.rs`:

```rust
pub(crate) mod resources;
pub(crate) use resources::*;
```

**AC**:

1. `cargo check` passes.
2. `MoveTimer::new(150)` creates a 150ms repeating timer.

---

### M06 — systems/mod.rs + all 7 system file stubs

**Status**: Pending
**Files**: `src/systems/mod.rs`, `src/systems/movement.rs`, `src/systems/render.rs`, `src/systems/input.rs`, `src/systems/collision.rs`, `src/systems/food.rs`, `src/systems/scoring.rs`, `src/systems/lifecycle.rs`, `src/lib.rs`

Create `src/systems/mod.rs`:

```rust
pub(crate) mod movement;
pub(crate) mod render;
pub(crate) mod input;
pub(crate) mod collision;
pub(crate) mod food;
pub(crate) mod scoring;
pub(crate) mod lifecycle;
```

Create each system file with stub `pub(crate) fn` declarations (empty bodies). All stubs must compile without unused-import warnings — use `#[allow(unused)]` at module level if needed. Stubs per file:

- `movement.rs`: `spawn_initial_snake()`, `init_timer()`, `tick_move_timer()`, `move_snake()`
- `render.rs`: `render()`, `game_over_render()`
- `input.rs`: `read_input()`, `handle_resize()`
- `collision.rs`: `check_wall_collision()`, `check_self_collision()`, `handle_game_over()`
- `food.rs`: `spawn_food()`, `check_food_eaten()`, `grow_snake()`, `spawn_new_food()`
- `scoring.rs`: `update_score()`
- `lifecycle.rs`: `handle_quit()`, `handle_restart()`

All stub functions take zero Bevy system params — parameters are added during implementation without changing plugin.rs registration.

Add to `src/lib.rs`:

```rust
pub(crate) mod systems;
```

**AC**:

1. `cargo check` passes.
2. All 17 stub functions exist and are callable by name from `crate::systems::{module}::{fn}`.

---

### M07 — plugin.rs: GamePlugin wiring

**Status**: Pending
**Files**: `src/plugin.rs`, `src/lib.rs`

Create `src/plugin.rs` with:

- `GameSet` enum — variants `Tick, Move, Collision, Food, Score, Render` — derive `SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash`
- `GamePlugin` struct implementing `Plugin`

`build(&self, app)` does:

1. Initialize `TerminalSize` from `crossterm::terminal::size().unwrap_or((80, 24))`
2. Insert resources: `SnakeBody`, `PendingDirection`, `Score`, `TailPosition`, `TerminalSize`
3. Add events: `GameOverEvent`, `FoodEatenEvent`
4. Init state: `GameState`
5. Chain `GameSet`: `Tick → Move → Collision → Food → Score → Render`
6. Startup systems: `(spawn_initial_snake, spawn_food, init_timer).chain()`
7. PreUpdate systems: `(read_input, handle_resize).after(bevy_ratatui::event::InputSet::Post)`
8. Update systems per `GameSet` with `run_if(in_state(GameState::Playing))` on Move through Score
9. Render set: `render` with `in_state(Playing)`, `game_over_render` with `in_state(GameOver)`
10. Always-active: `handle_game_over`, `handle_quit`
11. GameOver-only: `handle_restart.run_if(in_state(GameState::GameOver))`

Add to `src/lib.rs`:

```rust
pub(crate) mod plugin;
```

**AC**:

1. `cargo check` passes with no errors.
2. `GameSet` variants chain in declared order: Tick → Move → Collision → Food → Score → Render.

---

### M08 — main.rs: full app wiring + panic hook

**Status**: Pending
**Files**: `src/main.rs`

Replace `fn main() {}` with full app setup:

1. Install `std::panic::set_hook` that calls `crossterm::terminal::disable_raw_mode()`, `crossterm::execute!(stdout(), LeaveAlternateScreen, Show)`, and prints the panic info to stderr.
2. Build and run: `App::new().add_plugins(MinimalPlugins).add_plugins(RatatuiPlugins::default()).add_plugins(GamePlugin).run()`

**AC**:

1. `cargo run` starts without panic and without crashing immediately.
2. Terminal is left in a functional state after `Ctrl-C` (raw mode restored, cursor visible, input echoed).

---

### M09 — movement.rs: spawn_initial_snake

**Status**: Pending
**File**: `src/systems/movement.rs`

Replace `spawn_initial_snake()` stub. Startup system:

1. Spawn `SnakeHead` entity at `(GRID_WIDTH / 2, GRID_HEIGHT / 2)` with `Direction::Right`
2. Spawn 2 `SnakeSegment` entities at `(cx - 1, cy)` order 0 and `(cx - 2, cy)` order 1
3. Populate `SnakeBody` resource ordered tail-first: `[seg2, seg1, head]`

System params: `Commands`, `ResMut<SnakeBody>`.

**AC**:

1. After startup: Bevy world contains exactly 1 `SnakeHead` entity and 2 `SnakeSegment` entities.
2. `SnakeBody.0.len()` is 3.
3. Head `GridPos` is `(30, 30)` (GRID_WIDTH/2, GRID_HEIGHT/2); segments at `(29, 30)` and `(28, 30)`.

---

### M10 — movement.rs: init_timer + tick_move_timer

**Status**: Pending
**File**: `src/systems/movement.rs`

Replace `init_timer()` stub — Startup system:

- `commands.insert_resource(MoveTimer::new(SPEED_INITIAL_MS))`

Replace `tick_move_timer()` stub — Update system in `GameSet::Tick`:

- `timer.0.tick(time.delta())`
- System params: `ResMut<MoveTimer>`, `Res<Time>`

**AC**:

1. `MoveTimer` resource exists after startup with 150ms repeating interval.
2. `timer.0.just_finished()` becomes true once per ~150ms during gameplay.

---

### M11 — movement.rs: move_snake

**Status**: Pending
**File**: `src/systems/movement.rs`

Replace `move_snake()` stub. Runs in `GameSet::Move` with `run_if(in_state(GameState::Playing))`.

System params: `Res<MoveTimer>`, `ResMut<PendingDirection>`, `ResMut<TailPosition>`, `Res<SnakeBody>`, `Query<&mut GridPos>`, `Query<&mut Direction, With<SnakeHead>>`.

Logic:

1. Early return if `!timer.0.just_finished()`.
2. If `pending_dir.0` is `Some(dir)`: apply `dir` to head's `Direction` component, then set `pending_dir.0 = None`.
3. Read head's current `Direction` component.
4. Save old tail position (first entity in `body.0`) to `TailPosition` resource.
5. Cascade positions tail-to-head: each entity in `body.0` gets the `GridPos` of the next entity.
6. Advance head `GridPos` by 1 cell in current `Direction` (Up: y-1, Down: y+1, Left: x-1, Right: x+1).

**AC**:

1. Snake head `GridPos` advances 1 cell in `Direction::Right` every ~150ms.
2. Body segments follow head correctly (no gaps, no overlaps after each tick).
3. `TailPosition` resource holds the old tail `GridPos` from before each cascade.

---

### M12 — render.rs: Playing state render

**Status**: Pending
**File**: `src/systems/render.rs`

Replace `render()` stub. Runs in `GameSet::Render` with `run_if(in_state(GameState::Playing))`.

Define locally in `render.rs`:

- `Cell` enum: `Empty, Head, Body, Food`
- `GameBoard([[Cell; GRID_WIDTH as usize]; GRID_HEIGHT as usize])` struct implementing `ratatui::widgets::Widget`
- Widget iterates buffer row-by-row, writes one char per cell: `H` (Head), `O` (Body), `*` (Food — renders as space until M16), space (Empty) via `buf.set_string(area.x + col, area.y + row, ch, Style::default())`

`render` system:

1. Build `GameBoard` buffer, populate from ECS queries: `Query<&GridPos, With<SnakeHead>>` → `Cell::Head`, `Query<&GridPos, With<SnakeSegment>>` → `Cell::Body`
2. Compute centering: `board_w = GRID_WIDTH as u16 + 2`, `board_h = GRID_HEIGHT as u16 + 2`, `offset_x = terminal_size.cols.saturating_sub(board_w) / 2`, `offset_y = terminal_size.rows.saturating_sub(board_h) / 2`
3. `ctx.draw()`: render `Block::default().borders(Borders::ALL)` at outer `Rect`, then render `GameBoard` at `block.inner(outer)`

System params: `ResMut<RatatuiContext>`, head query, segments query, `Res<TerminalSize>`.

Replace `game_over_render()` stub with minimal version: `ctx.draw(|_frame| {})` (proper implementation in M25).

**AC**:

1. `cargo run`: a bordered game board appears centered in the terminal.
2. Snake head renders as `H`; body segments render as `O`.
3. Board remains centered when terminal is at least 32 cols x 22 rows.

---

### M13 — input.rs: read_input (all 3 control schemes)

**Status**: Pending
**File**: `src/systems/input.rs`

Replace `read_input()` stub. Runs in `PreUpdate` after `InputSet::Post`.

System params: `EventReader<KeyMessage>` (verify exact type against bevy_ratatui 0.11 API — may be `KeyEvent` or `KeyboardMessage`), `ResMut<PendingDirection>`.

Logic:

1. Iterate events, skip if `kind != KeyEventKind::Press`
2. Map: `Up | Char('w') | Char('W') | Char('k')` → `Direction::Up`; `Down | Char('s') | Char('S') | Char('j')` → `Direction::Down`; `Left | Char('a') | Char('A') | Char('h')` → `Direction::Left`; `Right | Char('d') | Char('D') | Char('l')` → `Direction::Right`
3. Overwrite `pending.0 = Some(candidate)` — no reversal guard in Phase 1

Keep `handle_resize()` as stub (implemented in M27).

**AC**:

1. Pressing `Up`, `w`, `W`, or `k` sets `PendingDirection` to `Some(Direction::Up)`.
2. All 12 directional keys (4 groups x 3 aliases each) correctly map to their direction.
3. Non-directional keys are ignored (no panic, no effect).

---

### M14 — collision.rs: check_wall_collision + handle_game_over

**Status**: Pending
**File**: `src/systems/collision.rs`

Replace `check_wall_collision()` stub. Runs in `GameSet::Collision` with `run_if(in_state(GameState::Playing))`:

- Query `&GridPos` with `SnakeHead`. If `x < 0 || x >= GRID_WIDTH || y < 0 || y >= GRID_HEIGHT`: emit `GameOverEvent`.
- System params: `Query<&GridPos, With<SnakeHead>>`, `EventWriter<GameOverEvent>`.

Replace `handle_game_over()` stub. Always-active:

- Read `EventReader<GameOverEvent>`. If any event: `next_state.set(GameState::GameOver)`.
- System params: `EventReader<GameOverEvent>`, `ResMut<NextState<GameState>>`.

Keep `check_self_collision()` as stub (implemented in M22).

**AC**:

1. Snake hitting left/right wall (x < 0 or x >= 60) transitions to `GameState::GameOver`.
2. Snake hitting top/bottom wall (y < 0 or y >= 60) transitions to `GameState::GameOver`.
3. In `GameOver` state: `move_snake` does not run (game freezes; board visible but static).

---

## Phase 2 — Details

Food spawning, eating, growth, scoring, self-collision, reversal guard, game-over overlay, restart, quit, resize handling.

---

### M15 — food.rs: spawn_food

**Status**: Pending
**File**: `src/systems/food.rs`

Replace `spawn_food()` stub. Startup system (runs after `spawn_initial_snake` in chain):

1. Collect all `GridPos` entities into `HashSet<(i32, i32)>`
2. Loop: pick random `(x, y)` in `[0, GRID_WIDTH) x [0, GRID_HEIGHT)` via `rand::thread_rng().gen_range()`
3. If not in occupied set: `commands.spawn((Food, GridPos { x, y }))` and break

System params: `Commands`, `Query<&GridPos>`.

Keep `check_food_eaten()`, `grow_snake()`, `spawn_new_food()` as stubs.

**AC**:

1. After startup: exactly 1 `Food` entity exists.
2. Food `GridPos` is within `[0, 60) x [0, 60)`.
3. Food does not spawn on any snake segment position.

---

### M16 — render.rs: food rendering

**Status**: Pending
**File**: `src/systems/render.rs`

Update `render` system:

1. Add `Query<&GridPos, With<Food>>` parameter
2. After segments loop, populate food cells: `grid[y][x] = Cell::Food`
3. Change `Cell::Food` arm in `Widget::render` from `" "` to `"*"`

**AC**:

1. Food renders as `*` on the board at its `GridPos`.
2. Head cell overwrites food cell if they coincide (head renders on top).

---

### M17 — food.rs: check_food_eaten

**Status**: Pending
**File**: `src/systems/food.rs`

Replace `check_food_eaten()` stub. Runs in `GameSet::Food` with `run_if(in_state(GameState::Playing))`:

1. Query head `GridPos` and food `(Entity, &GridPos)`.
2. If `head_pos == food_pos`: `commands.entity(food_entity).despawn()`, emit `FoodEatenEvent`.

System params: `Query<&GridPos, With<SnakeHead>>`, `Query<(Entity, &GridPos), With<Food>>`, `Commands`, `EventWriter<FoodEatenEvent>`.

**AC**:

1. Head entering food cell: `FoodEatenEvent` emitted and food entity despawned.
2. Head not on food: no event, no despawn.
3. Food entity count goes from 1 to 0 on the frame the head enters the food cell.

---

### M18 — food.rs: grow_snake

**Status**: Pending
**File**: `src/systems/food.rs`

Replace `grow_snake()` stub. Runs in `GameSet::Food` with `run_if(in_state(GameState::Playing))`:

1. Read `EventReader<FoodEatenEvent>`. For each event:
2. Read `TailPosition.0` (saved by `move_snake` in the same tick).
3. Spawn `SnakeSegment { order: body.0.len() }` at `TailPosition` GridPos.
4. Insert new entity at index 0 of `SnakeBody.0` (tail end).

System params: `Commands`, `ResMut<SnakeBody>`, `Res<TailPosition>`, `EventReader<FoodEatenEvent>`.

**AC**:

1. Eating food: `SnakeBody.0.len()` increases by 1.
2. New segment spawns at the `TailPosition` saved by `move_snake`.
3. After 5 foods: snake is 8 entities long (3 initial + 5 grown).

---

### M19 — food.rs: spawn_new_food

**Status**: Pending
**File**: `src/systems/food.rs`

Replace `spawn_new_food()` stub. Runs in `GameSet::Score` with `run_if(in_state(GameState::Playing))`:

1. Read `EventReader<FoodEatenEvent>`. For each event:
2. Collect all `GridPos` entities into `HashSet<(i32, i32)>`.
3. Loop: pick random `(x, y)` in `[0, GRID_WIDTH) x [0, GRID_HEIGHT)`.
4. If not in occupied set: spawn `(Food, GridPos { x, y })` and break.

System params: `Commands`, `Query<&GridPos>`, `EventReader<FoodEatenEvent>`.

**AC**:

1. After eating food: exactly 1 new `Food` entity spawns by end of the same frame.
2. New food `GridPos` is not occupied by any snake entity.
3. Total `Food` entity count stays at 1 throughout play.

---

### M20 — scoring.rs: update_score + speed recalculation

**Status**: Pending
**File**: `src/systems/scoring.rs`

Replace `update_score()` stub. Runs in `GameSet::Score` with `run_if(in_state(GameState::Playing))`:

1. Read `EventReader<FoodEatenEvent>`. For each event:
2. Increment `score.0 += 1`.
3. Compute `new_ms = SPEED_MIN_MS.max(SPEED_INITIAL_MS.saturating_sub(score.0 as u64 * SPEED_DECREMENT_PER_FOOD))`.
4. Rebuild timer: `*timer = MoveTimer::new(new_ms)`.

System params: `ResMut<Score>`, `ResMut<MoveTimer>`, `EventReader<FoodEatenEvent>`.

**AC**:

1. Score increments by 1 per food eaten; `Score(0)` → `Score(1)` on first eat.
2. At score 20: `MoveTimer` interval is 70ms (floor). At score 10: 110ms.
3. Timer is rebuilt so new interval takes effect on the next tick.

---

### M21 — render.rs: score in block title

**Status**: Pending
**File**: `src/systems/render.rs`

Update `render` system:

1. Add `Res<Score>` to system params.
2. Change block construction: `Block::default().borders(Borders::ALL).title(format!(" Score: {} ", score.0))`.

**AC**:

1. Score 0: block title shows ` Score: 0 `.
2. After eating 3 foods: block title shows ` Score: 3 `.
3. Title updates on the frame after the food is eaten.

---

### M22 — collision.rs: check_self_collision

**Status**: Pending
**File**: `src/systems/collision.rs`

Replace `check_self_collision()` stub. Runs in `GameSet::Collision` with `run_if(in_state(GameState::Playing))`:

1. Query head `GridPos`. Query all segment `GridPos`.
2. If head `GridPos` matches any segment `GridPos`: emit `GameOverEvent` and return.

System params: `Query<&GridPos, With<SnakeHead>>`, `Query<&GridPos, With<SnakeSegment>>`, `EventWriter<GameOverEvent>`.

**AC**:

1. Head entering any segment's `GridPos` → `GameOverEvent` → `GameState::GameOver`.
2. Snake moving in open space → no `GameOverEvent`.
3. Triggers correctly regardless of snake length.

---

### M23 — input.rs: direction reversal guard

**Status**: Pending
**File**: `src/systems/input.rs`

Update `read_input` system:

1. Add `Query<&Direction, With<SnakeHead>>` parameter.
2. Before writing to `PendingDirection`: if `candidate == head_dir.opposite()`, `continue` (skip this keypress).

**AC**:

1. Snake moving `Right`: pressing `Left`/`h`/`a` → `PendingDirection` unchanged, snake continues right.
2. Snake moving `Right`: pressing `Up`/`k` → `PendingDirection` = `Some(Up)`, snake turns up.
3. All 4 opposite pairs guarded: Up↔Down, Left↔Right.

---

### M24 — lifecycle.rs: handle_quit

**Status**: Pending
**File**: `src/systems/lifecycle.rs`

Replace `handle_quit()` stub. Always-active system (no state gate):

1. Read `EventReader<KeyMessage>` (same event type as M13).
2. Filter `KeyEventKind::Press`.
3. If `KeyCode::Char('q') | KeyCode::Esc`: emit `AppExit::Success`.

System params: `EventReader<KeyMessage>`, `EventWriter<AppExit>`.

Keep `handle_restart()` as stub (implemented in M26).

**AC**:

1. Pressing `q` during play exits game; terminal is in normal state (cursor visible, input echoed).
2. Pressing `Esc` during play exits game; terminal is in normal state.
3. Pressing `q` in `GameOver` state exits game.

---

### M25 — render.rs: game_over_render overlay

**Status**: Pending
**File**: `src/systems/render.rs`

Replace stub `game_over_render()` with full overlay. Runs in `GameSet::Render` with `run_if(in_state(GameState::GameOver))`.

System params: `ResMut<RatatuiContext>`, `Res<Score>`, head query, segments query, food query, `Res<TerminalSize>`.

Logic:

1. Build stale `GameBoard` buffer from current ECS state (same population logic as `render`).
2. Compute centering (same as `render`).
3. `ctx.draw()`: render `Block` + `GameBoard` (stale board), then overlay:
   - Compute centered overlay `Rect` (~26 wide x 5 tall) within the board area.
   - Render `Clear` widget at overlay rect.
   - Render `Paragraph` with `"GAME OVER\nScore: {score}\n\nr = restart   q = quit"` centered.

**AC**:

1. In `GameState::GameOver`: stale board visible with "GAME OVER" overlay centered on it.
2. Overlay shows `Score: N` with the final score value.
3. Overlay shows `r = restart   q = quit` instruction line.

---

### M26 — lifecycle.rs: handle_restart

**Status**: Pending
**File**: `src/systems/lifecycle.rs`

Replace `handle_restart()` stub. Runs with `run_if(in_state(GameState::GameOver))`.

System params: `EventReader<KeyMessage>`, `Commands`, `ResMut<SnakeBody>`, `ResMut<PendingDirection>`, `ResMut<Score>`, `ResMut<MoveTimer>`, `ResMut<TailPosition>`, `ResMut<NextState<GameState>>`, `Query<&mut GridPos>`, `Query<&mut Direction, With<SnakeHead>>`, `Query<Entity, With<SnakeSegment>>`, `Query<Entity, With<Food>>`, `Query<&GridPos>`.

Logic on detecting `KeyCode::Char('r')` with `KeyEventKind::Press`:

1. Despawn all `SnakeSegment` entities.
2. Keep head entity (last in `body.0`). Clear `body.0`.
3. Reset head `GridPos` to `(GRID_WIDTH / 2, GRID_HEIGHT / 2)` and `Direction` to `Right`.
4. Spawn 2 initial `SnakeSegment` entities at `(cx-1, cy)` and `(cx-2, cy)`.
5. Rebuild `SnakeBody`: `[seg2, seg1, head_entity]`.
6. Reset: `score.0 = 0`, `pending.0 = None`, `tail_pos.0 = None`, `*timer = MoveTimer::new(SPEED_INITIAL_MS)`.
7. Despawn all `Food` entities. Spawn new food at random unoccupied cell (same logic as `spawn_food` in M15).
8. `next_state.set(GameState::Playing)`.

**AC**:

1. Pressing `r` in `GameOver`: game transitions to `Playing` on the next frame.
2. After restart: `Score` is 0, `MoveTimer` interval is 150ms, snake is at center with 3 entities moving right.
3. After restart: exactly 1 `Food` entity exists, not on a snake cell.
4. Pressing `r` in `Playing` state: no effect.

---

### M27 — input.rs: handle_resize

**Status**: Pending
**File**: `src/systems/input.rs`

Replace `handle_resize()` stub. Runs in `PreUpdate` after `InputSet::Post`:

1. Read `EventReader<ResizeEvent>` (verify exact type name in bevy_ratatui 0.11 API — may be `ResizeMessage`).
2. For each event: update `terminal_size.cols` and `terminal_size.rows`.

System params: `EventReader<ResizeEvent>`, `ResMut<TerminalSize>`.

**AC**:

1. Resizing terminal while game runs: board re-centers on the next rendered frame.
2. `TerminalSize` resource reflects the new dimensions after a resize event.

---

## Phase 3 — Polish

---

### M28 — render.rs: visual styling

**Status**: Pending
**File**: `src/systems/render.rs`

Update `GameBoard` `Widget::render` to use `ratatui::style::{Color, Style}`:

- `Cell::Head` → `"@"` with `Style::default().fg(Color::LightGreen)`
- `Cell::Body` → `"O"` with `Style::default().fg(Color::Green)`
- `Cell::Food` → `"*"` with `Style::default().fg(Color::LightRed)`
- `Cell::Empty` → `" "` with `Style::default()`

**AC**:

1. Snake head renders as `@` in bright green.
2. Snake body segments render as `O` in green.
3. Food renders as `*` in bright red.

---

### M29 — render.rs: game-over overlay styling

**Status**: Pending
**File**: `src/systems/render.rs`

Update `game_over_render` overlay `Paragraph` to use styled `Text`:

- "GAME OVER" line: `Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD)`
- Score line: `Style::default()` (terminal default)
- "r = restart   q = quit" line: `Style::default().fg(Color::DarkGray)`

**AC**:

1. "GAME OVER" text is bold red.
2. Score line is default terminal color.
3. Restart/quit hint is dark gray.

---

## RFC Goal Traceability

| RFC Goal | Milestones |
|----------|-----------|
| Rust dependency stack | M01 |
| Single-crate module structure | M01–M07 |
| ECS components, resources, events | M04, M05 |
| Bevy states | M03, M07 |
| All systems with ordering | M06–M14, M15–M27 |
| Input handling + direction buffering | M13, M23, M27 |
| Rendering pipeline | M12, M16, M21, M25, M28, M29 |
| Progressive speed mechanism | M20 |
| Config constants | M02 |
| Panic hook | M08 |

---

## Change Log

- 2026-04-19: Initial plan — 29 milestones across 3 phases
