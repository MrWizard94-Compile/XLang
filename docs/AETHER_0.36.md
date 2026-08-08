# Aether 0.36 Toolchain Contract (M19e active-frame cancellation)

**Status:** Current workspace package contract
**Language surface:** Aether **0.11** plus the bounded M19e `task weave` /
`checkpoint` forms
**Artifact output:** AETH **v12** for M19e source; AETH **v11** when source has
no task frame
**Authoring:** `aether.ast/v8`, `aether.edit/v8`, `aether.diagnostic/v8`
**Decision:** [ADR-042](ADR-042-m19e-active-frame-cancel.md)
**Design:** [M19e active-frame cancellation and destruction](DESIGN-M19E-T-RX-ACTIVE-FRAME-CANCEL.md)
**Validation:** [M19e validation matrix](M19E-VALIDATION-MATRIX.md)

## What changed

Package 0.36 implements the M19e vertical slice: deterministic cancellation of
an already-started, resource-owning child frame at an explicit safe point. It
adds exactly these source forms:

```aether
task weave worker [] -> Whole:
  checkpoint
  yield 1
```

`task` is a semantic role, not an advisory annotation. A task is a non-`main`,
total guest weave returning `Whole`; parameters may be only owned `Whole` or
`Truth`; it may own at most one direct root arena; and it must contain a
`checkpoint`. Its `while` bodies must begin directly with `checkpoint` so every
generated backward edge is verifier-checkable.

Tasks are callable only through `spawn call` inside a checkpointed `together`
nursery. Their closed subset rejects ordinary, handle, forward, host, foreign,
stdout, comptime, effect, nested-nursery, task-handle, timeout, and external
cancellation forms. A checkpointed nursery permits only eligible task children
and resource-free Copy-only companions; an erroring companion requires an
`Error[Whole]` parent.

## Runtime behavior

A checkpointed nursery is single-threaded and deterministic. It visits children
in source-order round robin. A task runs until `checkpoint` or `yield`; a
companion runs to `yield` or `raise`.

On the first companion error, the VM:

1. preserves every result already committed by a completed child;
2. leaves destinations for pending and cancelled children unchanged;
3. destroys each parked task's live locals in descending slot order;
4. zeroes and revokes each private task lane, then zeroes and releases the
   nursery slab after all children are terminal; and
5. re-raises the original `Whole` error after the nursery quiesces.

Cancellation executes no guest cleanup code and supplies no user-visible task
handle or cancellation API. A main-owned resource may span a terminal M16
`handle call`; it is not implicitly destroyed by a child cancellation.

## AETH v12

Only a source program that declares a task emits AETH v12. V12 extends each
function descriptor after its kind with:

```text
flags:u8
frame_arena_capacity:u32-le
```

`TASK_FRAME` (`0x01`) is valid only for an eligible total guest `Whole` task.
Opcode `TASK_CHECKPOINT` (`67`) is valid only in a flagged v12 task with an
empty operand stack and no live access loan. A task's backward jumps must target
that opcode. V4–v11 decoders, verifier behavior, opcodes, and runtime semantics
are retained exactly; `TASK_CHECKPOINT` is rejected before execution in older
artifacts.

For task-bearing v12 artifacts, the header `arena_capacity` is exact:

```text
main direct arena capacity
+ maximum checkpointed-nursery sum of task direct frame-arena capacities
```

The compiler emits the value and the verifier recomputes it. Nursery admission
reserves the complete slab before any child starts; an undersized runtime arena
fails before partial execution.

## Authoring v8

The v8 AST makes every weave's `task` Boolean explicit and represents
`Checkpoint` as a typed statement. Structural edits retain v7's bounded path
model but are revalidated against the full M19e semantic/capacity model before
the CLI seed-compiles and writes output. See
[AETHER_AUTHORING_PROTOCOL_v8.md](AETHER_AUTHORING_PROTOCOL_v8.md) and the
checked-in JSON schemas under `schemas/`.

## Compatibility

| Input or interface | 0.36 behavior |
| --- | --- |
| AETH v4–v10 artifact | Accepted with its historical verifier and VM meaning. |
| AETH v11 artifact | Accepted with its historical synchronous nursery behavior. |
| Source without `task weave` | Seed/default and bootstrap compile to AETH v11. |
| Source with `task weave` | Seed/default and bootstrap compile to AETH v12 and require the M19e closed subset. |
| `aether.ast/v7` / `aether.edit/v7` | Historical protocols; product `structure` / `apply-edit` use v8 only. |
| Checked-in seed compiler | Still AETH v11 as a compiler program; it emits v12 for valid M19e source and is dual-compared with the bootstrap. |

No prior artifact, source file, or authoring request is silently reinterpreted.

## Explicit non-goals

This package does not add OS threads, parallel execution, arbitrary preemption,
task handles, timeout APIs, manual cancellation, callbacks, destructors,
finalizers, resource arguments/results across task boundaries, nested task
nurseries, individual allocation reclamation, host/foreign/stdout work in a
task, a network registry, a native backend, or ambient guest authority.

## Local verification entry points

```powershell
cargo run -p aether-cli -- compile .\examples\active-cancel.ae --output .\target\active-cancel.aeth
cargo run -p aether-cli -- run .\target\active-cancel.aeth
cargo run -p aether-cli -- compile .\examples\task-frame-capacity.ae --output .\target\task-frame-capacity.aeth
cargo run -p aether-cli -- run .\target\task-frame-capacity.aeth
cargo run -p aether-cli -- compile .\examples\task-loop.ae --output .\target\task-loop.aeth
cargo run -p aether-cli -- run .\target\task-loop.aeth
```

The full implementation evidence, including seed/bootstrap byte identity,
hostile-artifact rejection, legacy compatibility, schema validation, and
zero-warning commands, is recorded in
[DELIVERY_REPORT-2026-08-08-M19E-T-RX-IMPLEMENTATION.md](DELIVERY_REPORT-2026-08-08-M19E-T-RX-IMPLEMENTATION.md).

*End of AETHER_0.36.md*
