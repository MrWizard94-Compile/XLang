# Engineering Standards

```
Document: Engineering Standards
Module ID: MOD-ENG-001
Version: 1.1.0
Status: Binding
Authority: AGENTS.md
Applies When: Any source code is created or modified
Dependencies:
  - AGENTS.md
  - constitution/03-DEFINITION-OF-DONE.md
Overrides: None
```

---

## ENG-WARN-001 — Zero Warnings / Errors

* No compiler error, warning, lint issue, static analysis finding, type error, runtime warning, deprecation notice, or nullability violation is harmless.
* Fix at **root cause**. Do not suppress unless unavoidable third-party behavior after cleaner alternatives fail.
* Suppressions require: documented justification, planned removal date, tracking note, and human visibility in the delivery report.
* Treat warnings as errors. Zero-warning builds are the steady state.
* Third-party warnings: upgrade, configure surgically, or fix upstream — not blanket ignore.
* Runtime warnings in testing/profiling must be fixed.
* **Stop-ship**: a delivery with open warnings is incomplete under `CONST-DONE-001` regardless of feature completeness.
* CI or local builds that “pass with warnings” do **not** satisfy this rule.

## ENG-STACK-001 — Version & Project Appropriateness

* Use APIs, patterns, idioms, and language features matching **declared** versions (build files, lockfiles, project docs).
* Do not silently backport newer-major patterns or forward-port outdated ones unless intentional and documented.
* Stay consistent with existing architecture and style unless improving them is the task.
* Respect the runtime model (client/server, concurrency, tick vs real-time, etc.).

## ENG-RESEARCH-001 — Research-First When Uncertain

* If not 100% certain about an API, thread-safety, deprecation, performance, or best practice — research first.
* Prefer official docs, library source, version-specific issues, in-repo patterns.
* Cross-verify conflicts. Do not guess. When still unsure, ask the human.
* Note research date/sources for rapidly evolving areas.

## ENG-CODE-001 — Code Quality

* Strict null/undefined safety for the language. No raw types, unsafe casts, or type-erasure abuse.
* Follow project lint/format/static analysis exactly; violations fail the build.
* Intention-revealing names; simple correct solutions over clever ones.
* Document non-obvious behavior, design decisions, and invariants.
* Avoid premature generalization (Rule of Three).
* Prefer immutable data and pure functions for core logic where practical.

## ENG-FW-001 — Framework & Stack Idioms

* Respect target framework lifecycle, registration, DI, events, config, error propagation, and extensibility patterns.
* When porting: semantic mapping, tests, documented divergences.
* Mind concurrency, memory, I/O, and async models.

## ENG-REPRO-001 — Reproducibility & Offline-First

* Same source + same lockfiles → same binary/output.
* Prefer locked dependency versions (detail: `DEPENDENCIES.md`).
* Generators and asset pipelines produce stable, deterministic output.
* Prefer offline-capable workflows after initial setup.
* Document non-deterministic steps and controls.
* Novel kernels must be verifiable without external services.

## ENG-SCOPE-001 — Empire-Wide Applicability

* Engineering standards apply across all portfolio domains.
* Project-local notes may tailor idioms; they may not weaken `ENG-WARN-001` or `CONST-*`.

---

*Canonical home for zero-warnings, stack fidelity, research-first, and general code quality. Link rule IDs from other docs.*
