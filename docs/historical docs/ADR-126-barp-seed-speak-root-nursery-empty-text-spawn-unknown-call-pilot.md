# ADR-126: BARP — seed SPEAK root nursery exact empty Text spawn unknown-target pilot

**Status:** Accepted — implemented and release verified
**Date:** 2026-08-14
**Decision makers:** Human director (ongoing Aether maturity direction); implementation agent
**Related Rule IDs:** CONST-DEP-001, DOC-ADR-001, RND-INVAR-001, SEC-INPUT-001, TEST-BEHAVIOR-001
**Depends on:** ADR-010, ADR-072, ADR-121, ADR-122, ADR-123, ADR-124, ADR-125

## Context

M7 accepts an owned Text literal for an ordinary total nursery callee:

~~~aether
together:
  spawn call count "" into result
~~~

`parse_spawn_statement` parses a spawn call before `validate_spawn` resolves
its target. That validator resolves the target before argument count, type, and
ownership validation. It explicitly permits a matching owned Text literal for a
Text parameter. Therefore the exact valid empty Text atom plus an unknown target
has stable target-existence priority.

M19e task frames intentionally accept only owned Whole or Truth copy parameters.
This ADR therefore validates a later-declared ordinary `weave` with an owned
Text parameter; it does not broaden task-frame ABI or task source surface.
ADR-121 through ADR-125 cover only zero-argument, bounded Whole, and exact Truth
literal children. The smallest Text-family extension is exactly `""`, with no
nonempty or escaped Text parsing.

## Decision

1. Preserve ADR-121's ordinary total-Whole root `together:` prior-line state,
   four-space immediate-child requirement, and ADR-122–125 branch priority.
2. Only if those witnesses did not match, recognize exactly
   `spawn call target "" into destination`.
3. Require a nonempty opaque target, literal `"" into ` immediately after that
   target's first delimiter, and a nonempty destination suffix.
4. Reuse the existing canonical top-level declaration-header scan only for the
   opaque target. Do not parse nonempty/escaped Text, Bytes, names, other
   arguments, task identity, signature, destination, nesting, effect/result,
   resource, ownership, or scheduler policy.
5. If no declaration header matches, direct forge emits exactly one
   `aether.seed-error/v1` packet with `AE-SEED-011`, `origin: "seed-speak"`,
   position `1:1`, blank Bytes, and the established unknown-call message.
   Product forge preserves that packet.
6. A later-declared ordinary `weave worker [message: Text] -> Whole` called with
   `""` remains valid, seed/bootstrap byte-identical, verified, and executable.

## Consequences

- The checked-in seed gains one lexical M7 target-existence witness. It adds no
  accepted-source, AETH, VM, verifier, forge ABI, dependency, task-frame, or
  host-capability surface.
- `""` is a recognition condition, not a Text parser. Nonempty or escaped Text,
  Bytes, names, multiple arguments, and Text task-frame parameters retain the
  full compiler or a separately accepted pilot.
- The branch reuses existing source-line state and Whole/Text scratch state. It
  introduces no seed binding and leaves the `v132` unused-local self-host variant
  probe intact.
- The bootstrap compiler remains authoritative for full Text tokenization and
  escaping, unique-value ownership, task-frame parameter restrictions, argument
  count/type/mode, destination legality, error propagation, resource policy, and
  scheduler behavior.

## Validation plan

1. A red regression proves the ADR-125 seed emitted no direct `AE-SEED-011`
   packet for a canonical empty-Text root-nursery unknown target.
2. Direct forge proves exactly one schema/code/message/position/origin packet
   with blank Bytes; product compilation proves the merged packet.
3. A later-declared ordinary one-Text-parameter weave called with `""` proves
   seed/bootstrap identity, verification, and execution.
4. Nonempty Text, Bytes, a name, repeated empty Text, missing destination,
   delayed, descendant, wrong-parameter, erroring-parent, and missing-world
   sources prove lexical, type, caller-state, delimiter, and priority boundaries.
5. The rebuilt seed must pass the full release gate, including four-way seed
   identity and technical-preview consumer verification.

## Alternatives considered

| Alternative | Decision |
| --- | --- |
| Recognize arbitrary Text literals | Rejected: quote, escape, UTF-8, and length parsing would create lexical/parser authority. |
| Recognize Text names | Rejected: binding and unique-ownership tracking need a separate proof. |
| Permit Text task-frame parameters | Rejected: M19e closes task frames to owned Whole/Truth copy parameters. |
| Validate ordinary weave signature or destination | Rejected: those remain M7 semantic responsibilities. |
| Leave exact empty Text bootstrap-only | Rejected: the literal is valid, useful, and has stable target-resolution priority. |

## Honesty boundary

This ADR covers only an ordinary total-Whole `weave` with a root line exactly
`together:` followed immediately by a four-space
`spawn call target "" into destination` line. It does not claim any task-weave
Text parameter, nonempty or escaped Text, Bytes, names, Truth literals beyond
ADR-124/125, Whole literals beyond ADR-121/122/123, multiple arguments, full
nested parsing, later nursery children, blank-line tolerance, task identity,
destination validation, header parsing, effect/result/signature validation,
resource/ownership policy, scheduler behavior, source spans beyond fixed `1:1`,
full `AE-SEED-011` parity, or seed-native multi-file elaboration.

## Links

- [ADR-010](ADR-010-m7-structured-concurrency.md)
- [ADR-072](ADR-072-barp-product-seed-error-packet-abi.md)
- [ADR-121](ADR-121-barp-seed-speak-root-nursery-zero-argument-spawn-unknown-call-pilot.md)
- [ADR-122](ADR-122-barp-seed-speak-root-nursery-single-digit-whole-spawn-unknown-call-pilot.md)
- [ADR-123](ADR-123-barp-seed-speak-root-nursery-two-digit-positive-whole-spawn-unknown-call-pilot.md)
- [ADR-124](ADR-124-barp-seed-speak-root-nursery-bright-truth-spawn-unknown-call-pilot.md)
- [ADR-125](ADR-125-barp-seed-speak-root-nursery-dim-truth-spawn-unknown-call-pilot.md)
- [Delivery report](DELIVERY_REPORT-2026-08-14-BARP-ROOT-NURSERY-EMPTY-TEXT-SPAWN-UNKNOWN-CALL-SPEAK.md)
- [BARP design](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
- [BARP validation matrix](../Current%20state/BARP-VALIDATION-MATRIX.md)

---

*End of ADR-126.*
