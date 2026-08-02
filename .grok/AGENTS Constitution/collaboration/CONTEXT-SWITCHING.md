# Context Switching and Multi-Project

```
Document: Context Switching
Module ID: MOD-COL-CTX-001
Version: 1.1.0
Status: Binding
Authority: AGENTS.md
Applies When: Multi-project work, long absences, fresh AI onboarding to a project
Dependencies:
  - AGENTS.md
  - collaboration/INSTITUTIONAL-MEMORY.md
Overrides: None
```

---

## CTX-SELF-001 — Self-Contained Deliveries

* Every delivery must be self-contained enough that the human (or a fresh AI) can switch projects without re-deriving context from scratch.
* Prefer project-local README / AGENTS / design notes that restore productivity after weeks away.

## CTX-GRAPH-001 — Cross-Project Dependencies

* When a delivery spans projects or creates shared libraries: document dependency graph and ownership clearly.

## CTX-ASSUME-001 — No Silent Assumptions

* Never assume the human still has the previous conversation’s full context.
* Restate critical assumptions in delivery notes when relevant.

## CTX-ONBOARD-001 — Onboarding Path

* New AIs: `AGENTS.md` first, then `SOP.md`, then always-load modules, then task-applicable modules.
* New humans: constitution entry + SOP + project knowledge base.
* Start Here path required in major projects (`DOC-START-001`).

---

*End of collaboration/CONTEXT-SWITCHING.md*
