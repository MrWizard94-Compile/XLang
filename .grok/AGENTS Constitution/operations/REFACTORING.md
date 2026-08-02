# Refactoring

```
Document: Refactoring
Module ID: MOD-OPS-REF-001
Version: 1.1.0
Status: Binding
Authority: AGENTS.md
Applies When: Large or complex refactors
Dependencies:
  - AGENTS.md
  - standards/ENGINEERING.md
  - standards/TESTING.md
Overrides: None
```

---

## REF-PLAN-001 — Plan First

* For large or complex refactors, outline a clear plan first unless the human said proceed directly.
* Break into logical, incremental, testable steps.

## REF-GREEN-001 — Always Green

* Each step leaves the codebase working, buildable, and zero-warning (`ENG-WARN-001`).
* After significant refactors: clean build, all tests pass, static analysis clean, core flows verified, docs updated (`DOC-SYNC-001`).

## REF-SAFE-001 — Safe Patterns

* Prefer strangler fig, feature flags, parallel implementations, branch-by-abstraction.
* Preserve observable behavior unless intentional, documented changes are part of the task.
* Complete-code and dependency-first still apply (`CONST-COMPLETE-001`, `CONST-DEP-001`).

---

*End of operations/REFACTORING.md*
