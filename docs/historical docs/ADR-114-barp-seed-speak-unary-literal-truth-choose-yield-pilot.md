# ADR-114: BARP — seed SPEAK unary literal Truth choose-yield pilot

**Status:** Accepted — implemented and release verified
**Date:** 2026-08-14
**Decision makers:** Human director (ongoing Aether maturity direction); implementation agent
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `RND-INVAR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`
**Depends on:** ADR-070, ADR-072, ADR-109, ADR-112, ADR-113

## Context

ADR-070 already makes product compilation fail closed when a `yield` appears in
any truth-condition `choose` branch. ADR-109, ADR-112, and ADR-113 give the
checked-in seed direct-forge `AE-SEED-013` witnesses for canonical `choose
same`, `choose less`, `choose bright:`, and `choose dim:` branches. The grammar
also accepts exact unary literal Truth branches: `choose not bright:` and
`choose not dim:`. Before this decision, neither direct seed forge form emitted
the structured packet.

The full product preflight additionally covers unary Truth variables, nested
unary expressions, and other truth-condition shapes. Those require expression
parsing or a wider state model, so they remain outside this direct diagnostic
pilot.

## Decision

1. In the existing second source-line scan, the seed recognizes only an exact
   trimmed `choose not bright:` or `choose not dim:` line inside the existing
   ordinary `weave ... -> Whole:` context.
2. It records the branch indentation. A deeper line beginning with `yield `
   emits exactly one `aether.seed-error/v1` packet with `AE-SEED-013`,
   `origin: "seed-speak"`, position `1:1`, and blank Bytes.
3. The established shared diagnostic message remains `yield is not allowed
   inside a canonical truth choose branch`.
4. Product compilation retains ADR-070's pre-forge `host-preflight` result for
   these invalid sources. This direct-seed improvement does not reorder or
   relax the safety boundary.
5. Valid unary literal branches that revise a value then yield at weave root
   must remain seed/bootstrap byte-identical and execute with their expected
   result. Unary Truth variables such as `choose not flag:` remain outside this
   exact-literal scanner.

## Consequences

- Direct seed forge gains two deterministic, exact unary literal Truth
  witnesses for one established product diagnostic.
- The direct scan remains a shallow fixed-prefix/state recognizer, not an
  expression parser.
- Existing comparison and literal Truth behavior remains unchanged.
- No valid source, AETH byte/version, verifier, VM, forge ABI, grant, or host
  capability changes.

## Alternatives considered

| Option | Pros | Cons |
| --- | --- | --- |
| Exact unary literal `not bright` / `not dim` | Small, deterministic, grammar-proven witnesses | Does not cover variable or nested unary operands |
| All unary Truth expressions | Broader direct diagnostics | Would create a competing expression parser without separate proof |
| Move product preflight after forge | One apparent diagnostic source | Weakens ADR-070's fail-closed safety boundary |
| Leave unary literal forms host-only | No seed change | Retains two predictable direct-seed diagnostic gaps |

## Honesty boundary

This ADR covers only exact `choose not bright:` and `choose not dim:` lines in
a canonical ordinary-Whole weave, followed by a deeper `yield` line. It does
not claim general unary-expression parsing, bare variable or nested-`not`
coverage, full `AE-SEED-013` parity, source spans, or seed-native multi-file
elaboration.

## Links

- [ADR-070](ADR-070-product-yield-in-truth-choose-and-multimodule-choose.md)
- [ADR-109](ADR-109-barp-seed-speak-truth-choose-yield-pilot.md)
- [ADR-112](ADR-112-barp-seed-speak-less-choose-yield-pilot.md)
- [ADR-113](ADR-113-barp-seed-speak-literal-truth-choose-yield-pilot.md)
- [BARP design](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
- [BARP validation matrix](../Current%20state/BARP-VALIDATION-MATRIX.md)

---

*End of ADR-114.*
