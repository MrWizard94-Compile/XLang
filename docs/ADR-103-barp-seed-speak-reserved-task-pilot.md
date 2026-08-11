# ADR-103: BARP — seed SPEAK reserved-task surface pilot

**Status:** Accepted and implemented — full gate PASS
**Date:** 2026-08-11
**Decision makers:** Human director (ongoing Aether maturity direction); implementation agent
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `RND-INVAR-001`,
`SEC-INPUT-001`, `TEST-BEHAVIOR-001`
**Depends on:** ADR-081, ADR-089, ADR-098, ADR-102

## Context

The product path already rejects reserved task-runtime proposals with
`AE-SEED-014`: `timeout `, `task handle `, `handle task `,
`parallel together`, and `together parallel`. The Aether-written seed compiler
can otherwise reach opaque or misleading direct-forge outcomes for that surface.
The full seed diagnostic matrix remains deliberately incomplete, and a broad
semantic reimplementation would be higher-risk than the bounded proof needed
to mature the product compiler.

## Decision

1. Extend the seed preflight with a line-aware, canonical-lowercase scanner.
   After removing leading ASCII spaces from each source line, it detects exactly
   these five prefixes: `timeout `, `task handle `, `handle task `,
   `parallel together`, and `together parallel`.
2. On the first match, emit one stable `aether.seed-error/v1` SPEAK packet with
   `code: "AE-SEED-014"`, `origin: "seed-speak"`, and a bounded-reserved-surface
   message. The seed returns blank Bytes and does not continue into its normal
   compiler body.
3. Preserve existing seed-pilot precedence: this scan runs only after the
   established empty, tab, legacy-`fn`, raw-import, missing-world, and
   missing-main pilot checks have not fired.
4. The scanner operates on trimmed source lines rather than raw substrings, so
   a `speak` Text literal that contains a reserved phrase cannot trigger it.
5. Publish `AE-SEED-014` in the pilot list and BARP validation matrix. Keep
   `seed_speak_emit_conformance_complete() == false` and
   `seed_internal_error_packets() == false`.

## Consequences

- Direct forge gets a deterministic, structured fail-closed diagnostic for the
  documented canonical reserved-task prefixes.
- No task handles, timeouts, parallel execution, new AETH opcodes, VM behavior,
  host grant, or language surface is added.
- The host preflight remains the broader authority: this seed pilot does not
  claim Unicode-whitespace trimming, case-insensitive matching, complete task
  parsing, source-span precision, or parity for future task syntax.

## Alternatives considered

| Option | Pros | Cons |
| --- | --- | --- |
| Canonical-lowercase line-prefix pilot | Deterministic, literal-safe, small seed change with direct proof | Deliberately narrower than host classification |
| Full case-insensitive parser-equivalent seed classification | Closer host parity | Larger, riskier seed-language implementation without a full matrix design |
| Raw substring scan | Smallest implementation | False positives inside Text literals; violates the lexical invariant |
| Leave host-only guard | No seed risk | Does not reduce direct-seed diagnostic residual |

## Links

- Related ADRs: ADR-081, ADR-089, ADR-098, ADR-102
- Related code: `seed/aether_seed.ae`,
  `seed_reject_reserved_task_future_surface()` in `crates/xlang-core/src/lib.rs`
- Related validation: [BARP validation matrix](BARP-VALIDATION-MATRIX.md)

---

*End of ADR-103.*
