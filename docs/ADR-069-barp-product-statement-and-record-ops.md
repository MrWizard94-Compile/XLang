# ADR-069: BARP — product weave-body statements and primitive records

**Status:** Accepted — implemented  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `DOC-SYNC-001`, `TEST-BEHAVIOR-001`  
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md) / [DESIGN-BARP-001](DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)  
**Depends on:** ADR-065, ADR-068  

## Context

ADR-065/068 moved top-level weave replace/insert/delete off bootstrap base
`Program` parse. Statement-level body edits and top-level record ops still
required bootstrap AST on product-accepted units — residual Rust authority on
common M12 authoring paths.

## Decision

1. **Weave-body statement product path** (when product accepts LF base and
   `baseSource` matches):  
   - `replaceStatement` / `insertStatementAfter` / `deleteStatement` with path
     `weave:<name>/body/<i>`  
   - `insertStatementAt` with list `weave:<name>/body`  
   Statement JSON is formatted via the canonical statement printer (synthetic
   weave) and spliced by body statement span scan (indent-based; `otherwise:`
   continues a choose).  
2. **Top-level primitive record product path:**  
   - `replace` / `delete` on `record:<name>`  
   - `insertAfter` on `world` or `record:<name>` with primitive `Record` payload  
3. Result documents use `aether.product-edit/v1` with path  
   `product-weave-body-statement-ops` or `product-top-level-record-ops`.  
4. **Residual (at ship):** nested body lists were still bootstrap (closed by
   [ADR-071](ADR-071-barp-product-nested-body-list-ops.md)); non-primitive
   declaration types that need record indexes, full `aether.ast/v8`, dual-compare
   oracle, recovery `--bootstrap` remain.  
5. Tracker: `structural_edit_product_statement_and_record_ops() == true`.  

## Consequences

### Positive

- M12 weave-body surgery and simple record inserts no longer require bootstrap
  parse on product units  

### Costs

- Span scan is indent-heuristic (honest, product-accept gated)  
- Nested choose/while statement targets closed later in ADR-071  

## Links

- ADR-016, ADR-065, ADR-068, ADR-071, DESIGN-BARP-001  


---

*End of ADR-069.*
