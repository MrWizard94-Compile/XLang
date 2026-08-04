# ADR-017: bounded offline Language Server (`aether lsp`)

**Status:** Accepted; **M13a implemented** (package 0.17.0); M13b optional  
**Date:** 2026-08-04  
**Decision makers:** AGENTS Constitution; ADR-014 T-LSP after M12  
**Related Rule IDs:** `DOC-ADR-001`, `CONST-DEP-001`, `SEC-INPUT-001`, `DOC-SYNC-001`, `TEST-BEHAVIOR-001`  

## Context

M12 delivered statement-level structural edits. Editors still lack integrated
diagnostics and navigation. Unbounded LSP would risk a second compiler
authority and silent writes. ADR-014 places **T-LSP** after fine-grained edits.

## Decision

1. **Adopt** [DESIGN-M13-BOUNDED-LSP.md](DESIGN-M13-BOUNDED-LSP.md).  
2. Ship **`aether lsp`** (stdio, offline) as a thin host of `aether-core` APIs.  
3. **M13a** minimum: diagnostics, symbols, format, hover, definition (single-file).  
4. **M13b** optional follow-on: project/import-aware navigation.  
5. LSP **must not** emit product AETH or write files; seed remains product compile
   and CLI `apply-edit` authority.  
6. Diagnostics are **bootstrap**-based; document honesty vs seed product path.  
7. No TCP server, network, or model integration in M13.  

## Consequences

### Positive

- Professional editor DX without abandoning local-first law  
- Reuses existing diagnostic/structure/format surfaces  
- Clear authority split (bootstrap editor vs seed product)  

### Costs

- Bootstrap vs seed diagnostic differences visible to users  
- JSON-RPC implementation and editor integration effort  
- Multi-file intelligence deferred or phased  

### Risks

| Risk | Mitigation |
| --- | --- |
| Second compiler | Explicit non-goal; no AETH from LSP |
| Silent writes | INV-004; tests |
| Performance | Debounce; no seed on keystroke |
| Scope creep to full IDE | Matrix limits methods |

## Alternatives considered

| Option | Outcome |
| --- | --- |
| No LSP; CLI only | Rejected for mainstream DX goal |
| Embed rust-analyzer-style full analysis | Rejected (cost; second stack) |
| Seed diagnostics every keystroke | Rejected (latency) |
| TCP multi-user LSP | Rejected (local-first / threat) |

## Implementation gate

1. This ADR remains Accepted  
2. M13 matrix present  
3. Vertical slice: `aether lsp` initialize + didOpen diagnostics for invalid yield type  

## Links

- Design: [DESIGN-M13-BOUNDED-LSP.md](DESIGN-M13-BOUNDED-LSP.md)  
- Matrix: [M13-VALIDATION-MATRIX.md](M13-VALIDATION-MATRIX.md)  
- Prior: [ADR-016](ADR-016-m12-fine-grained-edits.md), [ADR-014](ADR-014-post-m10-track-portfolio.md)  

---

*End of ADR-017.*
