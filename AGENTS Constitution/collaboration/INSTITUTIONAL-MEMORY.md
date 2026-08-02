# Institutional Memory

```
Document: Institutional Memory
Module ID: MOD-COL-MEM-001
Version: 1.1.0
Status: Binding
Authority: AGENTS.md
Applies When: Decisions, architecture changes, onboarding, long-term context
Dependencies:
  - AGENTS.md
  - standards/DOCUMENTATION.md
Overrides: None
```

---

## MEM-LIVE-001 — Living Knowledge Base

* Maintain project-local and empire-level living documents: decisions, invariants, lessons, architectural context.
* Prefer AGENTS.md / CLAUDE.md / DESIGN.md / DECISIONS.md style files that stay up to date.

## MEM-SAME-001 — Same Delivery Updates

* When a delivery changes architectural assumptions, update relevant knowledge-base documents in the **same** delivery.

## MEM-FIRST-001 — Knowledge as Source

* Treat the knowledge base as first-class source code: complete, versioned, zero-warning in spirit.
* Highest-level institutional memory is the SOUL (`AGENTS.md`); modules and SOP extend it.

## MEM-HEALTH-001 — Empire Health Signals

When asked, report:

* % deliveries passing Section 0 first attempt  
* Average iteration cycles  
* Projects with clean zero-warning builds  
* Novel invention claims with executable tested cores  
* Context-switch tax  
* Living knowledge base presence  
* Absence of “almost done” features and port debt  
* Clean git history  

Degradation → propose concrete amendments (`CONST-AMEND-001`).

---

*End of collaboration/INSTITUTIONAL-MEMORY.md*
