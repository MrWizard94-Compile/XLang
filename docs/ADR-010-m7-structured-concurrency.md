# ADR-010: structured nursery concurrency

**Status:** Accepted for Aether 0.10 / M7 implementation

**Date:** 2026-08-03

**Decision makers:** WPAI product direction; user authorization to continue
language development (M7) under AGENTS Constitution

**Related Rule IDs:** RND-INVAR-001, RND-CORE-001, RND-DOC-001,
DOC-ADR-001, DOC-SYNC-001, TEST-BEHAVIOR-001, SEC-INPUT-001,
CONST-GATE-001

## Context

Roadmap M7 requires a lexical task-group model with join, failure,
cancellation, and effect-mediated blocking, without orphan tasks. M4 provides
abortive `Error[Whole]` but forbids ambient exceptions and resource-crossing
effect control. True OS threads would introduce non-determinism, host
capability questions, and seed-infeasible scheduling.

## Decision

Aether 0.10 implements M7 as a **deterministic structured nursery**:

1. Lexical `together:` blocks containing only `spawn call ... into` lines
   (1..=8).
2. Cooperative **source-order** execution (not parallel threads).
3. First child `Error[Whole]` cancels remaining unstarted spawns and re-raises.
4. Same clean resource boundary as M4 for weaves that use nurseries.
5. AETH v10 nursery opcodes; authoring v5; seed byte identity.

Full rules: [DESIGN-M7-STRUCTURED-CONCURRENCY.md](DESIGN-M7-STRUCTURED-CONCURRENCY.md).

## Consequences

### Positive

- Structured concurrency lifecycle is explicit and AI-auditable.
- Cancellation is total and deterministic without a scheduler.
- Reuses M4 error codes and `handle call` at outer boundaries.

### Costs

- No parallel speedup claim.
- No nested nurseries, task handles, timeouts, or resourceful tasks in M7.
- Sequential execution only.

## Alternatives rejected

| Alternative | Reason rejected for M7 |
| --- | --- |
| OS/thread-pool parallelism | Non-deterministic, host-capability, seed proof risk. |
| Detached/`go` tasks | Orphans; violates structured concurrency. |
| Resumable effect coroutines | Continuation linearity vs resources unresolved. |
| Mixing arenas/tables with nurseries now | Needs joint destruction/cancel model. |

## Implementation gate

Bootstrap + seed + verifier/VM + authoring v5 + cancel/join corpus + docs +
constitution gates. Broader concurrency remains a new decision.
