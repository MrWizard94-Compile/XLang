# PKG-001 Design: Offline Workspace Locks

**Status:** Implemented in package **0.35.0**
**Date:** 2026-08-08
**Decision record:** [ADR-041](ADR-041-pkg-001-offline-workspace-locks.md)
**Validation:** [PKG-001 validation matrix](PKG-001-VALIDATION-MATRIX.md)
**Delivery evidence:** [PKG-001 delivery report](DELIVERY_REPORT-2026-08-08-PKG-001-OFFLINE-WORKSPACE-LOCKS.md)
**Authority:** Human-ordered offline-package-polish backlog; current
[ROADMAP.md](../Current%20state/ROADMAP.md)
**Depends on:** M9 project locks, M18 offline workspaces, M22 cross-package imports
**Rule IDs:** `CONST-CONTRACT-001`, `CONST-DEP-001`, `DOC-ADR-001`,
`DOC-SYNC-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`, `CONST-COMPLETE-001`

---

## 1. Scope freeze

PKG-001 closes the reproducibility gap left deliberately open by M18: a
workspace can name local packages, but it cannot currently pin the identity of
each package manifest as a unit. The increment adds a complete **offline,
opt-in workspace lock workflow**.

It does not add a registry, URL, network fetch, version solver, package
archive, guest-visible capability, source form, AETH revision, or seed-language
surface. Existing unlocked `aether.workspace/v1` documents remain supported.

## 2. Core claim

> When an `aether.workspace/v1` document carries a complete PKG-001 lock,
> `aether workspace verify` checks every named package's path, project identity,
> raw project-manifest SHA-256 digest, and complete nested project unit lock
> locally. A locked `workspace build` performs that verification before writing
> its artifact.

The claim is intentionally limited to paths under the local workspace root and
the files named by the workspace and project documents. It is not package
publication, dependency solving, source signing, or remote trust.

## 3. Data model

PKG-001 adds one optional field to the existing `aether.workspace/v1` document:

```json
{
  "schema": "aether.workspace/v1",
  "name": "workspace_demo",
  "version": "0.1.0",
  "packages": [
    { "name": "util", "path": "util" },
    { "name": "app", "path": "app", "depends_on": ["util"] }
  ],
  "lock": {
    "packages": [
      {
        "name": "util",
        "path": "util",
        "project_name": "util_pkg",
        "project_version": "0.1.0",
        "project_sha256": "<64 lowercase hexadecimal characters>"
      },
      {
        "name": "app",
        "path": "app",
        "project_name": "app_pkg",
        "project_version": "0.1.0",
        "project_sha256": "<64 lowercase hexadecimal characters>"
      }
    ]
  }
}
```

`project_sha256` is the SHA-256 digest of the raw bytes of that package's
`aether.project.json`. Each locked package project must itself carry the
existing complete `aether.project/v1` unit lock. That nested lock pins the raw
source unit bytes; the workspace lock pins the project metadata and the unit
lock together.

The workspace lock lists every workspace package exactly once. Every lock
entry's `name` and `path` must exactly match the corresponding workspace package
declaration. `project_name` and `project_version` must exactly match the parsed
package project manifest during verification.

The v1 extension is optional so current unlocked workspaces retain their M18/M22
behavior. A toolchain predating PKG-001 rejects the new field under its existing
strict-schema policy rather than silently ignoring the lock.

## 4. Commands and write authority

```text
aether project lock <project-file> [--write]
aether workspace lock <workspace-file> [--write]
```

Without `--write`, each command writes the canonical fully locked JSON document
to standard output and performs no disk write. `--write` is the only form that
replaces the named input document; it is explicit, local, and uses no network.

The project command first verifies all declared units, then derives the existing
project unit hashes. The workspace command requires every package project to
already have that complete project lock, verifies every package in deterministic
dependency order, and then derives the workspace entries. It does not silently
edit package project files. The safe refresh sequence is therefore:

```text
aether project lock packages/util/aether.project.json --write
aether project lock packages/app/aether.project.json --write
aether workspace lock aether.workspace.json --write
```

For a locked workspace, `aether workspace build ...` invokes normal workspace
verification before elaborating and before the CLI writes its artifact. An
unlocked workspace retains the existing M22 build behavior; users opt into the
stronger pre-build whole-workspace integrity check by committing a lock.

## 5. Verification algorithm

### 5.1 Structural validation

The parser rejects unknown fields, missing/duplicate/extra lock package names,
path mismatch, empty project identity fields, and non-lowercase SHA-256 text.
These are `AE-WORKSPACE-005` lock-integrity errors.

### 5.2 Locked package verification

For each package in deterministic topological order:

1. Canonicalize the workspace package root and ensure it stays below the
   canonical workspace root.
2. Canonicalize `aether.project.json` and ensure the manifest itself remains
   below the canonical package root; a manifest symlink cannot escape.
3. Read raw manifest bytes, calculate SHA-256, parse the project document, and
   compare the lock's digest, name, and version to the observed values.
4. Require a complete nested project lock and run ordinary `verify_project`,
   which path-jails and checks every listed unit digest before seed compilation
   and artifact verification.

Verification continues across packages to report all failures. If a locked
package has a PKG-001 mismatch, the workspace error envelope uses
`AE-WORKSPACE-005`; ordinary nested-project failures retain their existing
`AE-WORKSPACE-004` envelope.

## 6. Invariants

| ID | Invariant |
| --- | --- |
| PKG-INV-001 | Only local M18/M22 package paths participate; no network, registry, URL, or version solving is introduced. |
| PKG-INV-002 | A workspace lock is complete: exactly one entry for every declared package, with exact package name and path. |
| PKG-INV-003 | A locked package binds its parsed project name/version and raw `aether.project.json` bytes. |
| PKG-INV-004 | A locked workspace requires every nested project to carry the existing complete unit lock, so source bytes are also checked. |
| PKG-INV-005 | Package and manifest paths are canonicalized and confined before any locked data is trusted. |
| PKG-INV-006 | Lock refresh is explicit: normal verify/build do not rewrite manifests; only `--write` writes. |
| PKG-INV-007 | A locked workspace build verifies before artifact output; a stale lock cannot be bypassed through `workspace build`. |
| PKG-INV-008 | Aether source, AETH v11, seed emission, VM semantics, and guest capability authority remain unchanged. |

## 7. Rejected alternatives

| Alternative | Decision | Reason |
| --- | --- | --- |
| Network registry or remote cache | Rejected | F-REGISTRY is a separate human law fork. |
| Version ranges or resolver | Rejected | Local workspaces have named paths, not a solver; this would imply package-management semantics not yet designed. |
| Lock only project manifest bytes | Rejected | An unlocked project manifest would leave source-unit identity mutable. |
| Auto-edit all package manifests from `workspace lock` | Rejected | One command would make multi-file mutations without a transaction. Explicit per-project lock refresh is auditable and fail-closed. |
| New workspace schema version | Rejected | The optional field is an additive v1 toolchain extension; old strict tools fail rather than ignore it. |

## 8. Stop conditions

- Any remote fetch, registry lookup, URL, ambient path search, or unpinned
  package source.
- A lock that can pass after its project manifest or a locked unit changes.
- A path or manifest symlink escaping its declared root.
- A `workspace build` that writes an artifact after a declared lock fails.
- Any source, AETH, seed, verifier, VM, or guest-capability change presented as
  necessary for this tooling-only increment.

---

*End of PKG-001 design.*
