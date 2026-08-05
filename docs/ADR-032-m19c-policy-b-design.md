# ADR-032: nursery Policy B cancel-destroy (historical design gate)

**Status:** Accepted (direction) — **superseded for cooperative product by ADR-036 (0.32)**  
**Date:** 2026-08-04  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `RND-INVAR-001`  
**Related:** ADR-023, ADR-027, ADR-028, ADR-035, ADR-036  

## Decision

1. Adopt [DESIGN-M19C-POLICY-B-CANCEL-DESTROY.md](DESIGN-M19C-POLICY-B-CANCEL-DESTROY.md).  
2. Historical: do **not** implement general free-on-cancel product theater.  
3. Record honest preconditions (resourceful spawn callees / ownership transfer).  
4. Keep M19b Policy A until M19d Policy A+.  
5. No free-on-raise.  
6. **Follow-on:** ADR-035 admits multi-weave arenas; ADR-036 claims only
   **cooperative** Policy B (unstarted cancel / return-end). Mid-frame cancel
   remains out of product scope.

## Rationale

Constitution forbids claiming Done for a safety feature with no admissible
programs and no dual-compare corpus. Policy B needs an M2/nursery ownership
precondition ADR before code.

## Consequences

### Positive

- Clear stop-ship against fake cancel-destroy  
- Guides next T-RX research without weakening M19b  

### Costs

- Resourceful spawn+cancel remains unavailable  

## Links

- DESIGN-M19C, M19C matrix (design gate only)

---

*End of ADR-032.*
