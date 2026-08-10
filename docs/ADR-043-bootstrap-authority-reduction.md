# ADR-043: Bootstrap Authority Reduction Program (BARP)

**Status:** Accepted (program) — Phase 0–2 complete; Phase 3 later  
**Date:** 2026-08-08 (Phase 1 2026-08-10; Phase 2 2026-08-10 via ADR-044)  
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
4. **Phase 2 (done; ADR-044):** validate-light product path — forge-first
   product bytecode without bootstrap validate pre-gate;
   `product_path_forges_before_bootstrap_validate() == true`.  
5. Bootstrap remains seed-rebuild + diagnostic + dual-compare oracle authority.  
6. Each phase ships with matrix green, DOC-SYNC, and honest Seed Profile claims.

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

- DESIGN-BARP-001, ADR-039 (M23), ADR-044 (Phase 2), SEED_PROFILE  

---

*End of ADR-043.*
