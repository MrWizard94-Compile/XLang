# Aether 0.20 Toolchain Contract (M15 comptime expansion)

**Status:** Historical package contract — language surface remains **0.11** / AETH **v11**; package **0.20** expands M5 comptime; current package contract is [AETHER_0.35.md](AETHER_0.35.md)
**Depends on:** [AETHER_0.11.md](AETHER_0.11.md) (M5), [ADR-019](ADR-019-m15-comptime-expansion.md)

## Purpose

Package 0.20 implements **M15 / T-CT**: `comptime bind` operands may be Whole
literals **or prior root-level immutable comptime Whole names** in the same
weave (source order). Fuel, purity, and `COMPTIME_WHOLE` emission are unchanged
from M5.

## Surface

```aether
world comptime_chain

weave main [] -> Whole:
  comptime bind cell <- product 8 8
  comptime bind row <- product cell 4
  comptime bind header <- sum 16 16
  comptime bind total <- sum row header
  yield total
```

Rules:

- One binary op per directive: `sum` | `difference` | `product` | `quotient` | `remainder`
- Operands: Whole literal **or** prior comptime name (no forward refs, no runtime names)
- Budget: at most 1,024 directives (program-wide)
- Pure: no host I/O, env, or M14 grant observation at compile time
- Result: ordinary immutable Whole local via `COMPTIME_WHOLE`

## Diagnostics

| Code | Meaning |
| --- | --- |
| `AE-COMPTIME-001` | Illegal form (including forward/runtime/unknown name operands) |
| `AE-COMPTIME-002` | Checked arithmetic failure |
| `AE-COMPTIME-003` | Budget exceeded |

## Explicit non-goals

Comptime calls, control flow, text/bytes/Truth results, type computation, macros,
source generation, user-settable fuel.

## Evidence

- Design: [DESIGN-M15-COMPTIME-EXPANSION.md](DESIGN-M15-COMPTIME-EXPANSION.md)
- Matrix: [M15-VALIDATION-MATRIX.md](M15-VALIDATION-MATRIX.md)
- Example: `examples/comptime-chain.ae` (exit 288)
- Seed dual-compare: chain + prior M5 corpus

*End of AETHER_0.20.md*
