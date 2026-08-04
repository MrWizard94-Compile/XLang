# M7 structured concurrency validation matrix

**Status:** Implementation gate for Aether 0.10 / M7 — satisfied by
[DELIVERY_REPORT-2026-08-03-M7-STRUCTURED-CONCURRENCY.md](DELIVERY_REPORT-2026-08-03-M7-STRUCTURED-CONCURRENCY.md)

**Date:** 2026-08-03

**Design:** [M7 structured nursery](DESIGN-M7-STRUCTURED-CONCURRENCY.md)

## Invariants under test

| Invariant | Evidence |
| --- | --- |
| M7-INV-001 lexical nursery | Parser/formatter reject spawn outside together; authoring round-trip. |
| M7-INV-002 source order | Dual scripts with different spawn order yield different store sequences. |
| M7-INV-003 cancel remaining | Failure fixture: later destination unchanged; error code propagates. |
| M7-INV-004 join destinations | Success fixture: all destinations hold child results. |
| M7-INV-005 body limits | Empty/9 spawns/nested/non-spawn body → `AE-TASK-001`. |
| M7-INV-006 signatures | Erroring child in total parent → `AE-TASK-002`/`AE-EFFECT-*`. |
| M7-INV-007 resource boundary | together + arena/buffer/table → `AE-TASK-003`. |
| M7-INV-008 main total | may-raise nursery in main rejected. |
| M7-INV-009 v10 + seed | Nursery ops only in v10; seed/bootstrap byte identity. |

## Acceptance

Constitution gates, seed hashes, cancel/success examples, and honest “no
parallelism claim” in the delivery report.
