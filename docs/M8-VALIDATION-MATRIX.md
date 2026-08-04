# M8 host ABI pilot validation matrix

**Status:** Implementation gate for Aether 0.11 / M8 — satisfied by
[DELIVERY_REPORT-2026-08-04-M8-HOST-ABI.md](DELIVERY_REPORT-2026-08-04-M8-HOST-ABI.md)

**Date:** 2026-08-04

**Design:** [M8 host ABI pilot](DESIGN-M8-HOST-ABI-PILOT.md)

## Invariants under test

| Invariant | Evidence |
| --- | --- |
| M8-INV-001 host weave form | Parser accepts body-less host weave; rejects body/`raises`. |
| M8-INV-002 primitive ABI | Illegal host param/result types → `AE-HOST-001`. |
| M8-INV-003 HOST_CALL | Artifact contains opcode 65 for host calls; guest calls remain CALL. |
| M8-INV-004 fail closed | Unknown/missing host service → deterministic `AE-HOST-003` / runtime error. |
| M8-INV-005 ownership | Host rejects record/resource args; borrow Text works. |
| M8-INV-006 guest return | Host invoke still cannot return resources/records. |
| M8-INV-007 pure fixture | `whole_inc` / `text_extent` match documented results. |
| M8-INV-008 seed parity | Host example seed≡bootstrap. |
| M8-INV-009 v11 only | HOST_CALL under v10 rejected. |

## Corpus

| Positive | Negative |
| --- | --- |
| whole_inc + text_extent example | host weave with body; raises; Buffer param |
| run under pure fixture | call undeclared host name; HOST_CALL in v10 artifact |
