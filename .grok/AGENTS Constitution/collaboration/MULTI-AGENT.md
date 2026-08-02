# Multi-Agent Collaboration

```
Document: Multi-Agent Collaboration
Module ID: MOD-COL-MA-001
Version: 1.1.0
Status: Binding
Authority: AGENTS.md
Applies When: Multiple AIs/agents or parallel agent threads
Dependencies:
  - AGENTS.md
  - operations/VERSION-CONTROL.md
  - collaboration/REVIEW-PACKAGING.md
Overrides: None
```

---

## AI-COORD-001 — Ownership First

* Establish clear ownership of files, subsystems, and responsibilities before work begins.
* Prefer sequential, dependency-ordered work over simultaneous edits to the same files.

## AI-COORD-002 — Parallel Boundaries

* When parallel work is required: clear branch or package boundaries and a **single integration owner**.
* Never allow two AIs to produce conflicting complete replacements of the same file without explicit reconciliation.

## AI-COORD-003 — One Coherent Package

* Every multi-agent delivery must still pass the full Section 0 checklist as **one coherent package** (`CONST-GATE-001`).
* Include a short Coordination Note: who did what, residual integration risks.
* Final package must look like it came from a single elite engineer.
* **Stop-ship**: conflicting complete replacements of the same file without reconciliation, or multiple partial packages presented as “done,” violate this rule.

## AI-COORD-004 — No Parallel Root SOUL Edits

* Only one agent may amend `AGENTS.md` / pack governance files in a given delivery window.
* Pack integrity (`GOV-INT-001`) must pass after multi-agent integration.

---

*Canonical home for multi-agent protocol.*
