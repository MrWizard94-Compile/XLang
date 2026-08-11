# ADR-067: BARP — product seed rebuild without `--bootstrap`

**Status:** Accepted — implemented  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `DOC-SYNC-001`, `TEST-BEHAVIOR-001`  
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md) / [DESIGN-BARP-001](DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)  
**Depends on:** ADR-049, ADR-051, ADR-064  

## Context

Default `aether compile` already forges via `compile_product_bytecode` (no
bootstrap). Gate dual-compare proved product ≡ bootstrap ≡ forged ≡ checked-in
seed (ADR-049). Docs and residual BARP lists still described seed rebuild as
requiring `compile --bootstrap`, overstating Rust authority.

## Decision

1. **Product is the seed rebuild path:**  
   `aether compile seed/aether_seed.ae --output <artifact>` (no `--bootstrap`)
   is the supported way to rebuild seed artifacts.  
2. **`--bootstrap` remains oracle/recovery emit** for dual-compare proofs and
   bootstrap diagnostics — not required for product seed identity.  
3. Gate continues dual-compare (bootstrap + product + forge + checked-in).  
4. Tracker: `product_seed_rebuild_without_bootstrap() == true`.  
5. Update honesty trackers/docs so residual bootstrap no longer lists “seed
   rebuild” as an exclusive bootstrap role.  

## Consequences

### Positive

- Honest independence claim: Aether seed rebuilds Aether seed without Rust
  compiler on the product path  
- Operators use one compile command for seed and examples  

### Costs / residual

- Dual-compare still needs bootstrap emit for proof  
- Checked-in seed pin still verified against both paths in full gate  

## Links

- ADR-049, ADR-051, ADR-064, `tools/aether-gate.ps1`  

---

*End of ADR-067.*
