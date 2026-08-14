# ADR-097: Task-model inventory surface (under ADR-081)

**Status:** Accepted — implemented (tooling inventory only)  
**Date:** 2026-08-11  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`  
**Depends on:** ADR-081, ADR-085, ADR-089, ADR-093  

## Context

ADR-081 forbids silent task handles/timeouts/parallelism. Tooling still needs a
single inventory that combines task-frame surface, checkpoint density, reserved
form hits, and spawn targets without implementing runtime handles.

## Decision

1. **`product_task_model_inventory(source)`** returns:  
   - optional `ProductTaskFrameSurface` when product accepts  
   - reserved form hits (timeout / task-handle / parallel)  
   - spawn call targets scanned from source  
   - `product_accepted` flag  
2. Does **not** add opcodes, VM handles, timeouts, or parallel schedulers.  
3. Tracker: `product_task_model_inventory_api() == true`.  

## Honesty

- Inventory only; real handles/timeouts/parallel still require implementable ADRs.  

## Links

- ADR-081, ADR-085, ADR-089, ADR-093, M19E-VALIDATION-MATRIX  

---

*End of ADR-097.*
