# M17d Design: structured offline test reports

**Status:** Accepted design for ADR-033 — implemented in package 0.30.0  
**Date:** 2026-08-04  
**Depends on:** M17/M17b/M17c  
**Rule IDs:** `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `SEC-INPUT-001`

---

## 1. Purpose

Human stdout summaries are not enough for local CI scripts. **M17d** adds
**optional** structured report files written only to caller-selected paths.

## 2. Core claim

> `aether test` and `aether project test` may write an offline structured report
> to an explicit path: native JSON (`aether.test-report/v1`) and/or a bounded
> JUnit-compatible XML suite. Stdout human summary is unchanged. Empty default
> = no report file. No network upload.

## 3. CLI

```text
aether test [path...] [--grant-*]... [--report <file.json>] [--report-junit <file.xml>]
aether project test <project> [--grant-*]... [--report <file.json>] [--report-junit <file.xml>]
```

## 4. JSON schema (conceptual)

```json
{
  "schema": "aether.test-report/v1",
  "language": "Aether",
  "version": "0.30.0",
  "passed": 2,
  "failed": 0,
  "results": [
    { "path": "examples/tests/zero_test.ae", "ok": true, "detail": "exit 0" }
  ]
}
```

## 5. JUnit XML (bounded)

Single `<testsuite>` with one `<testcase>` per result; failures as
`<failure message="..."/>`. No properties network, no attachments.

## 6. Non-goals

- Streaming CI dashboards  
- Coverage  
- Default report path (must be explicit)  
- Network publish  

## 7. Package

**0.30.0**

---

*End of DESIGN-M17D-STRUCTURED-TEST-REPORTS.md*
