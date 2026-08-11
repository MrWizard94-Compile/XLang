# ADR-058: BARP — LSP product diagnostics primary

**Status:** Accepted — implemented  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `TEST-BEHAVIOR-001`, `DOC-SYNC-001`  
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md)  
**Depends on:** ADR-017, ADR-055  

## Context

LSP published bootstrap-only diagnostics (`compile_source`), so the editor path
was bootstrap-authoritative even though product compile is seed-hosted.

## Decision

1. **`collect_diagnostics` uses product diagnostics primary** via
   [`product_diagnostics`] (ADR-055).  
2. Messages label **product seed diagnostics**; bootstrap remains
   `aether check` for full spans/codes.  
3. Raw `import unit` → `AE-SEED-012` (aligned with product path).  
4. Symbols / format / hover / definition still use bootstrap AST (no seed AST).  
5. Tracker: `lsp_product_diagnostics_primary() == true`.  

## Links

- ADR-017, ADR-055, ADR-056  

---

*End of ADR-058.*
