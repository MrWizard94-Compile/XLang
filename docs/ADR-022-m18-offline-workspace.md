# ADR-022: offline multi-package workspace (M18 / T-PKG)

**Status:** Accepted — **implemented in package 0.23.0**  
**Date:** 2026-08-04  
**Decision makers:** AGENTS Constitution; ADR-014 **T-PKG** after M17  
**Related Rule IDs:** `SEC-INPUT-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `CONST-DEP-001`

## Context

Single-root projects are proven (M9–M11). Monorepo-style local multi-package
graphs are still missing. Network registries remain blocked by law.

## Decision

1. Adopt [DESIGN-M18-OFFLINE-WORKSPACE.md](DESIGN-M18-OFFLINE-WORKSPACE.md).  
2. Add `aether.workspace/v1` with named local package paths and optional
   `depends_on`.  
3. Add `aether workspace verify` that path-jails, rejects cycles, and runs
   existing project verification per package.  
4. **Do not** add cross-package language imports or registries in M18.  
5. Package pin **0.23.0**.

## Consequences

### Positive

- Honest multi-package offline integrity  
- Reuses project verify  

### Costs

- Second document type  
- No unified cross-package lock yet  

### Risks

| Risk | Mitigation |
| --- | --- |
| Path escape | Same jail as projects |
| Fake “linked packages” claim | Explicit non-goal |

## Implementation gate

1. ADR Accepted  
2. Matrix + example workspace  
3. Cycle/escape negatives  
4. DOC-SYNC  

## Links

- Design / matrix: [DESIGN-M18](DESIGN-M18-OFFLINE-WORKSPACE.md), [M18 matrix](M18-VALIDATION-MATRIX.md)

---

*End of ADR-022.*
