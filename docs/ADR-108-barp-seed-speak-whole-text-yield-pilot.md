# ADR-108: BARP — seed SPEAK Whole Text-yield pilot

**Status:** Accepted and implemented — full/release gate PASS
**Date:** 2026-08-11
**Decision makers:** Human director (ongoing Aether maturity direction); implementation agent
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `RND-INVAR-001`,
`SEC-INPUT-001`, `TEST-BEHAVIOR-001`
**Depends on:** ADR-046, ADR-072, ADR-098, ADR-106

## Context

The product path already classifies a canonical Text literal returned from a
`Whole` weave as `AE-SEED-010`. Before this decision, a direct forge through the
checked-in Aether seed could emit an opaque type or verifier failure instead of
the structured seed-SPEAK packet. BARP needs to reduce that residual without
claiming general type-diagnostic parity or teaching the seed a second type
checker.

## Decision

1. Extend `seed/aether_seed.ae` after all established lexical, structural,
   reserved-task, and task-checkpoint pilots with a line-aware bounded scan.
2. The scan recognizes an ordinary source header only when a nonblank,
   canonical-indentation-zero line starts exactly with `weave ` and contains
   `-> Whole:`. It tracks that weave body until the next nonblank top-level
   source line or end of source.
3. While tracking that body, an indented trimmed source line starting exactly
   with `yield "` emits one `aether.seed-error/v1` packet with code
   `AE-SEED-010`, `origin: "seed-speak"`, and the stable position `1:1`.
   The seed returns blank Bytes and does not replace a higher-priority pilot.
4. On the product forge path, the established packet merge preserves this exact
   seed-emitted packet with `origin: "seed-speak"`. Other `AE-SEED-010` forms
   remain host-classified unless a later bounded pilot expressly covers them.
5. A `Text`-return weave, a `speak` statement, text that merely contains the
   characters `yield`, task weaves, and noncanonical headers are outside this
   bounded pilot. They remain the existing compiler/product-diagnostics domain.
6. Publish the pilot in the seed-SPEAK inventory and BARP matrix. Keep
   `seed_speak_emit_conformance_complete() == false` and
   `seed_internal_error_packets() == false`.

## Invariants

- A valid `Text`-return weave may yield a Text literal and reaches normal seed
  compilation.
- A valid `Whole` weave may `speak` a Text literal and then yield a Whole; it
  does not trigger this pilot.
- A missing `world` remains `AE-SEED-006`, even if its prospective `Whole`
  weave body has `yield "..."`.
- The seed source rebuild remains byte-identical under bootstrap, product, and
  forge proof.
- No source syntax, AETH opcode/version, verifier, VM behavior, host grant,
  network, process, filesystem, or shell authority changes.

## Consequences

Direct seed forge now has one deterministic, structured type-failure witness:
the canonical Text-literal result mismatch in an ordinary `Whole` weave. The
product forge path preserves that exact seed packet; the broader
`AE-SEED-010` family remains host-classified. This is diagnostic authority
reduction only, not a general semantic-authority change.

## Alternatives considered

| Option | Decision |
| --- | --- |
| Canonical line-state Text-literal pilot | Chosen: small, literal-safe, deterministic, and self-hostable. |
| Raw substring `yield "` scan | Rejected: would falsely reject `Text`-return weaves and text-bearing statements. |
| Full seed type checker | Rejected: materially broader than the named invariant and would create a competing semantic authority. |
| Retain host-only `AE-SEED-010` | Rejected: leaves a documented direct-seed diagnostic residual. |

## Honesty boundary

This does not claim Truth, name, call, arithmetic, control-flow, parameter,
record, resource, effect, or general return-type parity. It does not recognize
every spelling/formatting error and reports stable packet position `1:1`, not a
precise source span. Remaining seed-SPEAK conformance codes are
`AE-SEED-011` and `AE-SEED-013`; seed-native multi-file elaboration remains
separate work.

## Links

- ADR-046, ADR-072, ADR-098, ADR-106
- [BARP validation matrix](BARP-VALIDATION-MATRIX.md)
- [Seed Profile](SEED_PROFILE.md)
- [BARP design](DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)

---

*End of ADR-108.*
