# ADR-101: Task weave requires checkpoint (first implementable ADR-081 vertical)

**Status:** Accepted — implemented  
**Date:** 2026-08-11  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`  
**Depends on:** ADR-042, ADR-081, ADR-089  

## Context

ADR-081 forbids silent task handles/timeouts/parallelism. M19e task weaves without
any `checkpoint` currently fall into opaque seed forge failures. Tooling already
surfaces checkpoint density (ADR-093); product must fail closed earlier.

## Decision

1. Product preflight **`AE-SEED-015`:** each `task weave` body must contain at
   least one `checkpoint` line.  
2. Does **not** implement handles, timeouts, or parallel scheduling.  
3. Tracker: `product_requires_task_weave_checkpoint() == true`.  
4. This is the first **implementable** vertical under ADR-081 design bounds
   (fail-closed surface integrity, not new runtime authority).  

## Honesty

- Runtime task handles/timeouts/parallel still require separate ADRs.  

## Links

- ADR-042, ADR-081, ADR-089, M19E-VALIDATION-MATRIX  

---

*End of ADR-101.*
