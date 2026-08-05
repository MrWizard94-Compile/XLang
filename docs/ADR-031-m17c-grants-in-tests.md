# ADR-031: optional grants on offline tests (M17c)

**Status:** Accepted — **implemented in package 0.29.0**  
**Date:** 2026-08-04  
**Related Rule IDs:** `SEC-INPUT-001`, `TEST-BEHAVIOR-001`, `DOC-ADR-001`  
**Extends:** ADR-021, ADR-030, ADR-018

## Context

CLM-026 required a dedicated ADR before grants-in-tests. M14 already defined
capability-mediated grants for `aether run`.

## Decision

1. Adopt [DESIGN-M17C-GRANTS-IN-TESTS.md](DESIGN-M17C-GRANTS-IN-TESTS.md).  
2. Thread `HostGrantConfig` into test runners; default empty.  
3. Accept the same `--grant-*` flags as `aether run` on `aether test` and
   `aether project test`.  
4. Package **0.29.0**.  
5. Do not claim ambient or network test grants.

## Consequences

### Positive

- Host I/O programs can be offline-tested under explicit operator grants  
- Pure suites remain grant-free by default  

### Risks

| Risk | Mitigation |
| --- | --- |
| Silent grant leak into pure tests | Default empty; flags required |
| Path escape | Reuse M14 canonicalize + jail |

## Links

- DESIGN-M17C, M17C matrix, ADR-018, ADR-021  

---

*End of ADR-031.*
