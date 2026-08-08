# ADR-041: PKG-001 offline workspace locks

**Status:** Implemented — package **0.35.0**
**Accepted:** 2026-08-08
**Implemented:** 2026-08-08
**Decision makers:** Human-ordered offline-package-polish backlog; AGENTS Constitution
**Related Rule IDs:** `CONST-CONTRACT-001`, `CONST-DEP-001`, `DOC-ADR-001`,
`DOC-SYNC-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`, `CONST-COMPLETE-001`
**Portfolio:** M18/M22 offline package bounds; no F-REGISTRY authority

## Context

M18 proves local path-confined workspaces and M22 proves direct cross-package
imports. Their package documents do not bind a workspace to the raw bytes and
declared identity of each nested project manifest. M18 explicitly deferred a
unified cross-package lock.

Project-level locks already bind a project's unit bytes when authors opt in.
PKG-001 must compose that existing mechanism without inventing a registry,
network client, resolver, or guest-visible package capability.

## Decision

1. Adopt [DESIGN-PKG-001-OFFLINE-WORKSPACE-LOCKS.md](DESIGN-PKG-001-OFFLINE-WORKSPACE-LOCKS.md).
2. Extend `aether.workspace/v1` with an optional complete `lock.packages`
   list binding each declared package's name, relative path, parsed project
   name/version, and raw project-manifest SHA-256 digest.
3. Require a complete existing project unit lock for every package when a
   workspace lock is generated or verified.
4. Add explicit local `aether project lock` and `aether workspace lock`
   refresh commands. Without `--write`, they print canonical JSON; only
   `--write` replaces the named input manifest.
5. Make a locked `workspace build` verify the entire workspace before
   elaboration and artifact output. Leave unlocked workspace build behavior
   compatible with M22.
6. Strengthen workspace project-manifest resolution so the manifest itself
   cannot be a symlink escaping the package root.
7. Keep Aether language source forms at **0.11**, AETH at **v11**, and all
   guest capabilities unchanged. Ship as toolchain package **0.35.0**.

## Consequences

### Positive

- Fully local, reproducible package identity for locked workspaces.
- Nested project unit locks and workspace manifest locks cover different,
  complementary integrity boundaries.
- Locked build cannot silently compile stale package metadata into an artifact.
- Canonical lock output is suitable for AI-generated and human-reviewed
  manifests.

### Costs

- Authors refresh project locks before refreshing the workspace lock.
- Locked workspace builds verify all workspace packages first; this is a
  deliberate integrity cost rather than an incremental-build claim.
- A PKG-001 lock is unsupported by older strict v1 parsers, which fail closed
  instead of ignoring it.

## Alternatives considered

| Alternative | Outcome |
| --- | --- |
| Keep workspace documents unlocked indefinitely | Rejected — package identity remains mutable. |
| Workspace lock without nested project locks | Rejected — source-unit bytes remain unpinned. |
| One recursive command that writes every project plus workspace manifest | Rejected — multi-file partial-write recovery would need a separate transaction design. |
| Sidecar or remote lock store | Rejected — relocates or expands trust without need. |
| Registry, URL dependencies, semver solver | Rejected — F-REGISTRY and a separate package design would be required. |

## Implementation gate

1. Design and PKG-001 validation matrix accepted.
2. Parse, refresh, verify, and CLI paths have intended-behavior and hostile
   tests.
3. Locked workspace build is shown to fail before output on stale lock data.
4. Existing unlocked workspace fixture remains compatible.
5. Formatting, zero-warning linting, debug/release tests, full Aether gate,
   documentation sync, and delivery report are green.

## Implementation record

Package 0.35 provides the optional `lock` document field, canonical local
project/workspace lock refresh, locked-manifest and locked-unit verification,
post-canonicalization manifest confinement, locked-build preflight, a locked
checked-in M22 fixture, synchronized package contract, and final gate evidence.
See the [PKG-001 delivery report](DELIVERY_REPORT-2026-08-08-PKG-001-OFFLINE-WORKSPACE-LOCKS.md).

---

*End of ADR-041.*
