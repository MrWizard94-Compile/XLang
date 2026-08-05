# M19c Policy B cooperative validation matrix

**Status:** Bounded product with package 0.32 (after M19d)  
**Date:** 2026-08-04  
**ADR:** [ADR-036](ADR-036-m19c-policy-b-cooperative.md)

| ID | Case | Evidence |
| --- | --- | --- |
| P1 | Resourceful total spawn runs | `examples/spawn-arena.ae` exit 7 |
| P2 | Unstarted cancel has no spawn-local owners | Cooperative M7 + total callees (by construction) |
| N1 | Resource spawn arguments | AE-TASK-003 |
| N2 | Mid-frame cancel | **Not claimed** |

## Honesty

No free-on-raise. No mid-frame cancel engine.
