# Aether in XLang

This repository hosts Aether, a new local-first language and desktop workbench.
Aether **0.6.0** parses only Aether source, emits deterministic AETH v6
bytecode, verifies every artifact, and runs it in the Aether VM. It never
translates source to Rust, C, JavaScript, LLVM, or another language. Verified
AETH v4 and v5 artifacts remain compatible inputs; new 0.6 compilation emits
v6.

## Stage 7: bounded arenas and buffers on the seed-hosted compile path

**Default compilation is no longer bootstrap-hosted for user programs.**

- `aether compile` and Aether Studio invoke the **Aether-written seed compiler**
  (`seed/aether_seed.aeth`, embedded as `SEED_COMPILER_ARTIFACT`) through the
  forge ABI.
- The Rust core remains the **bootstrap**: rebuild the seed (`compile --bootstrap`),
  produce the AST for `check`, and verify seed output against bootstrap in tests.
- The seed self-hosts, and the shipped examples plus a complete canonical-surface
  regression corpus produce bytecode **byte-identical** to the Rust bootstrap.

Seed Profile Stage 7 emits the complete prior canonical surface plus Aether
0.6's documented M2 resource corpus: named locals/parameters, every statement
and shallow expression family, `borrow`/`move`/`access`, multi-weave `call`, hex
`bytes "..."` literals, UTF-8 text constants, canonical escapes, immutable
records, and closed `arena` / Whole-or-Truth-buffer outcomes. New compilation
emits verified AETH v6. See [docs/AETHER_0.6.md](docs/AETHER_0.6.md) and
[docs/SEED_PROFILE.md](docs/SEED_PROFILE.md).

## M3: versioned structural authoring contract

Aether 0.6 tooling now exposes a local, machine-readable `aether.ast/v1`
document and accepts bounded `aether.edit/v1` structural edits. The protocol
uses exact canonical source revisions to reject stale requests, supports typed
top-level record/weave insert, replace, and delete operations, reparses the
formatter-owned result, and seed-compiles it before the CLI or Studio writes or
persists source. See [docs/AETHER_AUTHORING_PROTOCOL_v1.md](docs/AETHER_AUTHORING_PROTOCOL_v1.md)
and [docs/ADR-005-structural-authoring-contract.md](docs/ADR-005-structural-authoring-contract.md).

### Still honest limits

- Full invalid-source diagnostic parity is not claimed; the Rust bootstrap
  remains the diagnostic authority (`aether check`).
- Records are intentionally non-recursive and immutable. Resource outcomes are
  immediate terminal `choose` conditions, not first-class values; Buffer owners
  cannot be weave results or cross the host ABI.
- Host invocation accepts and returns primitives only; use an Aether weave to
  project a record field.
- Future language extensions require their own seed-emission parity proof before
  they become part of the product compile surface.
- Bootstrap rebuild of the seed is still required after changing the seed source.
- M3 edits are deliberately top-level declaration operations. Fine-grained
  statement/expression edits and any claim of universal syntax-error-proof AI
  generation remain future, separately versioned work.

## North star and evidence-led roadmap

Aether 0.6 is the current executable contract, not the full long-range language
vision. The project is deliberately designing for AI-primary authorship while
keeping deterministic, locally verifiable compiler authority. Read the design
set in this order:

1. [docs/NORTH_STAR.md](docs/NORTH_STAR.md) — intended product direction and
   current-law constraints.
2. [docs/CORE_CLAIMS.md](docs/CORE_CLAIMS.md) — proven facts, accepted
   directions, research hypotheses, and prohibited claims.
3. [docs/research/](docs/research/) — primary-source reference study,
   decomposition, and evidence plan.
4. [docs/AETHER_0.6.md](docs/AETHER_0.6.md) and
   [docs/ADR-004-aeth-v6-bounded-resources.md](docs/ADR-004-aeth-v6-bounded-resources.md)
   — current resource contract and deliberate limits.
5. [docs/AETHER_AUTHORING_PROTOCOL_v1.md](docs/AETHER_AUTHORING_PROTOCOL_v1.md)
   and [docs/ADR-005-structural-authoring-contract.md](docs/ADR-005-structural-authoring-contract.md)
   — implemented M3 authoring contract and its limits.
6. [docs/ROADMAP.md](docs/ROADMAP.md) — completed M2/M3 scope and approved
   dependency order for future language work.

These documents do not claim effects, concurrency, SoA lowering, C interop,
fine-grained structural edits, or a native backend exist in 0.6.

## Workspace

- `crates/xlang-core` — bootstrap parser, typed resource semantic plan,
  emitter/verifier/VM, forge API, seed path
- `apps/xlang-cli` — `aether` CLI (seed compile by default)
- `apps/xlang-studio` — Tauri workbench (seed-hosted compile)
- `seed/` — Aether-written compiler source + checked-in artifact
- `examples/` — programs proven seed-identical to bootstrap, including M2 cases
- `AGENTS.md` — Level 4 entry → AGENTS Constitution pack

## Command Line

```powershell
Set-Location C:\WPAI\Software\XLang
cargo run -p aether-cli -- check (Resolve-Path .\examples\arena-buffer.ae)
cargo run -p aether-cli -- structure (Resolve-Path .\examples\arena-buffer.ae)
cargo run -p aether-cli -- compile (Resolve-Path .\examples\arena-buffer.ae) --output .\target\arena-buffer.aeth
cargo run -p aether-cli -- run .\target\arena-buffer.aeth

# Rebuild the seed safely after editing seed/aether_seed.ae.
# Promote it only after the bootstrap and self-forged hashes are identical.
cargo run -p aether-cli -- compile .\seed\aether_seed.ae --output .\target\aether_seed.bootstrap.aeth --bootstrap
cargo run -p aether-cli -- forge .\target\aether_seed.bootstrap.aeth .\seed\aether_seed.ae --output .\target\aether_seed.forged.aeth
Get-FileHash .\target\aether_seed.bootstrap.aeth, .\target\aether_seed.forged.aeth
Copy-Item .\target\aether_seed.bootstrap.aeth .\seed\aether_seed.aeth
cargo build -p aether-cli
```

## Desktop Studio

```powershell
Set-Location C:\WPAI\Software\XLang\apps\xlang-studio
npm install
npm run desktop:dev
```

Studio compiles with the seed path. Ollama remains optional, loopback-only review.
Its Structure and Apply edit controls use the same local M3 contract and never
make an AI model compiler authority.

## Governance

Binding quality law: [AGENTS.md](AGENTS.md) → universal AGENTS Constitution pack.
Product contract: [MANIFEST.md](MANIFEST.md), [docs/](docs/).
