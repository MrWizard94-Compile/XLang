# ADR-015: language modules (import unit / export weave)

**Status:** Accepted; **M11a + M11b implemented** (package 0.15.0)  
**Date:** 2026-08-04  
**Decision makers:** AGENTS Constitution process; follows ADR-014 default T-MOD  
**Related Rule IDs:** `DOC-ADR-001`, `CONST-DEP-001`, `TEST-BEHAVIOR-001`, `SEC-INPUT-001`, `DOC-SYNC-001`, `CONST-COMPLETE-001`  

## Context

M10 multi-unit projects list many `.ae` files but compile them independently.
Users and AI agents cannot share weaves across files without copy-paste.
ADR-014 selected **T-MOD language modules** as the default next design track.
The forge ABI accepts a single `Text`, so multi-file product compile must not
falsely claim seed authority without proof.

## Decision

1. **Adopt** [DESIGN-M11-LANGUAGE-MODULES.md](DESIGN-M11-LANGUAGE-MODULES.md) as
   the normative M11 design (D1–D14).  
2. **Ship modules in two proof phases:**  
   - **M11a:** Syntax + bootstrap multi-source `project build` + tests/docs.  
   - **M11b:** Seed multi-module dual-compare before multi-module becomes
     seed-hosted product authority.  
3. **Reject** silent source concatenation as the module system.  
4. **Reject** URL/absolute imports; imports ⊆ project unit paths only.  
5. **Require** `export weave` for cross-unit visibility; private by default.  
6. **Allow** `lib` units without `main`; forbid `main` in libs; one graph entry.  
7. **Forbid** import cycles (DAG).  
8. **Keep** single-file `aether compile` seed-hosted and **reject** `import unit`
   in single-file mode.  
9. **Defer** import of records/shapes, export of host weaves, re-exports,
   registry, and generics-across-modules.  

Canonical validation: [M11-VALIDATION-MATRIX.md](M11-VALIDATION-MATRIX.md).

## Consequences

### Positive

- Real multi-file programs after M10  
- Honest seed/bootstrap split until seed catches up  
- Path jail aligned with project tooling  
- Clear AI/human structure (`import` / `export` nodes)  

### Costs

- Two-phase proof (M11a/M11b) increases delivery steps  
- Bootstrap multi-source is a substantial core change  
- Authoring protocol version bump required  
- Until M11b, multi-module builds are not seed-hosted  

### Risks

| Risk | Mitigation |
| --- | --- |
| Fake modules via paste | D8 rejects; tests forbid banner-concat identity |
| Seed delay blocks “done” narrative | M11a Done is explicit bootstrap-hosted multi-module |
| Diamond imports | DAG + unique world/export rules |
| Scope creep to packages/registry | Non-goals; ADR-014 |

## Alternatives considered

| Option | Outcome |
| --- | --- |
| Text concatenation bundle into seed `Text` | Rejected as default (opacity); only if fully specified elaboration later |
| Separate AETH per module + dynamic link | Deferred (harder verify/run story) |
| All weaves public across project | Rejected (no encapsulation) |
| World-name imports without paths | Rejected (path identity matches projects) |
| Wait for seed multi-file before any modules | Rejected — bootstrap M11a unblocks language learning under honest claims |

## Implementation gate

Implementation of M11a may begin when:

1. This ADR remains **Accepted**  
2. M11 matrix is present  
3. First PR is a vertical slice: parse + one import call + project build + tests  

M11a must not claim seed multi-module identity.  
M11b requires its own dual-compare green before MANIFEST switches authority.

## Links

- Design: [DESIGN-M11-LANGUAGE-MODULES.md](DESIGN-M11-LANGUAGE-MODULES.md)  
- Matrix: [M11-VALIDATION-MATRIX.md](M11-VALIDATION-MATRIX.md)  
- Prior: [ADR-013](ADR-013-m10-multi-unit-projects.md), [ADR-014](ADR-014-post-m10-track-portfolio.md)  
- Mainstream plan: [ROADMAP-MAINSTREAM-MATURITY.md](ROADMAP-MAINSTREAM-MATURITY.md) §6  

---

*End of ADR-015.*
