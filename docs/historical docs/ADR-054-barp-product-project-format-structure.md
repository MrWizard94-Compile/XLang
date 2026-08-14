# ADR-054: BARP — product project format + product structure envelope

**Status:** Accepted — implemented
**Date:** 2026-08-10
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `DOC-SYNC-001`, `TEST-BEHAVIOR-001`
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md) / [DESIGN-BARP-001](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
**Depends on:** ADR-051, ADR-053

## Context

ADR-053 added single-file `format --product` (LF + seed accept). Multi-unit
`project format` still always used bootstrap `format_source` per unit.

CLI `structure` always required bootstrap AST (`aether.ast/v8`). Operators had
no product-only structure surface parallel to `check --product` /
`format --product`.

## Decision

1. **`format_project(..., product: bool)`:** when `product`, each unit uses
   [`format_source_product`] (no bootstrap). Default remains bootstrap
   `format_source`.
2. **CLI `aether project format --product [--write]`:** product multi-unit
   format path.
3. **`product_structure_json(source)`:** product accept + LF normalize; emit
   bounded JSON schema **`aether.product-structure/v1`** with
   `productAccepted`, `artifactBytes`, and `source` — **not** `aether.ast/v8`.
4. **CLI `aether structure --product <source>`:** uses product envelope only.
   Default `structure` remains bootstrap `aether.ast/v8`.
5. Trackers:
   - `product_project_format_without_bootstrap() == true`
   - `product_structure_without_bootstrap() == true`

## Consequences

### Positive

- Multi-unit product format without bootstrap AST
- Product structure/accept probe without bootstrap AST

### Costs / honesty

- Product structure is **not** full semantic AST
- Product project format is **not** full `format_program` rewrite

## Links

- ADR-053, DESIGN-BARP-001, SEED_PROFILE

---

*End of ADR-054.*
