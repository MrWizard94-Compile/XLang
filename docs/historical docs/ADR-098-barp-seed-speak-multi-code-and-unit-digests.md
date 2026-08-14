# ADR-098: BARP — multi-code seed SPEAK pilot + multi-source unit digests

**Status:** Accepted — implemented  
**Date:** 2026-08-11  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`  
**Depends on:** ADR-094  

## Context

ADR-094 proved seed SPEAK for empty source only. Operators need broader seed-side
structured diagnostics and multi-file inventory integrity without claiming full
seed-native multi-module elaboration.

## Decision

1. **Seed SPEAK multi-code pilot** (`seed/aether_seed.ae`, dual-compare rebuild):  
   - `AE-SEED-005` empty  
   - `AE-SEED-006` missing `world`  
   - `AE-SEED-004` weave present without main  
   - `AE-SEED-012` raw `import unit` (needle constructed at runtime to avoid
     self-host false positives)  
2. Trackers: `seed_speak_emit_multi_code_pilot()`, `seed_speak_emit_pilot_codes()`.  
3. **Multi-source unit digests:** each unit surface entry includes `source_sha256`.  
4. Tracker: `product_multi_source_unit_digests_api() == true`.  
5. Honesty: `seed_speak_emit_conformance_complete() == false`;  
   `seed_native_multi_module_elaboration() == false`.  

## Links

- ADR-086, ADR-094, FORGE_CONTRACT, SEED_PROFILE  

---

*End of ADR-098.*
