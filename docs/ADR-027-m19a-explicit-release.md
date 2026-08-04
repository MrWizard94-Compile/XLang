# ADR-027: explicit `release` end-of-life (M19a)

**Status:** Accepted — **implemented in 0.25.0** (bootstrap + seed dual-compare)  
**Date:** 2026-08-04  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `RND-INVAR-001`  
**Supersedes for foundation:** ADR-023 “next is release” — this is that slice  

## Context

ADR-023 forbids deeper T-RX until explicit destruction exists. M1 noted M2 had
no source `destroy`; M19a deliberately adds a **bounded** pure `release` without
user destructors.

## Decision

1. Adopt [DESIGN-M19A-EXPLICIT-RELEASE.md](DESIGN-M19A-EXPLICIT-RELEASE.md).  
2. Add root statement `release <name>` + AETH `RELEASE` (66).  
3. Relax weave-level abortive×resource ban; keep **site-level** clean boundary.  
4. Require seed dual-compare for release corpus.  
5. Package **0.25.0**.  
6. Still **do not** implement nursery×resource or free-on-raise.

## Consequences

### Positive

- Unlocks raise-after-cleanup proofs  
- Foundation for later M19b nursery policy  

### Costs

- New opcode + seed emission  
- Authors must release before abortive sites  

### Risks

| Risk | Mitigation |
| --- | --- |
| Arena released under live buffer | Static reject |
| Seed/bootstrap drift | Dual-compare mandatory |

## Links

- DESIGN-M19A, M19A matrix, ADR-023, ADR-020  

---

*End of ADR-027.*
