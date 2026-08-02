# ADOPT — Use, Move, and Point Projects at This Pack

```
Document: Pack Adoption Guide
Module ID: MOD-GOV-ADOPT-001
Version: 5.0.1
Status: Binding guidance (how to attach projects)
Authority: AGENTS.md
Applies When: Installing, moving, or linking the constitution pack into a project
Dependencies:
  - PACK.md
  - LOCK.md
  - AGENTS.md
  - templates/PROJECT-POINTER.template.md
Overrides: None
```

---

## What you are adopting

A **universal constitution pack**: quality law (`AGENTS.md` + modules) + process law (`SOP.md`) + verification tools.

You are **not** adopting a product codebase. Level 4 project law (PRDs, app architecture) stays in the project.

---

## Option A — Shared empire pack (recommended)

Keep one pack in a stable location (any disk/path). Each project’s root `AGENTS.md` **points** at it.

1. Place/copy this entire folder anywhere (example names: `AGENTS-Constitution/`, `law/agents-constitution/`).
2. In the project root, create `AGENTS.md` from `templates/PROJECT-POINTER.template.md`.
3. Set the relative path to this pack.
4. From the pack root, run:

```powershell
pwsh -File tools/verify-pack.ps1
```

5. Tell agents/humans: load project pointer → pack `AGENTS.md` → always-load set.

## Option B — Vendored copy inside a project

Copy the full pack into the repo (e.g. `law/AGENTS-Constitution/` or `AGENTS Constitution/`).

1. Copy tree intact (do not cherry-pick single files).
2. Project root pointer → vendored pack.
3. Run `tools/verify-pack.ps1` inside the vendored pack after copy.
4. Update pack deliberately; avoid silent drift across repos.

## Option C — Git submodule / subtree

1. Add pack as submodule or subtree at a fixed relative path.
2. Pin commit/tag for reproducibility.
3. Project pointer uses that relative path.
4. After update: `verify-pack.ps1` exit 0.

## Option D — RepoForge (Python project birth)

**RepoForge** (separate empire tool) bootstraps Python projects and can attach this pack:

```bash
# Pointer into this pack
python path/to/repoforge.py my-tool --profile cli \
  --constitution "path/to/this-pack"

# Vendor pack into law/AGENTS-Constitution/
python path/to/repoforge.py my-tool --constitution "path/to/this-pack" \
  --constitution-vendor
```

Also supported: `REPOFORGE_CONSTITUTION` env, `constitution_path` in `~/.repoforge.json`,
and auto-discovery of a nearby folder named `AGENTS Constitution` / `AGENTS-Constitution`
(or a monorepo layout that contains that pack).

RepoForge writes a Level-4 `AGENTS.md` pointer (same role as
`templates/PROJECT-POINTER.template.md`).

---

## Moving the pack

1. Move or rename the folder freely (`GOV-PORT-001`).
2. Update **project pointers** (relative paths) in every consumer project.
3. Re-run:

```powershell
pwsh -File tools/verify-pack.ps1
pwsh -File tools/self-audit.ps1
```

4. No edits to binding law are required solely because the path changed.

---

## Always-load set (for AI agents)

1. Project root pointer (if any)  
2. Pack `AGENTS.md`  
3. Pack `SOP.md`  
4. `constitution/03-DEFINITION-OF-DONE.md`  
5. `standards/ENGINEERING.md`  
6. `standards/TESTING.md`  
7. `standards/DOCUMENTATION.md`  

Then load modules per the applicability matrix in pack `AGENTS.md`.

---

## Do / Don’t

| Do | Don’t |
|----|--------|
| Copy the whole tree | Ship a half-pack of random modules |
| Use relative pointers | Hardcode absolute host paths into project law |
| Run verify after move/copy | Assume copy is intact without check |
| Keep Level 4 project docs local | Stuff product PRDs into the universal pack |
| Override with `PROJECT-OVERRIDE` template | Silently weaken `CONST-GATE-001` / `ENG-WARN-001` |

---

## Quick consumer layout

```text
my-project/
├── AGENTS.md                 # pointer only (template)
├── docs/                     # Level 4 project law
├── src/
└── (optional) law/
    └── AGENTS-Constitution/  # vendored pack
```

---

## Uninstall / replace

1. Remove or repoint project `AGENTS.md`.  
2. Delete vendored pack folder if used.  
3. Shared pack can remain for other projects.

---

*End of ADOPT.md*
