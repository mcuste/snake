# ADR-001: Bevy ECS as Game Logic Runtime

**Status**: Accepted
**Date**: 2026-04-19
**Deciders**: mcuste
**Affects**: All modules under `src/` — components, resources, systems, state, plugin
**Implements RFC**: [RFC-001](../rfc/RFC-001-terminal-snake-architecture.md)

## Context

PRD-001 requires an ECS-based architecture where game logic is separable from rendering, enabling future migration to a graphical renderer without rewriting game systems. The game is a terminal Snake with 15 systems, 3 entity archetypes, and 5 resources. The rendering target today is ratatui (terminal); the future target is Bevy's graphical pipeline. The ECS runtime choice determines whether that migration is a plugin swap or a full rewrite.

## Decision Drivers

- Game logic must be decoupled from rendering — PRD-001 hard requirement
- Future migration to Bevy graphical rendering must not require rewriting systems
- PRD-001 explicitly references `bevy_ratatui`, which only works with Bevy's ECS
- Scheduler, state machine, and event system should be declarative to reduce boilerplate for 15+ systems

## Decision

We will use Bevy with `MinimalPlugins` as the ECS runtime for all game logic. Components, resources, events, and systems use Bevy's ECS API exclusively. `bevy_render`, `bevy_winit`, and GPU-dependent plugins are excluded via feature flags. `bevy_log` is excluded to prevent stderr corruption of ratatui output.

## Consequences

### Positive

- All game systems (movement, collision, food, scoring) are portable to Bevy's graphical renderer by swapping the render plugin — no system rewrites
- Bevy's `States`, `Events`, `SystemSet::chain()`, and `run_if` conditions eliminate manual game-loop orchestration
- `MinimalPlugins` provides `Time`, `Timer`, and `ScheduleRunnerPlugin` — sufficient for terminal game timing

### Negative

- First compile is 60–120 seconds even with aggressive feature trimming — Bevy's proc-macro and codegen surface is large
- Bevy's release cycle (every ~3 months) means API churn; pinned version will eventually lag ecosystem

### Neutral

- Binary size is larger than a non-Bevy solution but still small (single-digit MB) — acceptable for a CLI tool
- Learning curve for Bevy's ECS patterns is a one-time cost

## Alternatives Considered

### hecs (lightweight archetypal ECS)

- **Description**: ~5k lines, no scheduler, no plugins, no state machine. Manual game loop with `loop { poll; update; render; sleep }`.
- **Pros**: Compile time ~5–10 seconds. Tiny dependency tree. Simple to understand entire stack.
- **Cons**: No scheduler — system ordering, run conditions, and state transitions require manual code. hecs entities cannot be used in a Bevy `App`.
- **Why rejected**: Migration to Bevy's graphical renderer requires porting every system and resource, not just swapping a render plugin. The PRD's ECS requirement exists specifically to enable that migration, which is only fulfilled if the ECS *is* Bevy's ECS.

### specs (parallel ECS)

- **Description**: Archetypal ECS with built-in parallel dispatch. Mature but maintenance-mode.
- **Pros**: Parallel system execution. Smaller than Bevy.
- **Cons**: Maintenance-mode — last major release was 2021. Different API surface from Bevy's ECS. No Bevy integration.
- **Why rejected**: Same migration problem as hecs — systems written for specs cannot run in a Bevy `App`. Additionally, specs is effectively unmaintained, creating long-term dependency risk.

## Confirmation

Verify during implementation: `main.rs` uses `MinimalPlugins` only (no `DefaultPlugins`). `Cargo.toml` disables `bevy/default-features` and enables only required features (`bevy_state`, `bevy_time`). `cargo tree` shows no `bevy_render` or `bevy_winit` in the dependency graph.

## References

- [RFC-001: Terminal Snake — Initial Architecture](../rfc/RFC-001-terminal-snake-architecture.md)
- [PRD-001: Terminal Snake Game](../prd/PRD-001-terminal-snake.md)
- [Bevy MinimalPlugins docs](https://docs.rs/bevy/latest/bevy/struct.MinimalPlugins.html)
