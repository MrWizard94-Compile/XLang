# ADR-106: BARP - seed SPEAK task-checkpoint pilot

**Status:** Accepted and implemented - full gate PASS
**Date:** 2026-08-11
**Decision makers:** Human director (ongoing Aether maturity direction); implementation agent
**Related Rule IDs:** CONST-DEP-001, DOC-ADR-001, RND-INVAR-001,
SEC-INPUT-001, TEST-BEHAVIOR-001
**Depends on:** ADR-042, ADR-101, ADR-103

## Context

The product path already rejects a task weave with no checkpoint as
AE-SEED-015. Until this decision, that diagnostic was host-only: directly
forging such source through the checked-in Aether seed could reach an opaque
failure. The seed-SPEAK program must reduce that residual without pretending to
reimplement the complete task parser or alter the closed M19e runtime model.

## Decision

1. Add a bounded preflight to seed/aether_seed.ae after the established
   AE-SEED-003/004/005/006/007/012/014 pilots.
2. It recognizes a task header only when the source line begins at canonical
   indentation zero with the exact prefix "task weave ". It tracks that body
   until the next nonblank top-level source line or end of source.
3. A body satisfies the pilot only when an indented source line is exactly
   "checkpoint". Prefixes such as "checkpointed", trailing tokens, and text
   literals do not satisfy it.
4. On the first unsatisfied task, the seed SPEAKs one aether.seed-error/v1
   packet with code "AE-SEED-015", origin "seed-speak", and the stable
   position 1:1. It returns blank Bytes and does not replace a higher-priority
   existing pilot diagnostic.
5. Align the host product preflight to the same canonical top-level and exact
   checkpoint rule. Its generated packet is correctly marked
   origin "host-preflight".
6. Publish the code in the seed-SPEAK pilot list and keep the full conformance
   tracker false.

## Invariants

- Valid M19e source with an exact nested checkpoint, including a task while
  body, reaches normal seed compilation and verified AETH v12.
- A checkpoint-shaped identifier cannot bypass the task safety rule.
- A text literal containing the word checkpoint cannot become a checkpoint.
- Existing pilot precedence is retained: earlier lexical/structural pilots and
  AE-SEED-014 win before this scan.
- No source syntax, AETH opcode/version, verifier, VM behavior, host grant,
  process, filesystem, network, or shell authority changes.

## Consequences

Direct forge now exposes the same bounded cooperative-cancellation prerequisite
as the product path for canonical task headers. This is a diagnostic-authority
reduction only; it does not add handles, timeouts, parallel execution, manual
cancellation, or parser-complete task diagnostics.

## Alternatives considered

| Option | Decision |
| --- | --- |
| Canonical line-state pilot | Chosen: small, literal-safe, deterministic, and directly self-hostable. |
| Prefix match for checkpoint | Rejected: a misspelling could falsely satisfy a safety requirement. |
| Full seed task parser | Rejected: materially broader than the named invariant and not required for this authority reduction. |
| Retain host-only AE-SEED-015 | Rejected: leaves a documented direct-seed diagnostic residual. |

## Honesty boundary

This pilot does not claim source spans, noncanonical/invalid-header
classification, parser parity, all task semantic diagnostics, or a complete
seed-SPEAK matrix. Remaining seed-SPEAK conformance codes are AE-SEED-010,
AE-SEED-011, and AE-SEED-013; seed-native multi-file elaboration remains
separate work.

## Links

- ADR-042, ADR-101, ADR-103
- BARP validation matrix
- BARP design
- Seed Profile

---

*End of ADR-106.*
