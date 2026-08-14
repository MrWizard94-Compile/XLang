# ADR-061: BARP — seed error packets + multi-file forge (accepted direction)

**Status:** Partially implemented — host packet ABI + SPEAK format + multi-source host forge **done** (ADR-072/075); seed-binary SPEAK emit + seed-native multi-file **open**  
**Date:** 2026-08-10  
**Related Rule IDs:** `DOC-ADR-001`, `CONST-DEP-001`, `RND-INVAR-001`  
**Depends on:** ADR-055, ADR-056  
**Follow-on:** [ADR-072](ADR-072-barp-product-seed-error-packet-abi.md), [ADR-075](ADR-075-barp-multi-source-forge-and-seed-error-speak-format.md)

## Context

BARP residual large items after ADR-058: (1) seed-internal structured error
packets, (2) multi-file forge ABI for seed-native multi-module.

## Decision

1. **Seed error packets:**  
   - **Host product packet ABI implemented (ADR-072):** `aether.seed-error/v1`,
     `product_error_packets`, optional decode of `AETHER_SEED_ERROR:{json}`.  
   - **Seed-binary emit still open:** requires seed rebuild + dual-compare + Seed
     Profile claim; `seed_internal_error_packets() == false`.  
2. **Multi-file forge (direction):** requires forge ABI expansion (multiple Text
   inputs or host unit loader). Until then `seed_native_multi_module_elaboration()`
   remains **false**; product multi-module stays host elaborate + seed emit
   (ADR-056).  
3. Trackers:  
   - `product_seed_error_packet_abi() == true` (ADR-072)  
   - `seed_internal_error_packets() == false`  
   - `seed_native_multi_module_elaboration() == false` (unchanged)  

## Non-goals remaining

Seed SPEAK packet emit; multi-file forge ABI.

## Links

- ADR-055, ADR-056, DESIGN-BARP-001  

---

*End of ADR-061.*
