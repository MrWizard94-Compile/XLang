# ADR-068: BARP — product top-level weave insertAfter and delete

**Status:** Accepted — implemented  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `DOC-SYNC-001`, `TEST-BEHAVIOR-001`  
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md) / [DESIGN-BARP-001](DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)  
**Depends on:** ADR-065  

## Context

ADR-065 moved weave **replace** off the bootstrap base `Program` parse. Insert
and delete of top-level weaves still required bootstrap AST even when product
already accepted the unit — residual Rust authority on common authoring ops.

## Decision

1. Expand the product text-splice path to all **top-level weave** ops on
   product-accepted LF base source with matching `baseSource`:  
   - `replace` (ADR-065)  
   - `insertAfter` (after a named weave)  
   - `delete` (named weave)  
2. Declaration payloads remain JSON → Weave (primitive-typed product subset);
   record/Buffer/shape-typed declarations fall back to bootstrap.  
3. Result document remains `aether.product-edit/v1` with  
   `path: "product-top-level-weave-ops"`.  
4. **Residual bootstrap:** statement-level body edits, record insert/delete/replace,
   full `aether.ast/v8`, dual-compare oracle, recovery `--bootstrap`.  
5. Trackers:  
   - `structural_edit_product_top_level_weave_ops() == true`  
   - `structural_edit_product_weave_replace() == true` (still true; replace is a subset)  

## Consequences

### Positive

- Insert/delete helper weaves no longer need bootstrap parse on product units  
- Clear residual boundary: statement/record AST still bootstrap  

### Costs

- Spacing after delete/insert is best-effort; product accept is the gate  
- Non-LF `baseSource` (bootstrap-canonical-only) still falls back  

## Links

- ADR-065, DESIGN-BARP-001  

---

*End of ADR-068.*
