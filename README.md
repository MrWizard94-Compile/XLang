# Aether in XLang

This repository hosts Aether, a new local-first language and CLI toolchain for
deterministic, AI-primary authorship. Aether **0.8.0** parses only Aether
source, emits deterministic AETH v8 bytecode, verifies every artifact, and runs
it in the Aether VM. It never translates source to Rust, C, JavaScript, LLVM,
or another language. Verified AETH v4/v5/v6/v7 artifacts remain compatible
inputs with their original meanings; new compilation emits v8.

## Seed-hosted compile path, M2 resources, M4 errors, and M5 comptime

**Default compilation is no longer bootstrap-hosted for user programs.**

- `aether compile` invokes the **Aether-written seed compiler**
  (`seed/aether_seed.aeth`, embedded as `SEED_COMPILER_ARTIFACT`) through the
  forge ABI.
- The Rust core remains the **bootstrap**: rebuild the seed (`compile --bootstrap`),
  produce the AST for `check`, and verify seed output against bootstrap in tests.
- The seed self-hosts, and the shipped examples plus a complete canonical-surface
  regression corpus produce bytecode **byte-identical** to the Rust bootstrap.

Seed Profile emits the complete prior canonical surface plus Aether 0.8's
documented M2 resource and M4 error corpora: named locals/parameters, every
statement and shallow expression family, `borrow`/`move`/`access`, multi-weave
calls, immutable records, closed `arena` / Whole-or-Truth-buffer outcomes, and
the explicit `raises Whole`, `raise`, `forward call`, and terminal `handle call`
forms. It also emits M5's root-only literal `comptime bind` as AETH v8
`COMPTIME_WHOLE` provenance. New compilation emits verified AETH v8. See
[docs/AETHER_0.8.md](docs/AETHER_0.8.md) and
[docs/SEED_PROFILE.md](docs/SEED_PROFILE.md).

```aether
weave leaf [value: Whole] -> Whole raises Whole:
  raise value

weave main [] -> Whole:
  bind mutable success <- 0
  bind mutable code <- 0
  handle call leaf 17 into success otherwise error into code
```

M4 is deliberately one bounded abortive `Error[Whole]` effect, not a general
exception or algebraic-effects system. Effect boundaries are copy-only and
cannot cross live owners, loans, arenas, buffers, or M2 outcomes.

M5 is deliberately one bounded evaluator, not a macro or build-script system:

```aether
weave main [] -> Whole:
  comptime bind table_width <- product 16 8
  comptime bind header_size <- sum 12 4
  yield sum table_width header_size
```

It accepts exactly five checked literal `Whole` operations, has a fixed
1,024-directive budget, and has no names, calls, loops, text, resource, effect,
or host authority surface. See
[docs/DESIGN-M5-DETERMINISTIC-COMPTIME.md](docs/DESIGN-M5-DETERMINISTIC-COMPTIME.md).

## Versioned structural authoring contract

Aether 0.8 tooling exposes a local, machine-readable `aether.ast/v3` document
and accepts bounded `aether.edit/v3` structural edits. The protocol
uses exact canonical source revisions to reject stale requests, supports typed
top-level record/weave insert, replace, and delete operations, reparses the
formatter-owned result, and seed-compiles it before the CLI writes source to an
explicit output path. It exposes effect annotations, M4 statement nodes, and a
required `Bind.stage` (`runtime` or `comptime`) without reinterpreting v1/v2.
See [docs/AETHER_AUTHORING_PROTOCOL_v3.md](docs/AETHER_AUTHORING_PROTOCOL_v3.md)
and [docs/ADR-005-structural-authoring-contract.md](docs/ADR-005-structural-authoring-contract.md).

### Still honest limits

- Full invalid-source diagnostic parity is not claimed; the Rust bootstrap
  remains the diagnostic authority (`aether check`).
- Records are intentionally non-recursive and immutable. Resource outcomes are
  immediate terminal `choose` conditions, not first-class values; Buffer owners
  cannot be weave results or cross the host ABI.
- M4 supports only `Error[Whole]`, `Whole` erroring results, and terminal
  handling. It has no effect inference, resumption, cleanup, cancellation, or
  resource/effect composition.
- M5 supports only one literal Whole arithmetic operation per explicit root
  directive. It has no dependency chaining, compile-time calls, control flow,
  macro expansion, build hooks, or configurable fuel.
- Host invocation accepts and returns primitives only; use an Aether weave to
  project a record field.
- Future language extensions require their own seed-emission parity proof before
  they become part of the product compile surface.
- Bootstrap rebuild of the seed is still required after changing the seed source.
- M3 edits are deliberately top-level declaration operations. Fine-grained
  statement/expression edits and any claim of universal syntax-error-proof AI
  generation remain future, separately versioned work.

## North star and evidence-led roadmap

Aether 0.8 is the current executable contract, not the full long-range language
vision. The project is deliberately designing for AI-primary authorship while
keeping deterministic, locally verifiable compiler authority. Read the design
set in this order:

1. [docs/NORTH_STAR.md](docs/NORTH_STAR.md) — intended product direction and
   current-law constraints.
2. [docs/CORE_CLAIMS.md](docs/CORE_CLAIMS.md) — proven facts, accepted
   directions, research hypotheses, and prohibited claims.
3. [docs/research/](docs/research/) — primary-source reference study,
   decomposition, and evidence plan.
4. [docs/AETHER_0.8.md](docs/AETHER_0.8.md) and
   [docs/ADR-004-aeth-v6-bounded-resources.md](docs/ADR-004-aeth-v6-bounded-resources.md)
   — current resource contract and deliberate limits.
5. [docs/AETHER_AUTHORING_PROTOCOL_v3.md](docs/AETHER_AUTHORING_PROTOCOL_v3.md)
   and [docs/ADR-005-structural-authoring-contract.md](docs/ADR-005-structural-authoring-contract.md)
   — implemented M3 authoring contract and its limits.
6. [docs/DESIGN-M4-TYPED-ERROR-EFFECTS.md](docs/DESIGN-M4-TYPED-ERROR-EFFECTS.md),
   [docs/ADR-007-m4-typed-error-effect.md](docs/ADR-007-m4-typed-error-effect.md),
   and [docs/M4-VALIDATION-MATRIX.md](docs/M4-VALIDATION-MATRIX.md) — implemented
   M4 design, executable semantic kernel, and proof matrix.
7. [docs/ROADMAP.md](docs/ROADMAP.md) — completed M2/M3/M4/M5 scope,
   and approved dependency order for future language work.

These documents do not claim general effects, concurrency, SoA lowering, C
interop, fine-grained structural edits, or a native backend exist in 0.8.

## Workspace

- `crates/xlang-core` — bootstrap parser, typed resource semantic plan,
  emitter/verifier/VM, forge API, seed path
- `apps/xlang-cli` — `aether` CLI (seed compile by default)
- `seed/` — Aether-written compiler source + checked-in artifact
- `examples/` — programs proven seed-identical to bootstrap, including M2 cases
- `AGENTS.md` — Level 4 entry → AGENTS Constitution pack

## Command Line

```powershell
Set-Location C:\WPAI\Software\XLang
cargo run -p aether-cli -- check (Resolve-Path .\examples\comptime.ae)
cargo run -p aether-cli -- structure (Resolve-Path .\examples\comptime.ae)
cargo run -p aether-cli -- compile (Resolve-Path .\examples\comptime.ae) --output .\target\comptime.aeth
cargo run -p aether-cli -- run .\target\comptime.aeth

# Rebuild the seed safely after editing seed/aether_seed.ae.
# Promote it only after the bootstrap and self-forged hashes are identical.
cargo run -p aether-cli -- compile .\seed\aether_seed.ae --output .\target\aether_seed.bootstrap.aeth --bootstrap
cargo run -p aether-cli -- forge .\target\aether_seed.bootstrap.aeth .\seed\aether_seed.ae --output .\target\aether_seed.forged.aeth
Get-FileHash .\target\aether_seed.bootstrap.aeth, .\target\aether_seed.forged.aeth
cargo run -p aether-cli -- compile .\seed\aether_seed.ae --output .\seed\aether_seed.aeth --bootstrap
cargo build -p aether-cli
```

## Product interface

The CLI is the shipped Aether product interface. It exposes canonical structure
and validated structural edits locally without introducing an application, model,
or network authority. The former Studio workbench was retired in
[docs/ADR-006-retire-aether-studio.md](docs/ADR-006-retire-aether-studio.md);
historical material remains under `legacy/` as reference only.

## Governance

Binding quality law: [AGENTS.md](AGENTS.md) → universal AGENTS Constitution pack.
Product contract: [MANIFEST.md](MANIFEST.md), [docs/](docs/).
