# M9 offline project tooling validation matrix

**Status:** Implementation gate for Aether 0.12 / M9 — satisfied by
[DELIVERY_REPORT-2026-08-04-M9-PROJECT-TOOLING.md](DELIVERY_REPORT-2026-08-04-M9-PROJECT-TOOLING.md)

**Date:** 2026-08-04

**Design:** [M9 project tooling](DESIGN-M9-PROJECT-TOOLING.md)

## Invariants under test

| Invariant | Evidence |
| --- | --- |
| M9-INV-001 schema | Malformed schema/name/units rejected. |
| M9-INV-002 units | Zero mains, two mains, bad role rejected. |
| M9-INV-003 path bounds | Absolute paths and `..` escape rejected. |
| M9-INV-004 lock | Wrong digest fails; correct digest passes. |
| M9-INV-005 compile | verify seed-compiles units; bad source fails AE-PROJECT-004. |
| M9-INV-006 format | Round-trip matches format_program. |
| M9-INV-007 offline | No network APIs in project/format path. |

## Corpus

| Positive | Negative |
| --- | --- |
| examples/project/aether.project.json | Escape path, lock mismatch, missing main |
| format host-pilot.ae | format invalid source fails |
