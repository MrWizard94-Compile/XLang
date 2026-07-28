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

This repository hosts **Aether**: a local-first language and desktop workbench.
Aether parses only Aether source, emits **AETH** bytecode, verifies every
artifact, and runs it in the Aether VM. It never transpiles source to Rust, C,
JavaScript, LLVM, or another language.

**Stage status:** Stage 3 (Seed Profile compiler) is implemented. Self-hosting is
claimed **only** for the Seed Profile with reproducible artifact comparison.
Full Aether 0.4 remains Rust-bootstrap compiled.

### Product docs (Level 4)

| Role | Path |
|------|------|
| Overview | [README.md](README.md) |
| Contract / release gate | [MANIFEST.md](MANIFEST.md) |
| Architecture | [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) |
| Language (0.4) | [docs/AETHER_0.4.md](docs/AETHER_0.4.md) |
| Seed Profile | [docs/SEED_PROFILE.md](docs/SEED_PROFILE.md) |
| Forge ABI | [docs/FORGE_CONTRACT.md](docs/FORGE_CONTRACT.md) |
| Migration audit | [AUDIT_REPORT.md](AUDIT_REPORT.md) |
| Legacy intake | [docs/LEGACY.md](docs/LEGACY.md) |

### Stack pins

| Layer | Pin |
|-------|-----|
| Language / package | Rust workspace, edition 2021, `rust-version = "1.88"` |
| Core crate | `aether-core` at `crates/xlang-core` (package version 0.4.0) |
| CLI binary | `aether` via `apps/xlang-cli` (package `aether-cli` 0.4.0) |
| Desktop | Tauri 2 + React/TypeScript at `apps/xlang-studio` (0.4.0) |
| Artifact format | AETH **v4** only (earlier versions intentionally rejected) |
| Seed compiler | `seed/aether_seed.ae` + checked-in `seed/aether_seed.aeth` |
| Optional AI | Docker Ollama, loopback HTTP only; never compiler authority |

### Invariants (may tighten pack; never weaken CONST-\*)

1. **Single bootstrap core** — CLI and Studio call the same Rust bootstrap core for full Aether 0.4.
2. **Verify before run / write** — VM and forge only accept verified AETH v4.
3. **No host capability leak** — invoked artifacts have no file, process, network, or shell authority; forge host owns I/O after verification.
4. **Honest self-host claims** — only Seed Profile may be called self-hosted, and only with byte-identical multi-generation proof plus a distinct-variant check.
5. **Local-first data** — Studio source/model choices stay in local WebView storage; no cloud sync of source or artifacts.
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
cargo run -p aether-cli -- compile .\seed\aether_seed.ae --output .\target\aether_seed.aeth
cargo run -p aether-cli -- forge .\target\aether_seed.aeth .\seed\aether_seed.ae --output .\target\aether_seed.forged.aeth

# Studio frontend
Set-Location .\apps\xlang-studio
npm run lint
npm test
npm run build
```

Release additionally requires Windows Tauri bundle build, forge contract verification, live Docker Ollama status check when claiming AI integration, and package inspection before delivery — see [MANIFEST.md](MANIFEST.md).

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
