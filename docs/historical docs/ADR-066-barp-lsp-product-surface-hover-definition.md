# ADR-066: BARP — LSP product-surface hover and definition

**Status:** Accepted — implemented
**Date:** 2026-08-10
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `DOC-SYNC-001`, `TEST-BEHAVIOR-001`
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md) / [DESIGN-BARP-001](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
**Depends on:** ADR-058, ADR-063, ADR-064

## Context

LSP diagnostics and document symbols already preferred the product path
(ADR-058/063), but Plain-symbol **hover** and **definition** still invoked
bootstrap `compile_source` / `structural_document_json` for local weaves and
records.

## Decision

1. **Hover (Plain):** resolve via [`product_surface_symbols`] when product
   accepts; report weave/record/world as product-surface markdown.
2. **Definition (Plain):** jump to product-surface line/column for matching
   local name; no bootstrap AST.
3. **Qualified import navigation** remains project-aware text/export scan
   (unchanged; no bootstrap).
4. **Recovery:** if product-surface trackers are disabled, fall back to
   bootstrap AST (tests / future flag off).
5. Tracker: `lsp_product_surface_hover_definition() == true`.

## Consequences

### Positive

- Local navigation matches product-accept authority
- No bootstrap on the default hover/definition path for product units

### Costs / residual

- No effect-type detail on hover (product surface has no effect field)
- Product-rejecting buffers fall back to plaintext / null (not bootstrap
  recovery unless trackers off)

## Links

- ADR-058, ADR-063, DESIGN-M13-BOUNDED-LSP

---

*End of ADR-066.*
