# ADR-127: BARP — seed-native M23 comptime-call source-order integrity

**Status:** Accepted — implemented and release verified
**Date:** 2026-08-14
**Amended:** 2026-08-21
**Decision makers:** Human director (ongoing Aether maturity direction); implementation agent
**Related Rule IDs:** CONST-DEP-001, DOC-ADR-001, RND-INVAR-001, SEC-INPUT-001, TEST-BEHAVIOR-001
**Depends on:** ADR-039, ADR-043, ADR-072, ADR-110, ADR-111

## Context

M23 accepts a root-level pure comptime call only when its target is a
textually prior, ordinary total guest weave returning `Whole`, with owned
`Whole` parameters and an allowed D2a body. Rust bootstrap already enforces
that source-order requirement. The checked-in seed evaluates the documented
M23 D2a corpus from raw source and emits `COMPTIME_WHOLE` without a
materialization bridge.

The initial ADR-127 diagnostic slice added the missing root
`comptime bind name <- call target args...` shape to the existing
AE-SEED-011 unknown-target line-state witness. That witness intentionally
looks for a declaration header anywhere in the source, so an unknown target
gets the established direct packet while an existing header avoids a false
unknown-target claim.

The regression exposed a more important product-path defect: the seed M23
evaluator itself searched the entire source for `weave target`, so a forward
declaration could be accepted by product compilation even though bootstrap
rejected it with `AE-COMPTIME-001`. This was a source-order semantic mismatch,
not merely a missing diagnostic. Product default compilation must not accept a
forward M23 call that the documented M23 contract forbids.

## Decision

1. Retain the initial bounded lexical witness for an unknown canonical root
   `comptime bind ... <- call target` form. It continues to use the established
   `AE-SEED-011` message for a target with no declaration header anywhere.
2. Change the seed's existing M23 resolver lookup from the whole source to the
   source prefix ending at the current `comptime bind` line. Match only the
   literal top-level ordinary header prefix `\nweave target [` in that prefix.
   The current line and all later declarations are excluded.
3. When the M23 resolver has a nonempty target but no such prior ordinary
   header, emit exactly one direct `aether.seed-error/v1` packet with
   `AE-SEED-011`, `origin: "seed-speak"`, position `1:1`, blank `Bytes`, and
   message `canonical comptime call target must name a prior top-level declared
   weave`.
4. Limit that resolver packet to a two-space root statement in the current
   ordinary, total, non-erroring `Whole` weave and only when no earlier seed
   diagnostic exists. Nested statements, task weaves, erroring parents,
   incomplete calls, and already-classified unknown targets remain outside this
   new packet branch.
5. Do not add a general parser, name binder, type checker, D2a eligibility
   checker, or multi-file resolver to seed SPEAK. The existing M23 evaluator and
   bootstrap remain authoritative for all other M23 rejection reasons.
6. Preserve valid source identity: earlier one-argument and zero-argument
   M23 pure calls remain seed/bootstrap byte-identical, verified, and
   executable.

## Consequences

- Product compilation now rejects the canonical forward M23 target through the
  seed path instead of accepting it, restoring the documented source-order
  invariant on this path.
- An entirely unknown target still produces only the established unknown-target
  packet. The new source-order branch checks the seed error flag first, avoiding
  duplicate packets and preserving diagnostic priority.
- This implementation adds no source form, AETH instruction, VM behavior,
  verifier rule, host capability, package protocol, registry behavior, or
  native output behavior.
- The lookup is deliberately a bounded ordinary-header witness. It does not
  prove the matched weave's return type, parameter modes, purity, D2a body,
  argument count, or evaluation eligibility. Those checks remain in the
  existing M23 semantics and bootstrap oracle.

## Validation evidence

1. Preserve the test-first evidence that the previous checked-in seed had no
   direct AE-SEED-011 packet for the canonical root unknown comptime call.
2. Direct forge and product compilation of an unknown target prove one existing
   `AE-SEED-011` seed-SPEAK packet with blank `Bytes`, without a duplicate
   source-order packet.
3. A forward-declared target proves direct seed forge emits one source-order
   `AE-SEED-011` packet, product compilation rejects it through packet merge,
   and bootstrap continues to reject it as `AE-COMPTIME-001`.
4. Earlier eligible one-argument and zero-argument pure helper calls prove
   seed/bootstrap identity, verifier acceptance, and the expected exit value.
5. Non-call, missing-target, nested, Text-literal, task, erroring-parent, and
   missing-world cases prove lexical, caller-state, and priority boundaries.
6. The seed was rebuilt through bootstrap, product, and forge; every artifact
   matched byte-for-byte, and the final release gate passed.

## Alternatives considered

| Alternative | Decision |
| --- | --- |
| Leave forward M23 calls accepted by the seed product path | Rejected: it violates the M23 source-order contract and creates a real product/bootstrap semantic mismatch. |
| Use the existing whole-source declaration scan for source order | Rejected: any later header would continue to make a forward target look valid. |
| Add a host bootstrap preflight for M23 source order | Rejected: the direct seed evaluator already owns this lookup; moving it to a host preflight would conceal rather than reduce bootstrap authority. |
| Implement complete M23 eligibility in the line-state scanner | Rejected: type, purity, D2a, and argument analysis would expand the seed beyond this bounded integrity correction. |

## Honesty boundary

This ADR proves only that a canonical root M23 call must name a prior ordinary
top-level `weave` header before the seed's existing M23 evaluation proceeds. It
does not claim full M23 diagnostic parity, full header parsing, return/parameter
validation, purity/D2a validation, argument validation, control flow, nested
calls, host/foreign/task classification beyond the guarded caller state,
multi-file elaboration, general expression parsing, source spans beyond fixed
`1:1`, or seed-native multi-file resolution.

## Links

- [ADR-039](ADR-039-m23-comptime-pure-calls.md)
- [ADR-043](ADR-043-bootstrap-authority-reduction.md)
- [ADR-072](ADR-072-barp-product-seed-error-packet-abi.md)
- [ADR-110](ADR-110-barp-seed-speak-unknown-call-pilot.md)
- [ADR-111](ADR-111-barp-seed-speak-root-yield-unknown-call-pilot.md)
- [M23 validation matrix](../Current%20state/M23-VALIDATION-MATRIX.md)
- [BARP design](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
- [BARP validation matrix](../Current%20state/BARP-VALIDATION-MATRIX.md)
- [Delivery report](DELIVERY_REPORT-2026-08-21-BARP-M23-COMPTIME-CALL-SOURCE-ORDER-SPEAK.md)

---

*End of ADR-127.*
