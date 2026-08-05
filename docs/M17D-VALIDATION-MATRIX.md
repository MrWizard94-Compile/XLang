# M17d structured test reports validation matrix

**Status:** Product path green (package 0.30.0)  
**Date:** 2026-08-04  
**ADR:** [ADR-033](ADR-033-m17d-structured-test-reports.md)

| ID | Case | Evidence |
| --- | --- | --- |
| P1 | `--report` writes JSON with schema | CLI unit test |
| P2 | `--report-junit` writes testsuite XML | CLI unit test |
| P3 | No flags → no report file required | existing suites |
| N1 | Unwritable report path | fail closed |

## Checklist

- [x] Design + ADR  
- [x] Implementation  
- [x] Tests  
- [x] DOC-SYNC 0.30  
