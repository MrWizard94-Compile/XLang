# Aether in XLang

This repository hosts Aether, a new local-first language and desktop workbench.
Aether **0.5.0** parses only Aether source, emits deterministic AETH v4 bytecode,
verifies every artifact, and runs it in the Aether VM. It never translates
source to Rust, C, JavaScript, LLVM, or another language.

## Stage 5: Seed-hosted compile path

**Default compilation is no longer bootstrap-hosted for user programs.**

- `aether compile` and Aether Studio invoke the **Aether-written seed compiler**
  (`seed/aether_seed.aeth`, embedded as `SEED_COMPILER_ARTIFACT`) through the
  forge ABI.
- The Rust core remains the **bootstrap**: rebuild the seed (`compile --bootstrap`),
  produce the AST for `check`, and verify seed output against bootstrap in tests.
- The seed self-hosts and, for every shipped example, produces bytecode
  **byte-identical** to the Rust bootstrap.

Seed Profile Stage 5 includes named locals/parameters (not only `vN`), multi-weave
`call` (including forward callees), hex `bytes "..."` literals, UTF-8 text
constants with correct byte lengths, and CRLF input. See
[docs/SEED_PROFILE.md](docs/SEED_PROFILE.md).

### Still honest limits

- Seed diagnostic fidelity is weaker than the Rust bootstrap (`aether check`).
- Prefer a trailing LF on the last source line for the seed line scanner.
- Expanding Seed Profile until it is the full language remains future work;
  bootstrap rebuild of the seed is still required when the seed source changes.

## Workspace

- `crates/xlang-core` — bootstrap parser/emitter/verifier/VM, forge API, seed path
- `apps/xlang-cli` — `aether` CLI (seed compile by default)
- `apps/xlang-studio` — Tauri workbench (seed-hosted compile)
- `seed/` — Aether-written compiler source + checked-in artifact
- `examples/` — programs proven seed-identical to bootstrap
- `AGENTS.md` — Level 4 entry → AGENTS Constitution pack

## Command Line

```powershell
Set-Location C:\WPAI\Software\XLang
cargo run -p aether-cli -- check (Resolve-Path .\examples\welcome.ae)
cargo run -p aether-cli -- compile (Resolve-Path .\examples\welcome.ae) --output .\target\welcome.aeth
cargo run -p aether-cli -- run .\target\welcome.aeth

# Rebuild seed with the Rust bootstrap (after editing seed/aether_seed.ae)
cargo run -p aether-cli -- compile .\seed\aether_seed.ae --output .\seed\aether_seed.aeth --bootstrap
cargo run -p aether-cli -- forge .\seed\aether_seed.aeth .\seed\aether_seed.ae --output .\target\aether_seed.forged.aeth
Get-FileHash .\seed\aether_seed.aeth, .\target\aether_seed.forged.aeth
```

## Desktop Studio

```powershell
Set-Location C:\WPAI\Software\XLang\apps\xlang-studio
npm install
npm run desktop:dev
```

Studio compiles with the seed path. Ollama remains optional, loopback-only review.

## Governance

Binding quality law: [AGENTS.md](AGENTS.md) → universal AGENTS Constitution pack.
Product contract: [MANIFEST.md](MANIFEST.md), [docs/](docs/).
