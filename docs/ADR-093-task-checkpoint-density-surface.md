# ADR-093: Task checkpoint density on product task-frame surface (under ADR-081)

**Status:** Accepted — implemented (read-only density API)  
**Date:** 2026-08-11  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`  
**Depends on:** ADR-042, ADR-081, ADR-085, ADR-089  

## Context

ADR-089 only reserved future task syntax. Tooling needs deeper **read-only**
insight into M19e checkpoint density without adding handles, timeouts, or
parallelism.

## Decision

1. Extend `product_task_frame_surface` with:  
   - `task_checkpoint_counts: Vec<(name, count)>`  
   - `total_checkpoint_count`  
2. Counts `OP_TASK_CHECKPOINT` in verified task weaves.  
3. Tracker: `product_task_checkpoint_density_api() == true`.  
4. Still **no** task handles, timeouts, or parallel scheduler.  

## Links

- ADR-081, ADR-085, M19E-VALIDATION-MATRIX  

---

*End of ADR-093.*
