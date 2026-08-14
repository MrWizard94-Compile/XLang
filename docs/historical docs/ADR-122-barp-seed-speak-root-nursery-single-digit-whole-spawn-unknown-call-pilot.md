# ADR-122: BARP — seed SPEAK root nursery single-digit Whole spawn unknown-target pilot

**Status:** Accepted — implemented and release verified
**Date:** 2026-08-14
**Decision makers:** Human director (ongoing Aether maturity direction); implementation agent
**Related Rule IDs:** CONST-DEP-001, DOC-ADR-001, RND-INVAR-001, SEC-INPUT-001, TEST-BEHAVIOR-001
**Depends on:** ADR-010, ADR-072, ADR-121

**Later scoped extensions:** ADR-123 independently adds one immediate positive
two-digit decimal-Whole argument shape (`10` through `99`), and ADR-124
independently adds one immediate exact-`bright` Truth argument shape. ADR-125
independently adds one immediate exact-`dim` Truth argument shape. Neither
broadens this ADR's one-digit decision. ADR-126 separately adds only one
immediate exact empty-Text argument shape.

## Context

M7 admits a nursery child with copy arguments:

~~~aether
together:
  spawn call count 3 into result
~~~

The documented grammar is `spawn call` followed by a target name, zero or more
atoms, `into`, and a destination. The bootstrap parser first validates the
source shape and then resolves the target before signature and argument type
validation. Therefore a syntactically valid one-digit Whole argument plus an
unknown target has a stable unknown-target diagnostic priority.

ADR-121 deliberately recognizes only the zero-argument immediate child. It
does not parse arguments. Expanding directly to arbitrary atoms, negative or
multi-digit Wholes, or multiple arguments would either require more lexical
authority or blur parser/error-priority boundaries.

The smallest useful next witness is one canonical single ASCII decimal digit
between the target and fixed ` into ` delimiter. It covers the existing
`task-loop` source shape while preserving the seed's verifier-first, line-state
discipline.

## Decision

1. Preserve ADR-121's ordinary total-Whole root `together:` prior-line state
   and four-space immediate-child requirement.
2. Only if the zero-argument witness did not match, recognize
   `spawn call target digit into destination` when `digit` is exactly one ASCII
   decimal character (`0` through `9`).
3. Require the first ASCII space after the opaque target to precede that digit,
   and require the byte immediately after the digit to start the literal
   ` into ` delimiter. A nonempty destination suffix is still required.
4. Reuse the existing top-level ordinary/export/host/foreign/task declaration
   header scan only for the opaque target. Do not parse or validate general
   atoms, task identity, signature, destination, nesting, effect/result,
   resource, ownership, or scheduling policy.
5. If no declaration header matches, direct forge emits exactly one
   `aether.seed-error/v1` packet with `AE-SEED-011`, `origin: "seed-speak"`,
   position `1:1`, blank Bytes, and the established unknown-call message.
   Product forge preserves that packet.
6. A later-declared checkpointed `task weave worker [limit: Whole] -> Whole`
   called with `3` remains valid, seed/bootstrap byte-identical, verified, and
   executable.

## Consequences

- The checked-in seed gains one narrow M7 argument-shaped target-existence
  witness. It adds no accepted-source, AETH, VM, verifier, forge ABI,
  dependency, or host-capability surface.
- Single-digit decimal is a lexical recognition condition, not a claim that the
  seed parses Whole literals. ADR-123 later admits only positive two-digit
  decimal, ADR-124 later admits only exact `bright`, and ADR-125 later admits
  only exact `dim`, and ADR-126 later admits only exact empty Text; signed,
  leading-zero, three-or-more-digit, name, `not dim`, nonempty/escaped Text,
  Bytes, move/borrow/access, and multi-argument shapes remain with the
  full parser.
- The branch reuses existing Whole scratch state after its glyph check and the
  ADR-121 parent state. It introduces no new seed binding and leaves the
  `v132` unused-local self-host variant probe intact.
- The bootstrap compiler remains authoritative for all M7 syntax, task-only
  target requirements, argument count/type/mode, destination legality, error
  propagation, resource policy, ownership, and scheduler behavior.

## Validation plan

1. A red regression proves the prior seed emitted no direct `AE-SEED-011`
   packet for a canonical one-digit root-nursery unknown target.
2. Direct forge proves exactly one schema/code/message/position/origin packet
   with blank Bytes; product compilation proves the merged packet.
3. A later-declared checkpointed one-Whole-parameter task helper proves
   seed/bootstrap identity, verification, and execution with exit code `3`.
4. Multi-digit, negative, name/`not dim`/general-Truth, multi-argument, delayed/descendant,
   missing-destination, erroring-parent, and missing-world sources prove
   literal, caller-state, delimiter, and priority boundaries.
5. The rebuilt seed must pass the full release gate, including four-way seed
   identity and technical-preview consumer verification.

## Alternatives considered

| Alternative | Decision |
| --- | --- |
| Recognize all `atom*` argument shapes | Rejected: this would create an unbounded seed lexer/parser claim. |
| Recognize arbitrary multi-digit or signed Whole literals | Rejected: canonical numeric lexical and range handling needs a separate proof. |
| Recognize one arbitrary name/Truth/Text atom | Rejected: semantic type and ownership priority would be too broad for this witness. |
| Validate the task signature or destination | Rejected: those remain M7 semantic responsibilities. |
| Leave all argument-bearing spawns bootstrap-only | Rejected: the exact one-digit delimiter shape is useful, stable, and directly provable. |

## Honesty boundary

This ADR covers only an ordinary total-Whole `weave ` with a root line exactly
`together:` followed immediately by a four-space
`spawn call target digit into destination` line, where `digit` is one ASCII
decimal character. It does not claim positive two-digit coverage beyond ADR-123,
exact-`bright` Truth coverage beyond ADR-124, exact-`dim` Truth coverage
beyond ADR-125, or exact empty-Text coverage beyond ADR-126; signed, leading-zero, three-or-more-digit, or negative Whole
literals, general atoms, multiple
arguments, full nested parsing, later nursery children,
blank-line tolerance, task identity, destination validation, header parsing,
effect/result/signature validation, resource/ownership policy, scheduler
behavior, source spans beyond fixed `1:1`, full `AE-SEED-011` parity, or
seed-native multi-file elaboration.

## Links

- [ADR-010](ADR-010-m7-structured-concurrency.md)
- [ADR-072](ADR-072-barp-product-seed-error-packet-abi.md)
- [ADR-121](ADR-121-barp-seed-speak-root-nursery-zero-argument-spawn-unknown-call-pilot.md)
- [ADR-123](ADR-123-barp-seed-speak-root-nursery-two-digit-positive-whole-spawn-unknown-call-pilot.md)
- [ADR-124](ADR-124-barp-seed-speak-root-nursery-bright-truth-spawn-unknown-call-pilot.md)
- [ADR-125](ADR-125-barp-seed-speak-root-nursery-dim-truth-spawn-unknown-call-pilot.md)
- [ADR-126](ADR-126-barp-seed-speak-root-nursery-empty-text-spawn-unknown-call-pilot.md)
- [Delivery report](DELIVERY_REPORT-2026-08-14-BARP-ROOT-NURSERY-SINGLE-DIGIT-WHOLE-SPAWN-UNKNOWN-CALL-SPEAK.md)
- [BARP design](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
- [BARP validation matrix](../Current%20state/BARP-VALIDATION-MATRIX.md)

---

*End of ADR-122.*
