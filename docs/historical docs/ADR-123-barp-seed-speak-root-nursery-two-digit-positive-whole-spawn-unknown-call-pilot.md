# ADR-123: BARP — seed SPEAK root nursery positive two-digit Whole spawn unknown-target pilot

**Status:** Accepted — implemented and release verified
**Date:** 2026-08-14
**Decision makers:** Human director (ongoing Aether maturity direction); implementation agent
**Related Rule IDs:** CONST-DEP-001, DOC-ADR-001, RND-INVAR-001, SEC-INPUT-001, TEST-BEHAVIOR-001
**Depends on:** ADR-010, ADR-072, ADR-121, ADR-122

**Later scoped extension:** ADR-124 independently adds only the immediate exact
`bright` Truth argument shape. It does not broaden this ADR's positive
two-digit-Whole decision.

## Context

M7 admits a nursery child with a copy Whole argument:

~~~aether
together:
  spawn call count 42 into result
~~~

The bootstrap Whole-literal predicate accepts ASCII decimal digits, rejects
leading zeroes for multi-digit literals, and rejects `-0`. `parse_spawn_statement`
parses the call source before `validate_spawn` resolves its target; that
validator resolves the target before checking task identity, signature, argument
count, mode, or type. Thus a canonical positive two-digit literal plus an
unknown target has stable unknown-target diagnostic priority, while leading-zero
literals fail earlier as malformed literals.

ADR-122 recognizes exactly one digit and deliberately excludes all multi-digit
forms. The smallest lexer-aligned extension is not arbitrary numeric parsing:
it is exactly two ASCII digits, with a nonzero first digit. This recognizes the
bounded range `10` through `99` without admitting signs, leading zeroes, or
three-or-more digit forms.

## Decision

1. Preserve ADR-121's ordinary total-Whole root `together:` prior-line state,
   four-space immediate-child requirement, and ADR-122's zero- then one-digit
   branch priority.
2. Only if those witnesses did not match, recognize
   `spawn call target digits into destination` when `digits` has exactly two
   ASCII decimal characters, the first is `1` through `9`, and the second is
   `0` through `9`.
3. Require the first ASCII space after the opaque target to precede the first
   digit, the byte after the second digit to begin literal ` into `, and a
   nonempty destination suffix.
4. Reuse the existing top-level ordinary/export/host/foreign/task declaration
   header scan only for the opaque target. Do not parse numeric values or
   validate task identity, signature, destination, nesting, effect/result,
   resource, ownership, or scheduling policy.
5. If no declaration header matches, direct forge emits exactly one
   `aether.seed-error/v1` packet with `AE-SEED-011`, `origin: "seed-speak"`,
   position `1:1`, blank Bytes, and the established unknown-call message.
   Product forge preserves that packet.
6. A later-declared checkpointed `task weave worker [limit: Whole] -> Whole`
   called with `42` remains valid, seed/bootstrap byte-identical, verified, and
   executable.

## Consequences

- The checked-in seed gains one narrow lexical M7 target-existence witness. It
  adds no accepted-source, AETH, VM, verifier, forge ABI, dependency, or
  host-capability surface.
- Positive two-digit decimal is a recognition condition, not a general Whole
  parser. Zero, one-digit, signed, leading-zero, three-or-more digit, `dim`,
  name, Text, Bytes, move/borrow/access, and multi-argument shapes retain their
  existing bounded witness or the full compiler; ADR-124 separately admits only
  exact `bright`.
- The branch reuses the existing Whole scratch state and ADR-121 parent state.
  It introduces no seed binding and leaves the `v132` unused-local self-host
  variant probe intact.
- The bootstrap compiler remains authoritative for complete Whole syntax and
  range handling, task-only target requirements, argument count/type/mode,
  destination legality, error propagation, resource policy, ownership, and
  scheduler behavior.

## Validation plan

1. A red regression proves the ADR-122 seed emitted no direct `AE-SEED-011`
   packet for a canonical positive two-digit root-nursery unknown target.
2. Direct forge proves exactly one schema/code/message/position/origin packet
   with blank Bytes; product compilation proves the merged packet.
3. The lexical endpoints `10` and `99`, plus a later-declared checkpointed
   one-Whole-parameter task called with `42`, prove range boundaries,
   seed/bootstrap identity, verification, and execution.
4. Leading-zero, three-digit, signed, `dim`/name/general-Truth, multi-argument, delayed,
   descendant, missing-destination, non-task-target, erroring-parent, and
   missing-world sources prove literal, caller-state, delimiter, and priority
   boundaries.
5. The rebuilt seed must pass the full release gate, including four-way seed
   identity and technical-preview consumer verification.

## Alternatives considered

| Alternative | Decision |
| --- | --- |
| Recognize arbitrary multi-digit Whole literals | Rejected: canonical numeric range handling needs its own proof. |
| Recognize signed two-digit literals | Rejected: sign and `-0` rules would broaden lexical authority. |
| Recognize leading-zero forms | Rejected: the bootstrap literal predicate rejects them before target resolution. |
| Recognize general Truth, name, Text, or ownership arguments | Rejected: those require broader type and ownership priority evidence; ADR-124 separately admits only exact `bright`. |
| Validate the task signature or destination | Rejected: those remain M7 semantic responsibilities. |
| Leave all positive two-digit spawns bootstrap-only | Rejected: the exact `10`–`99` delimiter shape is useful, lexer-aligned, and directly provable. |

## Honesty boundary

This ADR covers only an ordinary total-Whole `weave ` with a root line exactly
`together:` followed immediately by a four-space
`spawn call target digits into destination` line, where `digits` is a positive
two-character ASCII decimal Whole literal (`10` through `99`). It does not
claim zero/one-digit coverage beyond ADR-121/122, signed, leading-zero,
three-or-more digit, any Truth coverage beyond ADR-124 exact `bright`, general
atom, multiple argument, full nested parsing,
later nursery child, blank-line tolerance, task identity, destination
validation, header parsing, effect/result/signature validation,
resource/ownership policy, scheduler behavior, source spans beyond fixed `1:1`,
full `AE-SEED-011` parity, or seed-native multi-file elaboration.

## Links

- [ADR-010](ADR-010-m7-structured-concurrency.md)
- [ADR-072](ADR-072-barp-product-seed-error-packet-abi.md)
- [ADR-121](ADR-121-barp-seed-speak-root-nursery-zero-argument-spawn-unknown-call-pilot.md)
- [ADR-122](ADR-122-barp-seed-speak-root-nursery-single-digit-whole-spawn-unknown-call-pilot.md)
- [ADR-124](ADR-124-barp-seed-speak-root-nursery-bright-truth-spawn-unknown-call-pilot.md)
- [Delivery report](DELIVERY_REPORT-2026-08-14-BARP-ROOT-NURSERY-TWO-DIGIT-POSITIVE-WHOLE-SPAWN-UNKNOWN-CALL-SPEAK.md)
- [BARP design](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
- [BARP validation matrix](../Current%20state/BARP-VALIDATION-MATRIX.md)

---

*End of ADR-123.*
