# ADR-078: BARP — product multi-source compile + seed SPEAK diagnostic merge

**Status:** Accepted — implemented (host multi-file product path; seed SPEAK merge)  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`  
**Depends on:** ADR-061, ADR-072, ADR-075  

## Context

ADR-075 added multi-source envelope APIs. Product `compile_product_bytecode`
still required callers to know about multi-unit helpers. Seed-binary SPEAK
packets remain unproven for every failure mode.

## Decision

1. **`compile_product_bytecode` auto-detects** `aether.multi-source/v1` envelopes
   and dispatches to host multi-unit forge (elaborate + seed emit).  
2. Tracker: `product_compile_accepts_multi_source_envelope() == true`.  
3. **Seed SPEAK diagnostic merge:** when forge returns non-Bytes with SPEAK
   stdout containing `AETHER_SEED_ERROR`, product diagnostics prefer that packet.  
4. Tracker: `seed_speak_packet_diagnostic_merge() == true`.  
5. **Honesty residual:**  
   - `seed_internal_error_packets() == false` (seed does not systematically SPEAK)  
   - `seed_native_multi_module_elaboration() == false`  

## Links

- ADR-056, ADR-061, ADR-075  

---

*End of ADR-078.*
