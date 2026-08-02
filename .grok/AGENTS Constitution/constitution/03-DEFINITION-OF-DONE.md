# Definition of Done

```
Document: Definition of Done
Module ID: MOD-CONST-DONE-001
Version: 1.1.0
Status: Binding
Authority: AGENTS.md
Applies When: Always — before presenting any delivery
Dependencies:
  - AGENTS.md
  - constitution/00-CORE-LAW.md
Overrides: None
```

---

## CONST-DONE-001 — Machine-Checkable Done

A deliverable is **Done** only when **all** of the following are true:

| # | Requirement | Linked rules |
|---|-------------|--------------|
| 1 | Builds with **zero** compiler/linter/static-analysis warnings or errors on a clean checkout | `ENG-WARN-001` |
| 2 | All new and affected tests pass (including property-based and domain invariants where used) | `TEST-BEHAVIOR-001` |
| 3 | Documentation is complete, accurate, and synchronized | `DOC-SYNC-001` |
| 4 | Drop-in usable: human can compile, run, test, or ship with zero additional scaffolding | `CONST-COMPLETE-001` |
| 5 | No further “polish”, “wire-up”, “fill blanks”, or “later” work remains | `CONST-COMPLETE-001` |
| 6 | Resource budgets, hardware constraints, and cognitive-load packaging respected | `HW-RESPECT-001` |
| 7 | Delivery package is clean (no temp files, no half-applied patches) | `OPS-DEL-001` |
| 8 | Section 0 checklist fully executed and passed | `CONST-GATE-001` |
| 9 | Suggested commit messages and verification steps included | `REV-PACK-001` |

## CONST-DONE-002 — Spike Exception (Narrow)

* Exploratory spikes only when **explicitly scoped** by the human.
* Still: zero-warning, self-contained, documented enough to promote or discard without polluting mainline.
* Spikes are not a license for TODOs in production paths.

## CONST-DONE-003 — Rationale

Incomplete deliveries destroy momentum under high cognitive load and multi-project portfolios. Done means **immediately usable**, not “almost”.

---

*End of constitution/03-DEFINITION-OF-DONE.md*
