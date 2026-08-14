# ADR-109: BARP — seed SPEAK canonical truth-choose-yield pilot

**Status:** Accepted and implemented — product full/release gate PASS; approved Constitution scanner correction verified
**Date:** 2026-08-11
**Decision makers:** Human director (ongoing Aether maturity direction); implementation agent
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `RND-INVAR-001`,
`SEC-INPUT-001`, `TEST-BEHAVIOR-001`
**Depends on:** ADR-046, ADR-070, ADR-072, ADR-106, ADR-108

## Context

The product path rejects a `yield` nested inside a truth-condition `choose`
with `AE-SEED-013` before forge. That fail-closed host preflight remains
essential because the seed historically could emit broken control-flow bytes for
this invalid source family. Direct forge through the checked-in seed, however,
did not provide one structured seed-SPEAK witness for the most common canonical
`choose same` shape.

BARP can reduce that direct-seed diagnostic residual without making the seed a
general control-flow parser or relaxing the product's existing safety boundary.

## Decision

1. Extend `seed/aether_seed.ae` with a bounded line-state scan after the
   existing lexical, structural, and reserved-task pilots and before the
   task-checkpoint/Text-result pilots.
2. The scan enters only a nonblank, canonical-indentation-zero ordinary
   `weave ` header that contains `-> Whole:`. It leaves that body at the next
   nonblank top-level line.
3. Inside that body it arms only on a trimmed line starting exactly with
   `choose same `. It keeps the condition active through the matching
   `otherwise:` line and clears it when another nonblank line returns to or
   above the `choose` indentation.
4. While the condition is active, an indented line starting exactly with
   `yield ` emits one `aether.seed-error/v1` packet with code `AE-SEED-013`,
   `origin: "seed-speak"`, stable position `1:1`, and blank Bytes.
5. The existing product `AE-SEED-013` preflight remains authoritative for the
   full invalid family and therefore continues to produce its
   `origin: "host-preflight"` packet on the normal CLI product path. The new
   direct seed witness does not weaken that fail-closed behavior.
6. Publish the pilot in the seed-SPEAK inventory and BARP matrix. Keep
   `seed_speak_emit_conformance_complete() == false` and
   `seed_internal_error_packets() == false`.

## Invariants

- A `choose same` body that revises a local and reaches a root-level `yield`
  remains valid and reaches normal seed compilation.
- Resource `choose allocate` / `append` / `at` branches retain their permitted
  nested yields and do not trigger this truth-condition witness.
- Missing `world` remains higher-priority `AE-SEED-006` even if the text also
  has the canonical invalid `choose same` shape.
- Existing task-checkpoint and Text-literal pilots retain their documented
  scope; source that matches multiple malformed categories gets the earlier
  fail-closed condition.
- No source syntax, AETH opcode/version, verifier, VM behavior, host grant,
  network, process, filesystem, shell, or guest authority changes.

## Consequences

Direct seed forge now has one deterministic, structured control-flow failure
witness for the canonical `choose same` nested-yield form. Product CLI behavior
remains safely fail-closed through its established host preflight, while the
broader `AE-SEED-013` family—`less`, bare Truth conditions, noncanonical source,
and exact source spans—remains outside this pilot.

## Alternatives considered

| Option | Decision |
| --- | --- |
| Canonical ordinary-Whole / `choose same` line-state witness | Chosen: bounded, literal-safe, deterministic, and self-hostable. |
| Raw `choose` / `yield` substring match | Rejected: would confuse resource forms, Text, and unrelated source. |
| Full seed control-flow parser | Rejected: broader than the named authority-reduction invariant and a competing semantic authority. |
| Remove the product preflight | Rejected: it would expand the seed's responsibility before the full invalid family is proven. |
| Retain only host classification | Rejected: leaves the direct forge witness absent. |

## Honesty boundary

This does not claim full `AE-SEED-013` parity, source spans, a stack parser,
all Truth forms, or any change to valid Aether control flow. Remaining BARP
work includes `AE-SEED-011`, broader `AE-SEED-013` cases, and seed-native
multi-file elaboration.

## Links

- ADR-070, ADR-072, ADR-106, ADR-108
- [BARP validation matrix](../Current%20state/BARP-VALIDATION-MATRIX.md)
- [Seed Profile](../Current%20state/SEED_PROFILE.md)
- [BARP design](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)

---

*End of ADR-109.*
