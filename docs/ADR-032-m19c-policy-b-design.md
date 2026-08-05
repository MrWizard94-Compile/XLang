# ADR-032: nursery Policy B cancel-destroy is design-only

**Status:** Accepted (direction) — **implementation blocked**  
**Date:** 2026-08-04  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `RND-INVAR-001`  
**Related:** ADR-023, ADR-027, ADR-028  

## Decision

1. Adopt [DESIGN-M19C-POLICY-B-CANCEL-DESTROY.md](DESIGN-M19C-POLICY-B-CANCEL-DESTROY.md).  
2. **Do not implement** Policy B product code in this cycle.  
3. Record honest preconditions (resourceful spawn callees / ownership transfer).  
4. Keep M19b Policy A as the product nursery×resource rule.  
5. No free-on-raise.

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
