# ADR-049: BARP — product-authoritative `compile_with_seed` + product seed rebuild identity

**Status:** Accepted — implemented  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `RND-INVAR-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`  
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md)  
**Depends on:** ADR-044, ADR-047, ADR-048  

## Context

`compile_with_seed` already forged product bytes first (ADR-044) but still
**failed closed** when bootstrap `compile_source` rejected the same source after
seed success (“oracle mismatch”). That re-introduced bootstrap as a product
success gate for any caller using `compile_with_seed`.

Independence proof: the product path can rebuild `seed/aether_seed.aeth` from
`seed/aether_seed.ae` byte-identically (ADR-048 test). The release gate still
only checked bootstrap compile + forge, not the product path rebuild.

## Decision

1. **`compile_with_seed` product success = product bytecode only**
   (`compile_product_bytecode`). Bootstrap parse no longer fails the product
   call after seed verification.  
2. **Bootstrap `Program` is best-effort tooling fill:** when bootstrap accepts,
   `CompileOutput.program` is the bootstrap AST; when it rejects after product
   success, `program` is an empty placeholder (`world` empty, no weaves) and
   **must not** be treated as a semantic document — use `compile_source` /
   `check` for AST/diagnostics.  
3. **Prefer `compile_product_bytecode`** for product-only call sites.  
4. **`aether-gate` full/release** seed identity: bootstrap ≡ **product** ≡
   forged ≡ checked-in.  
5. Tracker: `compile_with_seed_product_authoritative() == true`.  

## Consequences

### Positive

- Product emit never blocked by bootstrap after seed verify  
- Gate proves product seed self-rebuild identity  
- Clearer product vs tooling AST  

### Costs

- Empty placeholder Program if bootstrap rejects after product (rare for corpus)  
- Callers must not assume `compile_with_seed().program` is always meaningful  

### Risks

| Risk | Mitigation |
| --- | --- |
| Misuse of empty Program | Docs + tracker; dual-compare tests use separate bootstrap |
| Silent product/bootstrap drift | Gate dual-compare + seed identity |

## Links

- DESIGN-BARP-001, ADR-044, ADR-048  

---

*End of ADR-049.*
