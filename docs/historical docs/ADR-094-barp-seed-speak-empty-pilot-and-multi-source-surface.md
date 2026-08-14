# ADR-094: BARP — seed SPEAK empty-source pilot + multi-source unit surface

**Status:** Accepted — implemented  
**Date:** 2026-08-11  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`  
**Depends on:** ADR-082, ADR-086, ADR-090  

## Context

Host SPEAK protocol and forge SPEAK-on-failure capture were product (ADR-082/086/090),
but seed.ae did not SPEAK. Full systematic SPEAK for every AE-SEED code is a large
rebuild; an incremental dual-compare pilot is needed. Multi-file still needs host
tooling inventory without claiming seed-native multi-module elaboration.

## Decision

1. **Seed SPEAK empty-source pilot:** when source measure after newline join is 1
   (empty input), `seed/aether_seed.ae` SPEAKs  
   `AETHER_SEED_ERROR:{… "code":"AE-SEED-005", "origin":"seed-speak"}`  
   and blanks the emitted artifact Bytes. Dual-compare rebuild identity holds.  
2. **Host:** SPEAK packet lines may contain JSON braces (legacy brace preflight
   exempts SPEAK packet lines). Verify failures merge SPEAK stdout
   (`forge_verify_merges_seed_speak()`).  
3. **Multi-source unit surface:** `product_multi_source_unit_surface` inventories
   envelope units without forge (host path).  
4. Trackers:  
   - `seed_speak_emit_empty_source_pilot() == true`  
   - `forge_verify_merges_seed_speak() == true`  
   - `product_multi_source_unit_surface_api() == true`  
   - `seed_speak_emit_conformance_complete() == false` (residual)  
   - `seed_internal_error_packets() == false` (residual)  
   - `seed_native_multi_module_elaboration() == false` (residual)  

## Honesty

- Product empty input still fails at host preflight first.  
- Only AE-SEED-005 empty path is seed-SPEAK proven; full matrix remains residual.  

## Links

- ADR-086, ADR-090, FORGE_CONTRACT, SEED_PROFILE  

---

*End of ADR-094.*
