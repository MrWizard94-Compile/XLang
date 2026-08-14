# ADR-019: bounded comptime expansion via name chaining (M15 / T-CT)

**Status:** Accepted — **implemented in package 0.20.0**  
**Date:** 2026-08-04  
**Decision makers:** AGENTS Constitution; ADR-014 **T-CT** after M14 delivery  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `SEC-INPUT-001`, `RND-INVAR-001`, `CONST-COMPLETE-001`

## Context

M5 (`comptime bind` with **literal-only** Whole arithmetic) is proven. Authors
still cannot chain compile-time sizes (`product` of a prior comptime value)
without falling back to runtime arithmetic. ADR-014 schedules **T-CT** after
modules/edits/LSP and before resource↔effect expansion; M14 host I/O must not
leak into compile-time evaluation.

A full comptime language (calls, control flow, type computation, macros) would
violate the M5 stop conditions and fuel/authority model in one step.

## Decision

1. **Adopt** [DESIGN-M15-COMPTIME-EXPANSION.md](DESIGN-M15-COMPTIME-EXPANSION.md)
   as the implementable M15 design.  
2. Expand M5 operands so each side of one binary Whole arithmetic op may be a
   **Whole literal** or a **prior root-level immutable `comptime bind` name** in
   the same weave.  
3. Keep: one op per directive, 1,024 budget, checked i64 arithmetic, pure
   evaluation, `COMPTIME_WHOLE` emission, no new host/comptime I/O.  
4. **Reject for M15:** forward refs, runtime names as operands, weave calls,
   control flow, text/bytes/Truth results, type-level work, source generation,
   user-settable fuel, env/file observation.  
5. Seed dual-compare on a fixed chain corpus is required before product claim.  
6. Suggested package pin at ship: **0.20.0**.

## Consequences

### Positive

- Real multi-step compile-time sizing without runtime noise  
- Preserves M5 purity and fuel story  
- No AETH opcode churn if fold-to-immediate remains  
- Clear ladder toward future T-CT slices without a big bang  

### Costs

- Seed and bootstrap must share ordered evaluation maps  
- Diagnostics must explain forward/runtime name errors  
- Still not a general metaprogramming system  

### Risks

| Risk | Mitigation |
| --- | --- |
| Accidental host observation | Explicit purity invariant + no call surface |
| Fuel DoS via deep chains | Cap remains 1,024 directives; O(n) eval |
| Seed/bootstrap drift | Dual-compare mandatory |
| Scope creep to calls/control | Stop conditions; new ADR required |

## Alternatives considered

| Option | Outcome |
| --- | --- |
| Skip chaining; jump to comptime calls | Rejected — larger soundness surface |
| Fold runtime `bind` automatically | Rejected — stage erasure |
| Raise budget instead of names | Rejected — does not add expressiveness safely |

## Implementation gate

1. This ADR **Accepted**  
2. Design + [M15-VALIDATION-MATRIX.md](M15-VALIDATION-MATRIX.md) present  
3. Vertical slice: chain example evaluates; forward-ref negative  
4. Seed≡bootstrap before “Proven now”  

## Links

- Design: [DESIGN-M15-COMPTIME-EXPANSION.md](DESIGN-M15-COMPTIME-EXPANSION.md)  
- Matrix: [M15-VALIDATION-MATRIX.md](M15-VALIDATION-MATRIX.md)  
- Prior: [ADR-008](ADR-008-m5-deterministic-comptime.md), [ADR-014](ADR-014-post-m10-track-portfolio.md)  
- Host boundary (must not leak into comptime): [ADR-018](ADR-018-m14-host-io-capabilities.md)

---

*End of ADR-019.*
