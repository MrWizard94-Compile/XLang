# ADR-121: BARP — seed SPEAK root nursery zero-argument spawn unknown-target pilot

**Status:** Accepted — implemented and release verified
**Date:** 2026-08-14
**Decision makers:** Human director (ongoing Aether maturity direction); implementation agent
**Related Rule IDs:** CONST-DEP-001, DOC-ADR-001, RND-INVAR-001, SEC-INPUT-001, TEST-BEHAVIOR-001
**Depends on:** ADR-010, ADR-072, ADR-110, ADR-116, ADR-120

**Later scoped extensions:** ADR-122 independently adds one immediate
single-digit decimal-Whole argument shape, ADR-123 independently adds one
immediate positive two-digit decimal-Whole argument shape, and ADR-124
independently adds one immediate exact-`bright` Truth argument shape. ADR-125
independently adds one immediate exact-`dim` Truth argument shape. None broadens
this ADR's zero-argument decision. ADR-126 separately adds only one immediate
exact empty-Text argument shape.

## Context

M7 defines a root nursery with nested child calls:

~~~aether
together:
  spawn call worker into result
~~~

The bootstrap parser requires a `spawn call` form with an `into` destination.
Its semantic pass owns task identity, target signature/effect/result, destination,
resource, ownership, and nursery policy validation. For an otherwise canonical
ordinary total-Whole root nursery whose target does not exist, the full compiler
eventually reaches its unknown-weave error family.

The checked-in seed has bounded `AE-SEED-011` target-existence witnesses for
root direct calls, an erroring root forward call, and a delimiter-bounded root
handle call. Its ordinary direct scanner intentionally examines only root
two-space statements. It therefore has no direct seed-SPEAK witness for a
nested nursery child, even when the child is the immediate canonical form.

A nursery scanner must not become a general nested parser. It needs a literal
parent line, one exact child indentation, a zero-argument delimiter shape, and
a nonempty destination suffix while leaving all M7 behavior to the full compiler.

## Decision

1. Preserve every existing direct-call scanner state. Add one prior-line state
   only when a trimmed, ordinary total-Whole top-level `weave ` header is active
   and its root two-space line is exactly `together:`.
2. Only while that state is active for the immediately following physical source
   line, recognize a four-space line beginning exactly `spawn call `.
3. Recognize only the canonical zero-argument shape
   `spawn call target into destination`: the first ASCII space after `target`
   must begin the literal ` into ` delimiter, and a suffix must remain after
   that delimiter. The target is the opaque text before that delimiter.
4. The existing top-level ordinary/export/host/foreign/task declaration-header
   scan operates only on that target. It does not validate the task identity,
   arguments, destination, nesting, header grammar, effect/result/signature,
   resource boundary, ownership, or nursery policy.
5. If no matching declaration header exists, direct forge emits exactly one
   `aether.seed-error/v1` packet with `AE-SEED-011`, `origin: "seed-speak"`,
   position `1:1`, blank Bytes, and the established unknown-call message.
   Product forge preserves the packet.
6. A later-declared checkpointed `task weave` target remains valid,
   seed/bootstrap byte-identical, verified, and executable. Nonempty or escaped
   Text literals,
   erroring parents, malformed delimiters, and missing `world` stay outside or
   above this bounded witness.

## Consequences

- The checked-in seed gains one bounded M7 child target-existence witness with
  no accepted-source, AETH, VM, verifier, forge ABI, dependency, or host
  capability change.
- The one-line state resets on a blank line, any other nonempty line, or a new
  top-level header. It cannot silently scan arbitrary nested or later spawn
  statements.
- The bootstrap compiler remains authoritative for M7 syntax, task-only target
  requirements, child arguments, result/destination compatibility, nesting,
  scheduling, error propagation, resource policy, and ownership.

## Validation plan

1. A red regression proves the prior seed emitted no direct `AE-SEED-011`
   packet for a canonical zero-argument root-nursery unknown target.
2. Direct forge proves exactly one schema/code/message/position/origin packet
   with blank Bytes.
3. Product compilation proves the merged `AE-SEED-011` / `seed-speak` packet.
4. A later-declared checkpointed task helper proves seed/bootstrap identity,
   verification, and execution.
5. A nonempty or escaped Text literal, an erroring parent nursery, intervening blank/non-spawn
   line, six-space descendant, multi-digit or multi-argument child, missing `into` /
   destination suffixes, and missing-world source prove literal, immediate
   parent/child-state, zero-argument, delimiter, and priority boundaries.
6. The rebuilt seed must pass the full release gate, including four-way seed
   identity and technical-preview consumer verification.

## Alternatives considered

| Alternative | Decision |
| --- | --- |
| Scan every `spawn call` line | Rejected: it would grant arbitrary nested source parser authority. |
| Parse arbitrary argument-bearing spawn calls | Rejected: argument/delimiter parsing belongs to the full M7 parser; ADR-122/123 later admit only separately proven one- and positive-two-digit lexical shapes. |
| Scan any descendant of `together:` | Rejected: it would broaden the witness beyond the immediate canonical child. |
| Validate task identity or destination | Rejected: those are semantic M7 responsibilities. |
| Leave nursery targets bootstrap-only | Rejected: the literal parent/child/delimiter shape is narrow and directly provable. |

## Honesty boundary

This ADR covers only an ordinary total-Whole `weave ` with a root line exactly
`together:` followed immediately by a four-space zero-argument
`spawn call target into destination` line. It does not claim general
argument-bearing spawn support (ADR-122 later covers one single-digit shape,
ADR-123 one positive two-digit shape, ADR-124 exact `bright`, ADR-125 exact
`dim`, and ADR-126 exact empty Text), full nested parsing, repeated or later nursery children, blank
line tolerance, destination validation, task identity, header parsing, effect/
result/signature validation, resource/ownership policy, scheduler behavior,
source spans beyond fixed `1:1`, full `AE-SEED-011` parity, or seed-native
multi-file elaboration.

## Links

- [ADR-010](ADR-010-m7-structured-concurrency.md)
- [ADR-072](ADR-072-barp-product-seed-error-packet-abi.md)
- [ADR-110](ADR-110-barp-seed-speak-unknown-call-pilot.md)
- [ADR-116](ADR-116-barp-seed-speak-zero-argument-unknown-call-pilot.md)
- [ADR-120](ADR-120-barp-seed-speak-root-forward-unknown-call-pilot.md)
- [ADR-126](ADR-126-barp-seed-speak-root-nursery-empty-text-spawn-unknown-call-pilot.md)
- [Delivery report](DELIVERY_REPORT-2026-08-14-BARP-ROOT-NURSERY-SPAWN-UNKNOWN-CALL-SPEAK.md)
- [BARP design](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
- [BARP validation matrix](../Current%20state/BARP-VALIDATION-MATRIX.md)

---

*End of ADR-121.*
