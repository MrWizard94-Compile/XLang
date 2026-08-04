# ADR-011: host ABI pilot (capability-closed)

**Status:** Accepted for Aether 0.11 / M8 implementation

**Date:** 2026-08-04

**Decision makers:** WPAI product direction; user authorization to continue
language development (M8) under AGENTS Constitution

**Related Rule IDs:** SEC-INPUT-001, RND-INVAR-001, RND-CORE-001,
DOC-ADR-001, DOC-SYNC-001, TEST-BEHAVIOR-001, CONST-GATE-001

## Context

Roadmap M8 requires a narrow, typed, ownership-aware host/foreign interface
pilot. Project law forbids ambient host capability in AETH guests and forbids
source translation to C/LLVM. A full C ABI would expand the threat model
prematurely.

Forge already defines a host-owned compile path. Guests still lack an explicit
way to call host-provided pure services under a closed catalog.

## Decision

Aether 0.11 implements M8 as:

1. **`host weave` declarations** — total external signatures, no body.
2. **Primitive-only host ABI** — Whole/Truth copy; Text/Bytes borrow only.
3. **`HOST_CALL` in AETH v11** — fail closed unless the host installs a match.
4. **Product pure fixture** — `whole_inc` and `text_extent` only; no I/O.
5. **Seed byte identity** and authoring v6 for `HostWeave` nodes.

C-facing FFI, dynamic libraries, and I/O-bearing host services are **out of
scope** and require a new ADR.

## Consequences

### Positive

- Explicit, AI-readable host boundary.
- Capability denial is testable (missing service / illegal types).
- Preserves forge and guest isolation.

### Costs

- No general foreign functions or C interop.
- Host catalog is tiny and pure.

## Alternatives rejected

| Alternative | Reason rejected for M8 |
| --- | --- |
| C header parsing | Hostile input + ambient ABI surface. |
| Guest file/network ops | Violates no-capability-leak law. |
| Implicit libc linkage | Non-local authority; not seed-auditable. |
| Reusing ordinary weaves with empty bodies | Blurs guest/host identity in AETH. |
