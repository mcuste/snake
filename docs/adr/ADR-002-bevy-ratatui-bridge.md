# ADR-002: bevy_ratatui as Terminal Rendering Bridge

**Status**: Accepted
**Date**: 2026-04-19
**Deciders**: mcuste
**Affects**: `src/main.rs`, `src/systems/render.rs`, `src/systems/input.rs`, `src/plugin.rs`
**Implements RFC**: [RFC-001](../rfc/RFC-001-terminal-snake-architecture.md)

## Context

The game runs in a terminal using ratatui for widget rendering. Bevy's ECS needs a bridge to ratatui's terminal lifecycle — raw mode management, input event forwarding from crossterm to Bevy's event system, and the `RatatuiContext` resource wrapping `ratatui::Terminal` for frame drawing. This bridge also must handle panic-safe terminal restoration to prevent leaving the terminal in raw mode on crash.

## Decision Drivers

- Terminal lifecycle (raw mode, alternate screen, cursor visibility, panic cleanup) is correctness-critical — bugs here leave the user's terminal broken
- Input events must be available as Bevy events so input systems participate in the ECS scheduler
- The bridge must drive Bevy's app loop at 60fps without blocking on terminal input (crossterm polling must be non-blocking)
- Minimize custom terminal management code — terminal lifecycle is well-defined, not a differentiating feature

## Decision

We will use `bevy_ratatui` as the bridge between Bevy's scheduler and ratatui's terminal lifecycle. `RatatuiPlugins` provides `ScheduleRunnerPlugin` for the 60fps outer loop, `KeyMessage` events from crossterm, and `RatatuiContext` for frame drawing. `main.rs` additionally installs `std::panic::set_hook` before `App::run()` as a belt-and-suspenders terminal restoration guarantee.

## Consequences

### Positive

- Raw mode entry/exit, alternate screen, cursor management, and panic hooks are handled by the library — zero custom terminal lifecycle code
- `KeyMessage` events integrate naturally with Bevy's `EventReader` — input systems are standard ECS systems
- `ScheduleRunnerPlugin::run_loop(16ms)` drives the app at 60fps with non-blocking crossterm polling per frame
- ratatui's `Terminal` frame-diffing avoids full-screen redraws — only changed cells are written

### Negative

- bevy_ratatui pins to a specific Bevy version (currently ^0.18) — updating Bevy requires waiting for bevy_ratatui to catch up or forking
- One additional transitive dependency tree (bevy_ratatui + its crossterm version) — slightly increases compile time and binary size
- `KeyMessage` uses crossterm's `KeyEvent` type, not Bevy's `KeyCode` — input system must translate crossterm types

### Neutral

- bevy_ratatui is maintained under the ratatui GitHub org — reasonable expectation of ongoing maintenance, though not guaranteed

## Alternatives Considered

### Crossterm direct (no ratatui, no bevy_ratatui)

- **Description**: Write terminal output using crossterm's `queue!` / `execute!` macros and ANSI escape sequences. Read input via crossterm's `poll` / `read` in a Bevy system. Manual raw mode and alternate screen management.
- **Pros**: No bevy_ratatui version coupling. Full control over every byte written. One fewer dependency tree.
- **Cons**: Must implement raw mode entry/exit, alternate screen, cursor hiding, and panic hook manually. No frame-diffing — must track changed cells to avoid flicker, or repaint everything and accept flicker. More implementation code for identical result.
- **Why rejected**: Terminal lifecycle management is non-trivial and correctness-critical. Bugs leave the terminal broken for the user. bevy_ratatui solves this completely. Re-implementing it adds implementation risk with no architectural benefit — the game logic and ECS design are identical either way. Version pinning (see ADR-003) mitigates the coupling risk without requiring a stack change.

### termion (alternative terminal library)

- **Description**: Use `termion` instead of crossterm/ratatui for terminal I/O. termion is Unix-only, wraps ANSI sequences directly.
- **Pros**: Simpler API for basic operations. Smaller dependency.
- **Cons**: Unix-only — no Windows support. No Bevy integration crate exists. No widget system — must implement all rendering from scratch. No frame-diffing.
- **Why rejected**: No Bevy integration means building a custom bridge equivalent to bevy_ratatui. No widget system means building rendering infrastructure equivalent to ratatui. Two custom implementations to replace two existing, maintained libraries. The Unix-only restriction further narrows compatibility.

## Confirmation

Verify during implementation: `Cargo.toml` includes `bevy_ratatui` with version compatible with the pinned Bevy version. `main.rs` adds `RatatuiPlugins`. Render systems access `ResMut<RatatuiContext>`. Input systems read `EventReader<KeyMessage>`. Terminal is restored correctly after `q`/Esc, after `Ctrl-C`, and after a panic (test by inserting `panic!()` temporarily).

## References

- [RFC-001: Terminal Snake — Initial Architecture](../rfc/RFC-001-terminal-snake-architecture.md)
- [ADR-001: Bevy ECS as Game Logic Runtime](ADR-001-bevy-ecs-runtime.md)
- [bevy_ratatui GitHub](https://github.com/ratatui/bevy_ratatui)
- [ratatui docs](https://ratatui.rs)
