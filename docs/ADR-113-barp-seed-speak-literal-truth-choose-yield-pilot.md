# ADR-113: BARP — seed SPEAK literal Truth choose-yield pilot

**Status:** Accepted — implemented and release verified
**Date:** 2026-08-13
**Decision makers:** Human director (ongoing Aether maturity direction); implementation agent
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `RND-INVAR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`
**Depends on:** ADR-070, ADR-072, ADR-109, ADR-112

## Context

ADR-070 already makes product compilation fail closed when a `yield` appears in
any truth-condition `choose` branch. ADR-109 and ADR-112 give the checked-in
seed direct-forge `AE-SEED-013` witnesses for canonical `choose same` and
`choose less` branches. The language also has exact literal Truth branch forms:
`choose bright:` and `choose dim:`. Before this decision, neither direct seed
forge form emitted the structured packet.

The full product preflight additionally covers bare Truth variables, `not`
expressions, and other truth-condition shapes. Those require parsing or a wider
state model, so they are not candidates for this literal pilot.

## Decision

1. In the existing second source-line scan, the seed recognizes only an exact
   trimmed `choose bright:` or `choose dim:` line inside the existing ordinary
   `weave ... -> Whole:` context.
2. It records the branch indentation. A deeper line beginning with `yield `
   emits exactly one `aether.seed-error/v1` packet with `AE-SEED-013`,
   `origin: "seed-speak"`, position `1:1`, and blank Bytes.
3. The shared diagnostic message changes from comparison-specific wording to
   `yield is not allowed inside a canonical truth choose branch`, accurately
   covering the existing comparison and new literal witnesses.
4. Product compilation retains ADR-070's pre-forge `host-preflight` result for
   these invalid sources. This direct-seed improvement does not reorder or
   relax the safety boundary.
5. Valid `bright` and `dim` branches that revise a value then yield at weave
   root must remain seed/bootstrap byte-identical and execute with their
   expected result. Resource `choose`, bare Truth variables, and `not` remain
   outside this scanner.

## Consequences

- Direct seed forge gains two deterministic, exact literal Truth witnesses for
  one established product diagnostic.
- The direct scan remains a shallow fixed-prefix/state recognizer, not a parser.
- Existing `same` / `less` behavior remains, with one more accurate shared
  message.
- No valid source, AETH byte/version, verifier, VM, forge ABI, grant, or host
  capability changes.

## Alternatives considered

| Option | Pros | Cons |
| --- | --- | --- |
| Exact `bright` and `dim` literals | Small, deterministic, and grammar-proven witnesses | Does not cover variables or unary truth expressions |
| All truth-condition syntax | Broader direct diagnostics | Would create a competing parser without separate proof |
| Move product preflight after forge | One apparent diagnostic source | Weakens ADR-070's fail-closed safety boundary |
| Leave literal forms host-only | No seed change | Retains two predictable direct-seed diagnostic gaps |

## Honesty boundary

This ADR covers only exact `choose bright:` and `choose dim:` lines in a
canonical ordinary-Whole weave, followed by a deeper `yield` line. It does not
claim general Truth-expression parsing, bare variable or `not` coverage, full
`AE-SEED-013` parity, source spans, or seed-native multi-file elaboration.

## Links

- [ADR-070](ADR-070-product-yield-in-truth-choose-and-multimodule-choose.md)
- [ADR-109](ADR-109-barp-seed-speak-truth-choose-yield-pilot.md)
- [ADR-112](ADR-112-barp-seed-speak-less-choose-yield-pilot.md)
- [BARP design](DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
- [BARP validation matrix](BARP-VALIDATION-MATRIX.md)

---

*End of ADR-113.*
