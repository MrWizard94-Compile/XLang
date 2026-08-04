# ADR-016: fine-grained structural edits (statement-level)

**Status:** Accepted (implementable) — **implementation not started**  
**Date:** 2026-08-04  
**Decision makers:** AGENTS Constitution; ADR-014 default T-EDIT after M11  
**Related Rule IDs:** `DOC-ADR-001`, `CONST-DEP-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`, `DOC-SYNC-001`  

## Context

v6 structural edits only replace whole top-level declarations. AI-first authoring
needs smaller, validated patches. Unrestricted JSON paths would create a second
unsafe language. ADR-014 prioritizes T-EDIT after modules.

## Decision

1. **Adopt** [DESIGN-M12-FINE-GRAINED-EDITS.md](DESIGN-M12-FINE-GRAINED-EDITS.md).  
2. Ship **`aether.edit/v7`** + **`aether.ast/v7`** with statement-level ops:
   `replaceStatement`, `insertStatementAt`, `insertStatementAfter`,
   `deleteStatement`.  
3. Paths are **typed strings** (`weave:name/body/N` and limited Choose/While
   nesting), not general JSONPath.  
4. Preserve exact `baseSource`, reparse, **seed compile before write**.  
5. Product `apply-edit` accepts **v7 only** at ship (clear error for v6).  
6. Defer expression-atom surgery, Together/spawn micro-edits, multi-file edits.  

## Consequences

### Positive

- Smaller AI patches with fail-closed validation  
- Builds on existing statement JSON from `structure`  
- Keeps capability boundary  

### Costs

- New protocol major version for clients  
- Nested path implementation complexity  
- Structure emission must stay stable for statement shapes  

### Risks

| Risk | Mitigation |
| --- | --- |
| Path language creeps to full zipper | Explicit non-goals; matrix forbids atom paths |
| Index fragility after prior ops | Ops applied in order on live program; document |
| Diagnostic noise | Stable AE-EDIT-01x codes |

## Alternatives considered

| Option | Outcome |
| --- | --- |
| Whole-weave replace only (status quo) | Insufficient for AI |
| Full JSONPath / JSON Patch | Rejected (unsafe second language) |
| Text diff apply | Rejected (no structure guarantees) |
| Keep v6 + v7 dual forever | Optional later; first ship v7-only for simplicity |

## Implementation gate

1. This ADR Accepted  
2. M12 matrix present  
3. Vertical slice: replace one Yield in main via apply-edit + seed compile  

## Links

- Design: [DESIGN-M12-FINE-GRAINED-EDITS.md](DESIGN-M12-FINE-GRAINED-EDITS.md)  
- Matrix: [M12-VALIDATION-MATRIX.md](M12-VALIDATION-MATRIX.md)  
- Prior: [AETHER_AUTHORING_PROTOCOL_v6.md](AETHER_AUTHORING_PROTOCOL_v6.md), [ADR-014](ADR-014-post-m10-track-portfolio.md)  

---

*End of ADR-016.*
