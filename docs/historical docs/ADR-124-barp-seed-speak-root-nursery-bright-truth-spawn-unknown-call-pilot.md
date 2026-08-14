# ADR-124: BARP — seed SPEAK root nursery exact bright Truth spawn unknown-target pilot

**Status:** Accepted — implemented and release verified
**Date:** 2026-08-14
**Decision makers:** Human director (ongoing Aether maturity direction); implementation agent
**Related Rule IDs:** CONST-DEP-001, DOC-ADR-001, RND-INVAR-001, SEC-INPUT-001, TEST-BEHAVIOR-001
**Depends on:** ADR-010, ADR-072, ADR-121, ADR-122, ADR-123

**Later scoped extension:** ADR-125 independently adds only the immediate exact
`dim` Truth argument shape. It does not broaden this ADR's exact-`bright`
decision.

**Later scoped extension:** ADR-126 independently adds only the immediate exact
empty-Text argument shape. It does not broaden this ADR's exact-`bright`
decision.

## Context

M7 accepts owned copy `Truth` task parameters, including the canonical literal
form:

~~~aether
together:
  spawn call count bright into result
~~~

`parse_spawn_statement` parses a spawn call before `validate_spawn` resolves
its target. That validator resolves the target before task identity, argument
count, mode, or type validation. Therefore an exact valid `bright` atom plus an
unknown target has stable target-existence priority. The later ADR-125 `dim`
extension proves only its own literal; names, unary forms, and general argument
syntax need separately bounded evidence.

ADR-121 through ADR-123 cover only zero-argument and decimal-Whole literal
children. The smallest type-family extension is one literal `Truth` token:
exactly `bright`, with no general `Truth` expression, name, or `dim` parsing.

## Decision

1. Preserve ADR-121's ordinary total-Whole root `together:` prior-line state,
   four-space immediate-child requirement, and ADR-122/123 branch priority.
2. Only if those witnesses did not match, recognize exactly
   `spawn call target bright into destination`.
3. Require a nonempty opaque target, literal `bright into ` immediately after
   that target's first delimiter, and a nonempty destination suffix.
4. Reuse the existing canonical top-level declaration-header scan only for the
   opaque target. Do not parse any other argument form or validate task identity,
   signature, destination, nesting, effect/result, resource, ownership, or
   scheduler policy.
5. If no declaration header matches, direct forge emits exactly one
   `aether.seed-error/v1` packet with `AE-SEED-011`, `origin: "seed-speak"`,
   position `1:1`, blank Bytes, and the established unknown-call message.
   Product forge preserves that packet.
6. A later-declared checkpointed `task weave worker [flag: Truth] -> Whole`
   called with `bright` remains valid, seed/bootstrap byte-identical, verified,
   and executable.

## Consequences

- The checked-in seed gains one lexical M7 target-existence witness. It adds no
  accepted-source, AETH, VM, verifier, forge ABI, dependency, or host-capability
  surface.
- `bright` is a recognition condition, not a general `Truth` parser. Exact
  `dim` is separately ADR-125; names, `not`, other expressions, and
  multi-argument forms retain the full compiler or a separately accepted pilot.
- The branch reuses existing source-line state and Whole/Text scratch state. It
  introduces no seed binding and leaves the `v132` unused-local self-host variant
  probe intact.
- The bootstrap compiler remains authoritative for complete expression syntax,
  task-only target requirements, argument count/type/mode, destination legality,
  error propagation, resource policy, ownership, and scheduler behavior.

## Validation plan

1. A red regression proves the ADR-123 seed emitted no direct `AE-SEED-011`
   packet for a canonical `bright` root-nursery unknown target.
2. Direct forge proves exactly one schema/code/message/position/origin packet
   with blank Bytes; product compilation proves the merged packet.
3. A later-declared checkpointed one-Truth-parameter task called with `bright`
   proves seed/bootstrap identity, verification, and execution.
4. Names, `not dim`, a repeated `bright` argument, missing destination, delayed,
   descendant, non-task-target, wrong-parameter, erroring-parent, and
   missing-world sources prove lexical, signature, caller-state, delimiter, and
   priority boundaries.
5. The rebuilt seed must pass the full release gate, including four-way seed
   identity and technical-preview consumer verification.

## Alternatives considered

| Alternative | Decision |
| --- | --- |
| Recognize `bright` and `dim` together | Rejected: each literal remains an independently provable scope. |
| Recognize arbitrary `Truth` expressions | Rejected: that becomes parser/type-checker authority. |
| Recognize `Truth` names | Rejected: binding and scope tracking need a separate proof. |
| Validate task signature or destination | Rejected: those remain M7 semantic responsibilities. |
| Leave exact `bright` bootstrap-only | Rejected: the literal is valid, useful, and has stable target-resolution priority. |

## Honesty boundary

This ADR covers only an ordinary total-Whole `weave` with a root line exactly
`together:` followed immediately by a four-space
`spawn call target bright into destination` line. It does not claim exact `dim`
beyond ADR-125, exact empty Text beyond ADR-126, names, `not`, general `Truth` expressions, Whole literals beyond ADR-121/122/123,
multiple arguments, full nested parsing, later nursery children, blank-line
tolerance, task identity, destination validation, header parsing,
effect/result/signature validation, resource/ownership policy, scheduler
behavior, source spans beyond fixed `1:1`, full `AE-SEED-011` parity, or
seed-native multi-file elaboration.

## Links

- [ADR-010](ADR-010-m7-structured-concurrency.md)
- [ADR-072](ADR-072-barp-product-seed-error-packet-abi.md)
- [ADR-121](ADR-121-barp-seed-speak-root-nursery-zero-argument-spawn-unknown-call-pilot.md)
- [ADR-122](ADR-122-barp-seed-speak-root-nursery-single-digit-whole-spawn-unknown-call-pilot.md)
- [ADR-123](ADR-123-barp-seed-speak-root-nursery-two-digit-positive-whole-spawn-unknown-call-pilot.md)
- [ADR-125](ADR-125-barp-seed-speak-root-nursery-dim-truth-spawn-unknown-call-pilot.md)
- [ADR-126](ADR-126-barp-seed-speak-root-nursery-empty-text-spawn-unknown-call-pilot.md)
- [Delivery report](DELIVERY_REPORT-2026-08-14-BARP-ROOT-NURSERY-BRIGHT-TRUTH-SPAWN-UNKNOWN-CALL-SPEAK.md)
- [BARP design](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
- [BARP validation matrix](../Current%20state/BARP-VALIDATION-MATRIX.md)

---

*End of ADR-124.*
