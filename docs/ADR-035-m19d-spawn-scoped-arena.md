# ADR-035: multi-weave arenas and resourceful total spawn callees (M19d option 3)

**Status:** Accepted — **implement in package 0.32.0**  
**Date:** 2026-08-04  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `RND-INVAR-001`  
**Related:** ADR-004, ADR-010, ADR-027, ADR-028, ADR-032, ADR-034  

## Context

ADR-034 recorded option **3** (spawn-scoped / multi-weave arenas) as the real
Policy B enabler but blocked product code. M19b Policy A still forbids all
resourceful spawn callees, so cancel-destroy has no admissible programs.

## Decision

1. Adopt [DESIGN-M19D-SPAWN-SCOPED-ARENA.md](DESIGN-M19D-SPAWN-SCOPED-ARENA.md).  
2. **Implement** multi-weave total arenas (header capacity = sum of declarations).  
3. **Relax** nursery Policy A so total spawn callees may use self-owned M2/M6
   resources without resource parameters (Policy A+).  
4. Keep spawn **arguments** non-resource; keep erroring weaves resource-free.  
5. Keep M19a `release` and no free-on-raise.  
6. Require seed≡bootstrap for the M19d corpus.  
7. Package **0.32.0**.  
8. Supersede the “implementation blocked” stance of ADR-034 for option 3 only;
   options 1–2 remain non-product.

## Consequences

### Positive

- Admissible resourceful spawn programs  
- Honest precondition for Policy B cooperative cancel (ADR-036)  
- Shared-pool capacity remains statically bounded  

### Costs

- Seed must sum arena capacities  
- Verifier allows multiple `OP_ARENA` sites  
- Static capacity sum can reject programs that would fit if mid-run reclaim existed  

### Risks

| Risk | Mitigation |
| --- | --- |
| Shared pool interference | Sum budget; dual-compare; capacity tests |
| Policy B overclaim | Separate ADR-036; no mid-frame cancel claim |
| Seed drift | Dual-compare matrix |

## Links

- DESIGN-M19D-SPAWN-SCOPED-ARENA, M19D matrix, AETHER_0.32  

---

*End of ADR-035.*
