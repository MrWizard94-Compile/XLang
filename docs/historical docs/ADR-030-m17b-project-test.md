# ADR-030: project `role: test` offline runner (M17b)

**Status:** Accepted — **implemented in package 0.28.0**  
**Date:** 2026-08-04  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `SEC-INPUT-001`  
**Extends:** ADR-021 (standalone `aether test`)

## Context

CLM-026 deferred project `role: test` until a dedicated ADR. Stdlib layer 1
needs module-import tests without inlining helpers.

## Decision

1. Adopt [DESIGN-M17B-PROJECT-TEST.md](DESIGN-M17B-PROJECT-TEST.md).  
2. Add `role: test` to `aether.project/v1` units.  
3. Add CLI `aether project test <project-file>`.  
4. Elaborate each test unit as entry with M11b dual-compare; pure run; exit 0.  
5. Package **0.28.0**.  
6. No grants or structured XML in this slice.

## Consequences

### Positive

- Stdlib and multi-module packages can test export weaves honestly  
- Reuses project path jail and seed product compile  

### Costs

- Schema role expansion  
- Empty test list fails closed  

### Risks

| Risk | Mitigation |
| --- | --- |
| Importing product main from tests | Ban import of main and test roles |
| Silent empty pass | Fail if zero test units |

## Links

- DESIGN-M17B, M17B matrix, ADR-021, AETHER_0.28  

---

*End of ADR-030.*
