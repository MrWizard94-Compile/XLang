# M18 offline workspace validation matrix

**Status:** Implementation green (package 0.23.0)  
**Date:** 2026-08-04  
**Design:** [DESIGN-M18-OFFLINE-WORKSPACE.md](DESIGN-M18-OFFLINE-WORKSPACE.md)  
**ADR:** [ADR-022](ADR-022-m18-offline-workspace.md)

## Cases

| ID | Case | Evidence |
| --- | --- | --- |
| P1 | util→app depends_on | `verifies_workspace_with_depends_on_order`, examples/workspace |
| N1 | cycle | `rejects_cycle_*` AE-WORKSPACE-003 |
| N2 | path `..` | AE-WORKSPACE-002 |
| N3 | missing project | AE-WORKSPACE-004 |

## Checklist

- [x] Core API  
- [x] CLI  
- [x] Example  
- [x] Tests  
- [x] DOC-SYNC 0.23  

## Sign-off

| Gate | Status |
| --- | --- |
| Design + ADR-022 | **Accepted / Implemented** |
| Implementation | **Green (0.23.0)** |
