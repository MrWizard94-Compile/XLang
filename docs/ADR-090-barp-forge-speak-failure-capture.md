# ADR-090: BARP — forge preserves SPEAK diagnostics on failure

**Status:** Accepted — implemented  
**Date:** 2026-08-11  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`  
**Depends on:** ADR-082, ADR-086  

## Context

When the seed compiler SPEAKs structured packets then fails with Error[Whole] or
a runtime error, host forge previously discarded stdout. That blocked seed SPEAK
packet merge on failure paths.

## Decision

1. Host `invoke_artifact` **appends SPEAK stdout** to forge/runtime error messages.  
2. Product compile maps forge failures through  
   `format_seed_product_error_with_seed_stdout` so packets in the detail string
   are preferred.  
3. Tracker: `forge_preserves_speak_on_failure() == true`.  
4. **Honesty:** seed.ae still does not systematically SPEAK all conformance codes
   (`seed_speak_emit_conformance_complete() == false`).  

## Links

- ADR-086, FORGE_CONTRACT  

---

*End of ADR-090.*
