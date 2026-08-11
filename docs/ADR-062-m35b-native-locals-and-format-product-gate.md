# ADR-062: M35b AETH→C locals + format product base gate

**Status:** Accepted — implemented  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`  
**Depends on:** ADR-053, ADR-057, ADR-059  

## Context

M35a lowered only local-free pure yield. Real pure programs use `bind` (STORE/LOAD).
Default `format_source` always surface bootstrap diagnostics even when product
also rejected the input.

## Decision

1. **M35b:** `lower_verified_aeth_to_c` accepts Whole locals with STORE/LOAD and
   Whole arithmetic (sum/difference/product/quotient/remainder). Still single
   total main; still SPEAK/host/foreign/nursery rejected.  
2. Tracker: `native_aeth_to_c_locals_pilot() == true`.  
3. **BARP:** `format_source` uses the same product-prefer gate as structural-edit
   base (ADR-057): if bootstrap fails and product fails, return product
   `AE-SEED-*`.  
4. Tracker: `format_source_product_base_gate() == true`.  

## Honesty

- Not full AETH→C; not bootstrap-free format rewrite.  
- Export-only lib units may still format via bootstrap when product rejects.  

## Links

- ADR-059, ADR-057, M35A-VALIDATION-MATRIX  

---

*End of ADR-062.*
