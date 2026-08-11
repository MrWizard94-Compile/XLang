# ADR-070: Product fail-closed yield-in-truth-choose + multi-module choose-revise

**Status:** Accepted — implemented  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `RND-INVAR-001`  
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md) / [DESIGN-BARP-001](DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)  
**Depends on:** ADR-046–055, ADR-056  
**Source:** Showcase discovery (`showcases/aether-ledger`)

## Context

While building the Aether-only multi-package showcase, multi-module product
builds failed with opaque `AE-SEED-002` jump-target verifier errors on programs
that used:

```aether
choose same x 7:
  yield 42
otherwise:
  yield -1
```

Bootstrap correctly rejects this as illegal (`yield` only at weave root for
truth-condition chooses; resource `choose allocate|append|…` may still yield).
The product seed path **forged broken bytecode** instead of rejecting — forcing
operators toward bootstrap diagnostics and hiding the real language rule.

Separately, multi-module product **does** support truth-`choose` when written
as revise-then-root-yield:

```aether
bind mutable code <- 1
choose same x 7:
  revise code <- 0
yield code
```

That pattern dual-compares seed≡bootstrap for multi-module projects.

## Decision

1. **Host preflight `AE-SEED-013`:** before seed forge, product path rejects
   `yield` nested under a truth-condition `choose` (`same` / `less` / bare
   truth name). Resource choose ops remain allowed to yield.  
2. **No bootstrap AST** for this reject — line/indent scanner only.  
3. **Residual classify:** jump-target / unreachable-instruction verify failures
   still map to `AE-SEED-013` if preflight is bypassed.  
4. **Document multi-module choose-revise** as the supported product pattern
   (tracker `multi_module_product_choose_revise_supported()`).  
5. **Showcase updated** to use legal multi-module control and document the rule.  

## Consequences

### Positive

- Product path no longer emits hostile AETH for a common illegal form  
- Multi-module independence for legal choose-revise is proven  
- Clear AE-SEED-013 + `aether check --bootstrap` hint  

### Costs / residual

- Preflight is host-side (not seed-internal packets; ADR-061 still open)  
- Nested structural body lists still residual bootstrap  

## Links

- Showcase: `showcases/aether-ledger/`  
- ADR-056 multi-module host elaborate + seed emit  

---

*End of ADR-070.*
