# ADR-033: structured offline test reports (M17d)

**Status:** Accepted — **implemented in package 0.30.0**  
**Date:** 2026-08-04  
**Related Rule IDs:** `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `SEC-INPUT-001`  
**Extends:** ADR-021, ADR-030, ADR-031

## Decision

1. Adopt [DESIGN-M17D-STRUCTURED-TEST-REPORTS.md](DESIGN-M17D-STRUCTURED-TEST-REPORTS.md).  
2. Add optional `--report` (JSON) and `--report-junit` (XML) on `aether test`
   and `aether project test`.  
3. Write only to explicit paths after the suite runs; fail closed on write error.  
4. Package **0.30.0**.  
5. Do not claim coverage or remote CI integration.

## Consequences

### Positive

- Local CI can parse pass/fail without scraping stdout  

### Risks

| Risk | Mitigation |
| --- | --- |
| Silent overwrite | Explicit path only; operator chooses |
| Path escape | Same as other CLI writes — caller-selected path |

## Links

- DESIGN-M17D, M17D matrix, AETHER_0.30  

---

*End of ADR-033.*
