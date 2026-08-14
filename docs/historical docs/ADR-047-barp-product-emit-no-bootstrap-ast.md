# ADR-047: BARP — multi-module product emit without bootstrap AST

**Status:** Accepted — implemented  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `RND-INVAR-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`  
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md)  
**Depends on:** ADR-044, ADR-045  

## Context

After ADR-045, multi-module product build no longer dual-compare-gated on
bootstrap, but still called `compile_with_seed`, which **bootstrap-parses** the
elaborated source solely to fill `CompileOutput.program`. CLI project/workspace
build and project tests use only **bytecode**. That residual bootstrap parse was
product-path authority without product value.

## Decision

1. Multi-module product emit (`compile_project_*` / workspace package build)
   uses **`compile_product_bytecode` only** — zero bootstrap compiler invocation.  
2. Return type for those product APIs is **`Vec<u8>`** (verified AETH bytes), not
   `CompileOutput` with a bootstrap `Program`.  
3. Callers that need AST still use bootstrap (`compile_source` / `check` /
   structure / `compile_with_seed` for tooling dual-compare helpers).  
4. Dual-compare remains in unit tests (elaborate → bootstrap vs product bytes).  
5. Tracker: `product_multi_module_invokes_bootstrap() == false`.  

## Consequences

### Positive

- Project/workspace product build never runs Rust parse/validate/emit  
- Clearer product vs tooling API split  

### Costs

- Callers needing `Program` must use a separate bootstrap API  
- API type change for module/workspace compile functions  

### Risks

| Risk | Mitigation |
| --- | --- |
| Silent API misuse | Compile-time type change; docs |
| Drift | Dual-compare tests + aether-gate |

## Links

- DESIGN-BARP-001, ADR-045, ADR-046  

---

*End of ADR-047.*
