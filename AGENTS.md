# AGENTS — Project Entry Point (XLang / Aether)

**Status:** Level 4 pointer — legacy local constitutions replaced  
**Pack path (relative):** `../../AGENTS Constitution/`  
**Pack version:** see `../../AGENTS Constitution/VERSION`

This file does **not** duplicate the constitution. Binding quality law lives in the pack.

---

## Binding pack (Level 1–3)

| Role | Path |
|------|------|
| **Constitution (SOUL)** | [../../AGENTS Constitution/AGENTS.md](../../AGENTS Constitution/AGENTS.md) |
| **Process (SOP)** | [../../AGENTS Constitution/SOP.md](../../AGENTS Constitution/SOP.md) |
| **Identity / portability** | [../../AGENTS Constitution/PACK.md](../../AGENTS Constitution/PACK.md) |
| **Lock** | [../../AGENTS Constitution/LOCK.md](../../AGENTS Constitution/LOCK.md) |
| **Adopt** | [../../AGENTS Constitution/ADOPT.md](../../AGENTS Constitution/ADOPT.md) |
| **Integrity** | [../../AGENTS Constitution/INTEGRITY.md](../../AGENTS Constitution/INTEGRITY.md) |
| **Rule registry** | [../../AGENTS Constitution/RULE-REGISTRY.md](../../AGENTS Constitution/RULE-REGISTRY.md) |

### Always load

1. This file (project entry)
2. Pack `AGENTS.md`
3. Pack `SOP.md`
4. Pack `constitution/03-DEFINITION-OF-DONE.md`
5. Pack `standards/ENGINEERING.md`
6. Pack `standards/TESTING.md`
7. Pack `standards/DOCUMENTATION.md`

Then load pack modules per the applicability matrix in pack `AGENTS.md`.

### Verify pack

```powershell
pwsh -File "../../AGENTS Constitution/tools/verify-pack.ps1"
```

Must exit 0 after pack install/move/update (`GOV-INT-001`).

---

## Project-local law (Level 4)

### Mission

This repository hosts **Aether**: a local-first language and CLI toolchain.
Aether parses only Aether source, emits **AETH** bytecode, verifies every
artifact, and runs it in the Aether VM. It never transpiles source to Rust, C,
JavaScript, LLVM, or another language.

**Stage status:** Stage 7 (bounded arenas and Copy-element buffers), M4
(the bounded `Error[Whole]` effect), M5 (literal deterministic `comptime bind`),
M6 (explicit layout shapes and dual-layout tables), M7 (structured nurseries),
and M8 (capability-closed pure host ABI pilot) are implemented on the
seed-hosted product compile path in Aether 0.11 / AETH v11. Toolchain package
0.12 added offline project verify and format (M9); package **0.13** adds multi-unit nested project integrity (M10); package **0.18** completes modules (M11), fine-grained edits (M12), and bounded offline LSP M13a+M13b (`aether lsp [--project …]`); package **0.19** adds grant-backed host I/O (M14); package **0.20** adds comptime Whole name chaining (M15); package **0.21** allows terminal handle over live resources (M16); package **0.22** adds offline `aether test` (M17); package **0.23** adds offline multi-package workspaces (M18); package **0.24** adds stdlib layer 0 (M20) and cross-package imports (M22); package **0.25** adds product-path `release` (M19a; seed≡bootstrap proven); package **0.26** adds nursery×resource Policy A (M19b); package **0.27** expands pure stdlib layer 1 (M20b); package **0.28** adds project `role: test` / `aether project test` (M17b); package **0.29** adds optional grants on tests (M17c); package **0.30** adds structured test reports (M17d). Policy B cancel-destroy and FFI remain design-blocked (see `docs/HUMAN-AUTHORIZE-FFI.md`).
Default CLI compilation uses the Aether-written seed compiler. Rust
bootstrap remains for seed rebuild (`compile --bootstrap`), `check` AST, and
proof dual-compare. Seed Profile self-host, all shipped examples, the complete
prior canonical surface (including records), the documented M2 arena/buffer
corpus, the M4 error-effect corpus, the M5 comptime corpus, the M6 layout
corpus, the M7 nursery corpus, the M8 host-pilot corpus, the M19a release
corpus, and the M19b nursery-resource corpus match bootstrap byte-for-byte.
Full diagnostic parity is not claimed for the seed.

### Product docs (Level 4)

| Role | Path |
|------|------|
| Overview | [README.md](README.md) |
| Contract / release gate | [MANIFEST.md](MANIFEST.md) |
| Architecture | [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) |
| Language (0.11 surface / 0.13 toolchain current) | [docs/AETHER_0.13.md](docs/AETHER_0.13.md), [docs/AETHER_0.12.md](docs/AETHER_0.12.md), [docs/AETHER_0.11.md](docs/AETHER_0.11.md), [docs/AETHER_0.10.md](docs/AETHER_0.10.md), [docs/AETHER_0.9.md](docs/AETHER_0.9.md), [docs/AETHER_0.8.md](docs/AETHER_0.8.md), [docs/AETHER_0.7.md](docs/AETHER_0.7.md), [docs/AETHER_0.6.md](docs/AETHER_0.6.md), [docs/AETHER_0.5.md](docs/AETHER_0.5.md), [docs/AETHER_0.4.md](docs/AETHER_0.4.md) |
| Record decision | [docs/ADR-001-records-and-aeth-v5.md](docs/ADR-001-records-and-aeth-v5.md) |
| AI-first design foundation | [docs/NORTH_STAR.md](docs/NORTH_STAR.md), [docs/CORE_CLAIMS.md](docs/CORE_CLAIMS.md), [docs/ADR-002-ai-first-design-foundation.md](docs/ADR-002-ai-first-design-foundation.md) |
| M1/M2 resource decisions | [docs/DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md](docs/DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md), [docs/ADR-003-value-resource-semantics.md](docs/ADR-003-value-resource-semantics.md), [docs/ADR-004-aeth-v6-bounded-resources.md](docs/ADR-004-aeth-v6-bounded-resources.md), [docs/M1-VALIDATION-MATRIX.md](docs/M1-VALIDATION-MATRIX.md) |
| M3–M9 authoring, effects, layout, host ABI, project tooling | [docs/AETHER_AUTHORING_PROTOCOL_v6.md](docs/AETHER_AUTHORING_PROTOCOL_v6.md), [docs/ADR-007-m4-typed-error-effect.md](docs/ADR-007-m4-typed-error-effect.md), [docs/ADR-008-m5-deterministic-comptime.md](docs/ADR-008-m5-deterministic-comptime.md), [docs/ADR-009-m6-explicit-layout-shapes.md](docs/ADR-009-m6-explicit-layout-shapes.md), [docs/ADR-010-m7-structured-concurrency.md](docs/ADR-010-m7-structured-concurrency.md), [docs/ADR-011-m8-host-abi-pilot.md](docs/ADR-011-m8-host-abi-pilot.md), [docs/ADR-012-m9-project-tooling.md](docs/ADR-012-m9-project-tooling.md), [docs/DESIGN-M8-HOST-ABI-PILOT.md](docs/DESIGN-M8-HOST-ABI-PILOT.md), [docs/DESIGN-M9-PROJECT-TOOLING.md](docs/DESIGN-M9-PROJECT-TOOLING.md), [docs/M4-VALIDATION-MATRIX.md](docs/M4-VALIDATION-MATRIX.md), [docs/M5-VALIDATION-MATRIX.md](docs/M5-VALIDATION-MATRIX.md), [docs/M6-VALIDATION-MATRIX.md](docs/M6-VALIDATION-MATRIX.md), [docs/M7-VALIDATION-MATRIX.md](docs/M7-VALIDATION-MATRIX.md), [docs/M8-VALIDATION-MATRIX.md](docs/M8-VALIDATION-MATRIX.md), [docs/M9-VALIDATION-MATRIX.md](docs/M9-VALIDATION-MATRIX.md) |
| Research and roadmap | [docs/research/](docs/research/), [docs/ROADMAP.md](docs/ROADMAP.md), [docs/ROADMAP-MAINSTREAM-MATURITY.md](docs/ROADMAP-MAINSTREAM-MATURITY.md) |
| Seed Profile | [docs/SEED_PROFILE.md](docs/SEED_PROFILE.md) |
| Forge ABI | [docs/FORGE_CONTRACT.md](docs/FORGE_CONTRACT.md) |
| Studio retirement decision | [docs/ADR-006-retire-aether-studio.md](docs/ADR-006-retire-aether-studio.md) |
| Migration audit | [AUDIT_REPORT.md](AUDIT_REPORT.md) |
| Full project progress report | [docs/PROGRESS_REPORT-FULL-PROJECT.md](docs/PROGRESS_REPORT-FULL-PROJECT.md) |
| Completion readiness audit | [docs/AUDIT_REPORT-2026-08-04-COMPLETION-READINESS.md](docs/AUDIT_REPORT-2026-08-04-COMPLETION-READINESS.md) |
| Technical preview threat model | [docs/THREAT_MODEL-TECHNICAL-PREVIEW.md](docs/THREAT_MODEL-TECHNICAL-PREVIEW.md) |
| Capable host threat model v2 | [docs/THREAT_MODEL-v2-CAPABLE-HOST.md](docs/THREAT_MODEL-v2-CAPABLE-HOST.md) |
| M14 host I/O (implemented 0.19) | [docs/AETHER_0.19.md](docs/AETHER_0.19.md), [docs/DESIGN-M14-HOST-IO-CAPABILITIES.md](docs/DESIGN-M14-HOST-IO-CAPABILITIES.md), [docs/ADR-018-m14-host-io-capabilities.md](docs/ADR-018-m14-host-io-capabilities.md), [docs/M14-VALIDATION-MATRIX.md](docs/M14-VALIDATION-MATRIX.md) |
| M15 comptime expansion (implemented 0.20) | [docs/AETHER_0.20.md](docs/AETHER_0.20.md), [docs/DESIGN-M15-COMPTIME-EXPANSION.md](docs/DESIGN-M15-COMPTIME-EXPANSION.md), [docs/ADR-019-m15-comptime-expansion.md](docs/ADR-019-m15-comptime-expansion.md), [docs/M15-VALIDATION-MATRIX.md](docs/M15-VALIDATION-MATRIX.md) |
| M16 resource↔handle (implemented 0.21) | [docs/AETHER_0.21.md](docs/AETHER_0.21.md), [docs/DESIGN-M16-RESOURCE-EFFECT.md](docs/DESIGN-M16-RESOURCE-EFFECT.md), [docs/ADR-020-m16-resource-effect.md](docs/ADR-020-m16-resource-effect.md), [docs/M16-VALIDATION-MATRIX.md](docs/M16-VALIDATION-MATRIX.md) |
| M17 offline test runner (implemented 0.22) | [docs/AETHER_0.22.md](docs/AETHER_0.22.md), [docs/DESIGN-M17-OFFLINE-TEST-RUNNER.md](docs/DESIGN-M17-OFFLINE-TEST-RUNNER.md), [docs/ADR-021-m17-offline-test-runner.md](docs/ADR-021-m17-offline-test-runner.md), [docs/M17-VALIDATION-MATRIX.md](docs/M17-VALIDATION-MATRIX.md) |
| M18 offline workspace (implemented 0.23) | [docs/AETHER_0.23.md](docs/AETHER_0.23.md), [docs/DESIGN-M18-OFFLINE-WORKSPACE.md](docs/DESIGN-M18-OFFLINE-WORKSPACE.md), [docs/ADR-022-m18-offline-workspace.md](docs/ADR-022-m18-offline-workspace.md), [docs/M18-VALIDATION-MATRIX.md](docs/M18-VALIDATION-MATRIX.md) |
| M19–M22 portfolio continuation | [docs/AETHER_0.24.md](docs/AETHER_0.24.md); M19 [ADR-023](docs/ADR-023-m19-deeper-resource-effect.md); M19a release [docs/AETHER_0.25.md](docs/AETHER_0.25.md) / [ADR-027](docs/ADR-027-m19a-explicit-release.md); M19b nursery×resource [docs/AETHER_0.26.md](docs/AETHER_0.26.md) / [ADR-028](docs/ADR-028-m19b-nursery-resource.md); M19c Policy B design-only [docs/ADR-032-m19c-policy-b-design.md](docs/ADR-032-m19c-policy-b-design.md); M19d spawn precondition design-only [docs/ADR-034-m19d-resourceful-spawn-precondition.md](docs/ADR-034-m19d-resourceful-spawn-precondition.md); M20 [ADR-024](docs/ADR-024-m20-stdlib-layer0.md); M20b stdlib layer 1 [docs/AETHER_0.27.md](docs/AETHER_0.27.md) / [ADR-029](docs/ADR-029-m20b-stdlib-layer1.md); M17b project test [docs/AETHER_0.28.md](docs/AETHER_0.28.md) / [ADR-030](docs/ADR-030-m17b-project-test.md); M17c grants-in-tests [docs/AETHER_0.29.md](docs/AETHER_0.29.md) / [ADR-031](docs/ADR-031-m17c-grants-in-tests.md); M17d reports [docs/AETHER_0.30.md](docs/AETHER_0.30.md) / [ADR-033](docs/ADR-033-m17d-structured-test-reports.md); M21 [ADR-025](docs/ADR-025-m21-foreign-abi-pilot.md) + [threat v3](docs/THREAT_MODEL-v3-FOREIGN-ABI.md) + [HUMAN-AUTHORIZE-FFI.md](docs/HUMAN-AUTHORIZE-FFI.md) (**impl blocked**); M22 [ADR-026](docs/ADR-026-m22-cross-package-import.md) |
| Technical preview notes | [docs/RELEASE_NOTES-TECHNICAL-PREVIEW.md](docs/RELEASE_NOTES-TECHNICAL-PREVIEW.md) |
| M10 multi-unit projects | [docs/AETHER_0.13.md](docs/AETHER_0.13.md), [docs/DESIGN-M10-MULTI-UNIT-PROJECTS.md](docs/DESIGN-M10-MULTI-UNIT-PROJECTS.md), [docs/ADR-013-m10-multi-unit-projects.md](docs/ADR-013-m10-multi-unit-projects.md), [docs/M10-VALIDATION-MATRIX.md](docs/M10-VALIDATION-MATRIX.md) |
| Post-M10 track portfolio | [docs/ADR-014-post-m10-track-portfolio.md](docs/ADR-014-post-m10-track-portfolio.md), [docs/DESIGN-POST-M10-TRACK-PORTFOLIO.md](docs/DESIGN-POST-M10-TRACK-PORTFOLIO.md) |
| M11 language modules | [docs/AETHER_0.12-MODULES.md](docs/AETHER_0.12-MODULES.md), [docs/DESIGN-M11-LANGUAGE-MODULES.md](docs/DESIGN-M11-LANGUAGE-MODULES.md), [docs/ADR-015-m11-language-modules.md](docs/ADR-015-m11-language-modules.md), [docs/M11-VALIDATION-MATRIX.md](docs/M11-VALIDATION-MATRIX.md) |
| M12 fine-grained edits | [docs/AETHER_AUTHORING_PROTOCOL_v7.md](docs/AETHER_AUTHORING_PROTOCOL_v7.md), [docs/DESIGN-M12-FINE-GRAINED-EDITS.md](docs/DESIGN-M12-FINE-GRAINED-EDITS.md), [docs/ADR-016-m12-fine-grained-edits.md](docs/ADR-016-m12-fine-grained-edits.md), [docs/M12-VALIDATION-MATRIX.md](docs/M12-VALIDATION-MATRIX.md) |
| M13 bounded LSP | [docs/DESIGN-M13-BOUNDED-LSP.md](docs/DESIGN-M13-BOUNDED-LSP.md), [docs/ADR-017-m13-bounded-lsp.md](docs/ADR-017-m13-bounded-lsp.md), [docs/M13-VALIDATION-MATRIX.md](docs/M13-VALIDATION-MATRIX.md) |
| Mainstream maturity roadmap | [docs/ROADMAP-MAINSTREAM-MATURITY.md](docs/ROADMAP-MAINSTREAM-MATURITY.md) |
| Legacy intake | [docs/LEGACY.md](docs/LEGACY.md) |

### Stack pins

| Layer | Pin |
|-------|-----|
| Language / package | Rust workspace, edition 2021, `rust-version = "1.88"` |
| Core crate | `aether-core` at `crates/xlang-core` (package version 0.30.0) |
| CLI binary | `aether` via `apps/xlang-cli` (package `aether-cli` 0.30.0) |
| Artifact format | AETH **v4–v10** compatibility input + deterministic **v11** output with shape table, dual-layout tables, structured nurseries, effect metadata, host function kind, `HOST_CALL`, `COMPTIME_WHOLE`, and `RELEASE` (66; M19a/M19b seed≡bootstrap proven) (earlier/unknown versions rejected) |
| Product compile | Seed-hosted (`compile_with_seed` / embedded `SEED_COMPILER_ARTIFACT`) for the documented seed surface, including M19a `release`, M19b nursery×resource Policy A, multi-module stdlib layer 1, project `role: test`, optional test grants, and optional structured test reports |
| Bootstrap | `compile --bootstrap` / `check` AST / rebuild `seed/*.aeth` |
| Seed compiler | `seed/aether_seed.ae` + checked-in `seed/aether_seed.aeth` |

### Invariants (may tighten pack; never weaken CONST-\*)

1. **Seed-hosted product compile** — CLI default compile uses the Aether-written seed; bootstrap is not the product compiler path.
2. **Verify before run / write** — VM and forge only accept verified supported AETH v4, v5, v6, v7, v8, v9, v10, or v11.
3. **No ambient host capability leak** — default run installs only pure fixtures; grant-backed I/O (M14) requires explicit operator `--grant-*` roots/names and path jail; no shell/network; forge host owns I/O after verification.
4. **Honest self-host claims** — Seed Profile, shipped seed-path examples, the complete documented prior canonical surface (including immutable records), and the documented M2 arena/buffer, M4 error-effect, M5 comptime, M6 layout, M7 nursery, M8 host-pilot, M19a release, and M19b nursery-resource corpora match bootstrap in tests; do not claim full diagnostic parity or parity for future language extensions without proof.
5. **CLI authority boundary** — the CLI reads only caller-selected local files and writes source or artifacts only to an explicit output path after the required validation/seed-compile path; it has no model or network integration.
6. **Legacy is reference only** — `legacy/` is never a production build input.
7. **Zero-warning gate** — workspace Clippy `all = "deny"`; `unsafe_code = "forbid"`.

### Commands (quality gate)

From repository root `C:\WPAI\Software\XLang` (or equivalent checkout):

```powershell
# Preferred offline gate (TP-1 / day-to-day)
pwsh -File .\tools\aether-gate.ps1 -Mode quick

# Technical preview / release-blocking (includes seed forge identity)
pwsh -File .\tools\aether-gate.ps1 -Mode full

# Manual pack law (also invoked by the gate when pack is found)
pwsh -File "../../AGENTS Constitution/tools/verify-pack.ps1"
# Fallback if pack is vendored in-tree (untracked dump; not a product commit):
# pwsh -File ".\AGENTS Constitution\tools\verify-pack.ps1"
```

Release / technical preview additionally requires `cargo build --release -p aether-cli`,
SHA-256SUMS, forge contract verification, binary inspection/launch, and delivery
report — see [MANIFEST.md](MANIFEST.md).

### Overrides

Use pack `templates/PROJECT-OVERRIDE.template.md` only with named human approval.
Do not weaken `CONST-*`, `ENG-WARN-001`, `TEST-BEHAVIOR-001`, or `SEC-INPUT-001`
without a documented override.

### Evolution

| Artifact | Method |
|----------|--------|
| Universal law | Complete file replace in `AGENTS Constitution` pack + VERSION bump + verify-pack |
| This pointer | Update when relative pack path changes |
| Product docs | Keep in this repo (`README`, `docs/`, `MANIFEST.md`) |
| `SOUL.md` | Superseded stub only — do not restore monolith law here |

Product docs stay in this repository. Do not stuff Aether PRDs into the universal pack.

### Specialized Grok subagents (project kit)

Definitions live under [`.grok/`](.grok/) (see [`.grok/README.md`](.grok/README.md)).
**All project subagents are bound by AGENTS Constitution** — see
[`.grok/CONSTITUTION-BINDING.md`](.grok/CONSTITUTION-BINDING.md). They must
always-load the pack set above, enforce non-negotiable Rule IDs, and pass
`CONST-GATE-001` before claiming Done. Specialization may tighten, never weaken,
pack law.

Spawn with `subagent_type` matching the agent `name`. Manage via `/config-agents`.

| Agent | Role |
|-------|------|
| `aether-explorer` | Read-only domain map of core, seed, CLI, docs |
| `aether-core-engineer` | Rust bootstrap compiler / VM / CLI |
| `aether-seed-engineer` | Aether-written seed + self-host proof |
| `aether-spec-writer` | Specs and honest stage/self-host claims |
| `aether-gate-runner` | Run gates; map results to Section 0 Rule IDs |
| `aether-reviewer` | Read-only review vs pack Rule IDs + Aether invariants |

Personas: `agents-constitution`, `aether-honest-claims`, `aether-zero-warning`,
`aether-seed-discipline`.
