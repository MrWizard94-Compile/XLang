# Human Director Review Packaging

```
Document: Review Packaging
Module ID: MOD-COL-REV-001
Version: 1.1.0
Status: Binding
Authority: AGENTS.md
Applies When: Any delivery presented to the human
Dependencies:
  - AGENTS.md
  - constitution/03-DEFINITION-OF-DONE.md
  - operations/DELIVERY.md
Overrides: None
```

---

## REV-PACK-001 — Zero-Friction Handoff

Every delivery must include:

1. Clear title of what was delivered and why  
2. One-sentence summary of the change  
3. MANIFEST (or equivalent) listing every changed file with a one-line purpose each — template: `templates/MANIFEST.template.md`  
4. Full file contents for every changed/new file (complete replacements), or apply-ready package  
5. Suggested git commit message(s) ready to copy  
6. How to verify (exact commands, expected output, key checks)  
7. Known remaining risks or deliberate trade-offs  
8. Self-audit confirmation that Section 0 (`CONST-GATE-001`) was fully executed  
9. What the human should do next, in priority order  
10. If large: short “Review Path” (optimal inspection order)  

**Rationale**: Human time and hyperfocus are the scarcest resources.

## REV-PACK-002 — Rule ID Traceability

* Delivery reports should list satisfied Rule IDs (template: `templates/DELIVERY-REPORT.template.md`).
* Example minimum: `CONST-GATE-001`, `CONST-DONE-001`, `ENG-WARN-001`, `TEST-BEHAVIOR-001`, `DOC-SYNC-001`.

## REV-PACK-003 — Not Release Packaging

* Human **review** packaging is this module.
* Product **release** packaging is `operations/RELEASES.md` (`REL-PACKAGE-001`).

---

*Canonical home for human review packaging. Do not restate the full checklist in other modules.*
