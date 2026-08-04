# M19 Design: deeper resource ↔ effect (T-RX continuation)

**Status:** Accepted design direction — **not implementable as product code in this slice**  
**Date:** 2026-08-04  
**Decision record:** [ADR-023](ADR-023-m19-deeper-resource-effect.md)  
**Depends on:** M16 ([ADR-020](ADR-020-m16-resource-effect.md))  
**Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `RND-INVAR-001`

---

## 1. Purpose

M16 proved **terminal `handle` over live resources** in total weaves. Remaining
T-RX demand:

1. **Abortive** `raise`/`forward` after resources are fully ended  
2. **Nursery + arena/buffer** with cancel-safe cleanup  
3. Resource ops inside may-error weaves  

## 2. Why M19 is not a product implementation yet

| Slice | Blocker |
| --- | --- |
| Raise with prior arena in same weave | Arena stays live until lexical end; no `drop`/move-end form |
| Raise after “all resources dead” | Needs precise end-of-life / logical destruction API |
| Nursery + resource | Cancel of unstarted spawns must not orphan buffers; no cancel-cleanup model |
| Erroring weave owns buffer | Abort leaves owner without cleanup contract |

Implementing any of these without a destruction/cancel model would **fake safety**.

## 3. Candidate future slices (each needs its own vertical ADR)

### S1 — Explicit `release` / end-of-life (preferred foundation)

```text
release name   # logical destruction; binding unusable after
```

Only after `release` of all resource owners may abortive effect sites pass a
**liveness** clean boundary even if the weave once owned resources.

### S2 — Raise-after-release only

Remove weave-level abortive×resource ban; keep site-level full clean boundary
(no live unique/Arena/Buffer). Requires S1 or equivalent.

### S3 — Nursery + resource with cancel policy

Policy options (pick one in a later ADR):

- **A:** Forbid resource ops inside spawn callees; allow arena only in parent after nursery  
- **B:** On cancel, run deterministic logical destroy of spawn-local owners (needs S1)  

### S4 — Resource-carrying error payloads

Out of scope until Result/ADT and host ABI redesign.

## 4. Invariants (when any S* ships)

| ID | Invariant |
| --- | --- |
| M19-INV-001 | No abort leaves a live guest resource owner without a defined destroy |
| M19-INV-002 | M16 handle+resource remains legal |
| M19-INV-003 | Seed dual-compare for every admitted form |
| M19-INV-004 | No implicit free on raise |

## 5. Package / claims

- **No package bump** for design-only M19.  
- CORE_CLAIMS: **Accepted direction (not implemented)**.  

---

*End of DESIGN-M19-DEEPER-RESOURCE-EFFECT.md*
