# M20b stdlib layer 1 validation matrix

**Status:** Product path green (package 0.27.0)  
**Date:** 2026-08-04  
**ADR:** [ADR-029](ADR-029-m20b-stdlib-layer1.md)

| ID | Case | Evidence |
| --- | --- | --- |
| P1 | stdlib project verifies | `project verify` |
| P2 | stdlib project builds dual-compare | `project build` exit path |
| P3 | demo `double 21` | run exit **42** |
| P4 | `aether test stdlib/whole_test.ae` | exit 0 |
| P5 | truth/text imports compile in demo cone | main may import only whole; units validated as libs |
| N1 | host I/O in stdlib | non-goal (not added) |

## Checklist

- [x] Design + ADR-029  
- [x] Modules + project file  
- [x] Tests  
- [x] DOC-SYNC 0.27  
