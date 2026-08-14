# ADR-042: M19e deterministic active-frame cancellation and destruction

**Status:** Implemented in package **0.36.0** / AETH **v12**
**Date:** 2026-08-08
**Decision makers:** Human direction to proceed; AGENTS Constitution; T-RX
dependency order
**Related Rule IDs:** `CONST-DEP-001`, `CONST-COMPLETE-001`, `DOC-ADR-001`,
`DOC-SYNC-001`, `TEST-BEHAVIOR-001`, `SEC-INPUT-001`, `RND-INVAR-001`
**Related:** ADR-003, ADR-010, ADR-020, ADR-023, ADR-027, ADR-028, ADR-035,
ADR-036

## Context

M19d makes total resource-owning spawn callees admissible. ADR-036 then proves
only the behavior the current synchronous v11 nursery can honestly provide:
unstarted children have no frame to clean up, and started children run to
return. It explicitly does not claim mid-frame cancellation.

A real active-frame design must solve four coupled facts: a safely suspended
frame, deterministic first-failure order, destruction of live owners without
guest callbacks, and arena backing storage that cannot alias another parked
task. A cancellation flag alone cannot solve them.

## Decision

1. Adopt [DESIGN-M19E-T-RX-ACTIVE-FRAME-CANCEL.md](DESIGN-M19E-T-RX-ACTIVE-FRAME-CANCEL.md).
2. Introduce v12-only `task weave` and `checkpoint` source forms.
   A started task may be cancelled only while parked at a verifier-approved
   checkpoint; arbitrary preemption remains prohibited.
3. Use a deterministic single-thread source-order round-robin scheduler for a
   checkpointed nursery. The first companion `Error[Whole]` cancels pending
   children and destroys parked task frames before the nursery re-raises.
4. Give every task a private, pre-admitted arena lane inside a nursery slab.
   Destruction clears live locals in reverse slot order, then zeroes/revokes
   the lane; the whole slab is released only after join.
5. Keep no user destructors, finalizers, cancellation handlers, resource
   arguments/results, task handles, nested nurseries, host/foreign/stdout work
   in M19e children, OS-thread parallelism, individual free, or free-on-raise.
6. Require AETH v12 function metadata, `TASK_CHECKPOINT` opcode 67, an
   independently recomputed capacity plan, v4–v11 compatibility, seed parity,
    authoring v8, and the full [M19e validation matrix](../Current%20state/M19E-VALIDATION-MATRIX.md)
    as the required package 0.36 implementation gate.

## Consequences

### Positive

- The language gains a true active resource-frame cancellation target rather
  than a relabeled unstarted no-op.
- Cancellation, ownership, region capacity, and scheduling points are visible
  in source and independently verified in artifacts.
- The design stays local, deterministic, capability-closed, and feasible for
  seed/bootstrap dual comparison.

### Costs

- v12 adds a resumable frame machine and a new verifier/data-model path.
- Task syntax is intentionally more restrictive than ordinary total weaves.
- Existing v11 resource work remains the compatibility baseline; the first v12
  slice cannot mix arbitrary resourceful helper call graphs with task frames.

### Risks and mitigations

| Risk | Mitigation |
| --- | --- |
| A parked task retains an unsafe operand/loan state | Verifier requires empty stack and no loan at every checkpoint. |
| Sibling regions overlap or leak | Static exact capacity calculation, private lanes, slab-only release, zeroing tests. |
| Cancellation becomes host-timing dependent | One single-thread logical schedule; no wall clock or host threads. |
| External effects cannot be undone | Closed task/companion subset rejects host, foreign, stdout, and nested scheduling. |
| A partial feature creates a false claim | Matrix and seed/verifier/VM/authoring/documentation gate are one required delivery. |

## Alternatives rejected

| Alternative | Reason |
| --- | --- |
| Extend ADR-036 without a new artifact/runtime model | Cannot cancel a started recursive frame. |
| Preempt any instruction | Violates atomic source/resource transitions. |
| Host-thread implementation | Breaks deterministic VM and increases authority/race surface. |
| Implicit drop/free on parent raise | Violates M19's explicit ownership boundary. |
| User cleanup hooks | Introduces untyped effects during destruction. |

## Implementation evidence

This ADR authorized one complete M19e vertical slice:

1. source/AST/semantic/capacity plan;
2. v12 encoder, decoder, verifier, hostile-artifact corpus, and v4–v11 tests;
3. frame scheduler, task-region teardown, and deterministic runtime trace;
4. seed v12 byte identity and authoring v8 contracts;
5. every positive, negative, and security case in the M19e matrix;
6. zero-warning gates, synchronized AETHER/architecture/manifest documentation,
   and a delivery report.

All six are delivered together. The completed matrix identifies the exact source,
verifier, scheduler, seed, authoring, compatibility, and quality-gate tests;
[AETHER_0.36.md](AETHER_0.36.md) records the executable boundary and the
[implementation delivery report](DELIVERY_REPORT-2026-08-08-M19E-T-RX-IMPLEMENTATION.md)
records command-level evidence. CLM-030 remains the historical v11 cooperative
claim; CLM-039 is the bounded active-frame cancellation claim for v12 task
nurseries.

---

*End of ADR-042.*
