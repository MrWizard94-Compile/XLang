# ADR-071: BARP — product nested choose/while body-list structural ops

**Status:** Accepted — implemented  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `DOC-SYNC-001`, `TEST-BEHAVIOR-001`  
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md) / [DESIGN-BARP-001](DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)  
**Depends on:** ADR-065, ADR-068, ADR-069  
**Roadmap:** nested body-list product structural paths (post-ADR-070 residual)

## Context

ADR-069 moved weave-body statement ops and primitive records onto the product
text-splice path. Nested body lists still required bootstrap `Program` parse:

- `weave:<name>/body/<i>/whenBright` / `whenDim` (choose branches)  
- `weave:<name>/body/<i>/body` (while body)  

That residual forced recovery bootstrap for common M12 control-flow authoring
(insert/replace/delete inside choose/while), contradicting the BARP goal that
product-accepted units should not need bootstrap base AST for structural
statement surgery.

## Decision

1. **Product nested body-list path** when product accepts LF base and
   `baseSource` matches:  
   - `replaceStatement` / `insertStatementAfter` / `deleteStatement` with path  
     `weave:<name>/body/<outer>/whenBright|whenDim|body/<inner>`  
   - `insertStatementAt` with list  
     `weave:<name>/body/<outer>/whenBright|whenDim|body`  
2. **Text splice** uses the same indent-based statement span scan as weave-body
   ops, scoped to the outer choose/while statement region. Nested statement
   text is formatted at weave-body indent + 2 spaces.  
3. Result documents use `aether.product-edit/v1` with path
   `product-nested-body-list-ops` (no `program` AST tree).  
4. **Product accept** remains the post-edit gate (`compile_product_bytecode`).  
5. **Residual bootstrap:** dual-compare oracle, recovery `--bootstrap` flags,
   full `aether.ast/v8`, non-primitive declaration types that need bootstrap
   record indexes. Nested body lists are **no longer** residual.  
6. Tracker: `structural_edit_product_nested_body_list_ops() == true`.  

## Consequences

### Positive

- Choose/while statement-level M12 surgery no longer requires bootstrap AST on
  product-accepted units  
- Roadmap residual “nested body-list product structural paths” closed  

### Costs

- One-level nesting only (protocol already limits paths to one outer index)  
- Indent-heuristic spans remain honest and product-accept gated  

## Links

- ADR-016, ADR-069, ADR-070, DESIGN-BARP-001, ROADMAP  

---

*End of ADR-071.*
