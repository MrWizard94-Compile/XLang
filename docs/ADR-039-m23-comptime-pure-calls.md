# ADR-039: pure comptime weave calls (M23 / T-CT)

**Status:** Accepted (implementable) — **not yet product-implemented**  
**Date:** 2026-08-05  
**Decision makers:** Human ordered backlog (T-CT first); AGENTS Constitution  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `SEC-INPUT-001`, `RND-INVAR-001`, `CONST-COMPLETE-001`  
**Portfolio:** ADR-014 **T-CT**

## Context

M5 + M15 prove pure Whole arithmetic and name chaining at compile time. The next
honest T-CT step is **factoring** compile-time formulas into helper weaves without
opening host I/O, effects, resources, or general control-flow metaprogramming.

## Decision

1. **Adopt** [DESIGN-M23-COMPTIME-PURE-CALLS.md](DESIGN-M23-COMPTIME-PURE-CALLS.md).  
2. Allow `comptime bind name <- call W args…` where `W` is a **total guest**
   weave with Whole-only parameters/result and an M23-restricted body (no
   nested calls, no choose/while/nursery/handle/raise/resources/host).  
3. Arguments: Whole literals or prior comptime names only.  
4. Fold to existing `COMPTIME_WHOLE` (56); no new opcode; AETH v11.  
5. Keep 1,024 `comptime bind` budget; purity vs M14/M21 unchanged.  
6. **Require** seed≡bootstrap before Proven-now / default product claim.  
7. Suggested package at ship: **0.33.0**.  
8. **Reject for M23:** recursion, comptime control flow, host/foreign callees,
   multi-op nested trees, Text/Bytes/Truth comptime values.

## Consequences

### Positive

- Shared compile-time helpers without runtime noise  
- Clear ladder after M15  
- Still fuel- and purity-bounded  

### Costs

- Comptime interpreter for restricted callee bodies  
- Seed eval complexity  
- Authors must split logic into tiny pure weaves  

### Risks

| Risk | Mitigation |
| --- | --- |
| Scope creep to full comptime language | D2a body subset; new ADR for control/recursion |
| Host observation | Explicit purity rejects |
| Seed drift | Dual-compare gate |

## Alternatives considered

| Option | Outcome |
| --- | --- |
| Jump to comptime `choose`/`while` first | Deferred — higher DoS surface |
| Nested calls + recursion with fuel | Deferred — needs separate fuel ADR |
| Multi-op expression trees only | Smaller; does not unlock helpers; may follow later |

## Implementation gate

1. This ADR Accepted  
2. Design + [M23-VALIDATION-MATRIX.md](M23-VALIDATION-MATRIX.md) present  
3. Vertical slice + negatives  
4. Seed dual-compare green  
5. DOC-SYNC + delivery report  

## Links

- Design M23, matrix M23  
- Prior: ADR-008, ADR-019  
- Host/FFI must not leak: ADR-018, ADR-025  

---

*End of ADR-039.*
