# ADR-127: BARP — seed SPEAK root comptime-bind unknown pure-call pilot

**Status:** Accepted — implementation in progress
**Date:** 2026-08-14
**Decision makers:** Human director (ongoing Aether maturity direction); implementation agent
**Related Rule IDs:** CONST-DEP-001, DOC-ADR-001, RND-INVAR-001, SEC-INPUT-001, TEST-BEHAVIOR-001
**Depends on:** ADR-039, ADR-043, ADR-072, ADR-110, ADR-111

## Context

M23 accepts a root-level pure comptime call only when its target is a prior,
ordinary total guest weave returning `Whole`, with owned `Whole` parameters and
an allowed D2a body. The checked-in seed already interprets that bounded M23
surface from raw source and emits the same `COMPTIME_WHOLE` artifact as the
bootstrap compiler for the documented corpus.

The existing AE-SEED-011 direct-call witness recognizes runtime root `bind`,
`revise`, `yield`, `speak`, `handle`, `forward`, and selected nursery-spawn
call shapes. It does not recognize a root `comptime bind` call. An unknown M23
callee therefore remains outside the direct seed-SPEAK target-existence
witness even though the seed already owns the valid M23 execution path.

This ADR adds one lexical target-existence witness without attempting the M23
resolver. The invariant is that seed recognition must not claim source-order,
purity, result, parameter, argument, D2a-body, or comptime-evaluation
authority. Those remain bootstrap/full-compiler responsibilities.

## Decision

1. Preserve the existing ordinary total-`Whole` root-weave state and the
   established top-level declaration-header scan used by AE-SEED-011.
2. Only when prior direct-call witnesses did not match, recognize a trimmed,
   two-space root line beginning exactly `comptime bind ` that contains the
   fixed delimiter ` <- call `.
3. Require a nonempty opaque target after that delimiter. Reuse the existing
   target boundary: either the first later ASCII-space begins an opaque
   argument-bearing tail, or the target reaches the exact end of its line as a
   zero-argument call.
4. Reuse the declaration-header scan only to determine whether the opaque
   target has any top-level declaration header. Do not determine whether that
   declaration is prior, ordinary, total, `Whole`-returning, pure, M23-D2a,
   parameter-compatible, or otherwise valid for comptime execution.
5. When the target has no such declaration header, direct forge emits exactly
   one `aether.seed-error/v1` packet with `AE-SEED-011`,
   `origin: "seed-speak"`, position `1:1`, blank `Bytes`, and the established
   unknown-call message. Product forge preserves that seed packet.
6. Prior ordinary `weave` helpers with one `Whole` parameter and with no
   parameters, each called through a root `comptime bind`, remain
   seed/bootstrap byte-identical, verified, and executable.

## Consequences

- The seed gains one direct diagnostic witness for the already implemented M23
  source shape. It adds no accepted-source, AETH, VM, verifier, forge ABI,
  dependency, host-capability, native, or registry surface.
- The word `comptime` and fixed delimiter are recognition conditions, not a
  comptime parser or resolver. A declared but forward, host, foreign, task,
  erroring, non-Whole, impure, or incompatible target stays outside this
  witness and retains full compiler authority.
- The implementation must reuse existing line-state and Text scratch values;
  it must not add host callbacks, source materialization, or bootstrap pre-gate
  dependence to product compilation.

## Validation plan

1. A red regression proves the prior checked-in seed emits no direct
   `AE-SEED-011` packet for a canonical root comptime-bind unknown target.
2. Direct forge proves exactly one schema/code/message/position/origin packet
   with blank `Bytes`; product compilation proves the merged packet.
3. Prior eligible pure `Whole` helpers called both with one argument and with
   no arguments prove direct seed forge emits no packet, verifies, matches
   bootstrap byte-for-byte, and exits with the expected result.
4. No-call, no-target, non-root, task, erroring-parent, forward/declared-
   ineligible, Text-literal, and missing-world inputs prove lexical,
   caller-state, declaration, and diagnostic-priority boundaries.
5. The rebuilt seed must pass the full release gate, including four-way seed
   identity and technical-preview consumer verification.

## Alternatives considered

| Alternative | Decision |
| --- | --- |
| Implement a general M23 resolver in the seed SPEAK scan | Rejected: source-order, signature, purity, D2a, and comptime-evaluation checks would expand parser/type-checker authority. |
| Add a new comptime-specific target delimiter | Rejected: the existing next-space or exact-end-of-line target boundary is already bounded, tested, and avoids divergent call scanning. |
| Check whether the header is a prior eligible guest weave | Rejected: line-state header collection cannot prove M23 eligibility; bootstrap retains that responsibility. |
| Leave all comptime-call failures bootstrap-only | Rejected: this exact unknown-target case is stable, useful, and complements the seed-native valid M23 path. |

## Honesty boundary

This ADR covers only an ordinary total-`Whole` weave and an immediate
two-space root line beginning exactly `comptime bind `, with a nonempty opaque
destination prefix, fixed ` <- call ` delimiter, and nonempty opaque target
with either an opaque argument-bearing tail or an exact end-of-line zero-
argument boundary. It does not claim source order, name binding, M23
purity/D2a eligibility, host/foreign/task/effect
classification, signature/argument/type validation, comptime evaluation,
general expression parsing, source spans beyond fixed `1:1`, full
`AE-SEED-011` parity, or seed-native multi-file elaboration.

## Links

- [ADR-039](ADR-039-m23-comptime-pure-calls.md)
- [ADR-043](ADR-043-bootstrap-authority-reduction.md)
- [ADR-072](ADR-072-barp-product-seed-error-packet-abi.md)
- [ADR-110](ADR-110-barp-seed-speak-root-yield-unknown-call-pilot.md)
- [ADR-111](ADR-111-barp-seed-speak-root-yield-unknown-call-pilot.md)
- [BARP design](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
- [BARP validation matrix](../Current%20state/BARP-VALIDATION-MATRIX.md)

---

*End of ADR-127.*
