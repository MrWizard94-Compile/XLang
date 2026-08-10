# ADR-051: BARP — bootstrap-free `compile_with_seed` + product CLI check

**Status:** Accepted — implemented  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `DOC-SYNC-001`, `TEST-BEHAVIOR-001`  
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md) / [DESIGN-BARP-001](DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)  
**Depends on:** ADR-049, ADR-050  

## Context

ADR-049 made product success seed-authoritative, but `compile_with_seed` still
invoked bootstrap `compile_source` on every call for a best-effort `Program`
fill. No product caller used that AST; dual-compare and `check` already use
`compile_source` / `compile_to_bytecode` explicitly.

Default CLI `check` remains bootstrap-only full diagnostics. There was no
first-class product-path “does seed accept this source?” CLI surface without
writing an artifact via `compile`.

## Decision

1. **`compile_with_seed` never invokes bootstrap.** After
   [`compile_product_bytecode`] succeeds, `CompileOutput::program` is always the
   empty placeholder. Callers that need a semantic AST use `compile_source` /
   CLI `check` (bootstrap diagnostic authority). Callers that need only bytes
   should prefer `compile_product_bytecode`.  
2. **CLI `aether check --product <source>`** validates via
   `compile_product_bytecode` only (forge + verify + AE-SEED preflights). On
   success it reports product acceptance without AST dump. Default
   `aether check <source>` remains bootstrap full diagnostics + canonical AST.  
3. **Do not** remove dual-compare tests, verify-before-run, or seed recovery
   `--bootstrap` rebuild.  
4. Trackers:  
   - `compile_with_seed_invokes_bootstrap() == false`  
   - `product_cli_check_without_bootstrap() == true`  

## Consequences

### Positive

- Product API path is seed-only end-to-end (no hidden bootstrap tax)  
- Operators can product-validate sources without bootstrap diagnostics  
- Clear separation: product acceptance vs full diagnostic authority  

### Costs / honesty

- `CompileOutput.program` from `compile_with_seed` is never semantic  
- Product check does not print AST or bootstrap AE codes  

### Risks

| Risk | Mitigation |
| --- | --- |
| Silent AST consumers | No production use of `compile_with_seed().program`; docs + tracker |
| Product check over-claim | Message states product path; points to default check for full diagnostics |

## Links

- ADR-043, ADR-049, DESIGN-BARP-001, SEED_PROFILE, MANIFEST  

---

*End of ADR-051.*
