# Aether in XLang

This repository hosts Aether, a new local-first language and desktop workbench.
Aether **0.5.0** parses only Aether source, emits deterministic AETH v4 or v5 bytecode,
verifies every artifact, and runs it in the Aether VM. It never translates
source to Rust, C, JavaScript, LLVM, or another language.

## Stage 6: Immutable records on the seed-hosted compile path

**Default compilation is no longer bootstrap-hosted for user programs.**

- `aether compile` and Aether Studio invoke the **Aether-written seed compiler**
  (`seed/aether_seed.aeth`, embedded as `SEED_COMPILER_ARTIFACT`) through the
  forge ABI.
- The Rust core remains the **bootstrap**: rebuild the seed (`compile --bootstrap`),
  produce the AST for `check`, and verify seed output against bootstrap in tests.
- The seed self-hosts, and the shipped examples plus a complete canonical-surface
  regression corpus produce bytecode **byte-identical** to the Rust bootstrap.

Seed Profile Stage 6 emits the complete documented **canonical Aether 0.5 source
surface**: named locals/parameters (not only `vN`), every statement and shallow
expression family, `borrow`/`move`, multi-weave `call` (including forward
callees), hex `bytes "..."` literals, UTF-8 text constants, the canonical
`\\`, `\"`, `\n`, `\r`, and `\t` text escapes, and CRLF or LF input (including a
valid final line without a terminal LF). It also supports bounded immutable
nominal records: `record`, `make`, and explicit `field borrow` projection.
Programs without records remain AETH v4; record-bearing programs emit verified
AETH v5. See [docs/AETHER_0.5.md](docs/AETHER_0.5.md) and
[docs/SEED_PROFILE.md](docs/SEED_PROFILE.md).

### Still honest limits

- Full invalid-source diagnostic parity is not claimed; the Rust bootstrap
  remains the diagnostic authority (`aether check`).
- Records are intentionally non-recursive and immutable in 0.5. Host invocation
  accepts and returns primitives only; use an Aether weave to project a field.
- Future language extensions require their own seed-emission parity proof before
  they become part of the product compile surface.
- Bootstrap rebuild of the seed is still required after changing the seed source.

## North star and evidence-led roadmap

Aether 0.5 is the current executable contract, not the full long-range language
vision. The project is deliberately designing for AI-primary authorship while
keeping deterministic, locally verifiable compiler authority. Read the design
set in this order:

1. [docs/NORTH_STAR.md](docs/NORTH_STAR.md) — intended product direction and
   current-law constraints.
2. [docs/CORE_CLAIMS.md](docs/CORE_CLAIMS.md) — proven facts, accepted
   directions, research hypotheses, and prohibited claims.
3. [docs/research/](docs/research/) — primary-source reference study,
   decomposition, and evidence plan.
4. [docs/ROADMAP.md](docs/ROADMAP.md) — approved dependency order for future
   language work.

These documents do not claim that proposed allocators, effects, concurrency,
SoA lowering, C interop, structural edits, or a native backend exist in 0.5.

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
