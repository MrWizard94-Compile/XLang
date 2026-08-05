# M17b Design: project `role: test` units

**Status:** Accepted design for ADR-030 — implemented in package 0.28.0  
**Date:** 2026-08-04  
**Depends on:** M17 (ADR-021), M11 modules  
**Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `SEC-INPUT-001`

---

## 1. Purpose

M17 runs standalone `*_test.ae` files via single-file seed compile. Multi-module
stdlib helpers cannot be imported that way. **M17b** adds project-listed **test
units** that elaborate against the same project’s lib units.

## 2. Core claim

> A project may declare zero or more units with `role: test`. Each is a total
> program with `weave main` that may `import unit` lib units in the same project.
> `aether project test` elaborates each test entry (M11b dual-compare), pure-runs
> it, and requires exit code 0. No grants, network, or registry.

## 3. Decisions

### D1 — Schema

```json
{ "path": "import_whole_test.ae", "role": "test" }
```

- Exactly one `main` unit (unchanged).  
- Zero or more `test` units.  
- Test units **must** declare `weave main`.  
- Test units **must not** be imported by others.  
- Lib units still must not declare `main`.

### D2 — CLI

```text
aether project test <project-file>
```

- Verifies project schema/paths (same as light verify without optional lock digest
  re-check beyond document parse + unit load rules used by build).  
- For each `role: test` unit in declaration order: elaborate cone with that unit
  as entry → seed≡bootstrap → pure run → require exit 0.  
- Zero test units → fail closed with a clear error (no silent pass).  
- Process exit 0 iff all test units pass.

Standalone `aether test [path...]` is **unchanged**.

### D3 — Compile authority

Reuse M11b elaboration with a selectable entry path. No bootstrap-only product
path. Pure `run_bytecode` (empty grants).

### D4 — Non-goals

- Grants-in-tests  
- JUnit/XML  
- `role: test` without `main`  
- Parallel runners  

## 4. Package

**0.28.0**

---

*End of DESIGN-M17B-PROJECT-TEST.md*
