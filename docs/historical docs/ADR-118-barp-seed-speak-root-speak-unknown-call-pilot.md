# ADR-118: BARP — seed SPEAK root speak-call unknown-target pilot

**Status:** Accepted — implemented and release verified
**Date:** 2026-08-14
**Decision makers:** Human director (ongoing Aether maturity direction); implementation agent
**Related Rule IDs:** CONST-DEP-001, DOC-ADR-001, RND-INVAR-001, SEC-INPUT-001, TEST-BEHAVIOR-001
**Depends on:** ADR-072, ADR-110, ADR-111, ADR-116, ADR-117

## Context

The canonical root plain statement `speak expression` accepts a direct weave
call expression. ADR-110, ADR-111, ADR-116, and ADR-117 give the checked-in
seed bounded `AE-SEED-011` target-existence witnesses for direct calls in bind,
root-yield, and root-revise forms, including the exact end-of-line
zero-argument boundary. A canonical root `speak call target` source reaches the
same name-resolution failure family, but it has no direct-seed SPEAK witness.

The seed already owns a two-pass, root-line scanner for this exact purpose. A
new fixed prefix can use its existing opaque target extraction and top-level
header-existence scan without becoming a statement parser or a Text type
checker.

## Decision

1. In the existing canonical ordinary `weave ... -> Whole:` root-line scan, the
   seed recognizes only a trimmed line beginning exactly `speak call `.
2. The candidate target begins immediately after that prefix and ends at the
   next ASCII space (argument-bearing form) or exact end of line
   (zero-argument form). The existing ordinary/export/host/foreign/task
   top-level header-existence scan operates only on that opaque target text.
3. If no matching header exists, direct forge emits exactly one
   `aether.seed-error/v1` packet with `AE-SEED-011`, `origin: "seed-speak"`,
   position `1:1`, blank Bytes, and the established unknown-call message.
   Product forge preserves that same seed packet.
4. A later-declared matching Text-result target remains valid, seed/bootstrap
   byte-identical, verified, and executable for both argument-bearing and
   zero-argument speak calls.
5. Missing `world` retains its higher-priority `AE-SEED-006` result. Text
   literals and all non-root/non-`speak call` expressions remain outside this
   witness.

## Consequences

- The checked-in seed owns one additional grammar-proven direct diagnostic
  witness; this changes no accepted valid source, AETH byte/version, verifier,
  VM, forge ABI, host capability, or dependency.
- The scanner checks only target-header existence. It does not establish that
  the called weave returns Text, that arguments match, or that effects and
  ownership are valid for `speak`.
- The full compiler retains semantic authority for every condition outside the
  explicitly named lexical witness.

## Validation plan

1. A red regression proves the prior checked-in seed does not emit a direct
   `AE-SEED-011` packet for canonical argument-bearing or zero-argument root
   speak-call unknown targets.
2. Direct forge proves one seed-native packet, blank Bytes, stable message, and
   `seed-speak` origin for each exact source form.
3. Product compilation proves the merged `AE-SEED-011`/`seed-speak` packet.
4. Later-declared Text-result helpers prove seed/bootstrap byte identity,
   verification, and execution for both target boundaries.
5. A call-shaped Text literal and missing-world source prove false-positive and
   priority boundaries; earlier bind/root-yield/root-revise witnesses remain in
   the complete self-host suite.
6. The rebuilt seed must pass the release gate, including four-way seed identity
   and technical-preview consumer verification.

## Alternatives considered

| Alternative | Decision |
| --- | --- |
| Recognize any `call` token in every expression position | Rejected: it would be a competing expression parser and could steal unrelated diagnostics. |
| Implement Text result, argument, effect, or ownership checks in the scanner | Rejected: those are full semantic-analysis responsibilities. |
| Leave root speak calls host-only | Rejected: the exact prefix and existing header scan provide a narrow, directly provable witness. |
| Change Aether source syntax or AETH call encoding | Rejected: diagnostic maturity requires neither language nor runtime expansion. |

## Honesty boundary

This ADR covers only a canonical ordinary-Whole root `speak call` line with one
opaque target, terminated by a next-space argument boundary or end of line. It
does not claim full `AE-SEED-011` parity, Text result validation, signature,
type, effect, ownership, source spans, nested calls, all expression forms, or
seed-native multi-file elaboration.

## Links

- [ADR-072](ADR-072-barp-product-seed-error-packet-abi.md)
- [ADR-110](ADR-110-barp-seed-speak-unknown-call-pilot.md)
- [ADR-111](ADR-111-barp-seed-speak-root-yield-unknown-call-pilot.md)
- [ADR-116](ADR-116-barp-seed-speak-zero-argument-unknown-call-pilot.md)
- [ADR-117](ADR-117-barp-seed-speak-revise-unknown-call-pilot.md)
- [BARP design](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
- [BARP validation matrix](../Current%20state/BARP-VALIDATION-MATRIX.md)

---

*End of ADR-118.*
