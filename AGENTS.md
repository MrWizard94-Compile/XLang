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
0.12 added offline project verify and format (M9); package **0.13** adds multi-unit nested project integrity (M10); package **0.18** completes modules (M11), fine-grained edits (M12), and bounded offline LSP M13a+M13b (`aether lsp [--project …]`); package **0.19** adds grant-backed host I/O (M14); package **0.20** adds comptime Whole name chaining (M15); package **0.21** allows terminal handle over live resources (M16); package **0.22** adds offline `aether test` (M17); package **0.23** adds offline multi-package workspaces (M18); package **0.24** adds stdlib layer 0 (M20) and cross-package imports (M22); package **0.25** adds product-path `release` (M19a; seed≡bootstrap proven); package **0.26** adds nursery×resource Policy A (M19b); package **0.27** expands pure stdlib layer 1 (M20b); package **0.28** adds project `role: test` / `aether project test` (M17b); package **0.29** adds optional grants on tests (M17c); package **0.30** adds structured test reports (M17d); package **0.31** adds M21 foreign weave pilot (Whole-only; `--grant-lib`; human residual-risk accepted; seed≡bootstrap proven for foreign-pilot); package **0.32** adds M19d multi-weave arenas and Policy A+ resourceful total spawn callees with cooperative Policy B bounds (ADR-035/036); package **0.33** adds M23 pure comptime weave calls; BARP Phase 1 (package **0.36** era) makes the seed interpret raw M23 D2a call source natively (no materialization bridge); package **0.34** adds RTP-001 ASCII Text fast-path (no AETH change); package **0.35** adds PKG-001 offline workspace locks; package **0.36** adds M19e active-frame cancellation (`task weave` / `checkpoint`, AETH **v12**).
Default CLI compilation uses the Aether-written seed compiler. Rust
bootstrap remains for seed rebuild (`compile --bootstrap`), default `check`
AST / format / structure / LSP, and proof dual-compare. Product acceptance
Default CLI check/format/structure, product compile, and seed rebuild are seed path
(ADR-064/067); bootstrap is recovery/oracle (`--bootstrap`, dual-compare, residual
structural AST outside product weave-replace ADR-065). LSP hover/definition are
product-surface (ADR-066). Multi-module is host elaborate + seed emit (ADR-056). Seed Profile self-host, all shipped seed-path examples,
the complete prior canonical surface (including records), the documented M2–M8
corpora, the M19a release corpus, the M19b nursery-resource corpus, the M19d
spawn-arena corpus, the M21 foreign-pilot corpus, and the M23/M19e documented
corpora match bootstrap byte-for-byte where claimed in tests. Full diagnostic
parity is not claimed for the seed. M23 pure calls are seed-native under the
D2a body subset (`seed_interprets_m23_comptime_calls_natively`; ADR-043 Phase 1).

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
| Current package contract | [docs/AETHER_0.36.md](docs/AETHER_0.36.md) — M19e active-frame cancel; also [0.33](docs/AETHER_0.33.md) M23, [0.34](docs/AETHER_0.34.md) RTP-001, [0.35](docs/AETHER_0.35.md) PKG-001 |
| M19–M23 portfolio | M19a–d [docs/AETHER_0.25.md](docs/AETHER_0.25.md)–[0.32](docs/AETHER_0.32.md); M19e [ADR-042](docs/ADR-042-m19e-active-frame-cancel.md); M23 [ADR-039](docs/ADR-039-m23-comptime-pure-calls.md); full status [docs/PROGRESS_REPORT-FULL-PROJECT.md](docs/PROGRESS_REPORT-FULL-PROJECT.md) |
| Technical preview 0.36 | [docs/DELIVERY_REPORT-2026-08-08-TECHNICAL-PREVIEW-0.36.md](docs/DELIVERY_REPORT-2026-08-08-TECHNICAL-PREVIEW-0.36.md), [docs/THREAT_MODEL-0.36-TECHNICAL-PREVIEW.md](docs/THREAT_MODEL-0.36-TECHNICAL-PREVIEW.md) |
| Technical preview notes | [docs/RELEASE_NOTES-TECHNICAL-PREVIEW.md](docs/RELEASE_NOTES-TECHNICAL-PREVIEW.md) |
| M10 multi-unit projects | [docs/AETHER_0.13.md](docs/AETHER_0.13.md), [docs/DESIGN-M10-MULTI-UNIT-PROJECTS.md](docs/DESIGN-M10-MULTI-UNIT-PROJECTS.md), [docs/ADR-013-m10-multi-unit-projects.md](docs/ADR-013-m10-multi-unit-projects.md), [docs/M10-VALIDATION-MATRIX.md](docs/M10-VALIDATION-MATRIX.md) |
| Post-M10 track portfolio | [docs/ADR-014-post-m10-track-portfolio.md](docs/ADR-014-post-m10-track-portfolio.md), [docs/DESIGN-POST-M10-TRACK-PORTFOLIO.md](docs/DESIGN-POST-M10-TRACK-PORTFOLIO.md) |
| M11 language modules | [docs/AETHER_0.12-MODULES.md](docs/AETHER_0.12-MODULES.md), [docs/DESIGN-M11-LANGUAGE-MODULES.md](docs/DESIGN-M11-LANGUAGE-MODULES.md), [docs/ADR-015-m11-language-modules.md](docs/ADR-015-m11-language-modules.md), [docs/M11-VALIDATION-MATRIX.md](docs/M11-VALIDATION-MATRIX.md) |
| M12 fine-grained edits | [docs/AETHER_AUTHORING_PROTOCOL_v7.md](docs/AETHER_AUTHORING_PROTOCOL_v7.md), [docs/DESIGN-M12-FINE-GRAINED-EDITS.md](docs/DESIGN-M12-FINE-GRAINED-EDITS.md), [docs/ADR-016-m12-fine-grained-edits.md](docs/ADR-016-m12-fine-grained-edits.md), [docs/M12-VALIDATION-MATRIX.md](docs/M12-VALIDATION-MATRIX.md) |
| M13 bounded LSP | [docs/DESIGN-M13-BOUNDED-LSP.md](docs/DESIGN-M13-BOUNDED-LSP.md), [docs/ADR-017-m13-bounded-lsp.md](docs/ADR-017-m13-bounded-lsp.md), [docs/M13-VALIDATION-MATRIX.md](docs/M13-VALIDATION-MATRIX.md) |
| Mainstream maturity roadmap | [docs/ROADMAP-MAINSTREAM-MATURITY.md](docs/ROADMAP-MAINSTREAM-MATURITY.md) |
| Law forks F-NATIVE / F-REGISTRY (authorized; M35a/M24a pilots) | [docs/HUMAN-AUTHORIZE-NATIVE.md](docs/HUMAN-AUTHORIZE-NATIVE.md), [docs/HUMAN-AUTHORIZE-REGISTRY.md](docs/HUMAN-AUTHORIZE-REGISTRY.md), [docs/ADR-059-f-native-authorized-m35a-aeth-to-c.md](docs/ADR-059-f-native-authorized-m35a-aeth-to-c.md), [docs/ADR-060-f-registry-authorized-m24a-offline-cache.md](docs/ADR-060-f-registry-authorized-m24a-offline-cache.md) |
| Human ordered backlog (2026-08-05) | [docs/BACKLOG-HUMAN-2026-08-05.md](docs/BACKLOG-HUMAN-2026-08-05.md) |
| M23 T-CT pure comptime calls | [docs/DESIGN-M23-COMPTIME-PURE-CALLS.md](docs/DESIGN-M23-COMPTIME-PURE-CALLS.md), [docs/ADR-039-m23-comptime-pure-calls.md](docs/ADR-039-m23-comptime-pure-calls.md), [docs/M23-VALIDATION-MATRIX.md](docs/M23-VALIDATION-MATRIX.md) |
| BARP bootstrap authority reduction | [docs/ADR-043-bootstrap-authority-reduction.md](docs/ADR-043-bootstrap-authority-reduction.md)–[docs/ADR-050-barp-phase3b-product-preflight-expansion.md](docs/ADR-050-barp-phase3b-product-preflight-expansion.md), [docs/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md](docs/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md) — through ADR-050 |
| Legacy intake | [docs/LEGACY.md](docs/LEGACY.md) |

### Stack pins

| Layer | Pin |
|-------|-----|
| Language / package | Rust workspace, edition 2021, `rust-version = "1.88"` |
| Core crate | `aether-core` at `crates/xlang-core` (package version **0.36.0**) |
| CLI binary | `aether` via `apps/xlang-cli` (package `aether-cli` **0.36.0**) |
| Artifact format | AETH **v4–v11** compatibility input + deterministic **v11** default / **v12** when M19e task frames present (`TASK_CHECKPOINT` 67; `RELEASE` 66) |
| Product compile | Seed forge-first (ADR-049); multi-module/edit product gates; `AE-SEED-001`–`007` preflights (ADR-050); dual-compare tests/gate; M19e v12 |
| Foreign ABI (M21) | Whole-only pilot; `--grant-lib KEY=PATH`; host load after verify; residual native risk accepted by human; seed dual-compare proven |
| Bootstrap | recovery `--bootstrap` / dual-compare oracle / residual structural AST (product owns seed rebuild ADR-067) |
| Seed compiler | `seed/aether_seed.ae` + checked-in `seed/aether_seed.aeth` |
| Authoring | `aether.ast/v8`, `aether.edit/v8`, `aether.diagnostic/v8` |

### Invariants (may tighten pack; never weaken CONST-\*)

1. **Seed-hosted product compile** — CLI default compile uses the Aether-written seed; bootstrap is not the product compiler path.
2. **Verify before run / write** — VM and forge only accept verified supported AETH v4–v12 (v12 for M19e task frames).
3. **No ambient host capability leak** — default run installs only pure fixtures; grant-backed I/O (M14) requires explicit operator `--grant-*` roots/names and path jail; no shell/network; forge host owns I/O after verification.
4. **Honest self-host claims** — dual-compare only where proven (including M19e/M23 corpora as documented); M23 seed-native D2a body subset only (no nested calls/control-flow callees); do not claim full diagnostic parity.
5. **CLI authority boundary** — the CLI reads only caller-selected local files and writes source or artifacts only to an explicit output path after the required validation/seed-compile path; it has no model or network integration.
6. **Legacy is reference only** — `legacy/` is never a production build input.
7. **Zero-warning gate** — workspace Clippy `all = "deny"`; `unsafe_code = "deny"` (scoped `allow` only for M21 `ffi` load path after human residual-risk acceptance).

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
