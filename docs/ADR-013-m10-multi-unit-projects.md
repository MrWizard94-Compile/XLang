# ADR-013: multi-unit offline projects (independent compilation units)

**Status:** Accepted (design); implementation not started  
**Date:** 2026-08-04  
**Decision makers:** Human P0 freeze (TP-1 + TP-2 + P4.1); agent design under AGENTS Constitution  
**Related Rule IDs:** `DOC-ADR-001`, `DOC-SYNC-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`, `CONST-DEP-001`, `CONST-COMPLETE-001`  

## Context

M9 introduced `aether.project/v1` with optional multi-unit capacity, but the
product only shipped a single-unit fixture, nested paths were schema-blocked,
and multi-unit semantics were unspecified. Language-level modules do not exist.
P0 completion planning queued **P4.1 multi-unit offline projects** after the
technical preview.

Expanding into true multi-file **linking** would require a language import
model, seed emission proof, and a larger threat surface. Expanding into a
network registry would violate offline-first law.

## Decision

1. **Ship multi-unit offline integrity** as M10 tooling: many local units,
   nested relative paths, full locks, independent seed-compile of each unit.  
2. **Do not** introduce language modules, imports, or cross-unit symbol
   resolution in M10.  
3. **Keep** schema id `aether.project/v1` with an expanded path grammar (not a
   silent registry). Unknown fields remain denied.  
4. **Treat** `lib` as a tooling role only; each unit must still be a complete
   seed-valid program in the pilot corpus.  
5. **Add** `project format` and clarify multi-unit `project verify --output-dir`
   artifact naming.  
6. **Defer** multi-package graphs, URL deps, and lib-without-main to later ADRs.

Canonical design: [DESIGN-M10-MULTI-UNIT-PROJECTS.md](DESIGN-M10-MULTI-UNIT-PROJECTS.md).  
Validation: [M10-VALIDATION-MATRIX.md](M10-VALIDATION-MATRIX.md).

## Consequences

### Positive

- Real multi-file project trees for AI/human workflows without a registry.  
- Nested `src/` / `lib/` layouts with fail-closed path rules.  
- Clear honesty: independent units, not fake linking.  
- No seed rewrite required for M10.

### Costs

- Developers may expect cross-file calls; docs must refuse that claim.  
- Lib units still need a standalone `main` (or other complete program shape)
  until a language module ADR.  
- Artifact naming uses flat `__` mapping instead of nested output trees.

### Risks

| Risk | Mitigation |
| --- | --- |
| Scope creeps into modules | Stop condition in design; matrix forbids cross-unit resolve tests as positive |
| Path escape via nested tricks | Canonicalize + segment grammar; negative corpus |
| Schema confusion v1 vs v2 | Explicit path expansion notes in schema description |

## Alternatives considered

| Option | Pros | Cons | Outcome |
| --- | --- | --- | --- |
| A. Multi-unit integrity only (this ADR) | Offline, seed-free host, honest | No cross-file language | **Accepted** |
| B. Concatenate libs + main into one source | Single artifact | Order fragility, identity, diagnostics | Rejected |
| C. Full import/module system now | Real multi-file language | Seed + language surface + large design | Deferred |
| D. Network multi-package registry | Familiar DX | Offline law / threat model | Rejected |
| E. New `aether.project/v2` only for nested paths | Clear version bump | Unnecessary churn for path grammar | Rejected for M10 |

## Links

- Prior: [ADR-012](ADR-012-m9-project-tooling.md)  
- Design: [DESIGN-M10-MULTI-UNIT-PROJECTS.md](DESIGN-M10-MULTI-UNIT-PROJECTS.md)  
- Threat model: [THREAT_MODEL-TECHNICAL-PREVIEW.md](THREAT_MODEL-TECHNICAL-PREVIEW.md)  
- Roadmap completion targets / P4.1 queue  

## Implementation gate

Implementation may begin only when:

1. This ADR remains Accepted.  
2. M10 validation matrix is present.  
3. Implement turn treats the matrix as the test plan before claiming Done.
