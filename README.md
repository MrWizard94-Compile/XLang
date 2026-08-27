# Aether in XLang

This repository hosts Aether, a local-first language and CLI toolchain for
deterministic, AI-primary authorship. Aether toolchain package **0.37.0**
(language surface **0.11** plus bounded M19e task semantics) parses only Aether
source, emits deterministic AETH bytecode, verifies every artifact,
and runs it in the Aether VM. **Default product path** is seed→AETH→VM; it does
not translate Aether source to Rust, C, JavaScript, or another language as the
product compiler. Optional F-NATIVE lowers **already-verified** AETH to C/object/
LLVM/exe when the operator requests it (host toolchain required); `--target` is
closed to the M35j matrix, with host dual-run and cross link-only behavior. Verified AETH
v4–v10 artifacts remain compatibility inputs with their original meanings.
Source without task frames emits v11; valid M19e task source emits v12.

**Executable contract:** [MANIFEST.md](MANIFEST.md) · **Documentation:**
[docs/README.md](docs/README.md) · **Claims:**
[docs/CORE_CLAIMS.md](docs/Current%20state/CORE_CLAIMS.md) · **Current delta:**
[docs/AETHER_0.37.md](docs/Current%20state/AETHER_0.37.md)

## Executable systems showcase

[Aether Atlas](showcases/aether-atlas/README.md) is a bounded deterministic
offline journal-recovery system written primarily in Aether. It composes
locked packages, source parsing, records, resources, table layouts, nurseries,
task-frame cancellation, explicit host grants, local package publication, and
reproducibility evidence without claiming ambient I/O, networking, crypto, or
general-purpose storage semantics that the current language does not provide.

**Current bounded task-frame behavior:**
[M19e active-frame cancellation](docs/historical%20docs/DESIGN-M19E-T-RX-ACTIVE-FRAME-CANCEL.md) /
[ADR-042](docs/historical%20docs/ADR-042-m19e-active-frame-cancel.md). Package 0.36 accepts
closed-subset `task weave` / `checkpoint` source, emits v12 task frames, and
deterministically cancels parked frames on a later eligible sibling failure.

## Seed-hosted compile path, resources, effects, comptime, and layout

**Default compilation is seed-emitted for user programs**, including M19a
`release`, M19b nursery×resource Policy A, M19d multi-weave arenas, M19e task
frames, M21 foreign pilot declarations, and M23's documented seed-native
raw-source evaluator (with no bootstrap materialization bridge).

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
**0.37** add offline projects/modules/LSP, grant-backed host I/O, a **bounded
foreign weave pilot** (Whole-only, explicit library grant; not sandboxed),
comptime name chaining, resource+handle, `aether test` / `aether project test`
(optional grants and reports), workspaces, stdlib layer 1, cross-package
imports, product-path `release`, nursery×resource Policy A+, multi-weave arenas,
cooperative Policy B bounds, pure bounded comptime helper calls, and M19e's
restricted active-frame cancellation. Package 0.34 adds only the internal
RTP-001 ASCII Text runtime fast path; 0.35 adds PKG-001 optional local workspace
locks; 0.36 adds `task weave`, `checkpoint`, AETH v12, and authoring v8; and
0.37 adds M25 transparent local source-package pack, verify, publish, cache
install, and cache verification without changing source, AETH, seed, VM, or
guest authority.
M19e adds no registry, network authority, guest cancellation API, or host
capability. See
[docs/AETHER_0.11.md](docs/Current%20state/AETHER_0.11.md),
[docs/AETHER_0.37.md](docs/Current%20state/AETHER_0.37.md), and [docs/SEED_PROFILE.md](docs/Current%20state/SEED_PROFILE.md).

ADR-128/129/130 add separate, bounded seed-native source-bundle profiles under
the same `compile_bundle [borrow bundle: Text] -> Bytes` ABI. v1 carries one
pure Whole library and one entry; v2 carries one foundation library, one bridge
library, and one entry in an exact transitive chain; v3 carries two independent
pure Whole leaves, one merge library, and one entry in an exact two-leaf fan-in.
The verified seed owns framing, import matching, name mangling, and call
rewriting; the product route does not call the host module elaborator. This is
**not** general seed-native M11/M22: ordinary project/workspace module graphs
remain host-elaborated then seed-emitted. Use
`aether compile examples\seed-bundle-fanin.aeb --output <artifact.aeth>` or
`aether forge-bundle <compiler.aeth> <bundle.aeb> --output <artifact.aeth>`.
See [ADR-128](docs/historical%20docs/ADR-128-barp-seed-native-whole-library-bundle-profile.md),
[ADR-129](docs/historical%20docs/ADR-129-barp-seed-native-transitive-library-chain-profile.md),
[ADR-130](docs/historical%20docs/ADR-130-barp-seed-native-fanin-bundle-profile.md),
[the SBP validation matrix](docs/Current%20state/SBP-VALIDATION-MATRIX.md), and
[the SBP-003 threat model](docs/Current%20state/THREAT_MODEL-SBP-003-SEED-FANIN.md).

BARP's bounded direct-seed diagnostic pilot now SPEAKs
AE-SEED-003/004/005/006/007/010/011/012/013/014/015. Its task-checkpoint portion
recognizes only canonical top-level task headers and exact indented
`checkpoint` statements; its AE-SEED-010 portion recognizes only a Text literal
or exact `yield bright` / `yield dim` literal returned from a canonical ordinary
`Whole` weave; its AE-SEED-013 portion
recognizes only a nested `yield` below canonical `choose same`, `choose less`,
  exact `choose bright:` / `choose dim:`, or exact unary-literal `choose not
  bright:` / `choose not dim:` in such a weave;
and its AE-SEED-011 portion recognizes canonical direct `bind … <- call`, root
`yield call`, root `revise name <- call`, root `speak call`, or delimiter-bounded
root `handle call` forms whose target has no matching
top-level declaration header (ordinary, export, host, foreign, or task),
including an exact end-of-line zero-argument target. It establishes target
existence only. A separate literal `-> Whole raises Whole:` header state covers
canonical root `forward call` targets; destination, Text result, header,
terminality, effect, and call-kind legality remain with the full compiler.
Neither changes accepted valid source nor claims full diagnostic parity. See
[ADR-110](docs/historical%20docs/ADR-110-barp-seed-speak-unknown-call-pilot.md) and
[ADR-111](docs/historical%20docs/ADR-111-barp-seed-speak-root-yield-unknown-call-pilot.md),
[ADR-112](docs/historical%20docs/ADR-112-barp-seed-speak-less-choose-yield-pilot.md), and
[ADR-113](docs/historical%20docs/ADR-113-barp-seed-speak-literal-truth-choose-yield-pilot.md), and
[ADR-114](docs/historical%20docs/ADR-114-barp-seed-speak-unary-literal-truth-choose-yield-pilot.md), and
[ADR-115](docs/historical%20docs/ADR-115-barp-seed-speak-whole-truth-yield-pilot.md).
[ADR-116](docs/historical%20docs/ADR-116-barp-seed-speak-zero-argument-unknown-call-pilot.md)
adds exact end-of-line zero-argument direct-call target witnesses.
[ADR-117](docs/historical%20docs/ADR-117-barp-seed-speak-revise-unknown-call-pilot.md)
adds canonical root revise-call target witnesses with the same bounded tail rules.
[ADR-118](docs/historical%20docs/ADR-118-barp-seed-speak-root-speak-unknown-call-pilot.md)
adds canonical root speak-call target witnesses while leaving Text-result legality
to the full compiler.
[ADR-119](docs/historical%20docs/ADR-119-barp-seed-speak-root-handle-unknown-call-pilot.md)
adds canonical root handle-call target
witnesses only when the required `into` and `otherwise error into` delimiters
are present; complete M4 legality remains with the full compiler.
[ADR-120](docs/historical%20docs/ADR-120-barp-seed-speak-root-forward-unknown-call-pilot.md)
adds canonical root forward-call target
witnesses only under a literal erroring-Whole header marker; complete M4 header,
terminality, target-effect, and call legality remain with the full compiler.
[ADR-121](docs/historical%20docs/ADR-121-barp-seed-speak-root-nursery-zero-argument-spawn-unknown-call-pilot.md)
adds a literal ordinary total-Whole root
nursery witness only for an immediate zero-argument
`spawn call target into destination` child; M7 task, destination, nesting,
resource, and scheduler legality remain with the full compiler.
[ADR-122](docs/historical%20docs/ADR-122-barp-seed-speak-root-nursery-single-digit-whole-spawn-unknown-call-pilot.md)
adds only the adjacent one-digit decimal-Whole child form
`spawn call target digit into destination`; general arguments, task identity,
destination, nesting, resource, and scheduler legality remain with the full
compiler.
[ADR-123](docs/historical%20docs/ADR-123-barp-seed-speak-root-nursery-two-digit-positive-whole-spawn-unknown-call-pilot.md)
adds only the adjacent positive two-digit decimal-Whole child form
`spawn call target digits into destination` for `10` through `99`; complete
Whole syntax/range handling, general arguments, task identity, destination,
nesting, resource, and scheduler legality remain with the full compiler.
[ADR-124](docs/historical%20docs/ADR-124-barp-seed-speak-root-nursery-bright-truth-spawn-unknown-call-pilot.md)
adds only the adjacent exact `Truth`-literal child form
`spawn call target bright into destination`; the separate exact-`dim` scope is
ADR-125, while names, general expressions, task identity, destination, nesting,
resource, and scheduler legality remain with the full compiler.
[ADR-125](docs/historical%20docs/ADR-125-barp-seed-speak-root-nursery-dim-truth-spawn-unknown-call-pilot.md)
adds only the adjacent exact `Truth`-literal child form
`spawn call target dim into destination`; `bright` remains ADR-124, while names,
general expressions, task identity, destination, nesting, resource, and
scheduler legality remain with the full compiler. Exact empty `Text` is a
separate ADR-126 witness.
[ADR-126](docs/historical%20docs/ADR-126-barp-seed-speak-root-nursery-empty-text-spawn-unknown-call-pilot.md)
adds only the adjacent exact empty-`Text` child form
`spawn call target "" into destination`; it uses an ordinary `weave` with a
`Text` parameter only for valid-source evidence and does not add Text
task-frame support. Nonempty or escaped Text, Bytes, names, task identity,
destination, nesting, resource, and scheduler legality remain with the full
compiler.

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
seed-native under its documented D2a subset with no bootstrap materialization
bridge, as documented in [docs/AETHER_0.33.md](docs/historical%20docs/AETHER_0.33.md).

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
[docs/DESIGN-M6-EXPLICIT-LAYOUT-SHAPES.md](docs/historical%20docs/DESIGN-M6-EXPLICIT-LAYOUT-SHAPES.md).

## Versioned structural authoring contract

Aether tooling exposes a local, machine-readable `aether.ast/v8` document and
accepts bounded `aether.edit/v8` structural edits. The protocol
uses exact canonical source revisions to reject stale requests, supports typed
top-level record/shape/weave insert, replace, and delete operations, reparses the
formatter-owned result, and seed-compiles it before the CLI writes source to an
explicit output path. It exposes effect annotations, M4/M5/M6/M7 nodes, and a
required `Bind.stage` (`runtime` or `comptime`). V8 adds the explicit
`Weave.task` Boolean and typed `Checkpoint` statement; M23 uses the existing
`Call` expression node. See [docs/AETHER_AUTHORING_PROTOCOL_v8.md](docs/Current%20state/AETHER_AUTHORING_PROTOCOL_v8.md)
and [docs/ADR-005-structural-authoring-contract.md](docs/historical%20docs/ADR-005-structural-authoring-contract.md).

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

Aether 0.37 is the current executable contract, not the full long-range language
vision. The project is deliberately designing for AI-primary authorship while
keeping deterministic, locally verifiable compiler authority. Read the design
set in this order:

1. [docs/NORTH_STAR.md](docs/Current%20state/NORTH_STAR.md) — intended product direction and
   current-law constraints.
2. [docs/CORE_CLAIMS.md](docs/Current%20state/CORE_CLAIMS.md) — proven facts, accepted
   directions, research hypotheses, and prohibited claims.
3. [docs/AETHER_0.37.md](docs/Current%20state/AETHER_0.37.md),
   [docs/ADR-041-pkg-001-offline-workspace-locks.md](docs/historical%20docs/ADR-041-pkg-001-offline-workspace-locks.md),
   [docs/ADR-040-runtime-text-ascii-fast-path.md](docs/historical%20docs/ADR-040-runtime-text-ascii-fast-path.md),
   [docs/ADR-039-m23-comptime-pure-calls.md](docs/historical%20docs/ADR-039-m23-comptime-pure-calls.md),
   and [docs/ADR-042-m19e-active-frame-cancel.md](docs/historical%20docs/ADR-042-m19e-active-frame-cancel.md)
   — current package-integrity, runtime, and language-surface boundaries.
4. [docs/SEED_PROFILE.md](docs/Current%20state/SEED_PROFILE.md) — exact seed-emitted product
   path and bootstrap responsibilities.
5. [docs/PROGRESS_REPORT-FULL-PROJECT.md](docs/Current%20state/PROGRESS_REPORT-FULL-PROJECT.md)
   and [docs/AUDIT_REPORT-2026-08-11-FULL-PROJECT.md](docs/historical%20docs/AUDIT_REPORT-2026-08-11-FULL-PROJECT.md)
   — current progress and full project audit (2026-08-11).
6. [docs/ROADMAP.md](docs/Current%20state/ROADMAP.md) — implemented scope and approved
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

# M32a/M32b: local verified-execution evidence over a fixed, embedded pure corpus.
# Compilation is excluded; each sample includes AETH verification, decode, and VM execution.
cargo run -p aether-cli -- bench --list
# --profile is a non-secret, lowercase local context label. It emits report v2.
cargo run -p aether-cli -- bench all --warmup 3 --iterations 11 --profile win11-rust-1.88-release-fixed --report .\target\aether-bench-baseline.json
# After a separately verified candidate build under the same declared profile:
cargo run -p aether-cli -- bench all --warmup 3 --iterations 11 --profile win11-rust-1.88-release-fixed --report .\target\aether-bench-candidate.json
cargo run -p aether-cli -- bench compare .\target\aether-bench-baseline.json .\target\aether-bench-candidate.json --report .\target\aether-bench-comparison.json

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

### M25 local source packages

M25 packages only a complete locked project manifest and its declared Aether
units. It is an explicit local workflow, not a dependency resolver or remote
registry:

```powershell
cargo run -p aether-cli -- pkg pack .\examples\package-publish\source\aether.project.json --output .\target\local-math.bundle
cargo run -p aether-cli -- pkg verify .\target\local-math.bundle
cargo run -p aether-cli -- pkg publish .\target\local-math.bundle --cache .\target\aether-package-cache
cargo run -p aether-cli -- pkg install --cache .\target\aether-package-cache --name local_math --version 1.0.0 --output .\target\workspace\local_math
cargo run -p aether-cli -- project verify .\target\workspace\local_math\aether.project.json
```

The installed directory is a normal locked project. Add it deliberately to an
`aether.workspace/v1` file and declare existing M22 `depends_on` authorization
before other packages import it. Bundle/cache inputs are hostile-file checked;
they are not signed or remotely authenticated. See [the 0.37 contract](docs/Current%20state/AETHER_0.37.md)
and [its threat model](docs/Current%20state/THREAT_MODEL-0.37-LOCAL-PACKAGES.md).

## Product interface

The CLI is the shipped Aether product interface. It exposes canonical structure
and validated structural edits locally without introducing an application, model,
or network authority. No Studio workbench or app is part of the current Aether
product surface. M32a adds `aether bench`: an offline measurement command for
only the embedded `welcome`, `arena-buffer`, and `task-loop` workloads. It
seed-compiles each selected source once, explicitly verifies its AETH artifact,
then samples `verify + decode + execute` with empty grants. It accepts no
caller-supplied source or artifact. M32b preserves the unprofiled v1 report and
adds opt-in `--profile` v2 reports with a safe environment fingerprint and
hashed observable stdout. `aether bench compare` reads only two explicit,
bounded v2 JSON reports, rejects differing profile, environment, workload, or
observable behavior, and never executes report data. The comparison is a local
methodology record—not a timing threshold, optimization claim, toolchain
attestation, or cross-machine result. Reports write only to explicit paths after
success. See [ADR-104](docs/historical%20docs/ADR-104-m32a-verified-execution-benchmarks.md) and
[ADR-105](docs/historical%20docs/ADR-105-m32b-profile-bound-comparisons.md).

## Local technical-preview package

The supported distribution channel is a local, checksummed preview folder, not
a public release. Build and verify the current package with:

```powershell
pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release
```

This runs the full source/seed gate, builds `aether.exe`, stages the
version-derived `dist\aether-0.37.0-tp` package, verifies its exact hashes and
behavior as a consumer, and proves the verifier rejects an unlisted package
file. It does not commit, tag, push, or publish anything. The package is
`UNLICENSED`; public distribution requires a separate human licensing and
release decision. See [the 0.37 local-package notes](docs/Current%20state/RELEASE_NOTES-0.37-LOCAL-PACKAGES.md)
and [current local-package threat model](docs/Current%20state/THREAT_MODEL-0.37-LOCAL-PACKAGES.md).

## Governance

Product contract: [MANIFEST.md](MANIFEST.md), [docs/AETHER_0.37.md](docs/Current%20state/AETHER_0.37.md),
and the linked ADR/matrix evidence.
