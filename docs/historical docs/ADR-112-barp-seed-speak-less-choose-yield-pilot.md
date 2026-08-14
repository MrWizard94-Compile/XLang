# ADR-112: BARP — seed SPEAK canonical less-choose-yield pilot

**Status:** Accepted — implemented and release verified
**Date:** 2026-08-13
**Decision makers:** Human director (ongoing Aether maturity direction); implementation agent
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `RND-INVAR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`
**Depends on:** ADR-070, ADR-072, ADR-109

## Context

ADR-070 already makes product compilation fail closed when a `yield` appears in
any truth-condition `choose` branch: Rust preflight owns that broader safety
boundary because it runs before forge. ADR-109 gave the checked-in Aether seed
one direct-forge witness for the canonical `choose same` form. A canonical
`choose less` branch is a distinct, common comparison form. Before this
decision, direct forge through the seed did not emit the structured
`AE-SEED-013` packet for that source shape.

BARP reduces product-path bootstrap authority by adding bounded seed behavior
only where the seed can prove it without becoming a second parser or changing
the existing product preflight boundary.

## Decision

1. In the existing second source-line scan, the seed recognizes a canonical
   `choose less ` line inside an ordinary `weave ... -> Whole:` body in addition
   to ADR-109's `choose same ` form.
2. It records the comparison branch indentation. A deeper line beginning with
   `yield ` causes exactly one `aether.seed-error/v1` packet with
   `AE-SEED-013`, `origin: "seed-speak"`, stable position `1:1`, and blank
   Bytes output.
3. At adoption, the packet message named the shared bounded comparison-choose
   witness rather than incorrectly naming only the `same` form. ADR-113
   generalizes that shared wording to the truth-choose family.
4. Product compilation retains ADR-070's pre-forge `host-preflight` origin for
   this invalid source. This decision improves direct seed forge diagnosis; it
   does not reorder or relax the safety check.
5. A valid comparison branch that revises a value and yields at the weave root
   must remain seed/bootstrap byte-identical. Resource `choose` forms remain
   outside this scanner.

## Consequences

- Direct use of the checked-in seed reports a structured, deterministic
  `AE-SEED-013` packet for one more high-frequency invalid control-flow form.
- The valid language, AETH bytes, verifier, VM, forge ABI, host capability
  boundary, product preflight, and source-span behavior do not change.
- The seed stays a shallow, bounded line-state recognizer. It does not claim
  general comparison parsing, all truth conditions, nested body analysis,
  source spans, or full diagnostic parity.

## Validation contract

The delivery must prove all of the following:

1. The new integration test is red against the prior checked-in seed and green
   after rebuilding the seed.
2. Direct forge of the canonical `choose less` invalid source emits one
   seed-native `AE-SEED-013` packet.
3. Product compilation retains `AE-SEED-013` with `host-preflight` origin.
4. A root-yield comparison source verifies and matches bootstrap byte-for-byte.
5. Existing `choose same`, resource-choice, missing-world priority, seed
   rebuild, and full release-gate evidence remain green.

## Alternatives considered

| Alternative | Decision |
| --- | --- |
| Add every truth-condition form in one change | Rejected: expands the bounded witness into a competing parser without separate evidence. |
| Move product preflight after forge | Rejected: would weaken ADR-070's fail-closed safety boundary. |
| Leave the form host-only | Rejected: retains a small, deterministic direct-seed diagnostic gap. |
| Change source syntax or AETH control flow | Rejected: diagnostic maturity requires neither. |

## Honesty boundary

This ADR covers only a canonical ordinary-Whole `choose less` branch and a
deeper `yield` statement. It does not claim full `AE-SEED-013` parity, all
comparison/truth syntax, parser parity, general source spans, or seed-native
multi-file elaboration.

## Links

- [ADR-070](ADR-070-product-yield-in-truth-choose-and-multimodule-choose.md)
- [ADR-109](ADR-109-barp-seed-speak-truth-choose-yield-pilot.md)
- [BARP design](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
- [BARP validation matrix](../Current%20state/BARP-VALIDATION-MATRIX.md)

---

*End of ADR-112.*
