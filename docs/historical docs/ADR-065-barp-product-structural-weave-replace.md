# ADR-065: BARP — product structural weave replace without bootstrap base AST

**Status:** Accepted — implemented
**Date:** 2026-08-10
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `DOC-SYNC-001`, `TEST-BEHAVIOR-001`
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md) / [DESIGN-BARP-001](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
**Depends on:** ADR-048, ADR-057, ADR-064

## Context

`apply_structural_edit` always base-parsed via bootstrap `compile_source` to
obtain a `Program` AST, even when the only operation was a top-level weave
`replace` and product already accepted the base source. That kept Rust bootstrap
on the happy path for the most common structural edit.

## Decision

1. **Product weave-replace path:** when
   - product seed accepts the LF-normalized base source,
   - `baseSource` equals that LF source, and
   - every operation is top-level weave `replace`,
   apply the edit by parsing declaration JSON → format weave text → splice by
   top-level weave span — **without** bootstrap `Program` base parse.
2. **Accept gate remains product seed** (ADR-048).
3. **Result document** for this path is `aether.product-edit/v1` (canonical
   source + productAccepted), not full `aether.ast/v8`.
4. **Fallback:** insert/delete/statement/record ops, product-rejecting base units
   (e.g. export-only libs), baseSource that is bootstrap-canonical-only, or
   declaration payloads that need record/type indexes → existing bootstrap AST
   path.
5. Tracker: `structural_edit_product_weave_replace() == true`.

## Consequences

### Positive

- Common weave body replacements no longer require bootstrap parse authority
- Clear product-edit schema for bootstrap-free results

### Costs / residual

- Full AST document and non-replace ops still use bootstrap
- Product path does not reformat whole programs (`format_program`); splice preserves surrounding text

## Links

- ADR-048, ADR-057, ADR-064, DESIGN-BARP-001

---

*End of ADR-065.*
