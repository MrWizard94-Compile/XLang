# Delivery Operations

```
Document: Delivery Operations
Module ID: MOD-OPS-DEL-001
Version: 1.1.0
Status: Binding
Authority: AGENTS.md
Applies When: Presenting work, multi-file packages, increments under SOP
Dependencies:
  - AGENTS.md
  - constitution/03-DEFINITION-OF-DONE.md
  - collaboration/REVIEW-PACKAGING.md
Overrides: None
```

---

## OPS-DEL-001 — Clean Packages

* Deliver complete drop-in units or a clean multi-file package with MANIFEST.
* No temporary files, backup files, editor artifacts, or half-applied patches.
* Process order of research → ship is owned by `SOP.md`; this module owns delivery package quality.

## OPS-DEL-002 — Complete Replacements Default

* Default to complete file replacements (or complete new files) rather than fragmentary patches — unless tooling workflow uses patches and the package remains drop-in complete.

## OPS-DEL-003 — Quality Metrics (When Asked)

Be ready to report:

* Section 0 first-attempt pass?  
* Iteration cycles?  
* Suppressions (why)?  
* New dependencies (justification)?  
* Novel invention claim strengthened?  

Friction patterns → propose module or SOUL amendments (`CONST-AMEND-001`).

## OPS-DEL-004 — Cross-Links (Do Not Restate)

| Concern | Canonical home |
|---------|----------------|
| Review packaging checklist | `collaboration/REVIEW-PACKAGING.md` `REV-PACK-001` |
| Release artifacts | `operations/RELEASES.md` `REL-PACKAGE-001` |
| Commits | `operations/VERSION-CONTROL.md` |
| Gate | `AGENTS.md` `CONST-GATE-001` |

---

*End of operations/DELIVERY.md*
