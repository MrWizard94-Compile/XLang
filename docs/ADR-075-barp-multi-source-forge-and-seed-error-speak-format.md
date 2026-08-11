# ADR-075: BARP — multi-source forge envelope + seed-error SPEAK format

**Status:** Accepted — implemented (host multi-file forge path; host SPEAK format)  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`  
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md) / [ADR-061](ADR-061-barp-seed-error-packets-and-multifile-forge-direction.md)  
**Depends on:** ADR-055, ADR-056, ADR-072  

## Context

ADR-061 residual: (1) seed SPEAK packet emit, (2) multi-file forge. Seed-binary
SPEAK rewrite remains large; multi-module product still needed a **host**
multi-unit forge surface without a project file.

## Decision

1. **SPEAK-compatible product errors:** every product `AE-SEED-*` failure message
   embeds a trailing `AETHER_SEED_ERROR:{json}` line (`aether.seed-error/v1`).  
   Tracker: `product_seed_error_speak_format() == true`.  
   Honesty: `seed_internal_error_packets() == false` (seed binary still does not
   SPEAK).  
2. **Multi-source envelope** `aether.multi-source/v1` with `units[{path,source}]`.  
3. **Host multi-unit product forge:** `compile_product_multi_unit` /
   `compile_product_multi_source_envelope` host-elaborate in memory + seed emit.  
   Cross-package imports rejected on this pilot path.  
4. Trackers: `product_multi_source_forge_envelope() == true`;  
   `seed_native_multi_module_elaboration() == false` unchanged.  

## Links

- ADR-056, ADR-061, ADR-072, DESIGN-BARP-001  

---

*End of ADR-075.*
