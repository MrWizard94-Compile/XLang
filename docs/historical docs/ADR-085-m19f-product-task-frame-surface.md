# ADR-085: M19f — product task-frame surface inspection (under ADR-081)

**Status:** Accepted — implemented (read-only surface API)  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`  
**Depends on:** ADR-042 (M19e), [ADR-081](ADR-081-broader-task-model-design.md)  

## Context

ADR-081 forbade silent expansion of task handles/timeouts/parallelism. Tooling
still needs a **product** way to inspect M19e task-frame surface on verified
AETH without bootstrap AST.

## Decision

1. **`product_task_frame_surface(bytecode)`** returns:  
   - artifact version  
   - task weave names  
   - task weave count  
   - total task frame arena capacity  
2. **Read-only:** verify + parse artifact metadata; no new opcodes, no handles,
   no timeouts, no parallelism.  
3. Tracker: `product_task_frame_surface_api() == true`.  
4. Follow-on implementable ADRs still required for handles/timeouts/parallel.  

## Links

- ADR-042, ADR-081, M19E-VALIDATION-MATRIX  

---

*End of ADR-085.*
