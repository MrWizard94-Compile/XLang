# AGENTS — Project Entry Point (Pointer)

**Project:** `<PROJECT_NAME>`  
**Pack:** AGENTS Constitution (universal)  
**Pack path (relative to this file):** `<RELATIVE_PATH_TO_PACK>/`  

This file is **Level 4 project law entry only**. It does not duplicate the constitution.

---

## Binding pack

→ **Constitution (SOUL):** [`<RELATIVE_PATH_TO_PACK>/AGENTS.md`](<RELATIVE_PATH_TO_PACK>/AGENTS.md)  

→ **Process (SOP):** [`<RELATIVE_PATH_TO_PACK>/SOP.md`](<RELATIVE_PATH_TO_PACK>/SOP.md)  

→ **Adopt / move guide:** [`<RELATIVE_PATH_TO_PACK>/ADOPT.md`](<RELATIVE_PATH_TO_PACK>/ADOPT.md)  

→ **Pack identity:** [`<RELATIVE_PATH_TO_PACK>/PACK.md`](<RELATIVE_PATH_TO_PACK>/PACK.md)  

---

## Always load (via pack)

* Pack `AGENTS.md`  
* Pack `SOP.md`  
* Pack `constitution/03-DEFINITION-OF-DONE.md`  
* Pack `standards/ENGINEERING.md`  
* Pack `standards/TESTING.md`  
* Pack `standards/DOCUMENTATION.md`  

Then load pack modules per the applicability matrix in pack `AGENTS.md`.

---

## Project-local law (Level 4)

* Architecture / DESIGN: `<path>`  
* PRD / product docs: `<path>`  
* ADRs: `<path>`  
* Overrides (if any): use pack `templates/PROJECT-OVERRIDE.template.md` and store under this project  

Project law may **tighten** standards. It may not weaken `CONST-*`, `ENG-WARN-001`, `TEST-BEHAVIOR-001`, or `SEC-INPUT-001` without a documented override.

---

## Verify pack (from pack root)

```powershell
pwsh -File "<RELATIVE_PATH_TO_PACK>/tools/verify-pack.ps1"
```

Must exit `0` after pack install, move, or update.

---

## Evolution

* Do not edit the pack by scattering project notes into it.  
* Evolve pack via complete file replacements per pack `AGENTS.md` amendment rules.  
* Evolve this pointer when the relative pack path changes.
