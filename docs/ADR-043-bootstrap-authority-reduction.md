# ADR-043: Bootstrap Authority Reduction Program (BARP)

**Status:** Accepted (program) — Phase 0 complete; **Phase 1 complete** (seed-native M23)  
**Date:** 2026-08-08 (Phase 1 implemented 2026-08-10)  
**Related Rule IDs:** `CONST-DEP-001`, `RND-INVAR-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`  
**Design:** [DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md](DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)

## Context

Aether product compile already emits via the seed forge ABI. Before Phase 1, Rust
bootstrap still owned parse/validate on the product path and **M23
materialization** (`lower_m23_comptime_calls_for_seed`). Human direction
(2026-08-08): after a live release gate stamp, begin reducing bootstrap
authority under Constitution.

## Decision

1. Adopt DESIGN-BARP-001 as the multi-phase program.  
2. **Do not** weaken dual-compare, verify-before-run, or seed-hosted emission.  
3. **Phase 1 (done):** seed-native M23 pure comptime calls; product path forges
   original source; materialization bridge removed;
   `seed_interprets_m23_comptime_calls_natively() == true`.  
4. Bootstrap remains seed-rebuild + diagnostic + dual-compare oracle authority.  
5. Each phase ships with matrix green, DOC-SYNC, and honest Seed Profile claims.

## Consequences

### Positive

- Clear ladder toward seed-owned product validation/emission  
- Honest accounting of remaining Rust authority  

### Costs

- Seed complexity for M23 interpreter  
- Careful dual-compare during transition  

### Risks

| Risk | Mitigation |
| --- | --- |
| Silent loss of dual-compare | Forbidden by this ADR |
| Seed accepts invalid M23 | Fail closed + bootstrap oracle tests |
| Scope creep to full seed diagnostics | Phase 3 separate ADR |

## Links

- DESIGN-BARP-001, ADR-039 (M23), SEED_PROFILE claim 17  

---

*End of ADR-043.*
