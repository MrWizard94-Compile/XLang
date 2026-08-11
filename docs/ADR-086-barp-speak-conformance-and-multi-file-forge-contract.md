# ADR-086: BARP — SPEAK emit conformance matrix + multi-file forge contract

**Status:** Accepted — implemented (conformance matrix + contract trackers)  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`  
**Depends on:** ADR-061, ADR-082  

## Context

Seed.ae systematic SPEAK emit and seed-native multi-file remain large rebuilds.
Product still needs a **conformance list** of SPEAK codes and a clear multi-file
forge ABI contract while residuals stay honest.

## Decision

1. **`seed_speak_emit_conformance_codes()`** lists AE-SEED codes host preflights
   already cover and seed is required to eventually SPEAK.  
2. **`seed_speak_emit_conformance_complete() == false`** until seed.ae emits all
   codes (rebuild + dual-compare).  
3. **`forge_multi_source_abi_contract() == true`:** multi-source product path is
   the multi-file forge contract (host elaborate + seed emit).  
4. **`seed_native_multi_module_elaboration() == false`** unchanged.  

## Links

- ADR-056, ADR-082, FORGE_CONTRACT, SEED_PROFILE  

---

*End of ADR-086.*
