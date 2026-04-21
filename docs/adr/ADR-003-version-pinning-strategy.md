# ADR-003: Version Pinning Strategy for Bevy and bevy_ratatui

**Status**: Accepted
**Date**: 2026-04-19
**Deciders**: mcuste
**Affects**: `Cargo.toml`
**Implements RFC**: [RFC-001](../rfc/RFC-001-terminal-snake-architecture.md)

## Context

bevy_ratatui targets a specific Bevy major version (currently ^0.18). Bevy releases new major versions roughly every 3 months, often with breaking API changes. bevy_ratatui may lag Bevy releases by weeks or months. The game depends on both crates working together — an incompatible version pair causes compile failures. RFC-001 resolved this: always use Bevy + bevy_ratatui together; pin to compatible versions; never fall back to an alternative stack.

## Decision Drivers

- Bevy and bevy_ratatui must compile together — incompatible versions are a hard build failure
- The game's stack is fixed at Bevy + bevy_ratatui per RFC-001 — stack switching is not an option for version conflicts
- Development velocity matters — time spent on version upgrades does not advance the game
- Bevy's 3-month release cycle means the pinned version will lag within one quarter

## Decision

We will pin Bevy and bevy_ratatui to mutually compatible versions in `Cargo.toml` at implementation start. We will not update either dependency independently. When updating, both must be updated together to a verified-compatible pair. If the latest Bevy version has no compatible bevy_ratatui release, we will stay on the current pinned versions rather than updating Bevy alone.

## Consequences

### Positive

- Build never breaks due to transitive version conflicts between the two core dependencies
- No time spent chasing Bevy API changes that don't benefit the game
- Deterministic builds — same versions across all developer machines and CI

### Negative

- Pinned Bevy version will lag current within one quarter — new Bevy features, performance improvements, and bug fixes are unavailable until bevy_ratatui catches up
- Security patches in newer Bevy versions are not automatically picked up — requires manual monitoring
- If bevy_ratatui is abandoned, the project is stuck on the last compatible Bevy version until a fork is created or an alternative bridge is built

### Neutral

- `Cargo.lock` already pins exact versions; this ADR makes the pinning intentional and documented rather than incidental

## Alternatives Considered

### Always use latest Bevy, fork bevy_ratatui if needed

- **Description**: Track Bevy's latest release. If bevy_ratatui hasn't caught up, fork it and port to the new Bevy version.
- **Pros**: Access to latest Bevy features and fixes. Never blocked by upstream bevy_ratatui.
- **Cons**: Forking and porting bevy_ratatui on every Bevy release costs developer time. The fork must be maintained until upstream catches up. Bevy's breaking changes are unpredictable in scope — porting effort ranges from trivial to multi-day.
- **Why rejected**: The game is feature-complete at the PRD-001 scope. There is no Bevy feature on the horizon that the terminal snake game needs. Porting effort is pure overhead with no gameplay benefit. If a future need arises (e.g., graphical migration), this strategy can be adopted then via a new ADR.

### Update dependencies independently, fix conflicts as they arise

- **Description**: Run `cargo update` periodically. When Bevy and bevy_ratatui conflict, resolve on a case-by-case basis.
- **Pros**: Simple policy. May work if version gaps are small.
- **Cons**: `cargo update` may pull a new Bevy minor that's semver-compatible but has behavior changes. Conflicts appear as opaque compile errors in transitive dependencies. Debugging version conflicts in Bevy's large dependency tree is time-consuming.
- **Why rejected**: Reactive conflict resolution wastes more developer time than proactive pinning saves. The game has no need for rolling updates — it targets a fixed feature set (PRD-001).

## Confirmation

Verify in `Cargo.toml`: both `bevy` and `bevy_ratatui` have exact or tightly bounded version requirements (e.g., `"=0.18.1"` or `"0.18"`). `cargo tree -i bevy` shows a single Bevy version in the dependency graph (no duplicates). Document the verified-compatible version pair in a comment in `Cargo.toml`.

## References

- [RFC-001: Terminal Snake — Initial Architecture](../rfc/RFC-001-terminal-snake-architecture.md)
- [ADR-001: Bevy ECS as Game Logic Runtime](ADR-001-bevy-ecs-runtime.md)
- [ADR-002: bevy_ratatui as Terminal Rendering Bridge](ADR-002-bevy-ratatui-bridge.md)
