# ADR-115: BARP — seed SPEAK Whole Truth-literal yield pilot

**Status:** Accepted — implemented and release verified
**Date:** 2026-08-14
**Decision makers:** Human director (ongoing Aether maturity direction); implementation agent
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `RND-INVAR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`
**Depends on:** ADR-046, ADR-072, ADR-108

## Context

ADR-108 gives the checked-in seed a direct-forge `AE-SEED-010` witness for a
Text literal returned from a canonical ordinary `Whole` weave. The language
also has two exact Truth-literal return lines: `yield bright` and `yield dim`.
Before this decision, direct forge emitted no structured packet for either
form, and the product path classified each as host-side `AE-SEED-010` after
forge or verification detail.

The full compiler covers Truth variables, calls, unary expressions, other
result-type mismatches, and type interactions. Those require semantic analysis
and remain outside this fixed-line diagnostic pilot.

## Decision

1. Extend ADR-108's existing source-line scan only while it is inside a
   canonical ordinary `weave ... -> Whole:` body.
2. An exact trimmed `yield bright` or `yield dim` line emits exactly one
   `aether.seed-error/v1` packet with `AE-SEED-010`, `origin: "seed-speak"`,
   position `1:1`, blank Bytes, and the message `Whole weave cannot yield a
   Truth literal`.
3. A `Truth`-return weave with the same return lines remains valid and reaches
   normal seed compilation. Text literals, Truth variables, unary forms, calls,
   and all other return expressions remain outside this exact-literal scan.
4. Product forge merges the exact seed packet so these two source forms report
   `origin: "seed-speak"`. Other `AE-SEED-010` forms remain host-classified
   unless a later bounded pilot expressly covers them.
5. Existing higher-priority pilots remain dominant; a missing `world` remains
   `AE-SEED-006` even if the source includes one of these return lines.

## Consequences

- Direct seed forge gains two deterministic, exact Truth-literal result-type
  witnesses for the established `AE-SEED-010` family.
- The seed remains a line-state recognizer here, not a type checker or an
  expression parser.
- Existing Text-literal behavior from ADR-108 remains unchanged and preserves
  its Text-specific message.
- No valid source, AETH byte/version, verifier, VM, forge ABI, grant, or host
  capability changes.

## Alternatives considered

| Option | Pros | Cons |
| --- | --- | --- |
| Exact `yield bright` / `yield dim` lines | Small, deterministic, and grammar-proven witnesses | Does not cover variables, calls, or unary expressions |
| General Whole-return type checking | Broader direct diagnostics | Creates a competing semantic/type checker without separate proof |
| Change the existing Text packet message | One shared message | Loses the accurate existing Text-specific diagnostic |
| Leave Truth literals host-classified | No seed change | Retains two predictable direct-seed diagnostic gaps |

## Honesty boundary

This ADR covers only exact `yield bright` and `yield dim` lines inside a
canonical ordinary-Whole weave. It does not claim Truth-variable, `not`, call,
name, arithmetic, resource, effect, general return-type, source-span, or
seed-native multi-file diagnostic parity.

## Links

- [ADR-072](ADR-072-product-seed-error-packet-abi.md)
- [ADR-108](ADR-108-barp-seed-speak-whole-text-yield-pilot.md)
- [BARP design](DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
- [BARP validation matrix](BARP-VALIDATION-MATRIX.md)

---

*End of ADR-115.*
