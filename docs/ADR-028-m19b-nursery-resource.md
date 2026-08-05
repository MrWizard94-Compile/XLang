# ADR-028: nursery × resource Policy A (M19b)

**Status:** Accepted — **implemented in 0.26.0**  
**Date:** 2026-08-04  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `RND-INVAR-001`  
**Supersedes for this slice:** ADR-023 “nursery+resource deferred” — only for Policy A  

## Context

ADR-023 deferred nursery×resource until a destruction model existed. M19a
shipped `release`. M7 still forbade any resource+nursery weave mix, which is
stronger than cancel-safety requires when spawn callees are pure.

## Decision

1. Adopt [DESIGN-M19B-NURSERY-RESOURCE.md](DESIGN-M19B-NURSERY-RESOURCE.md) Policy A.  
2. Allow parent resource ownership with `together` when spawn callees are
   resource-free and spawn args stay non-resource.  
3. Nursery site boundary = no live `access` loans (not full effect clean).  
4. Keep full clean boundary for abortive raise/forward.  
5. Package **0.26.0**; dual-compare mix corpus.  
6. Do **not** implement Policy B cancel-destroy or free-on-raise.

## Consequences

### Positive

- Total mains may hold arenas while joining pure nursery work  
- Aligns T-RX with M19a without new opcodes  

### Costs

- Callee resource scan on each spawn  
- Pure M7 model tests updated  

### Risks

| Risk | Mitigation |
| --- | --- |
| Resourceful spawn callee admitted | Static `weave_uses_resource` on callee name |
| Access loan spans nursery | Nursery boundary rejects AccessArena |
| Claim Policy B | Explicit non-goal |

## Links

- DESIGN-M19B, M19B matrix, ADR-023, ADR-027, ADR-010  

---

*End of ADR-028.*
