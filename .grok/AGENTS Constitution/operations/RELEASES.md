# Releases

```
Document: Releases
Module ID: MOD-OPS-REL-001
Version: 1.1.0
Status: Binding
Authority: AGENTS.md
Applies When: Shipping, packaging, distribution, version tags
Dependencies:
  - AGENTS.md
  - operations/DELIVERY.md
  - standards/SECURITY.md
  - collaboration/REVIEW-PACKAGING.md
Overrides: None
```

---

## REL-PACKAGE-001 — Release Artifacts

When a project reaches a releasable state, deliver complete release artifacts:

* Changelogs  
* Version tags  
* Build scripts  
* Packaging (mod jars, wheels, containers, installers, etc.)  
* Distribution notes  

## REL-DETERM-001 — Deterministic Artifacts

* Prefer deterministic, signed, or at least checksummed artifacts where practical.
* Builds obey `ENG-REPRO-001` / `DEP-PIN-001`.

## REL-UPGRADE-001 — Upgrade Paths

* Document upgrade paths and breaking changes clearly.
* Never leave “release later” open without a complete, ready-to-execute plan.

## REL-HARDEN-001 — Pre-Ship Controls

* Security and quality checks required before delivery (SOP phases 8–10).
* Critical findings resolved or formally accepted by the human.

---

*Canonical home for release packaging. Human review packaging is NOT here — see REVIEW-PACKAGING.md.*
