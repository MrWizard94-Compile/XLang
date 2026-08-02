# PACK — Universal AGENTS Constitution Identity

```
Document: Pack Identity & Portability Contract
Module ID: MOD-GOV-PACK-001
Version: 5.0.1
Status: Binding (governance)
Authority: AGENTS.md
Applies When: Moving, copying, adopting, or releasing this pack
Dependencies:
  - AGENTS.md
  - INTEGRITY.md
  - ADOPT.md
  - LOCK.md
Overrides: None
```

---

## Identity

| Field | Value |
|-------|-------|
| **Pack name** | AGENTS Constitution |
| **Pack version** | **5.0.1** (see also `VERSION`) |
| **Lock state** | **LOCKED baseline** — see [LOCK.md](LOCK.md) |
| **Classification** | Universal, reusable, movable quality + process law pack |
| **SOUL** | `AGENTS.md` |
| **Process** | `SOP.md` |
| **Not** | A single project’s app code, secrets store, or build output |

This folder is a **self-contained constitution pack**. It may live on a desktop, in a monorepo, on a USB drive, in git, or as a submodule. **Location is not identity.**

---

## GOV-PORT-001 — Path independence

* Law and tools use **pack-relative** paths only.
* No machine-specific absolute paths in binding documents (`AGENTS.md`, modules, `SOP.md`, governance files, templates).
* Tools resolve pack root from `$PSScriptRoot/..` or `-PackRoot`.
* The pack remains valid after rename or move of the folder.

## GOV-PORT-002 — Self-contained drop-in

* All required law, modules, templates, and tools ship inside this folder.
* No network dependency to *apply* the constitution (offline-first after copy).
* Optional: project-local Level 4 docs live **outside** the pack and point **in**.

## GOV-PORT-003 — Stable identity under move

* Version identity is `VERSION` + `PACK.md` + `AGENTS.md` version.
* Renaming the parent folder does **not** require content edits.
* Consumers reference the pack by relative path from their project (see `ADOPT.md`).

## GOV-PORT-004 — Copy and mirror discipline

* Any copy is a full tree copy (or git clone/submodule).
* After copy/move: run `tools/verify-pack.ps1` (must exit 0).
* Prefer one **active** source of truth per human/empire; other copies are mirrors until deliberately promoted (`GOV-SYNC-001`).

## GOV-PORT-005 — No host coupling

* Forbidden in binding docs: user home paths, OneDrive paths, OS-specific absolute roots, single-machine hostnames as requirements.
* Generated reports under `reports/` may record absolute paths for diagnostics; they are **not** law.

## GOV-PORT-006 — Universal applicability

* Domain-agnostic: games, research, web, automation, tooling, monorepos.
* Project-specific product baselines belong in Level 4 project docs, not in this pack.
* Language/stack fidelity is declared **per project**; this pack requires version fidelity without naming one stack as mandatory.

---

## Pack layout (canonical)

```text
<pack-root>/
├── VERSION · PACK.md · LOCK.md · ADOPT.md · README.md
├── AGENTS.md · SOP.md
├── RULE-REGISTRY.md · MODULE-INDEX.md · INTEGRITY.md
├── Modularize.md           # design history (advisory)
├── constitution/ · standards/ · operations/
├── collaboration/ · specialist/ · templates/
├── tools/                  # portable PowerShell verifiers
├── docs/                   # advisory SOP evidence
├── reports/                # generated; not law
└── _archive_*/             # optional history; not required
```

---

## Versioning

| Kind | Where | Rule |
|------|--------|------|
| Pack release | `VERSION`, `PACK.md`, `LOCK.md` | Semver; bump on material pack change |
| SOUL | `AGENTS.md` header | Complete-file replace + changelog |
| Modules | module metadata `Version:` | Bump per file on change |
| Process | `SOP.md` header | Complete-file replace + revision history |

| Release | Meaning |
|---------|---------|
| **5.0.0** | Universal portable pack solidification |
| **5.0.1** | Cleanup + lockdown baseline (`LOCK.md`, non-law surfaces labeled, host-coupling scrub) |

---

## Minimum portable smoke test

From **any** working directory, after move/copy:

```powershell
pwsh -File "<pack-root>/tools/verify-pack.ps1"
pwsh -File "<pack-root>/tools/self-audit.ps1"
```

Both must exit `0`.

Full suite:

```powershell
pwsh -File "<pack-root>/tools/run-full-constitution-self.ps1"
```

---

*End of PACK.md*
