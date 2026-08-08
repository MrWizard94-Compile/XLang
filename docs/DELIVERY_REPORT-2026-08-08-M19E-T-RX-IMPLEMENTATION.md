# Delivery Report: M19e T-RX Active-Frame Cancellation Implementation

**Status:** Complete vertical implementation and verification in the working
tree; no commit, tag, publication, or public-release claim is implied
**Date:** 2026-08-08
**Package:** `aether-core` / `aether-cli` **0.36.0**
**Contract:** [AETHER_0.36.md](AETHER_0.36.md)
**Design:** [M19e active-frame cancellation and destruction](DESIGN-M19E-T-RX-ACTIVE-FRAME-CANCEL.md)
**Decision:** [ADR-042](ADR-042-m19e-active-frame-cancel.md)
**Acceptance record:** [M19e validation matrix](M19E-VALIDATION-MATRIX.md)

## Outcome

M19e is implemented as the smallest honest active-frame cancellation slice. A
valid total `task weave` parks only at an explicit verifier-approved
`checkpoint`. In a checkpointed nursery, a later eligible companion failure
cancels every pending or parked task deterministically, destroys live task-local
owners in reverse slot order, zeroes/revokes the private task lane, preserves
already-completed results, and re-raises the original `Whole` error only after
the nursery quiesces.

This is not arbitrary preemption, a task-handle API, a timeout system, parallel
execution, a cancellation callback mechanism, external-effect rollback, or a
general allocator.

## Delivered implementation

1. **Source and semantics**
   - Added `task weave` and `checkpoint` to the parser, formatter, canonical
     AST, diagnostics, and closed task/nursery semantic checks.
   - Enforced total `Whole` task eligibility, owned `Whole`/`Truth` parameters,
     no ordinary/handle/forward task call, no effect/host/foreign/stdout/nested
     nursery work, and direct checkpoint-first task loops.
   - Added exact M19e frame-capacity analysis and `AE-TASK-004`, `AE-TASK-005`,
     and `AE-RESOURCE-004` boundaries.

2. **AETH v12 and verifier**
   - Added v12 task function flags, per-function `frame_arena_capacity`, and
     `TASK_CHECKPOINT` opcode 67.
   - Preserved v4–v11 decode/verification/runtime behavior. Source without task
     frames continues to emit v11.
   - Added forged-artifact checks for flags, descriptors, checkpoints, stack
     state, calls, destinations, capacities, forbidden opcodes, and loop edges.

3. **Runtime and ownership**
   - Added resumable private task frames and source-order single-thread
     round-robin scheduling.
   - Pre-admits the complete nursery slab before any child runs.
   - Tears down cancelled or completed task frames in reverse local-slot order,
     skips moved/released values, zeroes private memory, and returns the slab
     LIFO after join.
   - Preserves main-owned resources across allowed terminal `handle call` use.

4. **Seed and examples**
   - Extended the Aether-written seed compiler to emit v12 descriptors,
     checkpoints, task loops, and exact capacity plans.
   - Promoted the byte-identical checked-in seed artifact. Final three-way
     bootstrap/forge/checked-in SHA-256:
     `DF4BBF08F33AF49BFE010E330373580C68B0F263057876E1D9A1D024EEC8CE0A`.
   - Added `examples/active-cancel.ae`, `examples/task-frame-capacity.ae`, and
     `examples/task-loop.ae`.

5. **Authoring and public contract**
   - Advanced local authoring to `aether.ast/v8`, `aether.edit/v8`, and
     `aether.diagnostic/v8`; every `Weave` has an explicit `task` Boolean and
     `Checkpoint` is typed.
   - Added v8 AST/edit/diagnostic schemas and
     [AETHER_AUTHORING_PROTOCOL_v8.md](AETHER_AUTHORING_PROTOCOL_v8.md).
   - Repaired pre-existing syntax defects in the historical v5–v7 AST JSON
     schema files; all checked-in schema JSON now parses successfully.

## Verification evidence

The final full gate completed successfully:

```powershell
pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode full
```

It passed the AGENTS Constitution pack integrity check (`GOV-INT-001`),
`cargo fmt --check`, Clippy with `-D warnings`, 99 core unit tests, 39 semantic
integration/seed tests, 19 CLI tests, 32 top-level seed≡bootstrap example
comparisons, host/project/module checks, and bootstrap≡forge≡checked-in seed
identity.

Direct product-path CLI proof additionally compiled and ran each M19e example
through default seed and direct bootstrap with identical artifact hashes:

| Fixture | Seed/bootstrap SHA-256 | Seed and bootstrap exit |
| --- | --- | --- |
| `active-cancel.ae` | `FAEA9EA7F06F56BC990D8458A6F4E05D225909B4CCDC01DC87DA376E905F37A3` | 9 |
| `task-frame-capacity.ae` | `7FCE05D8DDB09C020EDBD1E446746F0C5BC8A3968525DE724B96072913BC153A` | 3 |
| `task-loop.ae` | `5216A162BA8B3018DDE2484DD006DCDB2E88A47C85149A6330F2061C7DD1BE68` | 3 |

The CLI `structure` result for `task-loop.ae` reported `aether.ast/v8`,
`task: true`, and a `Checkpoint` as the loop body's first statement. The exact
positive, negative, hostile-artifact, and compatibility test mapping is in the
[completed M19e matrix](M19E-VALIDATION-MATRIX.md).

## Remaining boundaries

- The checked-in seed artifact is an AETH v11 compiler program; that is
  intentional. It emits v12 for valid task source and is dual-compared to the
  bootstrap.
- M19e does not broaden M7/v11 historical nursery behavior. V11 remains the
  unstarted-only cooperative model.
- No claim is made for general async tasks, OS threads, parallel execution,
  external/manual cancellation, handles, timeouts, nested task nurseries,
  callbacks/finalizers, resource arguments/results, task external effects,
  free-on-raise, a general free-list, public release readiness, or performance
  beyond existing scoped measurements.

## Constitution self-audit

| Rule family | Result |
| --- | --- |
| `CONST-COMPLETE-001` / `CONST-DEP-001` | Complete parser→semantic→artifact→verifier→VM→seed→authoring→docs vertical slice; no stub or partial scheduler path. |
| `TEST-BEHAVIOR-001` | Positive, negative, hostile, legacy compatibility, seed identity, and direct CLI behavior all executed. |
| `SEC-INPUT-001` | Forged v12 artifacts fail before VM execution; task capability/effect surface remains closed. |
| `ENG-WARN-001` | Final format and Clippy `-D warnings` gate passed without suppression. |
| `DOC-SYNC-001` | Manifest, package contract, seed profile, architecture, claims, roadmap, progress report, ADR, matrix, schema protocol, and delivery record synchronized. |
| `GOV-INT-001` | Constitution pack verifier passed in the final gate. |

*End of M19e implementation delivery report.*
