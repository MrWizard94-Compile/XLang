# ADR-045: BARP — product dual-compare is oracle-only (not a product gate)

**Status:** Accepted — implemented
**Date:** 2026-08-10
**Related Rule IDs:** `CONST-DEP-001`, `RND-INVAR-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md) / [DESIGN-BARP-001](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
**Depends on:** ADR-044 (forge-first product bytecode)

## Context

ADR-044 made single-file product compile forge-first without bootstrap
pre-validate. Multi-module **project/workspace product build** still ran
bootstrap `compile_to_bytecode` on every build and required byte identity
before returning (M11b dual-compare as a **product gate**).

That kept substantial Rust bootstrap authority on the default
`project build` / `workspace build` / project-test compile path, contrary to
DESIGN-BARP-001's Phase 2 intent that dual-compare is a **test/oracle** role.

## Decision

1. **Product multi-module compile** elaborates on the host, then emits via
   seed product path (`compile_with_seed` / product bytecode) **without** a
   bootstrap dual-compare gate.
2. **Dual-compare remains mandatory** in self-host / module unit tests and in
   `aether-gate` example dual-compare — bootstrap stays the proof oracle.
3. Tracker: `product_path_requires_bootstrap_dual_compare() == false`.
4. **Do not** remove dual-compare tests or weaken verify-before-run/write.
5. Companion: [ADR-046](ADR-046-barp-phase3a-product-diagnostics.md) for bounded
   product-path diagnostic messages (Phase 3a).

## Consequences

### Positive

- Project/workspace product builds no longer require bootstrap emission
- Clearer BARP ladder: seed emit vs bootstrap oracle

### Costs

- Drift between seed and bootstrap would ship until tests/gate catch it
- Tests and release gate remain the safety net

### Risks

| Risk | Mitigation |
| --- | --- |
| Silent seed/bootstrap drift | Dual-compare tests + aether-gate example dual-compare |
| Scope creep to drop dual-compare entirely | Forbidden by ADR-043 / DESIGN-BARP-001 |

## Links

- DESIGN-BARP-001, ADR-043, ADR-044, ADR-046

---

*End of ADR-045.*
