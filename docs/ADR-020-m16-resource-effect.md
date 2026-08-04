# ADR-020: resource ↔ handled-effect interaction (M16 / T-RX)

**Status:** Accepted — **implemented in package 0.21.0**  
**Date:** 2026-08-04  
**Decision makers:** AGENTS Constitution; ADR-014 **T-RX** after M15  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `SEC-INPUT-001`, `RND-INVAR-001`, `CONST-COMPLETE-001`

## Context

M4 forbids mixing M2/M6 resources with raise/forward/handle in one weave. That
prevents programs that allocate then handle a pure parse error. T-RX on the
portfolio requires a sound, bounded first interaction—not full effectful
resources or nursery cancel cleanup.

## Decision

1. **Adopt** [DESIGN-M16-RESOURCE-EFFECT.md](DESIGN-M16-RESOURCE-EFFECT.md).  
2. Allow **terminal `handle call`** in **total** weaves that own arenas, buffers,
   or tables; live ordinary owners may span the handle.  
3. Keep **abortive** control (`raises` / `raise` / `forward`) and **nurseries**
   resource-incompatible.  
4. Forbid exclusive **`access` loans** spanning a handle.  
5. Keep erroring callees copy-only and resource-free.  
6. No new AETH opcodes. Seed dual-compare a mix example before product claim.  
7. Suggested package pin: **0.21.0**.

## Consequences

### Positive

- Real total programs can allocate and handle pure errors  
- Preserves abort-cleanup non-goals without inventing drop  
- Clear ladder for later T-RX slices  

### Costs

- Two boundary checkers (abortive vs handle)  
- Docs/tests must restate M4 mix ban as abortive-only  

### Risks

| Risk | Mitigation |
| --- | --- |
| Access spanning handle | Explicit reject |
| Silent raise+resource | Weave-level abortive ban + full boundary |
| Seed/bootstrap drift | Dual-compare mandatory |

## Alternatives considered

| Option | Outcome |
| --- | --- |
| Result/ADT instead of handle | Deferred |
| Implicit free on raise | Rejected |
| Nursery+resource in same slice | Deferred (cancel model) |

## Implementation gate

1. This ADR Accepted  
2. Design + [M16-VALIDATION-MATRIX.md](M16-VALIDATION-MATRIX.md)  
3. Vertical slice: arena + handle success path  
4. Negatives: raise+resource, access across handle, nursery+resource  
5. Seed dual-compare  

## Links

- Design: [DESIGN-M16-RESOURCE-EFFECT.md](DESIGN-M16-RESOURCE-EFFECT.md)  
- Matrix: [M16-VALIDATION-MATRIX.md](M16-VALIDATION-MATRIX.md)  
- Prior: [ADR-007](ADR-007-m4-typed-error-effect.md), [ADR-004](ADR-004-aeth-v6-bounded-resources.md), [ADR-010](ADR-010-m7-structured-concurrency.md), [ADR-014](ADR-014-post-m10-track-portfolio.md)

---

*End of ADR-020.*
