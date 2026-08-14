# ADR-004: AETH v6 closed bounded-resource operations

**Status:** Accepted
**Date:** 2026-07-31
**Decision makers:** WPAI product direction, with recorded human approval of the
M1 direction and authorization to deliver the bounded M2 increment
**Related Rule IDs:** DOC-ADR-001, RND-INVAR-001, SEC-INPUT-001,
TEST-BEHAVIOR-001, DOC-SYNC-001, CONST-GATE-001

## Context

ADR-003 accepts explicit capability-oriented allocation, ownership-aware
resource behavior, bounded arenas, and seed-verifiable lowering as Aether's
direction. Its design package intentionally left the concrete M2 grammar and
artifact representation for implementation after approval.

A first implementation must keep the proof finite. In particular, a generic
first-class `Allocation[T]` result would introduce result typing, propagation,
joins, and cross-weave ownership cases before Aether has its planned typed
error/effect layer. A raw AST with a moved Boolean is also insufficient: the
emitter and verifier need resolved owner and region facts, and hostile AETH
must not substitute a placeholder for a moved owner.

## Decision

Aether 0.6 introduces AETH v6 and a deliberately closed M2 resource surface:

1. One `arena N` root binding may appear only in `main`, with a positive
   1,000,000-logical-byte maximum. Its capacity is stored in the v6 header.
2. `buffer Whole` and `buffer Truth` create unallocated owner placeholders.
   Only these copy element types are admitted.
3. `access arena` is an ephemeral capability argument usable only by `allocate`
   or an `access ...: Arena` parameter call. It cannot be bound, stored,
   returned, or cross a control-flow edge.
4. `allocate`, resource `append`, and `at` are closed `choose` conditions with
   an explicit dim branch. They directly name the mutable destination that is
   restored or atomically updated, so no branch can lose the owner.
5. Each resource outcome must be the terminal root choice (or a nested terminal
   resource choice); its branches yield immediately. Outcomes are not
   first-class Aether values in 0.6.
6. Bootstrap validation produces a private typed `SemanticResourcePlan` that
   records region, element, owner/borrow place, and destination per operation.
   AETH emission consumes that plan.
7. The v6 verifier tracks operand-stack buffer provenance: placeholder,
   transient borrow, or the exact moved local owner. `ALLOCATE` and
   `BUFFER_APPEND` require the owner moved from their destination slot;
   `BUFFER_AT` and `COUNT` require a transient borrow.
8. The VM reserves the full arena before guest execution. Allocation failure,
   arithmetic overflow, and insufficient capacity yield dim without consuming
   arena capacity or the original owner. Append-full and lookup-absent likewise
   preserve their relevant values.
9. Buffers cannot be weave results in this version. This prevents an
   allocation-success ambiguity from crossing a function boundary until a
   separately specified outcome-propagation feature exists.

The source, verifier, VM, seed compiler, forge boundary, canonical examples,
and documentation are all part of this one decision. A feature is not product
surface merely because the Rust bootstrap accepts it.

## Consequences

### Positive

- Allocation authority and capacity are visible in source and in artifact
  metadata; there is no ambient allocator.
- Each possible resource failure is locally handled, so the minimal state
  machine has no hidden owner join.
- AETH v4/v5 remain immutable compatibility formats while v6 has a clear
  verifier boundary.
- The verifier has a concrete provenance defense against a malformed artifact
  replacing a moved buffer owner with a fresh placeholder.
- The scope is small enough for byte-identical seed/bootstrap proof across both
  success and failure behavior.

### Deliberate limits

- No first-class resource outcome, propagation, generic result union, or
  buffer-returning weave exists yet.
- No individual free/reset, user destructor, dynamic record field, nested
  buffer, owned element, mutable element reference, or general reference/lifetime
  system exists.
- `revise` cannot replace a resource owner; the closed operations are the only
  M2 resource replacement mechanism.
- Buffer capacity is reserved at allocation time and reclaimed only at the end
  of the invocation. This favors deterministic accounting over early reuse.
- Existing Text/Bytes behavior remains compatibility behavior, not a claim of
  general arena routing or zero-copy references.

## Alternatives rejected

| Alternative | Reason rejected now |
| --- | --- |
| Generic `Allocation[T]` / `Append[T]` / `Lookup[T]` values | Requires a result/effect and propagation design that is not yet specified or seed-proven. |
| Ambient allocator or global heap | Hides authority and failure behavior. |
| Generic mutable references | Reintroduces escaping loans and lifetime inference before a finite verifier model exists. |
| Buffer weave results in 0.6 | Cannot express whether a returned buffer is allocated without a propagated closed outcome. |
| Trusting source validation alone | AETH is an untrusted input boundary; verifier provenance must reject hostile instruction streams. |
| Reinterpreting v4/v5 | Breaks stored artifacts and obscures version security boundaries. |

## Verification obligations

The accepted evidence set includes:

1. Source-positive behavior for Whole and Truth buffers, allocation exhaustion,
   append full, lookup fallback, and access-bound helpers.
2. Source-negative checks for arena placement/capacity, unsupported elements,
   unallocated borrow, mismatched restoration destination, nonterminal outcome,
   resource `revise`, and prohibited buffer result.
3. Hostile v6 bytes for oversized plans, invalid local destinations, missing
   plan/resource instruction mismatch, and forged placeholder substitution.
4. Bootstrap/seed byte identity and verified execution for every canonical M2
   example, plus seed self-reproduction.
5. The normal constitution quality gates with zero warnings.

## Links

- [Aether 0.6 specification](AETHER_0.6.md)
- [ADR-003: value/resource semantic direction](ADR-003-value-resource-semantics.md)
- [M1 design](DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md)
- [M1 validation matrix](M1-VALIDATION-MATRIX.md)
- [Seed Profile](../Current%20state/SEED_PROFILE.md)
