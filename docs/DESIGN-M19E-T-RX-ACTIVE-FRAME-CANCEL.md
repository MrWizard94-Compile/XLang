# M19e Design: deterministic active-frame cancellation and destruction (T-RX)

**Status:** Implemented in package **0.36.0**. M19e source emits AETH **v12**;
source without task frames continues to emit AETH v11.
**Date:** 2026-08-08
**Decision record:** [ADR-042](ADR-042-m19e-active-frame-cancel.md)
**Validation gate:** [M19e validation matrix](M19E-VALIDATION-MATRIX.md)
**Depends on:** M1/M2 ownership and arenas, M4 effects, M7 nurseries, M16,
M19a `release`, M19b Policy A+, M19c bounded cooperative Policy B, and M19d
multi-weave arenas
**Implemented contract:** package **0.36.0**, AETH **v12**, and authoring
protocol **v8**; the completed evidence is recorded in the linked matrix and
[implementation delivery report](DELIVERY_REPORT-2026-08-08-M19E-T-RX-IMPLEMENTATION.md).
**Rule IDs:** `CONST-DEP-001`, `CONST-COMPLETE-001`, `DOC-ADR-001`,
`DOC-SYNC-001`, `TEST-BEHAVIOR-001`, `SEC-INPUT-001`, `RND-INVAR-001`,
`ENG-WARN-001`

---

## 1. Purpose and current gap

M19c's product claim is intentionally narrow: under AETH v11 a nursery calls a
child synchronously, so a failure can cancel only a child that has not begun.
ADR-036 correctly does **not** claim active-frame cancellation, resource cleanup
at a cancellation point, or free-on-raise.

M19e adds the smallest honest capability that changes that fact:

> A started, resource-owning `task weave` can reach an explicit `checkpoint`,
> retain its private frame while parked, and be deterministically cancelled by
> a later sibling failure before it returns. Cancellation destroys every live
> task-local owner, clears the task's private arena region, leaves its parent
> destination unchanged, and then propagates the sibling's existing
> `Error[Whole]` only after the nursery has quiesced.

This is **not** arbitrary preemption and it is not the old unstarted-only
cooperative claim. A task must explicitly mark safe cancellation points, but
the cancellation target has a real, previously started frame and may own a
Buffer, table, Text, Bytes, record, and Arena at the point it is destroyed.

The design is deliberately a versioned, deterministic VM feature rather than a
host-thread feature. It preserves Aether's visible ownership, no-ambient-
authority, verifier-before-run, and seed-proof laws.

### 1.1 Evidence from the prior v11 boundary

Before M19e, the v11 VM created a recursive `execute_function` call for each
`NURSERY_SPAWN`; it has no suspended child-frame representation. The current
single `ArenaState` is a monotonic shared backing store, and `release` clears a
logical local but does not reclaim an individual allocation. Consequently, a
flag on the existing `NurseryFrame` cannot safely implement this feature.

M19e therefore requires all of the following together:

1. a source-visible task/cancellation boundary;
2. a v12 function descriptor with frame-arena facts;
3. a verifier that proves every suspension point is safe;
4. a resumable single-thread VM frame model; and
5. deterministic region teardown after all owners in a cancelled task have
   ended.

No implementation may claim M19e by changing only parser, only scheduler, or
only resource cleanup code.

---

## 2. Non-goals and preserved law

M19e is a bounded ownership/cancellation slice, not a general async runtime.

| Out of scope | Reason |
| --- | --- |
| OS threads, parallel execution, wall-clock scheduling | Would introduce nondeterminism and new host/runtime authority. |
| Arbitrary instruction-level preemption | Could interrupt a partially evaluated expression or resource transition. |
| Task handles, timeout APIs, external/manual cancellation | Need a separate authority, lifecycle, and effect design. |
| Cancellation callbacks, destructors, finalizers, `defer` | Destruction must never run guest code or hidden host I/O. |
| Free-on-raise of parent owners | Remains prohibited by M19; only the cancelled child frame is torn down. |
| Resource arguments, resource results, or loans across a spawn | Would create aliases into a task frame and invalidate regional teardown. |
| Nested nurseries or task-to-task calls | Would require a second ownership/scheduler proof. |
| Individual Buffer/Table reclamation or a general free-list | `release` remains logical end only; M19e reclaims only whole private frame regions. |
| Host, foreign, grant-I/O, stdout, or model interaction in an M19e child | Cancellation cannot roll back such external effects. |

All v4–v11 artifacts retain their exact decoder, verifier, and runtime
semantics. Source that does not use the new `task`/`checkpoint` forms continues
to emit v11 under its existing package contract. A v12 artifact is not a
reinterpretation of a v10 or v11 nursery.

---

## 3. Source contract

### D1 — explicit task declaration and checkpoint statement

M19e introduces exactly two canonical source forms:

```text
weave-declaration ::= [ "task" space ] "weave" name parameters "->" result [ "raises Whole" ] ":"
checkpoint-statement ::= "checkpoint"
```

Illustrative **implemented v12** source:

```aether
task weave worker [] -> Whole:
  bind memory <- arena 64
  bind mutable values <- buffer Whole
  choose allocate access memory move values 2 into values:
    checkpoint
    choose append move values 7 into values:
      yield 7
    otherwise:
      yield -2
  otherwise:
    yield -1
```

`task` is not an advisory annotation. It is a source, AST, AETH, verifier, and
VM contract. `checkpoint` is not a general yield, await, continuation, or
callback hook. It is an atomic statement boundary at which a task may park or,
if cancellation has already been requested, be destroyed.

### D2 — task-weave eligibility

A `task weave` is valid only when every condition below holds:

1. It is a guest weave, not `main`, `host weave`, or `foreign weave`.
2. It is total and returns `Whole`.
3. Its parameters, if any, are owned `Whole` or `Truth` only. It has no
   `borrow` or `access` parameter and no owner/resource input.
4. It owns at most one direct root `arena N` under the existing M19d capacity
   bounds. Its owner values are local; no resource can be a result or cross a
   spawn boundary.
5. It may use local Copy computation, `bind`, `revise`, `choose`, `while`,
   M2/M6 resource operations, `release`, `checkpoint`, and `yield`.
6. It may not use `speak`, `call`, `handle`, `forward`, `raise`, `together`,
   `comptime bind`, host/foreign invocation, or any future ambient/effectful
   form. This makes every running task self-contained.
7. It contains at least one `checkpoint`. Every `while` body in the task,
   including nested bodies, starts with a direct `checkpoint` statement. The
   compiler and verifier both prove that every generated backward jump targets
   a checkpoint.
8. It is referenced only by `spawn call`; ordinary calls, forwarded calls,
   handles, and comptime calls to a `task weave` are rejected.

Rule 7 guarantees a finite straight-line segment between checkpoints under the
existing non-recursive task subset. It is intentionally stricter than a vague
promise that a task will "eventually cooperate."

### D3 — M19e nursery eligibility

A `together` block becomes a **checkpointed nursery** when at least one of its
spawns targets a `task weave`. In that nursery every target must be either:

1. a valid total `task weave`; or
2. a resource-free, self-contained **companion weave** returning `Whole`, with
   Copy-only parameters/locals and no call, host/foreign, `speak`, resource,
   `release`, `handle`, `forward`, or nested nursery operation. A companion may
   be total or `raises Whole`; the latter is the only normal source of nursery
   failure in this slice.

Existing M7 rules still apply: one through eight spawns, lexical scope,
`Whole` mutable destinations, no resource arguments, and an Error[Whole]
parent when a companion may raise. An erroring parent remains resource-free;
M19e does not relax M16/M19a's parent abort boundary. A total parent may use a
checkpointed nursery only when all companions are total.

This closed child set prevents cancellation from attempting to undo host I/O,
stdout, FFI, external grants, a nested scheduler, or a resource alias.

### D4 — authoring representation

The structural contracts advance together to
`aether.ast/v8`, `aether.edit/v8`, and `aether.diagnostic/v8`:

- `Weave` gains an explicit Boolean/task role rather than a hidden textual
  convention.
- `Checkpoint` is a typed statement node.
- The validator exposes task/cancellation diagnostics with the same stable
  code/span rules as other source forms.
- Structural edits cannot add `task` or `checkpoint` without the full semantic
  eligibility check and canonical reformat.

The v7 authoring contract remains historical; the product contract is v8.

---

## 4. Deterministic scheduler semantics

### 4.1 Child state machine

Within one checkpointed nursery, each source-order child has one of these
runtime states:

```text
Pending(arguments) | Running(frame) | Parked(frame) | Completed(Whole)
| Failed(Whole) | Cancelled
```

Only one child is `Running` at any instant. The scheduler is a deterministic
single-thread round robin:

1. Start the first non-terminal child in source order.
2. A task child runs from start or resume until exactly one of: `checkpoint`,
   `yield`, or an invalid-runtime halt. A normal companion runs to `yield` or
   `raise`; it has no checkpoint opcode.
3. A task reaching `checkpoint` with no failure becomes `Parked`; the scheduler
   starts/resumes the next eligible child in source order.
4. A normal `yield` completes the child and commits its `Whole` result to the
   parent destination. A `raise` records the first failure in logical scheduler
   order.
5. On that first failure, every `Pending` child becomes `Cancelled` and drops
   only its Copy arguments. Every `Parked` task is immediately cancelled and
   destroyed. Completed destinations remain committed. No child starts or
   resumes after the failure.
6. After all children are terminal, the nursery tears down its frame-region
   slab. It then re-raises the recorded `Whole` code to the parent, exactly as
   M7 does after join.

There is no racy `Running` sibling at step 5: the failing companion is the one
currently running, so every other started task is parked at a verifier-approved
checkpoint. This is why a deterministic single-thread scheduler can provide a
strong cleanup boundary without preempting an instruction.

### 4.2 Observable result rules

| Child outcome | Parent destination |
| --- | --- |
| Completed before the first failure | Its returned `Whole` is retained. |
| Pending when failure is observed | Unchanged; no child frame existed. |
| Parked task cancelled after failure | Unchanged; its unreturned `Whole` is never committed. |
| All children complete | Every destination receives its child `Whole` result. |

The first observed failure is defined by source-order round-robin execution,
not wall-clock timing. The result is therefore replayable in the VM, seed
corpus, and tests.

### 4.3 Normative active-frame example

This is the implemented v12 active-frame proof fixture:

```aether
world active_cancel

task weave staged [] -> Whole:
  bind memory <- arena 64
  bind mutable values <- buffer Whole
  choose allocate access memory move values 2 into values:
    checkpoint
    choose append move values 7 into values:
      yield 7
    otherwise:
      yield -2
  otherwise:
    yield -1

weave fail [] -> Whole raises Whole:
  raise 9

weave run [] -> Whole raises Whole:
  bind mutable worker <- 0
  bind mutable failed <- 0
  together:
    spawn call staged into worker
    spawn call fail into failed
  yield worker

weave main [] -> Whole:
  bind mutable value <- 0
  bind mutable fault <- 0
  handle call run into value otherwise error into fault
```

The scheduler starts `staged`, allocates its local Buffer, and parks it at
`checkpoint`. `fail` then raises `9`. `staged` is a real active frame with live
Arena/Buffer owners, but it is cancelled before `append` or `yield`; `worker`
remains `0`, its private owners are destroyed, and `main` observes fault `9`.

---

## 5. Ownership, destruction, and frame arenas

### D5 — suspension safety

At a `checkpoint`, the verifier requires:

- an empty operand stack;
- no live `AccessArena`/ephemeral loan;
- no open nursery or outstanding child;
- a valid task function flag; and
- a control-flow path that preserves compatible local ownership state.

All preceding source operations have committed before the checkpoint. There is
therefore no partial `allocate`, `append`, `store`, move, or `revise` to roll
back. Cancellation never executes user code and never resumes after the safe
point.

### D6 — deterministic task destruction

Cancelling a `Parked` task performs these ordered VM-internal actions:

1. Mark the frame terminal as `Cancelled`; its `Whole` result may not be
   committed.
2. Assert and discard the empty transient stack.
3. Visit live locals in descending slot/declaration order. For each owner,
   clear the local exactly once; moved and already released locals are skipped.
4. After no task-local owner remains, zero the entire private arena lane and
   revoke the lane from guest access.
5. Retain the lane inside the nursery slab until every sibling is terminal;
   then zero the full slab defensively and return it as one LIFO region.

The same logical owner end occurs on ordinary task `yield`, except its `Whole`
result is transferred first. There are no destructors, callbacks, finalizers,
host calls, stdout writes, or cancellation handlers in either path.

`release` remains a logical local end. It does not make a subrange reusable
inside a live task and it is not invoked a second time by cancellation.

### D7 — private frame-region plan

The existing shared monotonic arena cannot safely free a sibling's allocation
while later siblings remain live. M19e replaces that use for v12 with nested,
private regions:

1. A v12 function descriptor declares its direct frame-arena capacity.
2. `main` may retain one direct M19d arena. In a program using `task weave`, no
   non-task guest weave other than `main` may declare an arena.
3. At a checkpointed nursery join, the VM reserves one contiguous nursery slab
   whose lanes are the capacities of its task targets in source order.
4. A task's `arena` capability is bound only to its own lane. Buffer/table
   offsets remain inside that lane; no child can observe a sibling's bytes.
5. Task weaves cannot call, nest nurseries, pass resources, or return resources,
   so no frame can retain a pointer/owner into another lane.
6. The slab is released only after the whole nursery has joined. This avoids a
   general free-list while allowing the top-level arena region to rewind safely.

For any v12 artifact that uses task cancellation, the header
`arena_capacity` is the exact conservative concurrent bound:

```text
main direct arena capacity
+ maximum over checkpointed nurseries of
  (sum of task-target direct frame-arena capacities in that nursery)
```

The compiler emits this value; the verifier recomputes it from v12 function
metadata and spawn targets, requires equality, and rejects values above the
existing 1,000,000-byte bound. This restriction is purposeful: it admits a
provable first live-frame design without hiding a dynamic allocator or a
general resource call graph behind the scheduler.

---

## 6. AETH v12 contract

### D8 — versioned function metadata

Compilation of M19e source emits **AETH v12**. v12 keeps the existing
header fields, records, shapes, values, and v11 instruction meanings except
where this document explicitly versions them.

Each v12 function descriptor extends the v11 post-effect fields as follows:

```text
kind:u8
flags:u8
frame_arena_capacity:u32-le
local_count:u16-le
...
```

Defined flag:

| Bit | Name | Requirement |
| --- | --- | --- |
| `0x01` | `TASK_FRAME` | Guest, total, Whole-returning, task-eligible function only. |

All other flag bits are rejected. Host/foreign descriptors require zero flags
and zero frame capacity. `frame_arena_capacity` equals the direct root arena
literal for `main` or a task weave, and is zero for every other v12 function in
this slice.

### D9 — opcode and nursery interpretation

| Opcode | v12 encoding | v12 meaning |
| --- | --- | --- |
| `NURSERY_BEGIN` (62) | unchanged | Opens a deterministic nursery declaration. |
| `NURSERY_SPAWN` (63) | unchanged | Captures Copy arguments and declares a pending child; it does not recursively run the child. |
| `NURSERY_END` (64) | unchanged | For a checkpointed nursery, reserves lanes, dispatches the deterministic scheduler, joins, tears down the slab, then continues or re-raises. |
| `TASK_CHECKPOINT` (67) | no operands | Empty-stack task safe point: park when healthy; permit immediate cancellation when a failure is pending. |

The same byte values in v10/v11 retain their former source-order synchronous
meaning. `TASK_CHECKPOINT` is invalid before v12. A v12 artifact with no task
flag has no authority to use opcode 67.

### D10 — verifier obligations

The v12 verifier rejects before execution when any of these facts fails:

1. Descriptor kind/flag/capacity fields are malformed, unknown, or inconsistent.
2. A task function has an illegal signature, effect, call, host/foreign,
   stdout, nested nursery, resource boundary, or non-checkpoint loop.
3. `TASK_CHECKPOINT` appears outside a task function, with a non-empty stack,
   a loan, or a non-task control-flow context.
4. A task target is reached by ordinary `CALL`, `HANDLE_CALL`, `FORWARD_CALL`,
   or comptime lowering rather than `NURSERY_SPAWN`.
5. A checkpointed nursery has a child that is neither a flagged task nor an
   eligible resource-free companion, crosses a non-Copy/resource parameter, or
   has an effect inconsistent with its parent.
6. The exact header capacity calculation, lane sum, per-lane bounds, or
   single-nursery restriction is invalid.
7. A v4–v11 artifact contains a v12 descriptor field or opcode interpretation.

Source validation mirrors these rules, but the verifier remains the authority
against a forged artifact.

---

## 7. VM architecture and security boundary

### D11 — resumable VM frames

v12 uses a small explicit execution-frame machine rather than recursively
calling `execute_function` for a nursery child. A frame owns its function index,
instruction position, locals, operand stack, optional private arena lane, and
terminal state. The scheduler stores only verified guest frames; it never gives
guest code a frame pointer, task handle, continuation, or host callback.

Normal calls/handles in the v12 path remain deterministic frame-stack
operations. M19e task and companion subsets deliberately exclude them, so a
parked task has no suspended child call stack. v4–v11 may remain on their
existing execution path or share a refactor only if compatibility tests prove
identical behavior.

### D12 — capability and hostile-input rules

- A task checkpoint does not grant file, process, network, shell, model,
  foreign-library, clock, thread, or cancellation authority.
- The only cancellation trigger is the already-defined first
  `Error[Whole]` from an eligible companion in the lexical nursery.
- Region capacity is pre-admitted and bounded before the first child starts;
  failure to reserve is a VM admission failure, never partial task execution.
- Teardown zeroes private guest bytes before region reuse and never exposes raw
  addresses, allocator handles, or residual task memory.
- A malformed artifact cannot create a task with a live loan at a checkpoint,
  target a sibling lane, smuggle a host call into a cancellation group, or
  under-declare the concurrent capacity budget.

---

## 8. Diagnostics and compatibility

The implementation reserves these source diagnostic families:

| Code | Meaning |
| --- | --- |
| `AE-TASK-004` | Invalid `task weave` or `checkpoint` placement/eligibility. |
| `AE-TASK-005` | Checkpointed-nursery child, isolation, or call-site violation. |
| `AE-RESOURCE-004` | Invalid M19e frame-arena/capacity plan. |

Existing `AE-TASK-001` through `AE-TASK-003`, `AE-EFFECT-*`, and
`AE-RESOURCE-*` retain their current meanings where their existing conditions
still apply. Hostile v12 artifacts need not mimic bootstrap wording, but must
fail before VM execution with a precise verifier error.

No AETHER_0.35, v11 seed, v7 authoring schema, existing source corpus, or
historical artifact is silently upgraded. [AETHER_0.36.md](AETHER_0.36.md)
publishes the v11/v12 compatibility table and v8 authoring boundary.

---

## 9. Implemented sequence and stop conditions

The design authorized a **single complete vertical implementation**, not a
partial parser or scheduler experiment. The completed implementation followed
this sequence:

1. Extend AST/parser/formatter/semantic model with `task` and `checkpoint`;
   implement all D2–D3 static restrictions and capacity analysis.
2. Add v12 encoding/decoding, function metadata, verifier state, negative
   artifact corpus, and immutable v4–v11 compatibility coverage.
3. Replace the v12 nursery execution path with explicit frames, deterministic
   round robin, fixed task lanes, owner teardown, slab zeroing, and result/error
   propagation.
4. Update the Aether seed so bootstrap and seed emit byte-identical v12 fixtures.
5. Advance authoring to v8 with typed task/checkpoint nodes and contract tests.
6. Deliver the entire [M19e validation matrix](M19E-VALIDATION-MATRIX.md), full
   constitution gates, AETHER/architecture/manifest synchronization, and an
   [implementation delivery report](DELIVERY_REPORT-2026-08-08-M19E-T-RX-IMPLEMENTATION.md).

Stop and do **not** ship M19e if any of these is necessary:

- arbitrary preemption, user cleanup code, external-effect rollback, or a
  host-scheduler race;
- a live loan/resource alias crossing a task boundary;
- a general free-list or hidden allocator policy to make the example work;
- a capacity plan the verifier cannot independently recompute;
- a task loop that can run indefinitely without a verifier-proven checkpoint;
- a seed/bootstrap mismatch, old-artifact regression, warning, or failed
  hostile-input test.

---

## 10. Alternatives considered

| Alternative | Decision | Reason |
| --- | --- | --- |
| Keep M19c unstarted-only cancellation and relabel it | Rejected | It does not destroy an active resource frame. |
| Interrupt any VM instruction | Rejected | Breaks expression/resource atomicity and makes artifacts harder to verify. |
| Use OS threads or a thread pool | Rejected | Adds nondeterministic scheduling, races, and host/runtime complexity. |
| Run user cancellation handlers/destructors | Rejected | Would introduce hidden effects and cleanup-order authority. |
| Free individual Buffer/Table allocations | Rejected | Reintroduces alias/double-free/free-list state outside the M2 model. |
| Keep one shared monotonic arena for parked siblings | Rejected | Cannot prove non-overlap or safe sibling teardown. |
| Versioned task frames + checkpoints + private lanes | Adopted | Makes the cancellation point, ownership boundary, capacity, and destruction order explicit and verifier-checkable. |

---

## 11. Research inputs

These sources informed the boundary; Aether does not copy their APIs or runtime
semantics. Accessed 2026-08-08.

- Rust's [destructor reference](https://doc.rust-lang.org/reference/destructors.html)
  supports the value of a defined destruction order. Aether deliberately rejects
  user-defined destructors and uses only VM-internal logical destruction.
- Swift's [concurrency documentation](https://docs.swift.org/swift-book/documentation/the-swift-programming-language/concurrency/)
  illustrates cooperative cancellation and the care needed around cancellation
  handlers. Aether chooses no handler API, avoiding the handler race surface.
- Tokio's [task documentation](https://docs.rs/tokio/latest/tokio/task/index.html)
  and [cancellation-safety discussion](https://docs.rs/tokio/latest/tokio/macro.select.html)
  reinforce the distinction between a safe suspension boundary and arbitrary
  interruption. Aether's `checkpoint` is a verifier-enforced statement boundary,
  not a Tokio-compatible API.

---

*End of M19e active-frame cancellation and destruction design.*
