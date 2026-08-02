# LOCK — Amendment Freeze Contract

```
Document: Pack Lock
Module ID: MOD-GOV-LOCK-001
Version: 5.0.1
Status: Binding (governance)
Authority: AGENTS.md
Applies When: Amending, forking, or releasing this pack after lockdown
Dependencies:
  - AGENTS.md
  - PACK.md
  - INTEGRITY.md
  - VERSION
Overrides: None
```

---

## Status

| Field | Value |
|-------|-------|
| **Pack VERSION** | **5.0.1** |
| **Lock state** | **LOCKED baseline** |
| **Meaning** | Structure and quality law are production-stable; changes are deliberate releases, not casual edits |

This is not a cryptographic seal. It is **process lock-down**: no silent drift, no partial rewrites, no host coupling.

---

## GOV-LOCK-001 — No casual amendment

Binding law (`AGENTS.md`, `constitution/`, `standards/`, `operations/`, `collaboration/`, `specialist/`, `SOP.md`, governance files) may change only when:

1. The human director explicitly requests or approves the change.  
2. The change is a **complete file replacement** of each affected file.  
3. `VERSION` is bumped (semver).  
4. `PACK.md` / `AGENTS.md` changelogs updated as required.  
5. `tools/verify-pack.ps1` exits **0**.  
6. Prefer `tools/self-audit.ps1` and/or `tools/run-full-constitution-self.ps1` exit **0** before calling the pack ready.

## GOV-LOCK-002 — Non-law surfaces

These are **not** binding law and may be regenerated or deleted without a constitution amendment:

| Path | Role |
|------|------|
| `reports/*` | Generated diagnostics |
| `_archive_*/*` | Historical pre-modular material |
| `docs/*` | SOP evidence / pack evolution notes (advisory unless cited by human as project law) |
| `Modularize.md` | Design history (advisory) |

## GOV-LOCK-003 — One active editor surface

* Treat **one** pack tree as the active source of truth for amendments.  
* Mirrors (other folders/repos) are copies until deliberately refreshed.  
* Do not “fix a little” on a mirror and forget the active tree (`GOV-SYNC-001`).

## GOV-LOCK-004 — Forbidden silent weakenings

Even with human direction for *features*, the following require an explicit override file (project Level 4) — they are not “tweaked away” inside this pack:

* `CONST-GATE-001`  
* `CONST-COMPLETE-001`  
* `CONST-DONE-001`  
* `ENG-WARN-001`  
* `TEST-BEHAVIOR-001`  
* `SEC-INPUT-001` / `SEC-SECRET-001`  

## GOV-LOCK-005 — Release checklist (lock gate)

Before labeling a pack release ready:

```powershell
pwsh -File tools/verify-pack.ps1
pwsh -File tools/self-audit.ps1
pwsh -File tools/write-checksums.ps1
```

Optional full suite:

```powershell
pwsh -File tools/run-full-constitution-self.ps1
```

All required tools must exit **0**.

---

*End of LOCK.md*
