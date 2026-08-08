# M19e active-frame cancellation and destruction — validation matrix

**Status:** Implemented and verified in package **0.36.0** / AETH **v12**
**Date:** 2026-08-08
**ADR:** [ADR-042](ADR-042-m19e-active-frame-cancel.md)
**Design:** [M19e active-frame cancellation and destruction](DESIGN-M19E-T-RX-ACTIVE-FRAME-CANCEL.md)
**Evidence:** [implementation delivery report](DELIVERY_REPORT-2026-08-08-M19E-T-RX-IMPLEMENTATION.md)

This is the completed acceptance record for the M19e vertical slice. AETH v12
is emitted only for valid task-bearing source; v4–v11 retain their prior
decoder, verifier, and VM behavior.

## Positive behavior

| ID | Required behavior | Verified evidence |
| --- | --- | --- |
| P1 | `task weave` + `checkpoint` parse, format, canonical AST v8, and structural edit round-trip | `authoring::tests::structural_document_round_trips_m19e_task_and_checkpoint_nodes`; `authoring::tests::v8_authoring_schemas_describe_the_live_task_contract`. |
| P2 | A started task allocates a private Buffer, parks at `checkpoint`, a later companion raises, and the task is cancelled before its later `append`/`yield` | `tests::m19e_active_task_frame_cancels_at_a_private_resource_checkpoint`; `examples/active-cancel.ae` CLI seed/bootstrap runs exit 9. |
| P3 | Parked task local owners are destroyed once in descending slot order; moved/released locals are not double-destroyed | `tests::m19e_active_task_frame_cancels_at_a_private_resource_checkpoint` proves `[4, 3, 2]` destruction order over Buffer/Arena/record-owned Text/Bytes; `tests::m19e_preserves_main_owners_across_handled_failure_and_never_double_releases_a_task_owner` proves released local absence. |
| P4 | Cancelled task lane is revoked and zeroed; the whole slab is zeroed then released after terminal children | `tests::m19e_active_task_frame_cancels_at_a_private_resource_checkpoint` and `tests::m19e_preserves_main_owners_across_handled_failure_and_never_double_releases_a_task_owner`. |
| P5 | Completed-before-failure child keeps its `Whole` destination; pending and cancelled destinations remain unchanged | `tests::m19e_scheduler_retains_completed_results_and_only_cancels_live_or_pending_children`. |
| P6 | A total checkpointed nursery completes all task results in deterministic source-order round robin | `tests::m19e_frame_plan_adds_main_capacity_to_the_largest_task_nursery`; `examples/task-frame-capacity.ae` exits 3. |
| P7 | A main-owned M2 arena can remain live across terminal `handle call` of a resource-free erroring nursery parent | `tests::m19e_preserves_main_owners_across_handled_failure_and_never_double_releases_a_task_owner` checks parent live-resource slots before and after child cancellation. |
| P8 | Direct root `release` inside a task remains logical only; later cancellation never double releases it | `tests::m19e_preserves_main_owners_across_handled_failure_and_never_double_releases_a_task_owner`. |
| P9 | Every task loop back edge targets a checkpoint and bounded iteration remains schedulable | `tests::m19e_task_loop_resumes_only_at_verifier_checkpoint_boundaries`; `examples/task-loop.ae` exits 3. |
| P10 | V12 capacity equals main direct capacity plus the largest task-lane sum; admission occurs before any child starts | `tests::m19e_frame_plan_adds_main_capacity_to_the_largest_task_nursery` proves header 96 and rejected pre-admission leaves no trace. |
| P11 | Bootstrap and seed emit byte-identical v12 artifacts for active-cancel, capacity, loop, forward-task, and legacy corpus | `seed_profile_compiler_forges_m19e_active_task_frames_byte_identically`; full `seed_self_host` suite; three-way seed identity gate. |
| P12 | v4–v11 artifacts, especially M7/M19d/M19c fixtures, retain prior behavior | Full `aether-gate.ps1 -Mode full` regression suites and all 32 top-level seed/bootstrap example comparisons. |
| P13 | Full documented M19e corpus builds/runs through default seed product compile and direct bootstrap | Direct CLI proof for `active-cancel` (exit 9), `task-frame-capacity` (exit 3), and `task-loop` (exit 3); each seed artifact hash equals its bootstrap artifact. |

## Negative source cases

| ID | Required rejection | Verified evidence |
| --- | --- | --- |
| N1 | `checkpoint` outside `task weave` | `tests::m19e_rejects_task_forms_outside_the_closed_cancellation_subset` → `AE-TASK-004`. |
| N2 | Erroring/main/host/foreign/non-Whole/resource-parameter task or ordinary task call | Same source-negative test → `AE-TASK-004` / `AE-TASK-005`. |
| N3 | Task has no checkpoint or a `while` whose body does not begin with direct `checkpoint` | Same source-negative test → `AE-TASK-004`. |
| N4 | Task uses call, handle, forward, raise, together, speak, comptime, or host/foreign work | Same source-negative test → `AE-TASK-004`. |
| N5 | Checkpointed nursery includes resourceful/non-isolated companion, resource argument, or invalid destination | Same source-negative test → `AE-TASK-005` / existing `AE-TASK-003`. |
| N6 | Checkpointed nursery has a may-raise companion inside a total parent | Same source-negative test → `AE-TASK-002`; no implicit effect conversion. |
| N7 | Task or non-main non-task resource declaration violates the v12 frame plan | Same source-negative test → `AE-RESOURCE-004`. |
| N8 | Source attempts a task handle, timeout, cancellation observation, or handler | Same source-negative test rejects the closed `cancel` spelling → `AE-TASK-001`; no such grammar/API exists. |

## Hostile artifact cases

| ID | Required rejection before VM execution | Verified evidence |
| --- | --- | --- |
| H1 | `TASK_CHECKPOINT` in v4–v11 or a non-task v12 function | `tests::verifier_rejects_m19e_checkpointed_nursery_hostile_forms_before_execution`; `tests::verifier_rejects_m19e_descriptor_and_task_call_escapes`. |
| H2 | Unknown v12 flag, task flag on an ineligible descriptor, or malformed frame capacity | Same hostile tests. |
| H3 | Checkpoint with non-empty operand stack, live loan, or open nursery | `tests::verifier_rejects_m19e_checkpointed_nursery_hostile_forms_before_execution`. |
| H4 | Ordinary/handle/forward call targets a task, or a checkpointed nursery targets an ineligible companion | `tests::verifier_rejects_m19e_descriptor_and_task_call_escapes`; `tests::verifier_rejects_m19e_checkpointed_nursery_hostile_forms_before_execution`. |
| H5 | Header capacity differs from recomputation, exceeds bound, or lane arithmetic is invalid | Same hostile tests and `tests::m19e_frame_plan_adds_main_capacity_to_the_largest_task_nursery`. |
| H6 | Task code smuggles forbidden call/host/stdout/resource-crossing opcode or a non-checkpoint loop edge | `tests::verifier_rejects_m19e_checkpointed_nursery_hostile_forms_before_execution`. |
| H7 | Nursery count/target/destination/argument shape is malformed under v12 scheduling | Same hostile test. |
| H8 | Existing malformed v4–v11 nursery/resource artifacts remain rejected | Historical verifier tests in the full 99-test core suite. |

## Delivery checklist

- [x] Full design, ADR, research inputs, and stop conditions recorded.
- [x] Current v11 cooperative scope retained and not relabeled.
- [x] Parser, formatter, semantic model, capacity analysis, and diagnostics.
- [x] AETH v12 encoder/decoder/verifier and hostile-artifact corpus.
- [x] Resumable VM frames, deterministic scheduler, private lanes, and teardown trace.
- [x] Seed v12 emission/self-host/dual-compare.
- [x] Authoring v8 contract, schemas, and structural tests.
- [x] All P/N/H rows, regression suite, Constitution pack check, zero-warning gates, contract sync, and delivery report.

The completed matrix records exact tests rather than widening the task model.
Any task handles, timeouts, manual cancellation, parallel scheduling, external
effects, or broader resource/effect composition requires a new ADR and matrix.

---

*End of M19e validation matrix.*
