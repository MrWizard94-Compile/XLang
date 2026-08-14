# ADR-116: BARP — seed SPEAK zero-argument unknown-call pilot

**Status:** Accepted — implemented and release verified
**Date:** 2026-08-14
**Decision makers:** Human director (ongoing Aether maturity direction); implementation agent
**Related Rule IDs:** CONST-DEP-001, DOC-ADR-001, RND-INVAR-001, SEC-INPUT-001, TEST-BEHAVIOR-001
**Depends on:** ADR-072, ADR-110, ADR-111

## Context

ADR-110 and ADR-111 give the checked-in seed a direct-forge AE-SEED-011 witness
for one canonical argument-bearing bind-call statement and root yield-call. The
language also permits a call to a zero-argument weave: call name. Before this
decision, a canonical zero-argument unknown target reached the seed forge
without a structured packet because the narrow recognizer required a space after
the target name.

The complete compiler remains authoritative for argument types/counts, effects,
host/foreign/task call rules, nested expressions, and non-canonical call forms.
Those are explicitly outside this pilot.

## Decision

1. Extend the existing ordinary-Whole direct-call line-state scan only when a
   canonical bind-call or root yield-call ends immediately after a nonempty
   target name.
2. The recognizer checks that exact target against the existing top-level
   ordinary/export/host/foreign/task weave-header scan. An absent target emits
   exactly one aether.seed-error/v1 packet with AE-SEED-011, origin
   seed-speak, position 1:1, blank Bytes, and the established unknown-call
   message.
3. A zero-argument target declared later remains valid and seed/bootstrap
   byte-identical. Call-shaped Text literals remain outside the scan.
4. Existing higher-priority pilots remain dominant: a missing world reports
   AE-SEED-006, not AE-SEED-011.
5. Argument-bearing ADR-110/111 behavior is unchanged.

## Consequences

- Direct seed forge gains two grammar-proven zero-argument unknown-call
  witnesses without broadening the AE-SEED-011 code family.
- The seed still performs only bounded target-existence scanning. It does not
  parse call expressions or independently check signature, type, effect,
  resource, host, foreign, task, or import semantics.
- No valid source form, AETH byte/version, verifier, VM, forge ABI, grant, or
  host capability changes.

## Alternatives considered

| Option | Pros | Cons |
|---|---|---|
| Exact end-of-line zero-argument target | Covers a common valid grammar form with bounded state | Does not cover expressions or nested calls |
| Parse all direct-call argument lists in the seed | Broader direct diagnostics | Creates an unproven second call parser |
| Leave zero-argument forms host-classified | No seed change | Retains an avoidable deterministic direct-seed gap |

## Honesty boundary

This ADR covers only exact end-of-line target names in canonical ordinary-Whole
direct bind/root-yield statements. It does not claim general name binding,
parser parity, call-signature parity, type/effect/resource diagnostics, source
spans, seed-native multi-file elaboration, or full seed SPEAK conformance.

## Links

- [ADR-072](ADR-072-barp-product-seed-error-packet-abi.md)
- [ADR-110](ADR-110-barp-seed-speak-unknown-call-pilot.md)
- [ADR-111](ADR-111-barp-seed-speak-root-yield-unknown-call-pilot.md)
- [BARP design](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
- [BARP validation matrix](../Current%20state/BARP-VALIDATION-MATRIX.md)

---

*End of ADR-116.*
