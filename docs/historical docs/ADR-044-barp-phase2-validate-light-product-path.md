# ADR-044: BARP Phase 2 — validate-light product path

**Status:** Accepted — implemented
**Date:** 2026-08-10
**Related Rule IDs:** `CONST-DEP-001`, `RND-INVAR-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `SEC-INPUT-001`
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md) / [DESIGN-BARP-001](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
**Depends on:** ADR-043 Phase 1 complete (seed-native M23; no materialization bridge)

## Context

After BARP Phase 1, product emission is seed forge with native M23 evaluation.
`compile_with_seed` still **bootstrap-validated before forge**, so Rust remained a
product-path validation authority even though emission was already seed-hosted.

Phase 2 goal (DESIGN-BARP-001): product compile may **forge seed first**;
bootstrap is **not required for every CLI product compile**; dual-compare remains
the test oracle. Bootstrap keeps seed rebuild, `check`/AST diagnostics, and
dual-compare proofs.

## Decision

1. **Product bytecode API:** introduce `compile_product_bytecode(source)` that
   forges through `SEED_COMPILER_ARTIFACT` and `verify_bytecode` only — **no**
   bootstrap parse/validate precondition.
2. **CLI default `compile` (non-`--bootstrap`)** and other product emit/write
   gates that only need AETH bytes use that path.
3. **`compile_with_seed`** forges product bytecode first, then bootstrap-parses
   for the returned `Program` AST (tooling / dual-compare callers). Bootstrap
   failure after verified seed emit fails closed as an oracle mismatch.
4. **Do not** claim seed diagnostic parity (Phase 3). CLI `check` / LSP remain
   bootstrap diagnostic authority.
5. **Do not** remove dual-compare tests or verify-before-run/write.
6. Tracker: `product_path_forges_before_bootstrap_validate() == true`.

## Consequences

### Positive

- Product emit authority is clearly seed-first
- Invalid inputs fail at seed forge/verify without bootstrap as a pre-gate
- Dual-compare and `check` roles stay honest

### Costs

- Product-path errors may be seed-opaque vs bootstrap codes
- `compile_with_seed` still pays bootstrap parse when callers need `Program`

### Risks

| Risk | Mitigation |
| --- | --- |
| Seed accepts bootstrap-invalid source | `verify_bytecode`; dual-compare corpus; fail-closed mismatch on `compile_with_seed` |
| Silent loss of dual-compare | Forbidden; modules/project dual-compare unchanged |
| Scope creep to full seed diagnostics | Phase 3 separate ADR |

## Implementation gate

1. This ADR Accepted
2. DESIGN-BARP-001 Phase 2 marked complete
3. Product path + dual-compare green
4. DOC-SYNC SEED_PROFILE / CLM-040 / MANIFEST / AGENTS

## Links

- DESIGN-BARP-001 § Phase 2
- ADR-043 program law
- SEED_PROFILE product compile claims

---

*End of ADR-044.*
