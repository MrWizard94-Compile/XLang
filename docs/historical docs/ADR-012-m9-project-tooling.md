# ADR-012: offline project metadata and tooling pilot

**Status:** Accepted for Aether 0.12 / M9 implementation

**Date:** 2026-08-04

**Decision makers:** WPAI product direction; user authorization to continue
Aether development under AGENTS Constitution

**Related Rule IDs:** DOC-ADR-001, DOC-SYNC-001, SEC-INPUT-001,
TEST-BEHAVIOR-001, CONST-GATE-001, OPS-DEL-001

## Context

Roadmap M9 calls for project metadata, dependency identity, formatter/LSP
integration, and release workflow proposal. Full LSP and multi-package
registries are large surfaces that would expand authority and cognitive load
before offline identity is proved.

## Decision

Aether 0.12 implements a **bounded offline pilot**:

1. Versioned `aether.project/v1` documents with local units and optional
   SHA-256 locks.
2. CLI `project verify` for schema, path confinement, lock checks, and
   seed-compile verification.
3. CLI `format` for canonical source emission.
4. A written release gate proposal; no public package registry.

LSP and multi-package dependency resolution remain **out of scope**.

## Consequences

### Positive

- Projects gain offline integrity checks AI and humans can share.
- Formatter becomes a first-class CLI tool for authoring loops.
- Authority boundary stays local-first.

### Costs

- No IDE language server in this pilot.
- No remote packages or automatic upgrades.

## Alternatives rejected

| Alternative | Reason rejected for M9 pilot |
| --- | --- |
| Full LSP server now | Large surface; needs protocol stability beyond project verify. |
| Network package registry | Violates offline-first proof order and expands trust. |
| Absolute path units | Path-escape and non-portable projects. |
