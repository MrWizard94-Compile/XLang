# M17c Design: optional grants on `aether test`

**Status:** Accepted design for ADR-031 — implemented in package 0.29.0  
**Date:** 2026-08-04  
**Depends on:** M17, M14 host I/O grants  
**Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`

---

## 1. Purpose

M17 pure-runs tests with empty grants. Host I/O examples cannot be regression-
tested via `aether test` without a grant path. **M17c** reuses the same
operator-selected `--grant-*` surface as `aether run`.

## 2. Core claim

> `aether test` and `aether project test` default to empty grants (pure fixtures
> only). Operators may pass explicit `--grant-read` / `--grant-write` /
> `--grant-env` roots/names with the same path-jail and validation as `aether
> run`. No ambient grants; no network.

## 3. CLI

```text
aether test [path...] [--grant-read <dir>]... [--grant-write <dir>]... [--grant-env <NAME>]...
aether project test <project-file> [--grant-read <dir>]... ...
```

- Paths and grant flags may interleave for `aether test`.  
- Empty grants preserve all existing pure suite behavior.  
- Pass still requires exit code 0.

## 4. Non-goals

- Default-on grants  
- JUnit/XML  
- Network grants  
- Ambient cwd as grant  

## 5. Package

**0.29.0**

---

*End of DESIGN-M17C-GRANTS-IN-TESTS.md*
