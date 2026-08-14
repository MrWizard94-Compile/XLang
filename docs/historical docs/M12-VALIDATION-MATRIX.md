# M12 fine-grained structural edits validation matrix

**Status:** Implemented (0.16.0) —
[DELIVERY_REPORT-2026-08-04-M12-FINE-GRAINED-EDITS.md](DELIVERY_REPORT-2026-08-04-M12-FINE-GRAINED-EDITS.md)  
**Date:** 2026-08-04  
**Design:** [DESIGN-M12-FINE-GRAINED-EDITS.md](DESIGN-M12-FINE-GRAINED-EDITS.md)  
**ADR:** [ADR-016](ADR-016-m12-fine-grained-edits.md)

## Invariants

| ID | Evidence |
| --- | --- |
| M12-INV-001 | apply-edit rejects v6 protocol |
| M12-INV-002 | atom/expression-only paths rejected |
| M12-INV-003 | stale baseSource rejected |
| M12-INV-004 | seed compile before write (CLI) |
| M12-INV-005 | wrong statement kind rejected |
| M12-INV-006 | OOB index rejected |
| M12-INV-007 | nested choose path works; illegal nest rejected |
| M12-INV-008 | no network |

## Positive

| ID | Case | Expect |
| --- | --- | --- |
| P1 | replaceStatement yield in main | canonical source; seed compiles |
| P2 | insertStatementAt body/0 | new bind first |
| P3 | insertStatementAfter body/0 | second statement |
| P4 | deleteStatement body/0 | remaining valid |
| P5 | replaceStatement inside whenBright | nested path |
| P6 | top-level v7 replace weave still works | regression |
| P7 | structure round-trip after edit | AST stable |

## Negative

| ID | Case | Expect |
| --- | --- | --- |
| N1 | v6 protocol | AE-EDIT-002 |
| N2 | path `weave:main/body/0/value` | AE-EDIT-010 |
| N3 | index 99 | AE-EDIT-011 |
| N4 | stale baseSource | existing stale code |
| N5 | Record declaration as statement | AE-EDIT-012 |
| N6 | path into Together spawn field | AE-EDIT-013 / 010 |
| N7 | ops > 32 | AE-EDIT-006 |

## Implementation checklist

- [x] Schemas v7  
- [x] Protocol doc v7  
- [x] apply-edit statement ops  
- [x] Core tests P*/N*  
- [x] CLI seed-before-write regression  
- [x] DOC-SYNC package 0.16  
- [x] Delivery report  

## Sign-off

| Gate | Status |
| --- | --- |
| Design + ADR-016 | **Accepted** |
| Implementation | **Done** (0.16.0) |
