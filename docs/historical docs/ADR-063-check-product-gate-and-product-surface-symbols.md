# ADR-063: check product base gate + product-surface symbols

**Status:** Accepted — implemented  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`  
**Depends on:** ADR-051, ADR-058, ADR-062  

## Context

Default `aether check` always surfaced bootstrap diagnostics. LSP
`documentSymbol` always required bootstrap AST even for product-valid sources.

## Decision

1. **Default check product base gate:** when bootstrap parse fails and product
   also fails, return product `AE-SEED-*` (same pattern as format/structural-edit).  
2. Tracker: `check_product_base_gate() == true`.  
3. **`product_surface_symbols(source)`:** product-accept then line-scan for
   top-level world/weave/record names (no bootstrap AST).  
4. **LSP `documentSymbol`:** prefer product-surface symbols when product accepts;
   fall back to bootstrap AST structure on product reject.  
5. Tracker: `product_surface_symbols_without_bootstrap() == true`.  

## Honesty

- Product symbols are surface names only (not full AST body tree).  
- Default check success still uses bootstrap for token count + canonical AST dump.  

## Links

- ADR-051, ADR-058, ADR-062  

---

*End of ADR-063.*
