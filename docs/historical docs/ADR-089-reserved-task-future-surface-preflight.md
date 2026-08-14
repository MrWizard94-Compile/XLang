# ADR-089: Reserved future task surface product preflight (under ADR-081)

**Status:** Accepted — implemented (fail-closed reserved forms)  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`  
**Depends on:** ADR-042, [ADR-081](ADR-081-broader-task-model-design.md), ADR-085  

## Context

ADR-081 forbids silent task handles/timeouts/parallelism. Authors still need
stable product diagnostics when those reserved forms appear, instead of opaque
seed forge failures.

## Decision

1. Product preflight **`AE-SEED-014`** rejects lines starting with:  
   - `timeout `  
   - `task handle ` / `handle task `  
   - `parallel together` / `together parallel`  
2. Does **not** implement handles, timeouts, or parallel scheduling.  
3. Tracker: `product_rejects_reserved_task_future_surface() == true`.  
4. Full handle/timeout/parallel features still require new implementable ADRs.  

## Links

- ADR-081, ADR-085, M19E-VALIDATION-MATRIX  

---

*End of ADR-089.*
