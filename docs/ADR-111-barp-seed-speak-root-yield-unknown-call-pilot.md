# ADR-111: BARP — seed SPEAK canonical root-yield unknown-call pilot

**Status:** Accepted — implemented and release verified
**Date:** 2026-08-11
**Decision makers:** Human director (ongoing Aether maturity direction); implementation agent
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `RND-INVAR-001`,
`SEC-INPUT-001`, `TEST-BEHAVIOR-001`
**Depends on:** ADR-046, ADR-052, ADR-072, ADR-110

## Context

ADR-110 moved one canonical direct `bind … <- call target value` unknown-name
witness into the checked-in seed. A canonical root `yield call target value`
form is a distinct statement shape and still falls through to the broader
product classifier. It is common in compact Aether helpers and can use the same
bounded target-existence proof without building a general seed name binder.

## Decision

1. Extend the ADR-110 source-line scan after the existing bounded seed pilots.
2. In an indentation-zero ordinary `weave … -> Whole:` body, the first pass may
   record at most one two-space-indented candidate that is either the existing
   direct bind form or an exact `yield call target value` form. Both forms
   require a nonempty target followed by a source-space argument.
3. The existing second pass recognizes an exact indentation-zero declaration
   header for that target: `weave <target> `, `export weave <target> `,
   `host weave <target> `, `foreign weave <target> `, or `task weave <target> `.
   This establishes target existence only. The full compiler retains authority
   over call kind, effect, arity, ownership, and result legality.
4. If the candidate has no matching header, the seed SPEAKs exactly one
   `aether.seed-error/v1` packet with `AE-SEED-011`, `origin: "seed-speak"`,
   stable position `1:1`, and blank Bytes.
5. Product forge merges the emitted packet unchanged. All other `call` forms
   remain under existing product classification and full bootstrap diagnostics.

## Invariants

- A canonical root `yield call` with an unknown target emits seed-native
  `AE-SEED-011`.
- A matching ordinary forward declaration and a matching host declaration do
  not receive an erroneous unknown-target packet.
- ADR-110's direct-bind witness remains unchanged.
- Text literals that contain `yield call` characters cannot become candidates.
- Missing `world` remains the higher-priority `AE-SEED-006` packet.
- The scan remains two bounded source passes and stores at most one target Text;
  it creates no declaration table and changes no VM work.
- No source syntax, AETH format, verifier, VM, grants, filesystem, process,
  network, shell, model, or guest authority changes.

## Consequences

Direct forge and the product path gain one additional deterministic
unknown-target witness from the Aether-written seed. This remains a narrow
source-shape proof, not general name resolution. Nested calls, no-argument
calls, `handle`/`forward`/`spawn` forms, module-qualified paths, signatures,
effects, arity, ownership, duplicate declarations, and source spans remain
outside the pilot.

## Alternatives considered

| Option | Decision |
| --- | --- |
| Add one canonical root-yield form to ADR-110's scanner | Chosen: high-frequency expression form, bounded implementation, existing target-boundary proof. |
| Implement all call statement forms | Rejected: turns this diagnostic witness into a competing general binder. |
| Retain host-only `AE-SEED-011` classification | Rejected: leaves a simple direct source witness outside seed authority. |
| Change the language or AETH call representation | Rejected: no language/runtime change is required for diagnostic maturity. |

## Honesty boundary

This does not claim full `AE-SEED-011` parity, source spans, declaration
parsing, general name binding, signature checking, effect checking, or general
call diagnostics. Broader `AE-SEED-011` / `AE-SEED-013` families and seed-native
multi-file elaboration remain BARP work.

## Links

- [ADR-110](ADR-110-barp-seed-speak-unknown-call-pilot.md)
- [BARP validation matrix](BARP-VALIDATION-MATRIX.md)
- [Seed Profile](SEED_PROFILE.md)
- [Forge contract](FORGE_CONTRACT.md)
- [BARP design](DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)

---

*End of ADR-111.*
