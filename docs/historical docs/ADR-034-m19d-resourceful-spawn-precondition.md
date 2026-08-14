# ADR-034: resourceful-spawn precondition for Policy B (design-only)

**Status:** Accepted (direction) — **superseded for option 3 by ADR-035 (package 0.32)**  
**Date:** 2026-08-04  
**Related:** ADR-032, ADR-004, ADR-010, ADR-035  

## Decision

1. Adopt [DESIGN-M19D-RESOURCEFUL-SPAWN-PRECONDITION.md](DESIGN-M19D-RESOURCEFUL-SPAWN-PRECONDITION.md).  
2. Historical: do not implement spawn-scoped arenas in the design-only cycle.  
3. Keep M19b Policy A until ADR-035 Policy A+.  
4. **Done:** ADR-035 implements option 3 (multi-weave arenas + resourceful total
   spawn callees) with dual-compare matrix.

## Rationale

Honest dependency order: precondition → Policy B → product claims.

---

*End of ADR-034.*
