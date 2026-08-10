# ADR-046: BARP Phase 3a — bounded product-path diagnostics

**Status:** Accepted — implemented (vertical slice)  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `SEC-INPUT-001`  
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md) / [DESIGN-BARP-001](DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)  
**Depends on:** ADR-044  

## Context

After Phase 2, invalid product compile often surfaces opaque forge/VM messages
(e.g. `unpack16 index is invalid`) while CLI `check` (bootstrap) already has
precise AE codes. DESIGN-BARP-001 Phase 3 asks for a **bounded seed diagnostic
subset** without claiming full parity.

## Decision

1. **Phase 3a (this ADR):** product-path failures map to stable **`AE-SEED-*`**
   messages on the host after seed forge/verify failure, plus an explicit hint
   to run `aether check` for full bootstrap diagnostics.  
2. **Seed fail-closed slice:** reject **odd indentation** (not a multiple of two
   spaces) by refusing to emit a valid product artifact (fail closed).  
3. **Bootstrap remains full diagnostic authority** for `check` / LSP / structure.  
4. Tracker: `seed_product_diagnostics_subset() == true` for the Phase 3a surface.  
5. Further seed-side AE codes (unbound names, type errors, …) need later ADR
   increments; do not claim parity.

### Phase 3a codes (product path)

| Code | When |
| --- | --- |
| `AE-SEED-001` | Seed forge failed (VM/runtime while interpreting guest source) |
| `AE-SEED-002` | Seed produced bytes that failed `verify_bytecode` |
| `AE-SEED-003` | Seed rejected non-canonical indentation (odd space count) |
| `AE-SEED-004` | Phase 3b: missing top-level `weave main` / `task weave main` (host preflight) |
| hint | Always include: use `aether check <source>` for bootstrap diagnostics |

## Consequences

### Positive

- Product failures become stable and actionable without restoring bootstrap pre-gate  
- Indent honesty: product path no longer silently accepts one-space bodies  

### Costs

- Still not full diagnostic parity  
- Host mapping depends on forge error text / seed cooperation for indent  

### Risks

| Risk | Mitigation |
| --- | --- |
| Over-claim parity | Explicit Phase 3a bound; bootstrap check remains canonical |
| False AE-SEED codes | Prefer coarse buckets; keep original detail in message |

## Links

- DESIGN-BARP-001 § Phase 3, ADR-045  

---

*End of ADR-046.*
