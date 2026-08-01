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

**Stage status:** Stage 7 (bounded arenas and Copy-element buffers) and M4
(the bounded `Error[Whole]` effect) are implemented on the seed-hosted product
compile path in Aether 0.7 / AETH v7.
Default CLI compilation uses the Aether-written seed compiler. Rust
bootstrap remains for seed rebuild (`compile --bootstrap`), `check` AST, and
proof dual-compare. Seed Profile self-host, all shipped examples, the complete
prior canonical surface (including records), and the documented M2
arena/buffer corpus match bootstrap byte-for-byte.
Full diagnostic parity is not claimed for the seed.

### Product docs (Level 4)

| Role | Path |
|------|------|
| Overview | [README.md](README.md) |
| Contract / release gate | [MANIFEST.md](MANIFEST.md) |
| Architecture | [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) |
| Language (0.7 current / 0.6, 0.5, and 0.4 historical) | [docs/AETHER_0.7.md](docs/AETHER_0.7.md), [docs/AETHER_0.6.md](docs/AETHER_0.6.md), [docs/AETHER_0.5.md](docs/AETHER_0.5.md), [docs/AETHER_0.4.md](docs/AETHER_0.4.md) |
| Record decision | [docs/ADR-001-records-and-aeth-v5.md](docs/ADR-001-records-and-aeth-v5.md) |
| AI-first design foundation | [docs/NORTH_STAR.md](docs/NORTH_STAR.md), [docs/CORE_CLAIMS.md](docs/CORE_CLAIMS.md), [docs/ADR-002-ai-first-design-foundation.md](docs/ADR-002-ai-first-design-foundation.md) |
| M1/M2 resource decisions | [docs/DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md](docs/DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md), [docs/ADR-003-value-resource-semantics.md](docs/ADR-003-value-resource-semantics.md), [docs/ADR-004-aeth-v6-bounded-resources.md](docs/ADR-004-aeth-v6-bounded-resources.md), [docs/M1-VALIDATION-MATRIX.md](docs/M1-VALIDATION-MATRIX.md) |
| M3/M4 authoring and error effect | [docs/AETHER_AUTHORING_PROTOCOL_v2.md](docs/AETHER_AUTHORING_PROTOCOL_v2.md), [docs/ADR-007-m4-typed-error-effect.md](docs/ADR-007-m4-typed-error-effect.md), [docs/M4-VALIDATION-MATRIX.md](docs/M4-VALIDATION-MATRIX.md) |
| Research and roadmap | [docs/research/](docs/research/), [docs/ROADMAP.md](docs/ROADMAP.md) |
| Seed Profile | [docs/SEED_PROFILE.md](docs/SEED_PROFILE.md) |
| Forge ABI | [docs/FORGE_CONTRACT.md](docs/FORGE_CONTRACT.md) |
| Studio retirement decision | [docs/ADR-006-retire-aether-studio.md](docs/ADR-006-retire-aether-studio.md) |
| Migration audit | [AUDIT_REPORT.md](AUDIT_REPORT.md) |
| Legacy intake | [docs/LEGACY.md](docs/LEGACY.md) |

### Stack pins

| Layer | Pin |
|-------|-----|
| Language / package | Rust workspace, edition 2021, `rust-version = "1.88"` |
| Core crate | `aether-core` at `crates/xlang-core` (package version 0.7.0) |
| CLI binary | `aether` via `apps/xlang-cli` (package `aether-cli` 0.7.0) |
| Artifact format | AETH **v4/v5/v6** compatibility input + deterministic **v7** output with bounded arena and effect metadata (earlier/unknown versions rejected) |
| Product compile | Seed-hosted (`compile_with_seed` / embedded `SEED_COMPILER_ARTIFACT`) |
| Bootstrap | `compile --bootstrap` / `check` AST / rebuild `seed/*.aeth` |
| Seed compiler | `seed/aether_seed.ae` + checked-in `seed/aether_seed.aeth` |

### Invariants (may tighten pack; never weaken CONST-\*)

1. **Seed-hosted product compile** — CLI default compile uses the Aether-written seed; bootstrap is not the product compiler path.
2. **Verify before run / write** — VM and forge only accept verified supported AETH v4, v5, v6, or v7.
3. **No host capability leak** — invoked artifacts have no file, process, network, or shell authority; forge host owns I/O after verification.
4. **Honest self-host claims** — Seed Profile, shipped examples, the complete documented prior canonical surface (including immutable records), and the documented M2 arena/buffer and M4 error-effect corpora match bootstrap in tests; do not claim full diagnostic parity or parity for future language extensions without proof.
5. **CLI authority boundary** — the CLI reads only caller-selected local files and writes source or artifacts only to an explicit output path after the required validation/seed-compile path; it has no model or network integration.
6. **Legacy is reference only** — `legacy/` is never a production build input.
7. **Zero-warning gate** — workspace Clippy `all = "deny"`; `unsafe_code = "forbid"`.

### Commands (quality gate)

From repository root `C:\WPAI\Software\XLang` (or equivalent checkout):

```powershell
# Pack law
pwsh -File "../../AGENTS Constitution/tools/verify-pack.ps1"

# Core + CLI + seed self-host proof
cargo fmt --all -- --check
cargo test -p aether-core
cargo test -p aether-cli
cargo clippy -p aether-core -p aether-cli -- -D warnings

# Stage examples
cargo run -p aether-cli -- check (Resolve-Path .\examples\welcome.ae)
cargo run -p aether-cli -- compile (Resolve-Path .\examples\welcome.ae) --output .\target\welcome.aeth
cargo run -p aether-cli -- run .\target\welcome.aeth

# Seed forge reproducibility
cargo run -p aether-cli -- compile .\seed\aether_seed.ae --output .\target\aether_seed.aeth --bootstrap
cargo run -p aether-cli -- forge .\target\aether_seed.aeth .\seed\aether_seed.ae --output .\target\aether_seed.forged.aeth

```

Release additionally requires a release CLI binary build, forge contract verification, and binary inspection/launch before delivery — see [MANIFEST.md](MANIFEST.md).

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
