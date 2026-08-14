# ADR-120: BARP — seed SPEAK root forward-call unknown-target pilot

**Status:** Accepted — implemented and release verified
**Date:** 2026-08-14
**Decision makers:** Human director (ongoing Aether maturity direction); implementation agent
**Related Rule IDs:** CONST-DEP-001, DOC-ADR-001, RND-INVAR-001, SEC-INPUT-001, TEST-BEHAVIOR-001
**Depends on:** ADR-007, ADR-072, ADR-110, ADR-116, ADR-117, ADR-118, ADR-119

## Context

M4 defines a terminal error-propagation statement:

~~~aether
forward call weave args...
~~~

The bootstrap parser requires the `forward call` form, and semantic validation
requires a root-final caller with `raises Whole` before it resolves the target.
For a syntactically canonical erroring-Whole root, an absent target reaches
`validate_effect_call` and produces the established `weave <name> has not been
declared` failure family.

The checked-in seed has bounded `AE-SEED-011` target-existence witnesses for
canonical total-Whole direct bind, root-yield, root-revise, root-speak, and
root-handle calls. Its existing source-line state deliberately recognizes only
total `-> Whole:` headers, so a canonical `forward call` has no direct
seed-SPEAK witness.

A forward scanner must require the caller's literal erroring-Whole header marker
without attempting to check root terminality, target effect, result
compatibility, argument legality, resource boundary, or the full header grammar.

## Decision

1. In the existing source-line pass, preserve total-Whole state exactly. Add a
   separate state only for a trimmed top-level line beginning `weave ` that
   contains the literal `-> Whole raises Whole:` header marker.
2. In that separate state, the seed recognizes only a trimmed root line
   beginning exactly `forward call `. It extracts an opaque target beginning
   immediately after the prefix and ending at the next ASCII space
   (argument-bearing) or exact end of line (zero-argument).
3. The existing top-level ordinary/export/host/foreign/task header-existence
   scan operates only on that target. It does not validate target error effect,
   result type, arguments, signature, caller terminality, resource boundary, or
   the full erroring header grammar.
4. If no matching header exists, direct forge emits exactly one
   `aether.seed-error/v1` packet with `AE-SEED-011`, `origin: "seed-speak"`,
   position `1:1`, blank Bytes, and the established unknown-call message.
   Product forge preserves the same packet.
5. Later-declared matching `raises Whole` targets remain valid,
   seed/bootstrap byte-identical, verified, and executable for argument-bearing
   and zero-argument forward calls. A total caller, a Text literal, an incomplete
   prefix, and missing `world` remain outside or above this bounded witness.

## Consequences

- The checked-in seed gains one M4 error-caller target-existence witness without
  changing accepted valid source, AETH bytes/versions, VM, verifier, forge ABI,
  dependencies, or host capability.
- The distinct header state avoids extending ADR-110/111/116/117/118/119's
  total-Whole scanner into every erroring weave statement.
- Full M4 semantic analysis remains authoritative for caller signature and
  terminality; target effect/result/signature; copy-only arguments; resource
  boundary; and all ownership rules.

## Validation plan

1. A red regression proves the prior seed did not emit a direct `AE-SEED-011`
   packet for canonical argument-bearing and zero-argument root forward-call
   unknown targets under an erroring-Whole caller.
2. Direct forge proves exactly one seed-native schema/code/message/position/
   origin packet with blank Bytes for both forms.
3. Product compilation proves the merged `AE-SEED-011`/`seed-speak` packet.
4. Later-declared `raises Whole` forwarding helpers behind a total `handle`
   prove seed/bootstrap identity, verification, and execution for both forms.
5. An erroring-weave Text literal, a total-caller forward, an incomplete
   forward prefix, and missing-world source prove literal, caller-state,
   malformed-prefix, and priority boundaries.
6. The rebuilt seed must pass the full release gate, including four-way seed
   identity and technical-preview consumer verification.

## Alternatives considered

| Alternative | Decision |
| --- | --- |
| Scan every `forward ` line | Rejected: it would misclassify non-call or malformed forward syntax. |
| Reuse the total-Whole call scanner | Rejected: it would make an erroring caller silently share total-caller scope. |
| Parse the complete `raises Whole` header and terminal statement position | Rejected: those are parser and M4 semantic responsibilities. |
| Validate target effect, result, arguments, or resources in the scanner | Rejected: those are full M4 semantic responsibilities. |
| Leave forward calls host-only | Rejected: the literal header state and exact call prefix provide a narrow, directly provable witness. |

## Honesty boundary

This ADR covers only a canonical ordinary erroring-Whole root
`forward call target` line with one opaque target, terminated by a next-space
argument boundary or end of line, while the latest top-level line has the
literal `-> Whole raises Whole:` marker. It does not claim full
`AE-SEED-011` parity, full header parsing, terminality, target effect/result
validation, signature/type/argument validation, resource/ownership validation,
source spans beyond the packet's fixed `1:1`, nested calls, all effect forms,
or seed-native multi-file elaboration.

## Links

- [ADR-007](ADR-007-m4-typed-error-effect.md)
- [ADR-072](ADR-072-barp-product-seed-error-packet-abi.md)
- [ADR-110](ADR-110-barp-seed-speak-unknown-call-pilot.md)
- [ADR-116](ADR-116-barp-seed-speak-zero-argument-unknown-call-pilot.md)
- [ADR-117](ADR-117-barp-seed-speak-revise-unknown-call-pilot.md)
- [ADR-118](ADR-118-barp-seed-speak-root-speak-unknown-call-pilot.md)
- [ADR-119](ADR-119-barp-seed-speak-root-handle-unknown-call-pilot.md)
- [Delivery report](DELIVERY_REPORT-2026-08-14-BARP-ROOT-FORWARD-UNKNOWN-CALL-SPEAK.md)
- [BARP design](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
- [BARP validation matrix](../Current%20state/BARP-VALIDATION-MATRIX.md)

---

*End of ADR-120.*
