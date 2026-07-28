# Aether in XLang

This repository hosts Aether, a new local-first language and desktop workbench.
Aether 0.4.0 parses only Aether source, emits deterministic AETH v4 bytecode,
verifies every artifact, and runs it in the Aether VM. It never translates
source to Rust, C, JavaScript, LLVM, or another language.

## Stage 4: Multi-Weave Seed Profile

Stage 4 expands the Aether-written Seed Profile compiler so it compiles
multi-weave programs with `call`, while still self-hosting. Stage 3 remains the
foundation: bounded `seek` / `number` / pack / unpack / poke primitives used by
the seed to build AETH v4 without a host parser callback.

The checked-in [seed/aether_seed.ae](seed/aether_seed.ae) is an Aether-written
compiler for the documented Seed Profile. Its verified artifact is
[seed/aether_seed.aeth](seed/aether_seed.aeth). The regression tests prove:

1. Rust bootstrap compilation of the seed source equals the checked-in artifact.
2. `aether forge` invokes that artifact to compile the same source.
3. The first and second Aether-produced generations are byte-identical to the
   bootstrap artifact.
4. The forged compiler creates a distinct, valid artifact for a source variant,
   so it is not returning a fixed stored artifact.
5. The seed forges a multi-weave program with `call` byte-identically to
   bootstrap; the artifact runs with the expected stdout and exit code.

This is a precise self-hosting claim for the Seed Profile only (now including
multi-weave + `call`). The complete Aether 0.4 language remains bootstrapped by
the Rust core. See [docs/SEED_PROFILE.md](docs/SEED_PROFILE.md) for the accepted
subset and its limits.

## Workspace

- `crates/xlang-core` contains the bootstrap parser, semantic checks, canonical
  formatter, AETH emitter, verifier, VM, typed invocation boundary, and
  self-hosting regression test.
- `apps/xlang-cli` builds the `aether` compiler and forge bridge.
- `apps/xlang-studio` is Aether Studio, the local Tauri desktop workbench.
- `seed` contains the Aether-written Seed Profile compiler and verified binary.
- `examples` contains regular Aether programs.
- `legacy` preserves V1, V2, and historical AI Studio material as reference
  only. It is not part of the production build.
- `AGENTS.md` is the Level 4 project entry; binding quality law is the universal
  **AGENTS Constitution** pack. `SOUL.md` is a superseded stub only.

## Command Line

    Set-Location C:\WPAI\Software\XLang
    cargo run -p aether-cli -- check (Resolve-Path .\examples\welcome.ae)
    cargo run -p aether-cli -- compile (Resolve-Path .\examples\welcome.ae) --output .\target\welcome.aeth
    cargo run -p aether-cli -- run .\target\welcome.aeth

Forge a Seed Profile compiler artifact locally:

    cargo run -p aether-cli -- compile .\seed\aether_seed.ae --output .\target\aether_seed.aeth
    cargo run -p aether-cli -- forge .\target\aether_seed.aeth .\seed\aether_seed.ae --output .\target\aether_seed.forged.aeth
    Get-FileHash .\target\aether_seed.aeth, .\target\aether_seed.forged.aeth

The two SHA-256 values must match. The checked-in AETH v4 artifact begins with
`AETH` followed by version byte `4`; earlier AETH versions are intentionally
rejected.

## Desktop Studio

    Set-Location C:\WPAI\Software\XLang\apps\xlang-studio
    npm install
    npm run desktop:dev

Studio builds the current Aether document, displays the verified AETH artifact
and VM output, and keeps compilation separate from AI review. Docker-hosted
Ollama is the only AI integration. It is loopback-only, optional, and local;
`qwen2.5:3b` remains the default reviewer for the GTX 1660 Ti 6 GB environment.

## Data Handling

Compilation, forge invocation, and the Seed Profile proof run locally. Studio
keeps its editor buffer and model selection in local WebView storage under
`aether.source` and `aether.model`. No source, artifact, model selection, or
financial data is uploaded, synchronized, or stored in a cloud service.

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md),
[docs/AETHER_0.4.md](docs/AETHER_0.4.md),
[docs/SEED_PROFILE.md](docs/SEED_PROFILE.md),
[docs/FORGE_CONTRACT.md](docs/FORGE_CONTRACT.md), and
[AUDIT_REPORT.md](AUDIT_REPORT.md) for implementation and verification details.
