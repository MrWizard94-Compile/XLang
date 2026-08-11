# ADR-061: BARP — seed error packets + multi-file forge (accepted direction)

**Status:** Accepted direction — **not fully implemented**  
**Date:** 2026-08-10  
**Related Rule IDs:** `DOC-ADR-001`, `CONST-DEP-001`, `RND-INVAR-001`  
**Depends on:** ADR-055, ADR-056  

## Context

BARP residual large items after ADR-058: (1) seed-internal structured error
packets, (2) multi-file forge ABI for seed-native multi-module.

## Decision

1. **Seed error packets (direction):** future seed may fail closed by returning a
   well-known reject path; host continues `product_diagnostics` / AE-SEED-* as the
   product ABI (ADR-055). Implementing seed-internal packets requires seed rebuild
   + dual-compare + Seed Profile claim update — separate vertical slice.  
2. **Multi-file forge (direction):** requires forge ABI expansion (multiple Text
   inputs or host unit loader). Until then `seed_native_multi_module_elaboration()`
   remains **false**; product multi-module stays host elaborate + seed emit
   (ADR-056).  
3. Trackers for direction honesty:  
   - `seed_internal_error_packets() == false`  
   - `seed_native_multi_module_elaboration() == false` (unchanged)  

## Non-goals this ADR

Product code for multi-file forge or seed packet emit.

## Links

- ADR-055, ADR-056, DESIGN-BARP-001  

---

*End of ADR-061.*
