# Delivery Report: PKG-001 Offline Workspace Locks

**Status:** Delivered as package **0.35.0** working-tree evidence; no commit or
publish action was performed
**Date:** 2026-08-08
**Contract:** [AETHER_0.35.md](AETHER_0.35.md)
**Design:** [DESIGN-PKG-001-OFFLINE-WORKSPACE-LOCKS.md](DESIGN-PKG-001-OFFLINE-WORKSPACE-LOCKS.md)
**Decision:** [ADR-041](ADR-041-pkg-001-offline-workspace-locks.md)
**Validation:** [PKG-001 validation matrix](PKG-001-VALIDATION-MATRIX.md)
**Rule IDs:** `CONST-CONTRACT-001`, `CONST-COMPLETE-001`, `CONST-DEP-001`,
`TEST-BEHAVIOR-001`, `ENG-WARN-001`, `SEC-INPUT-001`, `DOC-SYNC-001`,
`REV-PACK-001`, `SOP-PHASE-001`, `SOP-GATE-001`

---

## 1. Delivered scope

PKG-001 closes the local workspace-integrity gap deliberately left by M18 while
remaining inside the M18/M22 offline-package boundary.

1. `aether.project/v1` lock refresh is available through
   `aether project lock <project-file> [--write]`.
2. `aether.workspace/v1` accepts optional complete `lock.packages` entries that
   bind each declared package name/path, parsed project name/version, and raw
   `aether.project.json` SHA-256.
3. `aether workspace lock <workspace-file> [--write]` deterministically derives
   those entries after verifying every already locked nested project.
4. A locked workspace verifies the full nested project locks, and a locked
   `workspace build` completes that verification before writing an artifact.
5. Package roots and project manifests are canonicalized and confined; a
   project-manifest symlink cannot redirect a package outside its declared root.
6. `examples/workspace/` is a checked-in locked M22 fixture that builds the
   `app → util` graph and exits with 42.

Without `--write`, both refresh commands emit canonical JSON only. `--write`
replaces only the explicitly named project or workspace manifest. Workspace
refresh intentionally does not make recursive multi-file writes.

## 2. Preserved boundaries

This package adds no Aether source form, no AETH revision or bytecode rule, no
seed behavior, no VM semantic change, no guest capability, no registry, no URL
dependency, no network fetch, no version solver, no package archive, and no
native backend. Existing unlocked workspaces retain M18/M22 behavior.

The locked-build full-workspace preflight is an integrity guarantee, not an
incremental-build or general performance claim.

## 3. Intended-behavior and hostile-input evidence

The new core and CLI tests cover:

- project lock refresh after ordinary project verification, including repair of a
  structurally valid stale project lock;
- deterministic util-before-app workspace lock generation and canonical
  serialize/reparse verification;
- complete/malformed/path-mismatched lock-entry rejection;
- project-manifest raw-byte drift and parsed identity drift;
- missing nested project locks and changed locked source-unit bytes;
- locked build failure before any output artifact exists;
- explicit-only `--write` parsing and no-write CLI previews;
- post-canonicalization project-manifest containment, tested without depending
  on Windows symlink-creation privileges; and
- the checked-in locked workspace verify/build/run flow.

Lock structure and observed workspace-identity failures use
`AE-WORKSPACE-005`. Nested project verification failures remain in the
`AE-WORKSPACE-004` envelope and retain the nested project evidence.

## 4. Gate evidence

All commands below ran from `C:\WPAI\Software\XLang` on 2026-08-08 and passed:

| Command | Result |
| --- | --- |
| `cargo fmt --all -- --check` | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass; zero warnings accepted |
| `cargo test -p aether-core workspace` | Pass; 5 selected workspace tests, including the containment guard |
| `cargo test -p aether-cli lock` | Pass; 2 selected lock-command tests |
| `cargo test -p aether-cli` | Pass; 19 tests |
| `cargo test --workspace` | Pass; 19 CLI + 89 core + 38 integration/self-host tests; all green |
| `cargo test --release --workspace` | Pass; same release corpus, all green |
| `pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode full -SkipPack` | Pass; formatting, clippy, core/CLI tests, 29 seed≡bootstrap examples, host-pilot exit 48, project verification/build, and checked-in seed/forge SHA-256 identity |

Manual CLI evidence also passed:

```powershell
cargo run -p aether-cli -- workspace verify .\examples\workspace\aether.workspace.json
# Aether 0.35.0 workspace workspace_demo@0.1.0 verified 2 package(s)

cargo run -p aether-cli -- workspace lock .\examples\workspace\aether.workspace.json
# Canonical lock JSON printed; no --write mutation requested

cargo run -p aether-cli -- workspace build .\examples\workspace\aether.workspace.json --package app --output .\target\pkg001-workspace-app.aeth
.\target\debug\aether.exe run .\target\pkg001-workspace-app.aeth
# Aether 0.35.0 exited with 42
```

The full gate intentionally used `-SkipPack`: `tools/verify-pack.ps1` is absent
from the current checkout, so no package-preview verification is claimed by this
delivery. That omission does not alter the PKG-001 source, lock, CLI, or
workspace gate evidence above.

## 5. Documentation and traceability

The package contract, manifest, claim register, architecture, seed boundary,
near-term roadmap, historic-mainstream pointer, human backlog, README, examples,
validation matrix, ADR, and full progress report were synchronized to package
0.35. The authoritative starting points are [MANIFEST.md](../../MANIFEST.md),
[CORE_CLAIMS.md](../Current%20state/CORE_CLAIMS.md), and [AETHER_0.35.md](AETHER_0.35.md).

This repository already contained unrelated working-tree additions, removals,
and historical-delivery files. This report records PKG-001 only; it neither
stages nor commits unrelated material.

## 6. Constitution self-audit

| Rule | Result | Evidence |
| --- | --- | --- |
| `CONST-CONTRACT-001` | Pass | Scope freezes at offline local locks; F-REGISTRY remains blocked. |
| `CONST-COMPLETE-001` / `CONST-DEP-001` | Pass | Parser, data model, refresh, verification, CLI, fixtures, diagnostics, and documentation ship together. |
| `TEST-BEHAVIOR-001` | Pass | Positive, stale-data, malformed-input, confinement, explicit-write, and no-output-on-failure tests are present. |
| `SEC-INPUT-001` | Pass | Strict schemas, digest checks, path confinement, nested lock requirement, and explicit write authority fail closed. |
| `ENG-WARN-001` | Pass | Format and `clippy -D warnings` gates passed. |
| `DOC-SYNC-001` | Pass | Current contract, ADR, matrix, claims, roadmap, progress report, and delivery record agree on 0.35 scope. |
| `REV-PACK-001` | Pass with stated packaging caveat | Reproducible source/test evidence is recorded; preview-pack verification was not run because its verifier is absent. |
| `SOP-PHASE-001` / `SOP-GATE-001` | Pass | Design → ADR → matrix → implementation → hostile tests → full gates → delivery record completed. |

---

*End of PKG-001 delivery report.*
