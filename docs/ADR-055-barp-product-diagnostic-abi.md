# ADR-055: BARP — structured product diagnostic ABI

**Status:** Accepted — implemented (host-facing product surface)  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`  
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md) / [DESIGN-BARP-001](DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)  

## Context

Product failures used stable `AE-SEED-*` strings but lacked a first-class
collection API for tooling (LSP, gates). Seed-internal structured error packets
(seed binary emits typed error envelopes) remain future work — forge still fails
via VM/seed aborts on many invalid inputs, classified by host heuristics.

## Decision

1. **`product_diagnostics(source) -> Vec<Diagnostic>`:** structured product
   diagnostic collection; empty on product accept; one diagnostic with stable
   `AE-SEED-*` code on failure.  
2. **`AE-SEED-012`:** raw `import unit` on single-file product path (points to
   host elaborate + seed emit / `project build`).  
3. **Honesty:** this is the **host-facing product diagnostic ABI**, not a claim
   that every seed-internal failure path emits structured packets. Opaque VM
   failures remain classified (`AE-SEED-008` / `001`).  
4. Tracker: `product_diagnostic_abi() == true`.  

## Links

- ADR-046, ADR-052, ADR-056, ADR-058  

---

*End of ADR-055.*
