# Aether Production Manifest

## Contract

Aether **0.36.0** (base language **0.11** plus M11–M18, M19a–M19e, M20/M20b,
M17b–M17d, M21 pilot, M22 tooling and semantics, and M23 pure comptime calls;
AETH **v11** for source without task frames and AETH **v12** for M19e task
frames with `RELEASE` 66 and `TASK_CHECKPOINT` 67)
accepts Aether source, returns a canonical AST from the Rust bootstrap for
tooling, and **emits deterministic AETH bytecode primarily through the
Aether-written seed compiler** (forge ABI) for the documented seed surface,
including multi-weave arenas / resourceful total spawn callees
(`examples/spawn-arena.ae`), M19e task frames, and the M21 foreign weave pilot.
Multi-module and workspace build elaborate then seed-compile.
For M23 source, the bootstrap validates the restricted pure call; the seed
interprets raw `comptime bind <- call` under the D2a body subset and emits
`COMPTIME_WHOLE` without a materialization rewrite. The product artifact is
byte-identical to direct bootstrap output (`examples/comptime-calls.ae`).
Package 0.34 adds RTP-001, a private cached-ASCII VM Text representation that
accelerates scalar-equivalent operations for ASCII input. It changes no source
syntax, AETH version or bytes, verifier rule, seed artifact, host authority, or
authoring protocol.
Package 0.35 adds PKG-001 offline workspace locks: explicit `project lock` and
`workspace lock` refresh commands, optional complete local package identity
pins, project-manifest confinement, and locked-workspace build preflight. The
increment changes no guest source form, AETH artifact, seed behavior, VM
semantics, host capability, registry boundary, or network authority. See
[docs/AETHER_0.35.md](docs/AETHER_0.35.md).
Package 0.36 adds M19e bounded active-frame cancellation: explicit `task weave`
and `checkpoint` forms, v12 task descriptors, deterministic source-order
round-robin task frames, verifier-proven checkpoint loop edges, private
pre-admitted task lanes, and VM-internal reverse-slot teardown/zeroization on a
later eligible companion failure. It preserves v11 for source without task
frames and adds no host effect, task handle, timeout, manual cancellation, or
parallel execution. See [docs/AETHER_0.36.md](docs/AETHER_0.36.md).
Authoring uses `aether.ast/v8`, `aether.edit/v8`, and
`aether.diagnostic/v8`. The CLI provides offline
project/workspace verify, `aether test` / `aether project test` (optional
grants/reports), `aether lsp`, and `aether run` with optional `--grant-*`
including M21 `--grant-lib KEY=PATH`. Stdlib layer 1 is under `stdlib/`.
Verified AETH v4–v11 remain compatibility inputs. M21 foreign pilot is Whole-only
under explicit library grant; native code is **not** sandboxed. Cooperative
Policy B (unstarted cancel / return-end owners) remains the v11 compatibility
behavior. M19e's [active-frame cancellation design](docs/DESIGN-M19E-T-RX-ACTIVE-FRAME-CANCEL.md)
and [ADR-042](docs/ADR-042-m19e-active-frame-cancel.md) define the implemented,
strictly checkpointed v12 successor.

## Scope Boundary

This manifest is the executable Aether 0.36 product contract (0.11 core language,
M9–M23/M19a–M19e/M20/M20b/M17b–M17d/M21 pilot/M22 tooling plus RTP-001 and
PKG-001 with bounded task-frame semantics and honest seed limits). It intentionally does not promote long-range research directions to
implemented behavior. The
AI-first systems-language direction, evidence policy, and staged dependencies
are [docs/NORTH_STAR.md](docs/NORTH_STAR.md),
[docs/CORE_CLAIMS.md](docs/CORE_CLAIMS.md), and [docs/ROADMAP.md](docs/ROADMAP.md).
The accepted M1 direction is
[docs/DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md](docs/DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md).
The bounded executable M2 decision is
[docs/ADR-004-aeth-v6-bounded-resources.md](docs/ADR-004-aeth-v6-bounded-resources.md).
General generics, C-header interop, libloading, ambient guest I/O, and a native
backend are not Aether 0.11 surface area. M4 implements the bounded, testable
`Error[Whole]` capability in
[docs/DESIGN-M4-TYPED-ERROR-EFFECTS.md](docs/DESIGN-M4-TYPED-ERROR-EFFECTS.md)
and [docs/AETHER_0.7.md](docs/AETHER_0.7.md). M5 implements bounded explicit
literal compile-time `Whole` evaluation in
[docs/DESIGN-M5-DETERMINISTIC-COMPTIME.md](docs/DESIGN-M5-DETERMINISTIC-COMPTIME.md)
and [docs/AETHER_0.8.md](docs/AETHER_0.8.md). M6 implements explicit layout
shapes and dual-layout tables in
[docs/DESIGN-M6-EXPLICIT-LAYOUT-SHAPES.md](docs/DESIGN-M6-EXPLICIT-LAYOUT-SHAPES.md)
and [docs/AETHER_0.9.md](docs/AETHER_0.9.md). M7 implements structured nurseries
in [docs/AETHER_0.10.md](docs/AETHER_0.10.md). M8 implements the capability-closed
pure host ABI pilot in [docs/AETHER_0.11.md](docs/AETHER_0.11.md). M9 adds
offline project metadata and formatting in
[docs/AETHER_0.12.md](docs/AETHER_0.12.md). M10 multi-unit nested project tooling
is in [docs/AETHER_0.13.md](docs/AETHER_0.13.md). Structural authoring is local
tooling metadata, not a host-capability expansion; its current contract is
[docs/AETHER_AUTHORING_PROTOCOL_v8.md](docs/AETHER_AUTHORING_PROTOCOL_v8.md).

## Implemented Language Boundary

Aether 0.11 language surface (package 0.13) includes the 0.4/0.5 scalar and byte surface plus immutable nominal
records declared after `world` and before weaves. Record fields are bounded to
the primitive `Text`, `Whole`, `Truth`, and `Bytes` types; records cannot nest.
`make` constructs in declaration order and `field borrow` projects a cloned
immutable field. Records are unique values with explicit `borrow`/`move`, may
cross internal weave calls, and remain outside the primitive-only host invoke
ABI. Root bindings receive fixed slots; nested blocks may revise but cannot
introduce bindings.

The bounded M2 surface adds positive `arena N` root bindings (originally
main-only; **M19d / package 0.32** allows one arena per **total** weave),
`buffer Whole` / `buffer Truth` owner placeholders, and operation-scoped
`access`. AETH header capacity is the **sum** of all arena declarations
(≤ 1_000_000). `allocate`, resource `append`, and `at` appear only as terminal
closed `choose` conditions with explicit bright/dim branches. Allocation and
append restore the same moved mutable Buffer owner; lookup updates its mutable
copy destination only when bright. Resource owners cannot be revised or weave
results, and no resource value crosses the primitive host or forge ABI.

In an M19e task-bearing program, the v12 frame plan narrows direct resource
ownership to `main` and `task weave` only, with one direct arena per task. Each
task lane is statically private and can be reclaimed only as its full nursery
slab after join.

Bounded facilities include `encode`, `decode`, `extent`, `octet`, `slice`,
`fuse`, `append`, `seek`, `number`, `pack16`, `pack32`, `pack64`, `unpack16`,
`unpack32`, `poke`, and `poke32`.

Source is UTF-8. Names and keywords are lowercase ASCII. Canonical formatting
uses LF, exact two-space indentation, no tabs, and no trailing whitespace. CRLF
input is accepted by bootstrap formatters and by the seed line scanner; a valid
final source line need not end in a terminal LF.

M4 adds one explicit abortive `Error[Whole]` effect. A non-`main` weave may
write `raises Whole` only when it returns `Whole` and accepts ordinary owned
`Whole`/`Truth` copy parameters. `raise`, `forward call`, and one-line terminal
`handle call ... into success otherwise error into code` are the only M4 control
forms. The handler writes and yields its selected distinct mutable root `Whole`
destination. An ordinary call cannot invoke an erroring weave; M4 control cannot
cross live owners, loans, arenas, buffers, or M2 outcomes.

M5 adds root-only immutable `comptime bind` for exactly one `sum`,
`difference`, `product`, `quotient`, or `remainder` operation over literal
`Whole` operands. Every
directive has checked signed-64-bit arithmetic and one fixed evaluation unit;
one source program may contain at most 1,024 directives. M5 itself accepts no
names, calls, loops, text, bytes, records, resources, effects, macros, generated
source, host I/O, or user-adjustable budget. Its result remains an ordinary
immutable `Whole` local.

M6 adds root-level `shape` declarations of 1–8 `Whole` fields and
`table Shape layout rows|columns` arena-backed owners with closed
`allocate` / `store` / `load` outcomes. Capacity is `1..=1024`. Physical layout
is author-selected only; rows and columns share one logical model. Tables cannot
be weave results or host/forge values.

M7 adds lexical `together:` nurseries with `spawn call ... into` lines
(1 through 8). Execution is cooperative and source-ordered. First child
`Error[Whole]` cancels remaining unstarted spawns and re-raises. Nursery
weaves keep the M4 clean resource boundary; `main` remains total.

M8 adds body-less `host weave` declarations for pure host services. Host
parameters are only owned/copy `Whole`/`Truth` or `borrow Text`/`borrow Bytes`;
results are only `Whole`/`Truth`/`Text`/`Bytes`. Ordinary `call` of a host weave
emits `HOST_CALL` (65). The product pure fixture installs only `whole_inc` and
`text_extent`; missing services fail closed.

M14 (package 0.19) extends the host catalog with grant-backed `read_text` /
`read_bytes` / `write_text` / `write_bytes` / `env_get` under threat model v2.
Services install only when the operator passes matching `--grant-*` roots or
env names to `aether run` (or test host APIs). Guest paths are relative and
jailed under grant roots after canonicalize. There is no ambient guest
file/process/network/shell authority and no shell/network host weaves.

M15 (package 0.20) expands M5 `comptime bind` so operands may be prior
root-level immutable comptime Whole names (source order) as well as Whole
literals. One binary op per directive, 1,024-directive budget, pure evaluation,
and `COMPTIME_WHOLE` emission are unchanged. Forward refs, runtime names,
control flow, and host observation remain rejected; calls remain rejected
except for the later M23 subset below.

M23 (package 0.33; BARP Phase 1 seed-native) admits
`comptime bind name <- call weave args...` only for a textually prior total
**guest** weave with owned `Whole` parameters/result and a restricted
Whole-only `bind`/`revise`/terminal-`yield` body. Arguments are Whole literals
or earlier same-weave comptime names. Bootstrap validates; the seed evaluates
the D2a callee body without host, foreign, resource, effect, nursery,
control-flow, or nested-call authority, and folds to existing `COMPTIME_WHOLE`
(56). Product path forges original source and dual-compares with bootstrap. See
[docs/AETHER_0.33.md](docs/AETHER_0.33.md), [ADR-039](docs/ADR-039-m23-comptime-pure-calls.md),
and [ADR-043](docs/ADR-043-bootstrap-authority-reduction.md).

RTP-001 (package 0.34) is an implementation-only VM change. Runtime `Text`
stores cached ASCII provenance; ASCII `measure`, `glyph`, `cut`, and `seek` use
byte positions that are exactly scalar positions, while non-ASCII values retain
Unicode scalar traversal. It has no guest-visible form or artifact change. See
[docs/AETHER_0.34.md](docs/AETHER_0.34.md) and
[ADR-040](docs/ADR-040-runtime-text-ascii-fast-path.md).

M16 (package 0.21) allows a **total** weave to own M2/M6 resources and use
terminal `handle call` against a resource-free copy-only `Error[Whole]` callee.
Live Arena/Buffer/table/Text/Bytes/record owners may span the handle; exclusive
`access` loans may not. Abortive `raise`/`forward` and nurseries remain
incompatible with resource ownership in the same weave.

M17 (package 0.22) adds offline `aether test [path...]`: discover `*_test.ae`
under directories (or run explicit `.ae` files), seed-compile, pure-run without
grants, pass when `main` yields Whole **0**. Empty discovery fails closed.

M18 (package 0.23) adds offline `aether.workspace/v1` multi-package graphs:
named local package directories under the workspace root, optional acyclic
`depends_on`, and `aether workspace verify` that nested-verifies each
`aether.project.json`. No network registry.

PKG-001 (package 0.35) adds optional complete `lock.packages` to the existing
workspace document. Each entry binds the declared package name/path, parsed
project name/version, and raw SHA-256 of that package's `aether.project.json`.
A locked workspace requires each nested project to carry a complete existing
unit lock; verification therefore binds both project metadata and every locked
unit byte. `aether project lock` and `aether workspace lock` print refreshed
local JSON unless explicit `--write` is supplied. Locked workspace builds
verify before artifact output. No registry, URLs, version solver, remote cache,
or package publication path is implied.

M20 (package 0.24) ships stdlib layer 0 under `stdlib/` (pure Whole helpers).
M20b (package 0.27) expands layer 1: more Whole helpers plus pure `truth.ae` and
`text.ae` modules; demo multi-import still exits 42.

M17b (package 0.28) adds project unit `role: test` and CLI `aether project test`
to elaborate each test entry against project libs (M11b dual-compare) and pure-run
with exit 0.

M17c (package 0.29) allows optional `--grant-read`/`--grant-write`/`--grant-env`
on `aether test` and `aether project test` (default empty = pure).

M17d (package 0.30) adds optional `--report` (JSON `aether.test-report/v1`) and
`--report-junit` (offline JUnit-compatible XML) on both test runners.

M21 (package 0.31) adds `foreign weave` (Whole-only pilot) with host-side
libloading after `--grant-lib KEY=PATH`. Human residual-risk acceptance recorded;
seed-hosted product compile dual-compares to bootstrap for the foreign-pilot
corpus.

M19d (package 0.32) admits multi-weave total arenas (header capacity = sum) and
Policy A+ resourceful total spawn callees; cooperative Policy B is claimed only
for unstarted-cancel / return-end owner end (ADR-035/036).

M19e (package 0.36) adds only the closed active-frame cancellation subset:
non-main total `task weave` functions returning `Whole`, verifier-approved
`checkpoint` safe points, and checkpointed nurseries with task or Copy-only
companion children. A started task may park at a checkpoint and be cancelled by
the first later eligible companion failure. The VM destroys live task locals in
reverse slot order and zeroes the private lane; completed values remain and
pending/cancelled destinations remain unchanged. The AETH v12 header is main
direct capacity plus the largest checkpointed-nursery task-lane sum. No task
handle, timeout, manual cancellation, nested task nursery, external effect,
arbitrary preemption, or individual allocator free is admitted.

M22 (package 0.24) adds `import unit "path" from package name as alias` and
`aether workspace build --package` for depends_on-authorized cross-package lib
imports.

M19a (package 0.25) adds root `release <name>` and AETH `RELEASE` (66) on the
seed-hosted product path so abortive raise may follow cleanup. Seed≡bootstrap
for `examples/release-raise.ae` is proven.

M19b (package 0.26) admits parent M2/M6 resource ownership with structured
nurseries when spawn callees are resource-free (Policy A). Seed≡bootstrap for
`examples/nursery-resource.ae` is proven.


Every non-task 0.11 compilation emits AETH v11 with an arena-capacity header field, a
possibly empty bounded record table, a shape table, a function `effect_tag`, and
a function host-kind byte. v11 preserves prior verified forms and adds host
function entries plus `HOST_CALL` (65). AETH v4 through v11 remain accepted with
their original bytes and meanings; earlier and unknown versions are rejected. A
task-bearing source program emits AETH v12 with the task metadata and
`TASK_CHECKPOINT` rules above.

## Structural authoring boundary

`aether.ast/v8` describes successfully parsed Aether 0.11+module source after
formatter canonicalization. `aether.edit/v8` accepts a matching complete
canonical `baseSource` and bounded typed operations: top-level declaration
`replace`/`insertAfter`/`delete`, plus statement-level
`replaceStatement`/`insertStatementAt`/`insertStatementAfter`/`deleteStatement`
on weave body paths. The bootstrap revalidates the formatter-owned result; the
CLI seed-compiles it before writing source to the requested output path. V8
requires `Weave.task` and permits the typed `Checkpoint` node, while semantic
validation still enforces the full M19e closed subset. It
cannot execute code, write a file by itself, contact an AI/model service, change
artifact bytes, or grant a guest capability.

Stable `aether.diagnostic/v8` code/span envelopes make source and edit failures
machine-readable, including `AE-EFFECT-001` through `AE-EFFECT-004`,
`AE-COMPTIME-001` through `AE-COMPTIME-003`, `AE-LAYOUT-001` through
`AE-LAYOUT-003`, `AE-TASK-001` through `AE-TASK-005`, and `AE-RESOURCE-004`.
Every `Bind` node must
state `stage: "runtime"` or `stage: "comptime"`. The exact schemas, limits,
operation vocabulary, and compatibility policy are in
[docs/AETHER_AUTHORING_PROTOCOL_v8.md](docs/AETHER_AUTHORING_PROTOCOL_v8.md).

## Compile Path Boundary

| Path | Role |
|------|------|
| **Seed (default)** | Default CLI `compile` / `check` / `format` / `structure` / `project format` + LSP diagnostics/format/symbols — product seed path (ADR-064) |
| **Seed product API** | `compile_with_seed` / `product_diagnostics` / `product_surface_symbols` — never bootstrap |
| **Multi-module** | Host elaborate + seed emit (ADR-056); seed-native multi-file = false |
| **Bootstrap (recovery/oracle)** | `compile --bootstrap` seed rebuild; `check|format|structure --bootstrap` AST recovery; structural-edit base AST; LSP hover/definition; dual-compare tests/gate |
| **F-NATIVE M35a/b** | `compile --native-c` — verified AETH → ISO C pure Whole + locals/arithmetic (ADR-059/062); VM remains default |
| **F-REGISTRY M24a** | `registry pin-local` / `verify-cache` — offline digest pins only (ADR-060); no network |
| **Forge** | Host ABI only: `compile [borrow source: Text] -> Bytes` |

The seed artifact is checked in at `seed/aether_seed.aeth` and embedded as
`SEED_COMPILER_ARTIFACT` for offline deterministic product builds.

## Seed-Profile Self Hosting

`seed/aether_seed.ae` parses the complete documented canonical Aether **0.11**
language surface (package 0.13 tooling): all statement and shallow expression
forms, named locals/params, `borrow`/`move`/`access`, multi-weave `call`
including forward callees, hex bytes literals, UTF-8 text constants with the
five defined escapes, and LF/CRLF input with or without a final line terminator.
It also emits the bounded immutable record declaration, constructor, and
projection surface plus `arena`, Whole/Truth buffers, closed resource outcomes,
shape/table dual-layout emission, the bounded M4 effect metadata/instructions,
M5 literal `comptime bind` / `COMPTIME_WHOLE`, M7 nursery forms, and M8
`host weave` / `HOST_CALL` emission through ordinary `Bytes` operations with no
host parser callback. Offline project documents (`aether.project/v1`) are host
CLI tooling (M9), not seed surface.

Proofs in `crates/xlang-core/tests/seed_self_host.rs`:

1. Multi-generation self-host identity of the seed
2. Distinct source variant yields a different artifact
3. Multi-weave, forward-call, and CRLF fixtures match bootstrap
4. **All shipped `examples/*.ae` seed-compile byte-identically to bootstrap**
5. A complete prior canonical-surface corpus covering every statement,
   expression, ownership mode, literal mode, and accepted line termination
   matches bootstrap
6. The six documented M2 arena/buffer examples match bootstrap byte-for-byte,
   verify, and run with their expected outcomes
7. The handled-error and normal-erroring M4 fixtures match bootstrap byte-for-
   byte, verify, and run with their expected outcomes
8. The shipped M5 comptime fixture matches bootstrap byte-for-byte, verifies,
   and runs with its expected outcome
9. The shipped M6 layout-table fixture matches bootstrap byte-for-byte, verifies,
   and runs with its expected outcome
10. The shipped M7 nursery fixtures match bootstrap byte-for-byte, verify, and
   run with their expected outcomes
11. The shipped M8 `host-pilot` fixture matches bootstrap byte-for-byte, verifies,
    and runs with its expected pure-host outcome
12. Offline M9 project verify/format is exercised by core/CLI tests on the
    shipped `examples/project` fixture (host tooling, not seed emission)

This is full canonical Aether **0.11** source-emission parity for the documented
language surface, self-hosting of the seed, and seed-hosted compilation of the
shipped example corpus, plus offline M9 project integrity on the host CLI. It is
**not** a claim of full invalid-source diagnostic parity or of parity for future
language features without the same proof.

## Product Interface Boundary

The CLI is the only active product interface. Aether has no active desktop,
WebView, model, or network integration. Its AI-first design is expressed through
the versioned structural contracts and deterministic diagnostics, not by giving
an AI service compilation or persistence authority. The retired workbench is
recorded in [docs/ADR-006-retire-aether-studio.md](docs/ADR-006-retire-aether-studio.md).

## Project tooling boundary (M9 / M10)

Offline project documents use `aether.project/v1` with relative `.ae` units
(including nested forward-slash paths) and optional SHA-256 locks. M11a adds
`import unit` / `export weave` and:

```text
aether format <source-file> [--output <source-file>]
aether project verify <project-file> [--output-dir <dir>]
aether project format <project-file> [--write]
aether project build <project-file> --output <artifact.aeth>
```

`project build` elaborates the main unit’s import DAG, dual-compares
bootstrap≡seed AETH bytes, and writes the seed artifact. Single-file `compile`
remains seed-hosted and rejects raw `import unit` (use project build). See
[docs/DESIGN-M11-LANGUAGE-MODULES.md](docs/DESIGN-M11-LANGUAGE-MODULES.md),
[docs/DESIGN-M9-PROJECT-TOOLING.md](docs/DESIGN-M9-PROJECT-TOOLING.md), and
[docs/DESIGN-M10-MULTI-UNIT-PROJECTS.md](docs/DESIGN-M10-MULTI-UNIT-PROJECTS.md).

## Quality Gate

Preferred offline entry (when present): `pwsh -File tools/aether-gate.ps1`
(`-Mode quick` for day-to-day; `-Mode full` for full source and seed proof;
`-Mode release` before a local technical-preview package, including release
build, staging, consumer verification, and a negative integrity probe).

Manual equivalent: pack `verify-pack.ps1`, Rust format, Clippy `-D warnings`,
core/CLI/seed tests, CLI seed-compile of examples (dual-compare), forge
self-host hash check, and `aether project verify` on the shipped project
fixture. A technical-preview package also requires a release build, forge
identity, inspection and launch of the staged release binary, exact
SHA-256SUMS, current contract/release/threat-model documentation, and
successful consumer verification on the target Windows system. `-Mode release`
automates those local checks; it neither commits nor publishes the package.

For a scoped self-host regression measurement, run
`pwsh -NoProfile -File tools/measure-seed-self-host.ps1`. Its local timing
distribution supplements but never replaces the functional quality gate.

Local technical preview helpers derive the package version from the CLI manifest:

```powershell
pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release
# Or stage and verify explicitly:
pwsh -NoProfile -File .\tools\package-preview.ps1
pwsh -NoProfile -File .\dist\aether-0.36.0-tp\verify-preview.ps1
```

Current preview threat model: [docs/THREAT_MODEL-0.36-TECHNICAL-PREVIEW.md](docs/THREAT_MODEL-0.36-TECHNICAL-PREVIEW.md).

Historical 0.12 pure-surface freeze: [docs/THREAT_MODEL-TECHNICAL-PREVIEW.md](docs/THREAT_MODEL-TECHNICAL-PREVIEW.md).
Capable host I/O threat model (M14): [docs/THREAT_MODEL-v2-CAPABLE-HOST.md](docs/THREAT_MODEL-v2-CAPABLE-HOST.md).  
TP delivery: [docs/DELIVERY_REPORT-2026-08-04-TECHNICAL-PREVIEW.md](docs/DELIVERY_REPORT-2026-08-04-TECHNICAL-PREVIEW.md).  
M10 delivery: [docs/DELIVERY_REPORT-2026-08-04-M10-MULTI-UNIT-PROJECTS.md](docs/DELIVERY_REPORT-2026-08-04-M10-MULTI-UNIT-PROJECTS.md).  
M14 delivery: [docs/DELIVERY_REPORT-2026-08-04-M14-HOST-IO.md](docs/DELIVERY_REPORT-2026-08-04-M14-HOST-IO.md).
