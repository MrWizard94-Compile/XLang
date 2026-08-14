# PKG-001 offline workspace locks — validation matrix

**Status:** Implemented — package **0.35.0**
**Date:** 2026-08-08
**Design:** [DESIGN-PKG-001-OFFLINE-WORKSPACE-LOCKS.md](DESIGN-PKG-001-OFFLINE-WORKSPACE-LOCKS.md)
**ADR:** [ADR-041](ADR-041-pkg-001-offline-workspace-locks.md)

| ID | Case | Expected evidence |
| --- | --- | --- |
| PKG-P1 | `project lock` derives every unit's raw SHA-256 after ordinary project verification | Project refresh test produces a complete, parseable lock that `verify_project` accepts. |
| PKG-P2 | `workspace lock` derives deterministic dependency-ordered package entries | Core test refreshes a workspace and checks util before app with exact project identities. |
| PKG-P3 | A checked-in locked M22 workspace verifies | `examples/workspace` contains project and workspace locks; core and CLI tests verify it. |
| PKG-P4 | A checked-in locked M22 workspace builds and runs | `workspace build --package app` verifies first, emits verified AETH, and exits 42. |
| PKG-P5 | Existing unlocked workspace remains valid | Existing M18 happy-path test continues to verify without a workspace lock. |
| PKG-P6 | Stale but structurally valid workspace locks are refreshable | Refresh ignores previous digest values and derives current validated values. |
| PKG-N1 | Missing, duplicate, unknown, path-mismatched, or malformed workspace lock entries fail closed | Parser returns `AE-WORKSPACE-005`. |
| PKG-N2 | Changed package project-manifest bytes, name, or version fail the declared workspace lock | `verify_workspace` returns `AE-WORKSPACE-005`. |
| PKG-N3 | Missing nested project lock blocks workspace lock generation and verification | `AE-WORKSPACE-005`; source bytes are never treated as pinned without their project lock. |
| PKG-N4 | Changed locked source-unit bytes fail nested project verification | Existing project lock check is surfaced through workspace verification before a locked build. |
| PKG-N5 | A locked build does not write an artifact after integrity failure | CLI/core test asserts stale locked workspace build errors before output exists. |
| PKG-N6 | A canonical project-manifest target outside its package root is rejected | The post-canonicalization containment guard fails with `AE-WORKSPACE-002`; it is unit-tested independently of Windows symlink privilege. |
| PKG-H1 | Lock refresh has no implicit write authority | Without `--write`, CLI emits JSON only; `--write` is required for replacement. |
| PKG-H2 | No language, AETH, seed, VM, host capability, registry, or network behavior changes | Source/artifact compatibility and existing workspace/seed suites remain green. |

## Required gates

- [x] Design and ADR-041 accepted under existing M18/M22 authority
- [x] Core and CLI intended-behavior/negative tests
- [x] Checked-in locked workspace fixture
- [x] `cargo fmt --all -- --check`
- [x] `cargo clippy --workspace --all-targets -- -D warnings`
- [x] `cargo test --workspace`
- [x] `cargo test --release --workspace`
- [x] `pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode full -SkipPack`
- [x] Manifest, contract, claims, roadmap, progress report, and delivery report synchronized

*End of PKG-001-VALIDATION-MATRIX.md*
