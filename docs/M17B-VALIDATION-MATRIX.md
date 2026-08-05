# M17b project test validation matrix

**Status:** Product path green (package 0.28.0)  
**Date:** 2026-08-04  
**ADR:** [ADR-030](ADR-030-m17b-project-test.md)

| ID | Case | Evidence |
| --- | --- | --- |
| P1 | stdlib project test unit imports whole | `project test` exit 0 |
| P2 | CLI regression | `shipped_stdlib_project_test` |
| N1 | zero test units | fail closed |
| N2 | test unit without main | AE-MOD-006 |
| N3 | import test/main unit | AE-MOD-002 |

## Checklist

- [x] Design + ADR-030  
- [x] Schema + elaborate entry  
- [x] CLI  
- [x] Tests  
- [x] DOC-SYNC 0.28  
