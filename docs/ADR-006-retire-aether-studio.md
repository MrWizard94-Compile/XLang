# ADR-006: Retire the active Aether Studio workbench

**Status:** Accepted
**Date:** 2026-08-01
**Decision makers:** WPAI product direction; explicit human direction
**Related Rule IDs:** DOC-ADR-001, DOC-SYNC-001, CONST-COMPLETE-001,
CONST-DEP-001, ENG-WARN-001, SEC-INPUT-001, DEP-MIN-001

## Context

The active `apps/xlang-studio` Tauri/React workbench was created to support
invention of the Aether language. The language now has an executable,
seed-hosted compiler/VM, a CLI, an AETH verifier, an evidence-gated roadmap,
and M3's versioned structural authoring contracts without needing an active
desktop application.

Keeping the app made the repository harder to navigate and made the product
boundary less clear. It also carried a separate frontend, Tauri host, model
review integration, installer artifacts, and dependency graph that were not
needed for the language toolchain. The user directed that the app portion be
scrapped rather than retained as an active product surface.

## Decision

Remove the active Studio package at `apps/xlang-studio`, its Tauri workspace
member, its frontend/host/model dependencies, its generated Windows installers,
and its dedicated project-agent role. Remove active Studio, Tauri, and Ollama
claims and gates from current product documentation.

Retain the Aether core, seed compiler, CLI, AETH verifier/VM, forge ABI,
examples, structural authoring protocol, and all language design evidence. The
CLI remains the only active product interface. It owns explicit local file I/O:
`structure` emits to stdout; `compile`, `forge`, and `apply-edit` write only to
caller-selected output paths after their documented validation path succeeds.

Keep `legacy/`, including `legacy/aether-genesis-ai-studio`, as historical
reference material only. It is not restored to the production build or used as
an application replacement.

## Consequences

The repository has one active user-facing path: the Rust CLI. Removing the
desktop and model-review surface reduces dependency, packaging, and network
authority complexity while preserving the language's AI-first structural
contracts and deterministic compiler authority.

There is no active graphical editor, local WebView persistence, installer, or
model-review feature after this decision. Any future interface must be proposed
as a new product decision with an explicit authority/security boundary,
dependency rationale, tests, documentation, and constitution gate. It must not
be reintroduced merely by wiring legacy material or by bypassing the CLI/core
validation path.

The exact former implementation remains recoverable from Git history; the
repository's active tree and lockfile no longer carry it.

## Alternatives considered

| Option | Pros | Cons |
| --- | --- | --- |
| Keep Studio active and freeze it | Preserves a GUI immediately. | Retains an unneeded product surface, dependencies, artifacts, and confusing active scope. |
| Move Studio into `legacy/` | Keeps source visible outside history. | Duplicates historical material and blurs the reference-only boundary. |
| Retire the active Studio package and preserve history (chosen) | Leaves a clear language/CLI toolchain with a small active dependency surface. | A future GUI needs a fresh, fully justified design and implementation. |

## Links

* Related product contract: [MANIFEST.md](../MANIFEST.md), [ARCHITECTURE.md](ARCHITECTURE.md), [AETHER_AUTHORING_PROTOCOL_v2.md](AETHER_AUTHORING_PROTOCOL_v2.md).
* Related structural decision: [ADR-005](ADR-005-structural-authoring-contract.md).
* Related historical intake: [LEGACY.md](LEGACY.md), [AUDIT_REPORT.md](../AUDIT_REPORT.md).
