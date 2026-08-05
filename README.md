# Aether in XLang

This repository hosts Aether, a new local-first language and CLI toolchain for
deterministic, AI-primary authorship. Aether toolchain package **0.26.0**
(language surface **0.11**, AETH **v11**) parses only Aether source, emits
deterministic AETH bytecode, verifies every artifact, and runs it in the Aether
VM. It never translates source to Rust, C, JavaScript, LLVM, or another
language. Verified AETH v4–v10 artifacts remain compatible inputs with their
original meanings; new compilation emits v11.

**Executable contract:** [MANIFEST.md](MANIFEST.md) · **Claims:** [docs/CORE_CLAIMS.md](docs/CORE_CLAIMS.md) · **Law:** [AGENTS.md](AGENTS.md)

## Seed-hosted compile path, resources, effects, comptime, and layout

**Default compilation is seed-hosted for user programs**, including M19a
`release` and M19b nursery×resource Policy A (seed≡bootstrap proven).

- `aether compile` invokes the **Aether-written seed compiler**
  (`seed/aether_seed.aeth`, embedded as `SEED_COMPILER_ARTIFACT`) through the
  forge ABI.
- The Rust core remains the **bootstrap**: rebuild the seed (`compile --bootstrap`),
  produce the AST for `check`, and verify seed output against bootstrap in tests.
- The seed self-hosts, and the documented dual-compare corpus (shipped examples
  on the seed path, M2–M8 fixtures, M19a release, M19b nursery+resource, modules
  via elaboration, etc.) produces bytecode **byte-identical** to the Rust
  bootstrap where claimed in tests.

Language surface **0.11** includes M2 resources, M4 `Error[Whole]`, M5 comptime,
M6 layout, M7 nurseries, and M8 pure host weaves. Toolchain packages through
**0.26** add offline projects/modules/LSP, grant-backed host I/O, comptime name
chaining, resource+handle, `aether test`, workspaces, stdlib layer 0,
cross-package imports, product-path `release`, and nursery×resource Policy A.
See [docs/AETHER_0.11.md](docs/AETHER_0.11.md),
[docs/AETHER_0.26.md](docs/AETHER_0.26.md), and [docs/SEED_PROFILE.md](docs/SEED_PROFILE.md).

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

M6 adds author-visible layout for multi-field tables:

```aether
shape particle:
  mass Whole
  charge Whole

weave main [] -> Whole:
  bind memory <- arena 4096
  bind mutable parts <- table particle layout columns
  ...
```

`layout rows` and `layout columns` store the same logical cells with different
physical order. See
[docs/DESIGN-M6-EXPLICIT-LAYOUT-SHAPES.md](docs/DESIGN-M6-EXPLICIT-LAYOUT-SHAPES.md).

## Versioned structural authoring contract

Aether 0.11 tooling exposes a local, machine-readable `aether.ast/v6` document
and accepts bounded `aether.edit/v6` structural edits. The protocol
uses exact canonical source revisions to reject stale requests, supports typed
top-level record/shape/weave insert, replace, and delete operations, reparses the
formatter-owned result, and seed-compiles it before the CLI writes source to an
explicit output path. It exposes effect annotations, M4/M5/M6/M7 nodes, and a
required `Bind.stage` (`runtime` or `comptime`) without reinterpreting v1–v4.
See [docs/AETHER_AUTHORING_PROTOCOL_v5.md](docs/AETHER_AUTHORING_PROTOCOL_v5.md)
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
