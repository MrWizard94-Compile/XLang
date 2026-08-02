# Core Law

```
Document: Core Law
Module ID: MOD-CONST-CORE-001
Version: 1.1.0
Status: Binding
Authority: AGENTS.md
Applies When: Always (referenced by CONST-COMPLETE-001, CONST-DEP-001)
Dependencies:
  - AGENTS.md
Overrides: None
```

---

## CONST-COMPLETE-001 — No Partial Delivery

* No deferrals, stubs, placeholders, TODOs, “implement later”, “for now”, “skeleton”, or “wire-up later”.
* Deliver full, complete, wired-up, working code (or equivalent artifacts: configs, schemas, scripts, tests, docs, build files, assets, deployment manifests) every time.
* If a feature, class, method, module, endpoint, model, mixin, packet, state machine, training loop, or subsystem is requested, implement it fully — supporting code, registrations, DI/wiring, error handling, logging, tests, and integrations.
* Partial “happy path only” work that leaves error cases, nullability, cleanup, lifecycle hooks, or integrations unfinished is **not acceptable**.

## CONST-DEP-001 — Dependency-First

* If work depends on types, services, configs, helpers, tests, docs, generators, mixins, handlers, or systems that do not yet exist, **build and deliver those dependencies first** (or in the same complete delivery).
* Never leave broken references, missing imports, unresolved symbols, dangling registrations, or “we’ll add this later” gaps.
* After every delivery the project must build cleanly with zero warnings and be immediately runnable/testable.

## CONST-ONESHOT-001 — One-Shot Completeness

* Prefer drop-in solutions (or minimal, documented configuration).
* When the human requests incremental milestones, each slice is still a complete, tested, documented, zero-warning unit — not a skeleton.

## CONST-EMPIRE-001 — Empire Scope

* These laws apply to all projects in the portfolio: game mods, novel AI architectures, automation, web/backend/frontend/mobile, research, creative pipelines, tooling, monorepos, open-source foundations, and future domains.
* Core subsystems must be correct, efficient, and maintainable from day one.
* Exploratory spikes require explicit scope; still zero-warning and complete enough to promote or discard cleanly.

---

*Canonical home for completeness and dependency-first. Do not restate in full elsewhere — link CONST-COMPLETE-001 / CONST-DEP-001.*
