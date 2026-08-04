# Aether 0.21 Toolchain Contract (M16 resource ↔ handle)

**Status:** Current package contract — language surface remains **0.11** / AETH **v11**; package **0.21** relaxes M4 mix ban for terminal handle  
**Depends on:** [AETHER_0.11.md](AETHER_0.11.md), [ADR-020](ADR-020-m16-resource-effect.md)

## Purpose

Package 0.21 implements **M16 / T-RX** first slice: a **total** weave may own
M2/M6 resources (arena/buffer/table) and use terminal **`handle call`** to
discharge a resource-free `Error[Whole]` callee. Live owners may span the
handle. Abortive `raise`/`forward` and nurseries remain resource-incompatible.

## Example

```aether
world resource_handle

weave ok [] -> Whole raises Whole:
  yield 7

weave main [] -> Whole:
  bind memory <- arena 64
  bind mutable success <- 0
  bind mutable code <- 0
  handle call ok into success otherwise error into code
```

## Rules

| Allowed | Forbidden |
| --- | --- |
| Total weave + arena/buffer/table + terminal `handle` | `raises` weave that owns resources |
| Live Text/Bytes/record/Arena/Buffer across handle | Live `access` loan across handle |
| | `raise`/`forward` with live resources or in resource-owning weave |
| | `together` + resources (M7 unchanged) |

## Diagnostics

`AE-EFFECT-003` covers abortive/resource mixes and access spanning handle.
`AE-TASK-003` still covers nursery/resource mixes.

## Non-goals

Resource-carrying errors, raise-after-free proofs, nursery+arena cancel,
defer/finalizers.

## Evidence

- Design: [DESIGN-M16-RESOURCE-EFFECT.md](DESIGN-M16-RESOURCE-EFFECT.md)
- Matrix: [M16-VALIDATION-MATRIX.md](M16-VALIDATION-MATRIX.md)
- Example: `examples/resource-handle.ae` (exit 7)

*End of AETHER_0.21.md*
