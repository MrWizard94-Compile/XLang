# ADR-052: BARP — product-seed lib validation + Phase 3c diagnostic classification

**Status:** Accepted — implemented
**Date:** 2026-08-10
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `DOC-SYNC-001`, `TEST-BEHAVIOR-001`
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md) / [DESIGN-BARP-001](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
**Depends on:** ADR-047, ADR-050, ADR-051

## Context

Project verify validated standalone lib units by bootstrap-compiling a synthetic
`main` probe (`validate_lib_module_source` → `compile_to_bytecode`). That was
residual bootstrap authority on a product-adjacent path after ADR-047 removed
bootstrap from multi-module **emit**.

Product forge/verify failures were mapped only to `AE-SEED-001` / `AE-SEED-002`,
even when the detail string already carried usable signals (unknown weave,
type/stack mismatch, opaque seed VM failure on bad bind). DESIGN-BARP Phase 3
still listed deeper seed-side classification as later work — without claiming
full bootstrap diagnostic parity.

## Decision

1. **Lib unit validation uses product seed:**
   `validate_lib_module_source` probes with [`compile_product_bytecode`] on the
   synthetic main source. No bootstrap parse/emit on project-verify lib checks.
2. **Phase 3c classification (host mapping of product failure detail; not seed
   parity):**
   | Code | When |
   | --- | --- |
   | `AE-SEED-008` | Seed forge aborts with opaque VM/seed-internal failure (e.g. `unpack16 index is invalid`) — typical incomplete bind/parse |
   | `AE-SEED-010` | Product verify reports type/stack mismatch (e.g. yield Whole vs Text) |
   | `AE-SEED-011` | Unknown weave / call target in forge or verify detail |
   | `AE-SEED-001` / `002` | Remaining generic forge / verify failures |
3. **Do not claim** full bootstrap diagnostic parity or structured seed error ABI.
4. Bootstrap remains default `check`/format/structure/LSP AST, `--bootstrap`
   rebuild, dual-compare oracle.
5. Trackers:
   - `lib_module_validates_via_product_seed() == true`
   - `seed_product_diagnostics_phase3c() == true`

## Consequences

### Positive

- Project verify lib path is seed-only
- Product failures surface more specific stable codes without bootstrap

### Costs / honesty

- Classification is detail-string heuristics over forge/VM messages
- Opaque seed failures remain less precise than bootstrap `check`

## Links

- ADR-043, ADR-047, ADR-050, DESIGN-BARP-001, SEED_PROFILE

---

*End of ADR-052.*
