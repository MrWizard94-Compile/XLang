# ADR-021: offline `aether test` runner (M17)

**Status:** Accepted — **implemented in package 0.22.0**  
**Date:** 2026-08-04  
**Decision makers:** AGENTS Constitution; maturity T-TEST after M16  
**Related Rule IDs:** `SEC-INPUT-001`, `TEST-BEHAVIOR-001`, `DOC-ADR-001`, `CONST-DEP-001`

## Context

Aether has seed compile, pure run, projects, and grants, but no product command
to batch-check small programs. Maturity roadmap M21 calls for `aether test`.

## Decision

1. Adopt [DESIGN-M17-OFFLINE-TEST-RUNNER.md](DESIGN-M17-OFFLINE-TEST-RUNNER.md).  
2. Add CLI `aether test [path...]` with `*_test.ae` discovery and explicit file runs.  
3. Pass = seed-compile + verify + pure run with exit code 0.  
4. No grants, network, or project schema change in M17.  
5. Package pin **0.22.0**.

## Consequences

### Positive

- Local CI-style feedback without external runners  
- Reuses product compile/run authority  

### Costs

- File-level tests only (no weave attributes)  
- No structured XML report yet  

### Risks

| Risk | Mitigation |
| --- | --- |
| Path escape via walk | Symlink skip + root confinement |
| Nonzero-as-pass confusion | Document exit 0 only |

## Implementation gate

1. ADR Accepted  
2. Matrix present  
3. Pass/fail/empty discovery tests  
4. DOC-SYNC  

## Links

- Design / matrix: [DESIGN-M17](DESIGN-M17-OFFLINE-TEST-RUNNER.md), [M17 matrix](M17-VALIDATION-MATRIX.md)

---

*End of ADR-021.*
