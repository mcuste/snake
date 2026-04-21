# PRD-001: Terminal Snake Game

- **ID**: PRD-001
- **Status**: Accepted
- **Author**: Claude
- **Created**: 2026-04-19
- **Last Updated**: 2026-04-19

## Problem

Developers spend significant idle time waiting for compilation, CI pipelines, and AI agent responses. There's no quick-launch distraction available directly in the terminal where they're already working. Alt-tabbing to a browser game breaks flow; a terminal-native game launches instantly and stays in context.

## Personas & Use Cases

- **Waiting Dev** (developer in terminal): Wants a quick game to fill 30s-5min idle gaps without leaving the terminal or breaking focus.
- **Vim-native Dev** (keyboard-centric developer): Expects hjkl navigation and keyboard-only controls — no mouse dependency.

## Goals & Scope

- **Must have**:
  - Classic snake gameplay: directional movement, food collection, growth, collision death
  - Terminal rendering — runs in any modern terminal emulator
  - Multiple control schemes: arrow keys, WASD, hjkl (vim keys)
  - Score tracking visible during play
  - Game over state with instant restart (no re-launch needed)
  - Progressive difficulty — snake speeds up as score increases
  - Instant startup — under 1 second from command to gameplay

- **Should have**:
  - Centered game area that adapts to terminal size
  - Visual distinction between snake head, body, and food
  - Color-coded elements for quick readability

- **Non-goals**:
  - Multiplayer — single player only for v1
  - Persistent high scores — no file I/O for scores
  - Sound/audio — terminal context, silent operation
  - Mouse input — keyboard only
  - Wrap-around walls — hitting walls is game over

## User Stories

- As a **Waiting Dev**, I want to type a single command and immediately start playing so that I can fill short idle gaps.
  - **Acceptance**: Game renders and accepts input within 1 second of launch.
  - **Scenario**: Dev runs `cargo run` → sees game board with snake and food → presses arrow key → snake moves → eats food → score increments → snake grows.

- As a **Waiting Dev**, I want to restart instantly after dying so that I don't waste idle time on menus.
  - **Acceptance**: Single keypress resets game to initial state.
  - **Scenario**: Snake hits wall → "GAME OVER" appears with score → dev presses `r` → board resets, score zeros, snake returns to start position.

- As a **Vim-native Dev**, I want to use hjkl keys so that my hands stay on home row.
  - **Acceptance**: h/j/k/l map to left/down/up/right respectively, alongside arrow keys and WASD.
  - **Scenario**: Dev presses `k` → snake moves up. Presses `l` → snake turns right. All three control schemes work identically.

- As a **Waiting Dev**, I want to quit cleanly so that my terminal isn't left in a broken state.
  - **Acceptance**: `q` or `Esc` exits game and restores terminal to pre-launch state.
  - **Scenario**: Dev presses `q` mid-game → game exits → terminal prompt appears normally, cursor visible, input echoed.

## Behavioral Boundaries

- **Grid size**: 60 wide x 60 tall cells. Beyond terminal size: game area centers and clips gracefully.
- **Speed range**: Starts at ~150ms per move, speeds up to minimum ~70ms. Snake never becomes uncontrollable.
- **Direction reversal**: Pressing opposite direction (e.g., left while moving right) is ignored — prevents instant self-collision.
- **Food placement**: Food never spawns on the snake body. If snake fills entire grid: player wins (unlikely at 3600 cells, no special handling needed).

## Non-Functional Requirements

- **Startup**: Cold start under 1 second (release build)
- **Frame rate**: 60fps input polling, smooth visual updates
- **Binary size**: Minimal — no GPU dependencies, no windowing libraries
- **Compatibility**: Any terminal supporting ANSI colors (iTerm2, Alacritty, Kitty, Ghostty, WezTerm, Terminal.app)
- **Architecture**: ECS-based — game logic must be separable from rendering for future migration to graphical mode

## Risks & Open Questions

- **Risk**: Bevy minimal plugins may still pull heavy compile-time dependencies — likelihood: M — mitigation: disable default features aggressively, accept first compile is slow
- **Dependency**: `bevy_ratatui` crate must support the Bevy version used — if latest Bevy isn't supported yet, pin to the Bevy version that `bevy_ratatui` targets

## Success Metrics

- Playable snake game in terminal within single implementation session
- Startup under 1 second (release build)
- Dev enjoys playing it while waiting for builds

## References

- [bevy_ratatui](https://github.com/ratatui/bevy_ratatui) — Bevy + ratatui integration
- [ratatui](https://ratatui.rs) — terminal UI framework
