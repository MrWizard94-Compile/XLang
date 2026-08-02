# Pack Integrity

```
Document: Pack Integrity
Module ID: MOD-GOV-INT-001
Version: 1.2.1
Status: Binding (governance)
Authority: AGENTS.md
Applies When: Amending the constitution pack; before publishing a pack version; move/copy/mirror
Dependencies:
  - AGENTS.md
  - PACK.md
  - LOCK.md
  - RULE-REGISTRY.md
  - MODULE-INDEX.md
Overrides: None
```

This pack is a **governed system**, not a pile of Markdown. Integrity failures are defects (`CONST-COMPLETE-001` applied to the pack itself).

---

## GOV-INT-001 — Integrity Gate

Before claiming the pack is ready (or syncing a mirror), run:

```powershell
pwsh -File tools/verify-pack.ps1
```

Exit code `0` required. Non-zero aborts pack delivery.

---

## GOV-INT-002 — Required file set

All of the following must exist (relative to pack root):

### Root

* `VERSION`
* `PACK.md`
* `LOCK.md`
* `ADOPT.md`
* `AGENTS.md`
* `SOP.md`
* `README.md`
* `Modularize.md`
* `RULE-REGISTRY.md`
* `MODULE-INDEX.md`
* `INTEGRITY.md`

### Constitution

* `constitution/00-CORE-LAW.md`
* `constitution/01-AUTHORITY-AND-PRECEDENCE.md`
* `constitution/02-HUMAN-AI-CONTRACT.md`
* `constitution/03-DEFINITION-OF-DONE.md`

### Standards

* `standards/ENGINEERING.md`
* `standards/TESTING.md`
* `standards/SECURITY.md`
* `standards/DOCUMENTATION.md`
* `standards/PERFORMANCE.md`
* `standards/DEPENDENCIES.md`
* `standards/OBSERVABILITY.md`

### Operations

* `operations/DELIVERY.md`
* `operations/VERSION-CONTROL.md`
* `operations/RELEASES.md`
* `operations/REFACTORING.md`
* `operations/MIGRATIONS.md`
* `operations/PROJECT-LIFECYCLE.md`

### Collaboration

* `collaboration/MULTI-AGENT.md`
* `collaboration/CONTEXT-SWITCHING.md`
* `collaboration/REVIEW-PACKAGING.md`
* `collaboration/INSTITUTIONAL-MEMORY.md`

### Specialist

* `specialist/NOVEL-RND.md`
* `specialist/IP-AND-INVENTION.md`
* `specialist/NETWORKING.md`
* `specialist/LOW-LEVEL-SAFETY.md`
* `specialist/CONSTRAINED-HARDWARE.md`

### Templates

* `templates/MANIFEST.template.md`
* `templates/ADR.template.md`
* `templates/AUDIT.template.md`
* `templates/DELIVERY-REPORT.template.md`
* `templates/PROJECT-OVERRIDE.template.md`
* `templates/PROJECT-POINTER.template.md`

### Tools

* `tools/verify-pack.ps1`
* `tools/self-audit.ps1`
* `tools/run-full-constitution-self.ps1`
* `tools/write-checksums.ps1`

`_archive_*` is optional historical material; not required for integrity.  
`docs/` SOP evidence is recommended for pack evolution history; not all files are GOV-INT-002-required.  
`reports/*` is generated output (not required for GOV-INT-002).

---

## GOV-INT-003 — Rule ID integrity

1. Every `DOMAIN-TOPIC-NNN` ID appearing in pack Markdown (except this section’s examples) **must** be listed in `RULE-REGISTRY.md` as Active or Deprecated.  
2. Active Rule IDs must appear as a heading or bold definition in their **canonical home**.  
3. Forbidden aliases (registry deprecated table) must not appear outside the registry’s deprecated section.  
4. No Rule ID may claim two different canonical homes.

---

## GOV-INT-004 — Module manifest integrity

Every binding module under `constitution/`, `standards/`, `operations/`, `collaboration/`, `specialist/`, plus governance files, must open with a metadata block including:

* Document  
* Module ID (matching `MODULE-INDEX.md`)  
* Version  
* Status  
* Authority: `AGENTS.md`  
* Applies When  
* Dependencies  
* Overrides  

---

## GOV-INT-005 — Link integrity

* Root `AGENTS.md` module links must resolve to existing files.  
* Registry canonical homes must resolve.  
* Broken relative links are integrity defects.

---

## GOV-INT-006 — Anti-fragmentation

* Section 0 checklist exists **only** in root `AGENTS.md` (full form).  
* Modules may reference checklist items by number/Rule ID; they must not duplicate the full 15-point list.  
* Full rule text lives only at the canonical home (`CONST-ONEHOME-001`).

---

## GOV-SYNC-001 — Mirror / multi-copy discipline

When this pack is copied or mirrored:

1. Choose one **source of truth** for the amend.  
2. Apply complete file replacements.  
3. Run `tools/verify-pack.ps1` on **every** copy that remains in use.  
4. Bump `VERSION` / `PACK.md` / `AGENTS.md` changelog for material changes.  
5. Do not leave active copies on different major/minor versions without a documented reason.

Portability details: [PACK.md](PACK.md) (`GOV-PORT-*`). Adoption: [ADOPT.md](ADOPT.md).

---

## GOV-PORT-001 — Path independence (enforced)

* Binding law and tools must not embed host absolute paths.  
* `verify-pack.ps1` fails the pack if absolute host paths appear in binding files.  
* Reports may record absolute paths for diagnostics only.

## GOV-OVR-001 — Project overrides

* Project-local exceptions use `templates/PROJECT-OVERRIDE.template.md`.  
* Overrides require named human approval and expiration.  
* Overrides cannot waive safety/legal constraints.  
* Overrides cannot silently disable `CONST-GATE-001`, `CONST-COMPLETE-001`, or `ENG-WARN-001` without explicit human approval text in the override file.

---

## Manual self-audit (AI)

When amending the pack, confirm:

1. Human direction exists (`GOV-LOCK-001`)  
2. `verify-pack.ps1` → pass  
3. Registry updated if any Rule ID added/changed  
4. Module index updated if any file added/renamed  
5. `PACK.md` / `LOCK.md` / `VERSION` / folder map still accurate  
6. No forbidden aliases reintroduced  
7. No full restatement of rules outside canonical homes  
8. Version bumps on changed files  
9. No absolute host paths in binding law (`GOV-PORT-001`)  
10. After move/copy: re-run verify  
11. Prefer self-audit / full self-run green before calling locked pack ready  

See also [LOCK.md](LOCK.md).

---

*End of INTEGRITY.md*
