# Project Lifecycle

```
Document: Project Lifecycle
Module ID: MOD-OPS-LIFE-001
Version: 1.1.1
Status: Binding
Authority: AGENTS.md
Applies When: Project birth, growth, maintenance, archive, deprecation
Dependencies:
  - AGENTS.md
  - SOP.md
Overrides: None
```

---

## LIFE-PHASE-001 — Respect Phase

| Phase | Expectation |
|-------|-------------|
| Birth / Spike | Complete and zero-warning, scoped experimental |
| Growth / Production | Full constitutional rigor |
| Maintenance | Stability and clean ports |
| Archive / Deprecation | Clean final state, archive notes, “do not use for new work” markers |

## LIFE-NODEAD-001 — No Half-Dead Projects

* Never leave projects half-dead.
* Keep them healthy or archive them cleanly.

## LIFE-TOOL-001 — Tooling DX

* Prefer free, open, offline-capable, reproducible tooling.
* Document expected environment in project READMEs.
* New tooling: complete setup + verification steps.
* Avoid permanent paid/cloud seats unless already adopted.
* Human DX should be zero-friction.

## LIFE-BIRTH-001 — Project birth (RepoForge)

For **new Python projects**, prefer **RepoForge** (empire bootstrap tool) to:

1. Scaffold venv, git, profile deps, tests, optional remote.
2. Attach this constitution pack via `--constitution` (pointer) or `--constitution-vendor`.
3. Land a Level-4 `AGENTS.md` so agents load quality law immediately.

See pack [ADOPT.md](../ADOPT.md) Option D. Non-Python births still use
`templates/PROJECT-POINTER.template.md` manually.

## LIFE-PROCESS-001 — Process vs Lifecycle

* Product delivery phases (research → deliver) live in **SOP.md**.
* This module owns **project phase policy** and tooling DX, not the SOP step list.

---

*End of operations/PROJECT-LIFECYCLE.md*
