# ADR-082: BARP — stable seed SPEAK protocol + product multi-file host path

**Status:** Accepted — implemented (protocol + host multi-file product authority)  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`  
**Depends on:** ADR-061, ADR-072, ADR-075, ADR-078  

## Context

Residual after ADR-078: systematic seed-binary SPEAK packets and seed-native
multi-file elaboration. Full seed rewrite remains large; product operators still
need a **stable SPEAK contract** and a **complete multi-file product path**.

## Decision

1. **SPEAK protocol stable:** line form  
   `AETHER_SEED_ERROR:{aether.seed-error/v1 JSON}`  
   Helpers: `SEED_SPEAK_ERROR_PREFIX`, `format_seed_speak_error_line`.  
   Tracker: `seed_speak_error_protocol_stable() == true`.  
2. **Honesty:** `seed_internal_error_packets() == false` until seed.ae systematically
   SPEAKs on all failure modes (dual-compare rebuild required).  
3. **Product multi-file forge path complete:** multi-source envelopes are product
   authority via host elaborate + seed emit  
   (`product_multi_file_forge_host_path() == true`).  
4. **Honesty:** `seed_native_multi_module_elaboration() == false`.  

## Links

- ADR-056, ADR-061, ADR-078, FORGE_CONTRACT  

---

*End of ADR-082.*
