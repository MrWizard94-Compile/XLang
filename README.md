# Aether in XLang

This repository hosts Aether, a local-first language and CLI toolchain for
deterministic, AI-primary authorship. Aether toolchain package **0.36.0**
(language surface **0.11** plus bounded M19e task semantics) parses only Aether
source, emits deterministic AETH bytecode, verifies every artifact,
and runs it in the Aether VM. **Default product path** is seed→AETH→VM; it does
not translate Aether source to Rust, C, JavaScript, or another language as the
product compiler. Optional F-NATIVE lowers **already-verified** AETH to C/object/
LLVM/exe when the operator requests it (host toolchain required); `--target` is
closed to the M35j matrix, with host dual-run and cross link-only behavior. Verified AETH
v4–v10 artifacts remain compatibility inputs with their original meanings.
Source without task frames emits v11; valid M19e task source emits v12.

**Executable contract:** [MANIFEST.md](MANIFEST.md) · **Claims:**
[docs/CORE_CLAIMS.md](docs/CORE_CLAIMS.md) · **Current delta:**
[docs/AETHER_0.36.md](docs/AETHER_0.36.md)

**Current bounded task-frame behavior:**
[M19e active-frame cancellation](docs/DESIGN-M19E-T-RX-ACTIVE-FRAME-CANCEL.md) /
[ADR-042](docs/ADR-042-m19e-active-frame-cancel.md). Package 0.36 accepts
closed-subset `task weave` / `checkpoint` source, emits v12 task frames, and
deterministically cancels parked frames on a later eligible sibling failure.

## Seed-hosted compile path, resources, effects, comptime, and layout

**Default compilation is seed-emitted for user programs**, including M19a
`release`, M19b nursery×resource Policy A, M19d multi-weave arenas, M19e task
frames, M21 foreign pilot declarations, and M23 after its explicit bootstrap
materialization bridge.

- `aether compile` invokes the **Aether-written seed compiler**
  (`seed/aether_seed.aeth`, embedded as `SEED_COMPILER_ARTIFACT`) through the
  forge ABI.
- The Rust core remains the **bootstrap recovery/oracle**: dual-compare proofs,
  `--bootstrap` AST diagnostics, and residual structural AST. **Product** seed
  rebuild is `compile` without `--bootstrap` (ADR-067).
- The seed self-hosts, and the documented dual-compare corpus (shipped examples
  on the seed path, M2–M8 fixtures, M15 chaining, M19a release, M19b
  nursery+resource, M19d multi-weave arenas, M21 foreign-pilot, M23 pure
  comptime helpers, modules via elaboration, etc.) produces bytecode
  **byte-identical** to the Rust bootstrap where claimed in tests.

Language surface **0.11** includes M2 resources, M4 `Error[Whole]`, M5 comptime,
M6 layout, M7 nurseries, and M8 pure host weaves. Toolchain packages through
**0.36** add offline projects/modules/LSP, grant-backed host I/O, a **bounded
foreign weave pilot** (Whole-only, explicit library grant; not sandboxed),
comptime name chaining, resource+handle, `aether test` / `aether project test`
(optional grants and reports), workspaces, stdlib layer 1, cross-package
imports, product-path `release`, nursery×resource Policy A+, multi-weave arenas,
cooperative Policy B bounds, pure bounded comptime helper calls, and M19e's
restricted active-frame cancellation. Package 0.34 adds only the internal
RTP-001 ASCII Text runtime fast path; 0.35 adds PKG-001 optional local workspace
locks; and 0.36 adds `task weave`, `checkpoint`, AETH v12, and authoring v8.
M19e adds no registry, network authority, guest cancellation API, or host
capability. See
[docs/AETHER_0.11.md](docs/AETHER_0.11.md),
[docs/AETHER_0.36.md](docs/AETHER_0.36.md), and [docs/SEED_PROFILE.md](docs/SEED_PROFILE.md).

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

M5/M15/M23 remain deliberately bounded evaluators, not a macro or build-script
system:

```aether
weave main [] -> Whole:
  comptime bind table_width <- product 16 8
  comptime bind header_size <- sum 12 4
  yield sum table_width header_size
```

M5 accepts five checked Whole arithmetic operations; M15 adds earlier comptime
names; M23 adds one eligible pure helper call. The fixed 1,024-directive budget
remains. Comptime has no control flow, recursion, Text/Bytes/Truth evaluation,
resource/effect/host authority, macros, or configurable fuel. M23 source is
bootstrap-materialized before seed emission as documented in
[docs/AETHER_0.33.md](docs/AETHER_0.33.md).

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

Aether tooling exposes a local, machine-readable `aether.ast/v8` document and
accepts bounded `aether.edit/v8` structural edits. The protocol
uses exact canonical source revisions to reject stale requests, supports typed
top-level record/shape/weave insert, replace, and delete operations, reparses the
formatter-owned result, and seed-compiles it before the CLI writes source to an
explicit output path. It exposes effect annotations, M4/M5/M6/M7 nodes, and a
required `Bind.stage` (`runtime` or `comptime`). V8 adds the explicit
`Weave.task` Boolean and typed `Checkpoint` statement; M23 uses the existing
`Call` expression node. See [docs/AETHER_AUTHORING_PROTOCOL_v8.md](docs/AETHER_AUTHORING_PROTOCOL_v8.md)
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
- M23 supports only one eligible pure Whole helper call per explicit root
  directive. It has no recursion, control flow, nested calls, macro expansion,
  build hooks, Text/Bytes/Truth evaluation, or configurable fuel.
- M19e is limited to checkpointed, single-thread task-frame cancellation. It
  has no task handles, timeouts, manual cancellation, arbitrary preemption,
  external-effect rollback, nested task nurseries, or guest cleanup callbacks.
- Host invocation accepts and returns primitives only; use an Aether weave to
  project a record field.
- Future language extensions require their own seed-emission parity proof before
  they become part of the product compile surface.
- Bootstrap rebuild of the seed is still required after changing the seed source.
- Structural edits are bounded by the v8 path grammar; they are not arbitrary
  JSONPath or a claim of universal syntax-error-proof AI generation.

## North star and evidence-led roadmap

Aether 0.36 is the current executable contract, not the full long-range language
vision. The project is deliberately designing for AI-primary authorship while
keeping deterministic, locally verifiable compiler authority. Read the design
set in this order:

1. [docs/NORTH_STAR.md](docs/NORTH_STAR.md) — intended product direction and
   current-law constraints.
2. [docs/CORE_CLAIMS.md](docs/CORE_CLAIMS.md) — proven facts, accepted
   directions, research hypotheses, and prohibited claims.
3. [docs/AETHER_0.36.md](docs/AETHER_0.36.md),
   [docs/ADR-041-pkg-001-offline-workspace-locks.md](docs/ADR-041-pkg-001-offline-workspace-locks.md),
   [docs/ADR-040-runtime-text-ascii-fast-path.md](docs/ADR-040-runtime-text-ascii-fast-path.md),
   [docs/ADR-039-m23-comptime-pure-calls.md](docs/ADR-039-m23-comptime-pure-calls.md),
   and [docs/ADR-042-m19e-active-frame-cancel.md](docs/ADR-042-m19e-active-frame-cancel.md)
   — current package-integrity, runtime, and language-surface boundaries.
4. [docs/SEED_PROFILE.md](docs/SEED_PROFILE.md) — exact seed-emitted product
   path and bootstrap responsibilities.
5. [docs/PROGRESS_REPORT-FULL-PROJECT.md](docs/PROGRESS_REPORT-FULL-PROJECT.md)
   and [docs/AUDIT_REPORT-2026-08-11-FULL-PROJECT.md](docs/AUDIT_REPORT-2026-08-11-FULL-PROJECT.md)
   — current progress and full project audit (2026-08-11).
6. [docs/ROADMAP.md](docs/ROADMAP.md) — implemented scope and approved
   dependency order for future work.

**Honest law-fork surface (authorized, bounded):** F-NATIVE lowers **verified**
AETH to optional C/object/LLVM/exe (M35a–j; host toolchain required); F-REGISTRY
is host CLI offline pin + explicit signed fetch/CA (M24a–i), with the bounded
X.509-lite store checked as part of `registry verify-cache` and explicit local
Ed25519 root/certified-key setup. Default product
compile remains seed→AETH→VM. These docs do **not** claim general effects,
OS-thread parallel concurrency, broad FFI safety, full RFC 5280 X.509, or a
bundled hermetic native toolchain.

## Workspace

- `crates/xlang-core` — bootstrap parser, typed resource semantic plan,
  emitter/verifier/VM, forge API, seed path
- `apps/xlang-cli` — `aether` CLI (seed compile by default)
- `seed/` — Aether-written compiler source + checked-in artifact
- `examples/` — programs proven behaviorally and, where claimed,
  seed-emitted byte-identical to bootstrap

## Command Line

```powershell
Set-Location C:\WPAI\Software\XLang
cargo run -p aether-cli -- check (Resolve-Path .\examples\comptime-calls.ae)
cargo run -p aether-cli -- structure (Resolve-Path .\examples\comptime-calls.ae)
cargo run -p aether-cli -- compile (Resolve-Path .\examples\comptime-calls.ae) --output .\target\comptime-calls.aeth
cargo run -p aether-cli -- run .\target\comptime-calls.aeth

# Refresh local package integrity locks explicitly; without --write each command
# prints the candidate JSON and leaves disk unchanged.
cargo run -p aether-cli -- project lock .\examples\workspace\util\aether.project.json --write
cargo run -p aether-cli -- project lock .\examples\workspace\app\aether.project.json --write
cargo run -p aether-cli -- workspace lock .\examples\workspace\aether.workspace.json --write
cargo run -p aether-cli -- workspace verify .\examples\workspace\aether.workspace.json

# Rebuild the seed safely after editing seed/aether_seed.ae (product path, ADR-067).
# Dual-compare oracle still uses --bootstrap; promote only when hashes match.
cargo run -p aether-cli -- compile .\seed\aether_seed.ae --output .\target\aether_seed.product.aeth
cargo run -p aether-cli -- compile .\seed\aether_seed.ae --output .\target\aether_seed.bootstrap.aeth --bootstrap
cargo run -p aether-cli -- forge .\target\aether_seed.bootstrap.aeth .\seed\aether_seed.ae --output .\target\aether_seed.forged.aeth
Get-FileHash .\target\aether_seed.product.aeth, .\target\aether_seed.bootstrap.aeth, .\target\aether_seed.forged.aeth
cargo run -p aether-cli -- compile .\seed\aether_seed.ae --output .\seed\aether_seed.aeth
cargo build -p aether-cli
```

## Product interface

The CLI is the shipped Aether product interface. It exposes canonical structure
and validated structural edits locally without introducing an application, model,
or network authority. No Studio workbench or app is part of the current Aether
product surface.

## Local technical-preview package

The supported distribution channel is a local, checksummed preview folder, not
a public release. Build and verify the current package with:

```powershell
pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release
```

This runs the full source/seed gate, builds `aether.exe`, stages the
version-derived `dist\aether-0.36.0-tp` package, verifies its exact hashes and
behavior as a consumer, and proves the verifier rejects an unlisted package
file. It does not commit, tag, push, or publish anything. The package is
`UNLICENSED`; public distribution requires a separate human licensing and
release decision. See [the 0.36 preview notes](docs/RELEASE_NOTES-0.36-TECHNICAL-PREVIEW.md)
and [current threat model](docs/THREAT_MODEL-0.36-TECHNICAL-PREVIEW.md).

## Governance

Product contract: [MANIFEST.md](MANIFEST.md), [docs/AETHER_0.36.md](docs/AETHER_0.36.md),
and the linked ADR/matrix evidence.
