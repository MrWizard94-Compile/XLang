# ADR-102: BARP — seed SPEAK lexical-edge pilot

**Status:** Accepted — implemented; full gate PASS

**Date:** 2026-08-11

**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `RND-INVAR-001`,
`SEC-INPUT-001`, `TEST-BEHAVIOR-001`

**Depends on:** ADR-094, ADR-098

## Context

The checked-in seed compiler SPEAKs only four product preflight conditions:
empty source, missing `world`, missing `main`, and raw `import unit`. The BARP
conformance inventory also contains lexical input failures, but full seed
diagnostic parity remains deliberately out of scope.

## Decision

1. Extend the seed's bounded preflight before normal source parsing with two
   exact lexical detections:
   - `AE-SEED-003` when the input begins with an ASCII tab or contains an ASCII
     tab immediately after a line feed; and
   - `AE-SEED-007` when the input begins with `fn ` or contains `fn ` immediately
     after a line feed.
2. Each detection emits one stable `aether.seed-error/v1` SPEAK packet with
   `origin: "seed-speak"`, then prevents lower-priority pilot detections from
   replacing that code.
3. Detection is deliberately source-text lexical: it does not calculate the
   true line/column and reports the stable packet position `1:1`. The payload
   states this bounded condition rather than pretending to offer parser parity.
4. The preflight constructs the sensitive legacy `fn ` needle at runtime so
   the seed source does not match its own input scan during self-host rebuild.
5. Add these two codes to the published seed-SPEAK pilot list and a dedicated
   BARP validation matrix. Keep
   `seed_speak_emit_conformance_complete() == false` and
   `seed_internal_error_packets() == false`.

## Invariants

- A valid source whose string literal merely contains the characters `\\nfn`
  does not match, because the pilot searches actual source line-feed boundaries.
- The seed source and checked-in seed artifact must remain byte-identical under
  bootstrap, product, and forge rebuild proof.
- The guest language, AETH schema/opcodes, host grants, and default product
  authority do not change.

## Honesty

- This is not full `AE-SEED-003` indentation parity: odd-space indentation
  remains host-preflight/classification territory.
- This is not full `AE-SEED-007` legacy-syntax parity: other legacy tokens and
  brace forms remain host-preflight/classification territory.
- No claimed packet line/column accuracy beyond the stable envelope position.

## Links

- ADR-094, ADR-098, [BARP validation matrix](../Current%20state/BARP-VALIDATION-MATRIX.md),
  [Seed Profile](../Current%20state/SEED_PROFILE.md), [BARP design](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)

---

*End of ADR-102.*
