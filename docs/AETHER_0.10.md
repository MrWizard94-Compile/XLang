# Aether 0.10 Language Contract

**Status:** Current executable product contract — M7 structured nursery
concurrency

**Artifact output:** AETH v10

**Compatibility input:** verified AETH v4 through v10 artifacts

## Purpose

Aether 0.10 preserves Aether 0.9 and adds a **lexical structured nursery** with
join, first-failure propagation, and cancel-of-remaining-siblings. Execution is
cooperative and source-ordered. This is not OS-thread parallelism and does not
claim wall-clock speedup.

## New M7 surface

```aether
together:
  spawn call left into a
  spawn call right into b
```

Rules:

- 1 through 8 `spawn call` lines only inside `together:`
- No nested `together`, resources, or other statements in the block
- Spawn destinations are mutable root `Whole` bindings
- Children return `Whole`; if any child `raises Whole`, the parent must too
- First error cancels remaining unstarted spawns and re-raises
- Same clean M4 resource boundary for weaves that use nurseries
- `main` stays total (handle erroring helpers that use nurseries)

Diagnostics: `AE-TASK-001`, `AE-TASK-002`, `AE-TASK-003`.

## AETH v10

Same header layout as v9, plus nursery opcodes:

| Opcode | Role |
| --- | --- |
| `NURSERY_BEGIN` (62) | Start nursery (`count:u8`) |
| `NURSERY_SPAWN` (63) | Spawn/call/capture or cancel-skip |
| `NURSERY_END` (64) | Join; raise if cancelled |

## Authoring v5

`aether.ast/v5`, `aether.edit/v5`, `aether.diagnostic/v5` expose `Together` and
`Spawn`. v4 remains historical.

## Explicit non-goals

Parallel threads, detached tasks, nested nurseries, timeouts, resourceful
tasks, and host scheduler integration. See
[DESIGN-M7-STRUCTURED-CONCURRENCY.md](DESIGN-M7-STRUCTURED-CONCURRENCY.md).
