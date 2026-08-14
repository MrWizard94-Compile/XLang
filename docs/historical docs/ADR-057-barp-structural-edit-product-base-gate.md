# ADR-057: BARP — structural-edit product base gate

**Status:** Accepted — implemented  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `TEST-BEHAVIOR-001`  
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md)  
**Depends on:** ADR-048, ADR-055  

## Context

Structural-edit base parse still requires bootstrap `Program` AST (authoring).
Full base-parse independence is not feasible without a seed AST. When **both**
product and bootstrap reject the base source, bootstrap diagnostics were the
only error surface.

## Decision

1. **`canonicalize_source`:** attempt product accept; on bootstrap parse
   failure, if product also failed, return **product** `AE-SEED-*` error.  
2. When product fails but bootstrap accepts (export-only lib units), keep
   bootstrap base parse for authoring AST.  
3. Accept gate remains product seed (ADR-048).  
4. Tracker: `structural_edit_product_base_gate() == true`.  
5. **Honesty:** base AST still bootstrap; product only wins when both reject.  

## Links

- ADR-048, ADR-055  

---

*End of ADR-057.*
