# ADR-117: BARP — seed SPEAK root revise-call unknown-target pilot

**Status:** Accepted — implemented and release verified
**Date:** 2026-08-14
**Decision makers:** Human director (ongoing Aether maturity direction); implementation agent
**Related Rule IDs:** CONST-DEP-001, DOC-ADR-001, RND-INVAR-001, SEC-INPUT-001, TEST-BEHAVIOR-001
**Depends on:** ADR-072, ADR-110, ADR-111, ADR-116

## Context

The full compiler accepts the root plain statement `revise name <- expression`;
that expression may be a direct weave call. ADR-110, ADR-111, and ADR-116 give
the checked-in seed a bounded `AE-SEED-011` target-existence witness for
canonical direct calls in bind and root-yield forms, including an exact
end-of-line zero-argument target. A canonical root revise-call source reaches
the same product diagnostic family, but it has no direct-seed witness yet.

This is a small remaining product-path bootstrap-authority gap. It can be
reduced without treating the seed as a statement parser, type checker, or name
binder.

## Decision

1. In an existing canonical ordinary `weave ... -> Whole:` root-line scan, the
   seed recognizes only a trimmed line beginning exactly `revise ` that contains
   the existing literal separator ` <- call `.
2. The one candidate target starts immediately after that separator and ends at
   the next ASCII space (argument-bearing form) or at exact end of line
   (zero-argument form). The seed applies the existing top-level weave-header
   existence scan only to that opaque target text.
3. If no matching ordinary/export/host/foreign/task header is found, direct
   forge emits exactly one `aether.seed-error/v1` packet with `AE-SEED-011`,
   `origin: "seed-speak"`, position `1:1`, blank Bytes, and the existing
   unknown-call message. Product forge preserves the same seed packet.
4. A later-declared matching target remains valid, seed/bootstrap
   byte-identical, verified, and executable for both argument-bearing and
   zero-argument revise calls.
5. Missing `world` retains its existing higher-priority `AE-SEED-006` result.
   Text literals and non-`revise` statements stay outside this witness.

## Consequences

- The checked-in seed owns one more grammar-proven direct diagnostic witness;
  no guest language, AETH, verifier, VM, host ABI, capability, or dependency
  changes.
- The target scan remains a bounded source-line operation and reuses no new
  host authority or runtime state.
- The direct seed may not establish that the destination exists or is mutable,
  that arguments match, that the target is callable there, or that the result
  type/effects are legal. Those remain full-compiler responsibilities.

## Validation plan

1. A red regression proves the prior checked-in seed did not emit a direct
   `AE-SEED-011` packet for canonical argument-bearing and zero-argument
   revise-call unknown targets.
2. Direct forge then proves one seed-native packet, blank Bytes, stable message,
   and `seed-speak` origin for each exact source form.
3. Product compilation proves the merged `AE-SEED-011`/`seed-speak` packet.
4. Matching later declarations prove seed/bootstrap byte identity, verification,
   and execution for both tail forms.
5. A call-shaped Text literal and missing-world source prove false-positive and
   priority boundaries; existing bind/root-yield witnesses remain in the full
   self-host suite.
6. The rebuilt seed passes the complete release gate, including four-way seed
   identity and the technical-preview consumer verification.

## Alternatives considered

| Alternative | Decision |
|---|---|
| Implement all direct-call statement forms | Rejected: would turn a witness into a competing general statement parser. |
| Teach the seed destination mutability, signature, type, effect, and ownership checks | Rejected: that is full semantic analysis, not this bounded authority reduction. |
| Leave root revise calls host-only | Rejected: the exact existing separator and header scan make a safe direct witness feasible. |
| Change Aether syntax or AETH call encoding | Rejected: no language/runtime change is required for diagnostic maturity. |

## Honesty boundary

This ADR covers only a canonical ordinary-Whole root `revise name <- call`
line with one opaque target, terminated by a next-space argument boundary or
end of line. It does not claim full `AE-SEED-011` parity, declaration parsing,
destination validation, signature/type/effect/ownership checking, source spans,
all call statement forms, or seed-native multi-file elaboration.

## Links

- [ADR-072](ADR-072-barp-product-seed-error-packet-abi.md)
- [ADR-110](ADR-110-barp-seed-speak-unknown-call-pilot.md)
- [ADR-111](ADR-111-barp-seed-speak-root-yield-unknown-call-pilot.md)
- [ADR-116](ADR-116-barp-seed-speak-zero-argument-unknown-call-pilot.md)
- [BARP design](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
- [BARP validation matrix](../Current%20state/BARP-VALIDATION-MATRIX.md)

---

*End of ADR-117.*
