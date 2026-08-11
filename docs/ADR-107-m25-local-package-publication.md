# ADR-107: M25 offline local package publication

**Status:** Implemented in package 0.37.0
**Date:** 2026-08-11
**Decision makers:** Human director (mainstream-maturity objective); implementation agent
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `DOC-SYNC-001`,
`SEC-INPUT-001`, `TEST-BEHAVIOR-001`, `ENG-WARN-001`, `RND-INVAR-001`
**Depends on:** M18, M22, PKG-001

## Context

Locked local workspaces prove project identity and cross-package import, but a
team cannot turn one verified project into a portable, independently verifiable
local package and install it into another workspace. That misses the M25
publication/reuse milestone and the E4 requirement for third-party-style
internal reuse.

## Decision

1. Adopt [the M25 local package publication design](DESIGN-M25-LOCAL-PACKAGE-PUBLICATION.md).
2. Add strict `aether.package/v1` directory bundles containing only generated
   metadata, a locked project manifest, and its declared Aether source units.
3. Add explicit `aether pkg pack`, `verify`, `publish`, `install`, and
   `verify-cache` commands.
4. Require exact digest, project-lock, path-confinement, regular-file, and
   post-copy verification before any package is accepted, cached, or installed.
5. Keep cache publication local-only and deterministic. Existing package
   identity/content conflicts fail closed; M25 does not add a resolver,
   signatures, URLs, network access, or registry semantics.
6. Materialize an installed package as a normal locked project directory so it
   can be consumed through the existing explicit M18/M22 workspace graph.
7. Ship as Aether toolchain package 0.37.0 without a language or AETH change.

## Consequences

### Positive

- Projects can publish, audit, cache, install, and reuse an exact local source
  package without trusting mutable original paths.
- Consumers retain existing workspace path confinement, lock verification, and
  `depends_on` authorization rather than gaining an ambient package lookup.
- The workflow creates direct evidence for a reusable internal-package E4 gate.

### Costs

- Packagers must refresh a project lock before packing.
- Consumers explicitly materialize and declare installed packages in their
  workspace; there is no automatic resolver or workspace rewrite.
- The source-only package bounds intentionally exclude assets, generated files,
  FFI libraries, scripts, and network acquisition.

## Alternatives considered

| Alternative | Outcome |
| --- | --- |
| Keep path-only workspaces | Rejected: useful local composition but no portable publish/install lifecycle. |
| Make registry cache the source-package protocol | Rejected: F-REGISTRY trust and fetch policy should not silently define M25. |
| Build a network package manager | Rejected: materially broader authority and dependency semantics. |
| Copy a whole directory recursively | Rejected: admits hidden files, symlink ambiguity, and weak provenance. |

## Acceptance gate

1. Design, ADR, validation matrix, package contract, and user documentation are synchronized.
2. Core and CLI tests prove valid pack/verify/publish/install plus every named hostile boundary.
3. One installed package is consumed by two independent workspace packages through M22 imports.
4. The full gate is warning-free and the product/seed identity proof remains green.
5. Claims retain explicit non-claims for remote registry, resolver, signatures,
   general archives, arbitrary package payload, and guest package authority.

---

*End of ADR-107.*
