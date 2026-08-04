# M7 Design: structured nursery concurrency

**Status:** Accepted implementation design for Aether 0.10 / M7

**Date:** 2026-08-03

**Decision record:** [ADR-010](ADR-010-m7-structured-concurrency.md)

**Validation record:** [M7 validation matrix](M7-VALIDATION-MATRIX.md)

## Purpose and boundary

M7 proves that Aether can host a **lexical task group** with join, first-failure
propagation, and cancel-of-remaining-siblings without detached work, a hidden
scheduler, host threads, or ambient exceptions.

Research posture (Trio/Kotlin structured concurrency, M4 abortive effects):
tasks must not outlive their scope; failure is explicit; cancellation is
deterministic. Aether's AETH VM and seed proof require a **cooperative,
source-order** execution model rather than OS parallelism. Parallel speedup is
**not** an M7 claim.

## Core claim and invariants

| ID | Invariant |
| --- | --- |
| M7-INV-001 | A `together` block is a lexical nursery: every `spawn` is declared inside it, and no task survives the block. |
| M7-INV-002 | Spawns run in **source order**, cooperatively. Observable results do not depend on a host scheduler. |
| M7-INV-003 | On the first child `Error[Whole]`, remaining unstarted spawns are **cancelled** (never invoked). The nursery re-raises that code. |
| M7-INV-004 | After a successful nursery, every spawn destination holds its child `Whole` result; after failure, completed destinations keep values and cancelled ones are unchanged. |
| M7-INV-005 | Nursery bodies admit only `spawn call` lines (1..=8). No nested `together`, resources, loans, or other statements inside the block. |
| M7-INV-006 | Child calls return `Whole`. If any child `raises Whole`, the enclosing weave must `raises Whole`. Total children alone keep the parent total. |
| M7-INV-007 | Weaves using `together` obey the M4 clean boundary: no arena, buffer, table, access, or resource outcome in the same weave. |
| M7-INV-008 | `main` remains total: it cannot contain a nursery that may raise; it may only `handle call` an erroring helper that uses `together`. |
| M7-INV-009 | AETH v10 records nursery opcodes; v4–v9 keep immutable meanings. Seed matches bootstrap on the documented corpus. |

## Source surface

```text
together-block ::= "together" ":" newline spawn-line+
spawn-line     ::= indent "spawn call" name atom* "into" name
```

Example (total children):

```aether
world nursery_total

weave left [] -> Whole:
  yield 3

weave right [] -> Whole:
  yield 4

weave main [] -> Whole:
  bind mutable a <- 0
  bind mutable b <- 0
  together:
    spawn call left into a
    spawn call right into b
  yield sum a b
```

Example (first failure cancels remaining):

```aether
world nursery_cancel

weave ok [] -> Whole:
  yield 1

weave boom [] -> Whole raises Whole:
  raise 9

weave later [] -> Whole:
  yield 5

weave work [] -> Whole raises Whole:
  bind mutable a <- 0
  bind mutable b <- 0
  bind mutable c <- 0
  together:
    spawn call ok into a
    spawn call boom into b
    spawn call later into c
  yield sum a c

weave main [] -> Whole:
  bind mutable ok <- 0
  bind mutable code <- 0
  handle call work into ok otherwise error into code
```

On the failure path: `a == 1`, `c` remains `0` (cancelled), `code == 9`, and
`main` yields `9`.

## Semantics

1. Enter `together` with cancel flag clear.
2. For each `spawn` in order:
   - If cancelled, skip (do not call; destination unchanged).
   - Else invoke the child with the same argument rules as ordinary/`forward`
     calls for total vs erroring children (spawn of total uses ordinary call
     semantics; spawn of erroring is only legal when the parent may raise).
   - On normal `Whole` result, store into the destination mutable root `Whole`.
   - On `Error[Whole]`, set cancel, record the code, and continue the spawn
     list only to mark remaining as cancelled (no further calls).
3. After the list: if cancelled, `raise` the recorded code; else continue after
   the block.

There is no resumption, no detached task handle, no `async` coloring, and no
thread pool.

## Diagnostics

| Code | Meaning |
| --- | --- |
| `AE-TASK-001` | Illegal nursery shape (empty, too many spawns, nested together, non-spawn body). |
| `AE-TASK-002` | Spawn target/destination/signature violation. |
| `AE-TASK-003` | Nursery/effect/resource boundary violation (including may-raise in `main`). |

## AETH v10

New compilation emits AETH v10 (same header layout as v9: arena, records,
shapes, functions). v10-only opcodes:

| Opcode | Encoding | Contract |
| --- | --- | --- |
| `NURSERY_BEGIN` (62) | `count:u8` | Push nursery frame expecting `count` spawns. |
| `NURSERY_SPAWN` (63) | `func:u16-le`, `argc:u8`, `dest:u16-le` | If not cancelled, pop `argc` args, call `func`; store `Whole` to `dest` or cancel with error code. |
| `NURSERY_END` (64) | — | Pop frame; if cancelled, `Error[Whole]`; else continue. |

## Seed and authoring

- Seed parses `together` / `spawn call` and emits v10 nursery opcodes
  byte-identically for the corpus.
- Authoring advances to `aether.ast/v5`, `aether.edit/v5`,
  `aether.diagnostic/v5` with `Together` / `Spawn` nodes.
- v4 remains historical and is not silently upgraded.

## Performance and honesty

M7 does **not** claim wall-clock speedup. Its success metric is structured
lifecycle safety: no orphans, deterministic cancel, seed parity, and effect
transparency. Parallel runtimes would need a new ADR.

## Stop conditions

Do not ship M7 if:

1. tasks can detach or outlive `together`;
2. cancellation is racy or host-scheduler dependent;
3. resources and nurseries mix without a destruction model;
4. seed cannot match bootstrap on the documented corpus;
5. `main` or forge can observe an unhandled nursery error as a host exception.
