# ADR-029: stdlib layer 1 pure modules (M20b)

**Status:** Accepted — **implemented in package 0.27.0**  
**Date:** 2026-08-04  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`  
**Extends:** ADR-024 (layer 0)

## Context

Layer 0 proved stdlib as ordinary M11 lib units. Roadmap next product slice is
stdlib depth without weakening offline purity.

## Decision

1. Adopt [DESIGN-M20B-STDLIB-LAYER1.md](DESIGN-M20B-STDLIB-LAYER1.md).  
2. Expand `whole.ae` and add pure `truth.ae` + `text.ae` under `stdlib/`.  
3. Keep demo exit 42 (`double 21`); prove via project build dual-compare.  
4. Ship `whole_test.ae` for `aether test` (exit 0).  
5. Package **0.27.0**.  
6. Still forbid host I/O and registry in stdlib.

## Consequences

### Positive

- Broader pure helpers for AI/authoring demos  
- Truth/Text modules exercise multi-import graph  

### Costs

- Larger project unit set  
- More dual-compare wall time on project build  

### Risks

| Risk | Mitigation |
| --- | --- |
| Reserved weave names | Avoid reserved words (`extent`, …) |
| Text ownership mistakes | `borrow` on Text params; lib validation compile |

## Links

- DESIGN-M20B, M20B matrix, ADR-024, AETHER_0.27  

---

*End of ADR-029.*
