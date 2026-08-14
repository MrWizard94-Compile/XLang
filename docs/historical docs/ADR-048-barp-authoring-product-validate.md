# ADR-048: BARP — structural-edit accept gate is product seed

**Status:** Accepted — implemented  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `RND-INVAR-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`  
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md)  
**Depends on:** ADR-044, ADR-046  

## Context

`apply_structural_edit` already uses the bootstrap only to **parse** source into a
`Program` for pure in-memory edits (authoring AST). After applying operations it
re-ran **bootstrap** `compile_source` as the accept gate before returning
canonical source. The CLI then product-seed-compiled again before disk write.

That double-gated edits on Rust bootstrap validation even though product
authority is seed forge (ADR-044).

## Decision

1. **Initial parse/canonicalize** of the base source still uses bootstrap
   `compile_source` (authoring AST; not claimed for seed).  
2. **Post-edit accept gate** is **`compile_product_bytecode`** only — the edited
   canonical source must seed-forge and verify.  
3. Structural document JSON is serialized from the **edited in-memory `Program`**
   (no bootstrap re-parse required for accept).  
4. CLI may still product-seed-compile before write (defense in depth).  
5. Companion Phase 3b: product preflight `AE-SEED-004` for missing `weave main`
   (host scan, not bootstrap).  
6. Tracker: `structural_edit_accepts_via_product_seed() == true`.  

## Consequences

### Positive

- Edit accept path aligns with product seed authority  
- Less bootstrap on the authoring→product handoff  

### Costs

- Bootstrap type diagnostics no longer block edit accept; seed/`AE-SEED-*` do  
- Authoring parse remains bootstrap  

### Risks

| Risk | Mitigation |
| --- | --- |
| Invalid AST that seed still emits | verify_bytecode; dual-compare corpus |
| Diagnostic quality | ADR-046 codes + `aether check` hint |

## Links

- ADR-005 authoring, ADR-044, ADR-046, DESIGN-BARP-001  

---

*End of ADR-048.*
