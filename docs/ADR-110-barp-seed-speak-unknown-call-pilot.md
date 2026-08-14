# ADR-110: BARP — seed SPEAK canonical unknown-call pilot

**Status:** Accepted, implemented, and release-verified — `aether-gate.ps1 -Mode release` PASS (2026-08-11; includes the full-quality suite); approved Constitution scanner correction verified
**Date:** 2026-08-11
**Decision makers:** Human director (ongoing Aether maturity direction); implementation agent
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `RND-INVAR-001`,
`SEC-INPUT-001`, `TEST-BEHAVIOR-001`
**Depends on:** ADR-046, ADR-052, ADR-072, ADR-108, ADR-109

## Context

The product path classifies an unknown direct weave call as `AE-SEED-011` after
forge or verification detail. Before this decision, the checked-in seed did not
emit a structured packet for the canonical direct-call form, leaving the
product packet origin as `host-classify` even when the absence of the target
could be witnessed in source without bootstrap analysis.

BARP can move that one exact witness into the seed while preserving forward
declarations and avoiding a second general name binder.

## Decision

1. Extend `seed/aether_seed.ae` after all existing bounded diagnostic pilots
   with a two-pass source-line scan.
2. The first pass enters only a nonblank, canonical-indentation-zero ordinary
   `weave ` header containing `-> Whole:`. It records at most one candidate:
   a trimmed body line starting exactly `bind ` that contains ` <- call ` and a
   nonempty target followed by a source-space argument.
3. The second pass checks canonical indentation-zero declaration headers only.
   A target is known only when a line begins exactly `weave <target> `,
   `export weave <target> `, `host weave <target> `, `foreign weave <target> `,
   or `task weave <target> `. This permits declarations after the call site
   without accepting name prefixes or text content as declarations. It proves
   name existence only; the full compiler retains authority over whether a
   particular declaration kind may be called at that site.
4. If that one candidate has no matching header, the seed SPEAKs one
   `aether.seed-error/v1` packet with `AE-SEED-011`, `origin: "seed-speak"`,
   stable position `1:1`, and blank Bytes.
5. The product forge path merges this exact packet, so the named canonical
   source form now reports `origin: "seed-speak"`. All other call forms remain
   subject to the existing product classifier and full bootstrap diagnostics.
6. Publish the pilot in the seed-SPEAK inventory and BARP matrix. Keep
   `seed_speak_emit_conformance_complete() == false` and
   `seed_internal_error_packets() == false`.

## Invariants

- A canonical ordinary `Whole` direct call whose target has a matching declared
  top-level weave header, including an ordinary forward declaration or a host
  declaration, does not receive an erroneous unknown-target seed packet.
- A Text literal that contains call-shaped characters cannot become a call
  witness.
- Missing `world` remains higher-priority `AE-SEED-006`, even with an otherwise
  canonical unknown call.
- Existing lexical, structural, task, result, and control-flow pilots retain
  their established priority before this scan.
- The scan makes at most two bounded source passes and stores one target Text;
  it does not allocate a declaration table or alter runtime work.
- No source syntax, AETH opcode/version, verifier, VM behavior, host grant,
  network, process, filesystem, shell, or guest authority changes.

## Consequences

Direct forge and the product path now expose one deterministic unknown-call
packet from the Aether-written seed. The change is a narrow name-existence
witness, not general name resolution: no-argument calls, noncanonical layout,
module-qualified paths, signatures, effects, arity, ownership, duplicate
definitions, and source spans remain outside it. Recognizing host, foreign,
task, and exported declaration headers does not approve any special call
semantics; it prevents a wrong unknown-name packet while the full compiler
retains the applicable kind and effect checks.

## Alternatives considered

| Option | Decision |
| --- | --- |
| One canonical direct-call / top-level-header witness | Chosen: bounded, forward-safe, deterministic, and self-hostable. |
| Raw target substring search | Rejected: would treat Text and name prefixes as declarations. |
| Build a complete seed name table | Rejected: broader than the named diagnostic witness and a competing binder. |
| Reject forward calls | Rejected: conflicts with established Aether call semantics. |
| Retain host-only `AE-SEED-011` classification | Rejected: leaves an exact direct seed diagnostic residual. |

## Honesty boundary

This does not claim full `AE-SEED-011` parity, declaration parsing, signature
checking, effect checking, source spans, or general call diagnostics. Remaining
BARP work includes broader `AE-SEED-011` / `AE-SEED-013` forms and seed-native
multi-file elaboration.

## Links

- ADR-052, ADR-072, ADR-108, ADR-109
- [BARP validation matrix](BARP-VALIDATION-MATRIX.md)
- [Seed Profile](SEED_PROFILE.md)
- [Forge contract](FORGE_CONTRACT.md)
- [BARP design](DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)

---

*End of ADR-110.*
