# ADR-050: BARP Phase 3b — product preflight expansion + roadmap DOC-SYNC

**Status:** Accepted — implemented  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `DOC-SYNC-001`, `TEST-BEHAVIOR-001`, `SEC-INPUT-001`  
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md) / [DESIGN-BARP-001](DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)  
**Depends on:** ADR-046, ADR-049  

## Context

Phase 3a/early 3b added `AE-SEED-001`–`004`. Product still falls through to opaque
forge/VM failures for empty sources, missing `world`, and obvious legacy syntax.
Roadmap M23 “Done when” still mentioned bootstrap materialization after BARP
Phase 1 removed it (`DOC-SYNC-001` debt).

Authoring `canonicalize_source` always ran bootstrap `compile_source` twice even
when source was already canonical.

## Decision

1. **Expand product preflight (host, not full bootstrap semantics):**  
   - `AE-SEED-005` empty / whitespace-only source  
   - `AE-SEED-006` missing top-level `world` declaration  
   - `AE-SEED-007` legacy-syntax heuristics (`fn `, `return `, braces, etc.)  
2. **Do not claim** full seed diagnostic parity or seed parse of full language.  
3. **DOC-SYNC** `ROADMAP.md` M23 Done-when (seed-native, no materialization) and
   refresh BARP “immediate next” after ADR-049.  
4. **Authoring:** skip second bootstrap compile when formatted source is already
   canonical (less bootstrap work; same authority for base parse).  
5. Tracker: `seed_product_preflight_phase3b() == true`.  

## Consequences

### Positive

- Faster fail-closed product errors for common junk input  
- Roadmap matches BARP reality  
- Less double-parse cost on identity authoring paths  

### Costs / honesty

- Preflights are heuristic, not full semantics  
- Bootstrap `check` remains full diagnostic authority  

## Links

- ADR-046, ADR-049, DESIGN-BARP-001, ROADMAP.md  

---

*End of ADR-050.*
