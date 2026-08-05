# M17c grants-in-tests validation matrix

**Status:** Product path green (package 0.29.0)  
**Date:** 2026-08-04  
**ADR:** [ADR-031](ADR-031-m17c-grants-in-tests.md)

| ID | Case | Evidence |
| --- | --- | --- |
| P1 | pure suite unchanged without grants | existing CLI tests |
| P2 | host-io test passes with `--grant-read` | CLI unit test |
| N1 | host-io test fails without grant | CLI unit test |
| N2 | bad grant root | fail closed |

## Checklist

- [x] Design + ADR  
- [x] Implementation  
- [x] Tests  
- [x] DOC-SYNC 0.29  
