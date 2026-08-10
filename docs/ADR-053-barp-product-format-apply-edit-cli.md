# ADR-053: BARP — product format path + apply-edit CLI trusts product accept

**Status:** Accepted — implemented  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `DOC-SYNC-001`, `TEST-BEHAVIOR-001`  
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md) / [DESIGN-BARP-001](DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)  
**Depends on:** ADR-048, ADR-051, ADR-052  

## Context

Default `aether format` always used bootstrap `compile_source` + `format_program`
for full AST-canonical rewrite. Operators already had product-only **check**
(`check --product`) but no product-only format/normalize surface.

CLI `apply-edit` re-ran `compile_product_bytecode` after
`apply_structural_edit`, which **already** product-seed-accepts (ADR-048). That
second forge was residual product work and obscured that product authority lives
in core.

## Decision

1. **`format_source_product`:** LF/CRLF normalize + [`compile_product_bytecode`]
   accept only. Does **not** invoke bootstrap and does **not** claim full
   AST-canonical rewrite (default `format_source` remains bootstrap authority).  
2. **CLI `aether format --product`:** uses `format_source_product`. Optional
   `--output` unchanged. Default `format` remains bootstrap canonical.  
3. **CLI `apply-edit`:** trust core product accept; **do not** re-forge after
   `apply_structural_edit` success.  
4. Trackers:  
   - `product_format_without_bootstrap() == true`  
   - `apply_edit_cli_trusts_product_accept() == true`  

## Consequences

### Positive

- Product-only normalize+accept without bootstrap AST  
- Apply-edit write path no longer double-forges  
- Clearer product vs authoring format authority  

### Costs / honesty

- Product format is **not** full `format_program` rewrite  
- Base parse for structural edits remains bootstrap  

## Links

- ADR-048, ADR-051, DESIGN-BARP-001, SEED_PROFILE  

---

*End of ADR-053.*
