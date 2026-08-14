# ADR-064: Product-default CLI toolchain — bootstrap recovery/oracle only

**Status:** Accepted — implemented
**Date:** 2026-08-10
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `DOC-SYNC-001`, `TEST-BEHAVIOR-001`
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md) / [DESIGN-BARP-001](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
**Depends on:** ADR-051–063

## Context

Product seed already owns compile, diagnostics, and many CLI surfaces, but
**defaults** still used bootstrap for check/format/structure success paths. Human
direction (2026-08-10): finish eliminating bootstrap authority on the product
path.

Full deletion of the Rust bootstrap is **not** honest: seed rebuild still needs
`compile --bootstrap` (or an equivalent non-product path), dual-compare remains
the proof oracle, and structural-edit / LSP hover still need a `Program` AST.

## Decision

1. **Product is the default user-facing toolchain** for:
   - `aether check` → seed product path
   - `aether format` → LF normalize + product accept
   - `aether structure` → `aether.product-structure/v1`
   - `aether project format` → product unit format
   - LSP formatting → product format
2. **Bootstrap recovery flags:**
   - `check --bootstrap` → full bootstrap AST diagnostics
   - `format --bootstrap` → AST-canonical rewrite
   - `structure --bootstrap` → `aether.ast/v8`
   - `project format --bootstrap` → AST format per unit
   - `compile --bootstrap` → seed rebuild / dual-compare emit (unchanged)
3. Legacy `--product` remains accepted as a no-op synonym for the product default.
4. Trackers:
   - `product_default_cli_toolchain() == true`
   - `bootstrap_is_recovery_oracle_only() == true`
5. **Residual bootstrap (honest, not eliminated):** seed rebuild, dual-compare
   tests/gate, structural-edit base parse AST, LSP hover/definition AST.

## Consequences

### Positive

- Operators never hit bootstrap for normal check/format/structure
- Clear recovery story when full AST diagnostics are needed

### Costs

- Default format is **not** full `format_program` rewrite (use `--bootstrap`)
- Default structure is **not** full authoring AST

## Links

- ADR-051–063, DESIGN-BARP-001, SEED_PROFILE

---

*End of ADR-064.*
