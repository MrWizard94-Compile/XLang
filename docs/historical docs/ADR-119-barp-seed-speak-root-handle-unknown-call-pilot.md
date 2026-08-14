# ADR-119: BARP — seed SPEAK root handle-call unknown-target pilot

**Status:** Accepted — implemented and release verified
**Date:** 2026-08-14
**Decision makers:** Human director (ongoing Aether maturity direction); implementation agent
**Related Rule IDs:** CONST-DEP-001, DOC-ADR-001, RND-INVAR-001, SEC-INPUT-001, TEST-BEHAVIOR-001
**Depends on:** ADR-007, ADR-072, ADR-110, ADR-116, ADR-117, ADR-118

## Context

M4 defines a terminal total-weave discharge statement:

~~~aether
handle call weave args... into success otherwise error into code
~~~

The bootstrap compiler resolves the `weave` target before it validates the
target's effect, result, argument list, or mutable destinations. The checked-in
seed already owns bounded `AE-SEED-011` witnesses for canonical direct bind,
root-yield, root-revise, and root-speak calls. A canonical root handle call
with a missing target has no direct seed-SPEAK witness yet.

Unlike simple expression call forms, `handle` has required tail delimiters. A
new scanner must require those delimiters rather than reclassifying malformed
handle syntax as a missing target.

## Decision

1. In the existing canonical ordinary `weave ... -> Whole:` root-line scan, the
   seed recognizes only a trimmed line beginning exactly `handle call `.
2. It extracts the opaque target at the first following ASCII space and considers
   the line only when the remaining text contains both a later ` into ` and a
   later ` otherwise error into ` delimiter with nonempty trailing destination
   text.
3. The existing top-level ordinary/export/host/foreign/task header-existence
   scan operates only on that target. No destination, argument, signature, effect,
   result, ownership, or terminal-position semantic validation moves into the
   scanner.
4. If no matching header exists, direct forge emits exactly one
   `aether.seed-error/v1` packet with `AE-SEED-011`, `origin: "seed-speak"`,
   position `1:1`, blank Bytes, and the established unknown-call message.
   Product forge preserves the same packet.
5. Later-declared matching `raises Whole` targets remain valid,
   seed/bootstrap byte-identical, verified, and executable for argument-bearing
   and zero-argument handle calls. Missing `world` retains its higher-priority
   `AE-SEED-006` result.

## Consequences

- The checked-in seed gains one more fixed-form M4 name-resolution witness
  without changing accepted valid source, AETH bytes/versions, VM, verifier,
  forge ABI, dependencies, or host capability.
- Required literal delimiters prevent the scanner from treating a bare or
  structurally incomplete `handle call` prefix as this witness.
- Full M4 semantic analysis remains authoritative for effect eligibility,
  result type, argument modes/types/count, distinct mutable destinations, root
  terminality, and resource boundaries.

## Validation plan

1. A red regression proves the prior seed did not emit a direct `AE-SEED-011`
   packet for canonical argument-bearing and zero-argument root handle-call
   unknown targets.
2. Direct forge proves exactly one seed-native packet, blank Bytes, stable
   message, and `seed-speak` origin for both forms.
3. Product compilation proves the merged `AE-SEED-011`/`seed-speak` packet.
4. Later-declared `raises Whole` helpers prove seed/bootstrap identity,
   verification, and execution for both tail forms.
5. A handle-call-shaped Text literal, a missing-world input, and an incomplete
   handle tail prove the lexical, priority, and malformed-tail boundaries.
6. The rebuilt seed must pass the full release gate, including four-way seed
   identity and technical-preview consumer verification.

## Alternatives considered

| Alternative | Decision |
| --- | --- |
| Recognize every `handle ` statement | Rejected: it would steal parser/semantic responsibility from the full compiler. |
| Scan only the `handle call ` prefix | Rejected: a malformed handle tail could be misdiagnosed as an unknown target. |
| Validate effect, result, arguments, or destinations in the scanner | Rejected: those are full M4 semantic responsibilities. |
| Leave canonical handle calls host-only | Rejected: the fixed grammar delimiters make a narrow target-existence witness feasible. |

## Honesty boundary

This ADR covers only canonical ordinary-Whole root `handle call target ... into
success otherwise error into code` lines with the two required delimiters and
an opaque target ending at the first space. It does not claim full
`AE-SEED-011` parity, M4 semantics, effect/result/argument/destination
validation, source spans, all effect forms, or seed-native multi-file
elaboration.

## Links

- [ADR-007](ADR-007-m4-typed-error-effect.md)
- [ADR-072](ADR-072-barp-product-seed-error-packet-abi.md)
- [ADR-110](ADR-110-barp-seed-speak-unknown-call-pilot.md)
- [ADR-116](ADR-116-barp-seed-speak-zero-argument-unknown-call-pilot.md)
- [ADR-117](ADR-117-barp-seed-speak-revise-unknown-call-pilot.md)
- [ADR-118](ADR-118-barp-seed-speak-root-speak-unknown-call-pilot.md)
- [Delivery report](DELIVERY_REPORT-2026-08-14-BARP-ROOT-HANDLE-UNKNOWN-CALL-SPEAK.md)
- [BARP design](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
- [BARP validation matrix](../Current%20state/BARP-VALIDATION-MATRIX.md)

---

*End of ADR-119.*
