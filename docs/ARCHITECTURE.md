# Architecture

~~~mermaid
flowchart LR
    CLI["aether CLI"] -->|"compile"| SeedPath["compile_with_seed"]
    CLI -->|"structure / apply-edit"| Authoring["aether.ast/v8 + aether.edit/v8"]
    Authoring -->|"canonical validated source"| SeedPath
    SeedPath --> Bridge["bootstrap validation; M23 literal materialization only when required"]
    Bridge --> SeedArt["embedded seed AETH v11 compiler"]
    SeedArt --> Artifact["Verified AETH v11 or v12 artifact"]
    Artifact --> VM["Aether VM"]
    VM --> Result["stdout and exit value"]
    Check["aether check"] --> Bootstrap["Rust bootstrap AST"]
    SeedSrc["seed/aether_seed.ae"] -->|"compile (product ADR-067)"| SeedArtFile["seed/aether_seed.aeth"]
    Forge["aether forge"] -->|"verify + invoke"| SeedArt
    SeedArt --> Candidate["candidate AETH Bytes"]
    Candidate -->|"verify before write"| Forge
~~~

## Scope and product boundary

This document describes the implemented **Aether 0.37.0** architecture: the
canonical 0.11 source forms plus the later bounded product semantics recorded
in [MANIFEST.md](../MANIFEST.md). The current M19e task-frame increment is
specified in [AETHER_0.36.md](AETHER_0.36.md),
[DESIGN-M19E-T-RX-ACTIVE-FRAME-CANCEL.md](DESIGN-M19E-T-RX-ACTIVE-FRAME-CANCEL.md),
and [ADR-042](ADR-042-m19e-active-frame-cancel.md). The PKG-001 workspace-integrity
increment is specified historically in [AETHER_0.35.md](AETHER_0.35.md) and
[ADR-041](ADR-041-pkg-001-offline-workspace-locks.md). RTP-001 remains
specified in [AETHER_0.34.md](AETHER_0.34.md) and
[ADR-040](ADR-040-runtime-text-ascii-fast-path.md); the M23 language addition
remains specified in [AETHER_0.33.md](AETHER_0.33.md) and
[ADR-039](ADR-039-m23-comptime-pure-calls.md). The broader AI-first
systems-language direction remains separate in [NORTH_STAR.md](NORTH_STAR.md),
[CORE_CLAIMS.md](CORE_CLAIMS.md), and [ROADMAP.md](ROADMAP.md).

The current M25 local package increment is specified in
[AETHER_0.37.md](AETHER_0.37.md),
[DESIGN-M25-LOCAL-PACKAGE-PUBLICATION.md](DESIGN-M25-LOCAL-PACKAGE-PUBLICATION.md),
and [ADR-107](ADR-107-m25-local-package-publication.md). It is a host CLI
filesystem protocol only; it has no source, AETH, seed, VM, or guest-authority
effect.

Current product capabilities include verifier-first AETH v11/v12 emission,
seed-hosted compilation, bounded resources/effects/nurseries, grant-mediated
host I/O, the narrow foreign pilot, offline project/workspace tooling with
optional local workspace locks, transparent local source-package publication,
deterministic checkpointed task frames, and
versioned structural authoring v8. M23 adds
a deliberately narrow pure-Whole helper-call form at comptime; it does not add
an AETH instruction or runtime authority. General effects, OS-thread parallelism, generic type parameters,
C-header ingestion, arbitrary-node structural edits, a general or bundled
native backend,
and general metaprogramming remain outside the current contract.

M19e implements deterministic active-frame cancellation through a separate v12
task-frame path. Task source uses explicit checkpoints, source-order
single-thread round robin, verifier-proven private lanes, and VM-internal
destruction/zeroization. It preserves v11 nursery execution and shared-v11
arena behavior for every source/artifact that does not use a task frame.

Aether source continues to emit verified AETH only. Future designs may not
bypass verifier, forge, or host-capability boundaries.

## Compiler Boundary

The Rust bootstrap core uses pinned `serde`/`serde_json` only for strict local
JSON parsing and deterministic structural-document serialization. It parses
UTF-8 Aether source; validates names, types, ownership/moves, effects,
resources, structured control flow, comptime, and later bounded forms; builds
the resource plan; and emits AETH v11 or v12 directly. The bytecode verifier runs
before the VM. The compiler does not call a model, evaluate JavaScript, contact
a network service, or persist source.

Each source file declares one `world` and one or more named `weave`s. The source
validator creates a complete function-signature table before checking bodies,
which supports ordinary cross-weave calls while preserving exact argument arity,
types, and ownership modes. For M23, the comptime evaluator separately receives
only textually earlier guest weaves, so host, foreign, forward, and erroring
targets cannot acquire compile-time authority. Root bindings receive fixed local
slots; nested blocks can revise but cannot introduce bindings. This keeps
bytecode slots and control-flow state deterministic. The resource plan records
a lexical region name, element type, source owner/borrow place, and destination
for every closed resource operation; lowering consumes it rather than
re-scanning raw source syntax.

## Artifact Boundary

`AETH` is an Aether-owned binary format, not generated source for another
language. The verifier accepts historical v4 through v12 artifacts. Source
without task frames emits v11; valid M19e task source emits v12. Version 5 adds bounded nominal records, v6 adds bounded
arena/buffer resources, v7 adds `Error[Whole]` metadata/instructions, v8 adds
`COMPTIME_WHOLE` (56), v9 adds shapes/tables, v10 adds nurseries, and v11 adds
function host-kind metadata, `HOST_CALL`, and `RELEASE`; v12 adds task flags,
per-function frame capacity, and `TASK_CHECKPOINT` (67). M23 reuses the existing
v8 `COMPTIME_WHOLE` representation and deliberately makes no format change.
Its instruction set represents `Text`, `Whole`, `Truth`, bounded `Bytes`,
immutable records, opaque arena capability state, fixed Copy-element buffers,
immutable and mutable locals; moves; calls; control-flow jumps; text and byte
primitives including search and fixed-width packing/patching; stdout output;
and a typed yield.

The verifier rejects malformed headers, invalid UTF-8 text constants, oversized
bytes constants, invalid metadata, unknown opcodes, invalid local slots, reads
before initialization, illegal revisions, use-after-move, stack underflow, bad
operand types, invalid calls, invalid jump targets, non-convergent control-flow
states, and paths that do not terminate in `yield` before execution. For the
resource-capable versions it checks resource type/destination relations,
rejects persistent access loans, and tracks whether a Buffer operand is a
placeholder, a borrow, or the exact moved local owner.

The VM has no file, process, network, host-language evaluation, or guest
allocator API surface. Arithmetic detects `Whole` overflow. Text and bytes are capped at
1,000,000 bytes. `measure`, `glyph`, `cut`, and `seek` use Unicode scalar
positions; `extent`, `octet`, `slice`, `unpack*`, and `poke*` use bounded raw
byte positions. Internally, package 0.34 represents runtime Text as valid UTF-8
plus cached ASCII provenance. When the cache is true, byte positions equal
Unicode scalar positions, so the four scalar operations avoid repeated
traversal. Non-ASCII values retain scalar traversal. The cache is not serialized
and cannot change verifier or guest-visible AETH behavior.

AETH v4 remains valid for historical record-free programs and v5 for historical
record-bearing programs. Package 0.37 emits v11 for non-task source and v12 for
valid task source. Versions prior to v4 and unknown future versions are
intentionally rejected.

## Forge Boundary

The generic `invoke_bytecode` API invokes verified named weaves with checked
primitive host values. Records intentionally stay inside Aether call graphs;
host callers project a primitive field through a weave. The forge-specific API is narrower: it accepts only a verified
artifact exposing `compile [borrow source: Text] -> Bytes`, passes source text
as the sole value, receives the resulting bytes, and verifies those bytes before
the CLI writes them. The host bridge does not parse, translate, or alter the
source supplied to `compile`.

Every compiler artifact must still contain a valid zero-argument `main` weave
returning `Whole`, because all AETH artifacts remain independently verifiable
and runnable. The fixed compile ABI makes self-hosting proofs inspectable
without granting an artifact host capabilities.

## Seed-Hosted Product Compile Boundary

The Aether-written seed is the **default product compiler** for its documented
Seed Profile (ADR-064–067). Product path forges first without bootstrap
pre-validate; dual-compare remains the proof oracle:

- `compile_with_seed` / `compile_product_bytecode` embed `SEED_COMPILER_ARTIFACT`
  and forge user source (including seed rebuild).
- CLI `aether compile` uses that path by default (no `--bootstrap`).
- CLI `compile --bootstrap` and `check --bootstrap` are recovery/oracle only.

The Seed Profile directly emits the documented canonical 0.11 source surface
and later covered product forms into AETH v11 or v12 as task-frame metadata
requires. It directly accepts M5/M15 literal/name-chain comptime arithmetic and
the documented M23 D2a raw-source pure-call subset. The seed evaluates the
eligible restricted pure helper itself; no bootstrap materialization bridge is
used on that product path. Byte identity with direct bootstrap emission is
required. See [SEED_PROFILE.md](SEED_PROFILE.md) and
[AETHER_0.33.md](AETHER_0.33.md).

Bootstrap is not gone: it rebuilds the seed, supplies the full invalid-source
diagnostic path, and dual-checks proofs. Product bytecode is seed-produced.

## Structural Authoring Boundary

The current local wire contracts are `aether.ast/v8` and `aether.edit/v8`.
The AST exports the fully validated formatter-canonical semantic tree; the edit
protocol carries an exact canonical base source and typed bounded edits. Strict
parsing rejects duplicate JSON keys, unknown fields, unsupported versions,
invalid shapes, excessive depth,
and bounded-input violations before an edit can affect a Program. Existing
record type references are reindexed by nominal name when records move, and a
required `Bind.stage` records runtime versus comptime intent, so a structural
edit cannot silently change the stage of a binding. V8 makes `Weave.task`
explicit and adds a typed `Checkpoint` statement; the complete task/capacity
semantic check still runs after every edit. M23 uses the existing Call expression
node and does not alter either protocol version.

The core edit operation is pure: it formats and bootstrap-validates candidate
source but does not persist, execute, or invoke a model. The CLI then
seed-compiles the returned canonical source, which verifies the AETH artifact,
before writing it to the caller-selected output path. The protocol is not a
guest capability, does not modify Forge, and cannot become compiler authority
for an AI model.

## Package Integrity Boundary

`aether.project/v1` may carry a complete unit lock, and package 0.35 adds an
optional complete `lock.packages` list to `aether.workspace/v1`. The workspace
lock records a declared package's local path, parsed project identity, and raw
`aether.project.json` digest. It is checked only through the local filesystem:
there is no URL parser, resolver, registry client, cache fetcher, package script,
or guest-visible package capability.

For a locked workspace, project-manifest resolution canonicalizes the manifest
and rejects a symlink leaving the canonical package root. Verification then
checks the raw manifest digest and project name/version, requires the nested
complete unit lock, and runs ordinary project verification. `workspace build`
uses that whole-workspace preflight before it elaborates or writes an artifact.
`project lock` and `workspace lock` calculate candidate manifests in memory and
write only when the caller gives explicit `--write`; a workspace refresh does
not silently mutate all nested project manifests.

M25 adds a parallel, explicit source-package lifecycle rather than changing the
workspace lock protocol. `pkg pack` snapshots one complete locked project and
writes an `aether.package/v1` directory bundle containing only generated
metadata plus the project manifest and declared source units. `pkg verify`
checks strict path/tree/regular-file rules, per-file and domain-separated
content digests, and nested project integrity. `pkg publish` permits only a
verified bundle into an explicit local `packages/<name>/<version>` cache,
preserving a conflicting identity; `pkg install` writes only a verified
`project/` tree to an explicit absent output and rechecks it. No resolver,
fetch, automatic workspace edit, package scripts, signatures, or guest
capability is introduced.

## CLI Authority Boundary

The CLI owns caller-selected local file I/O. `structure` writes only the
semantic document to stdout. `compile`, `forge`, and `apply-edit` write only to
their explicit output paths; `compile` and `forge` verify AETH before writing,
and `apply-edit` seed-compiles before writing canonical source. No active
desktop, WebView, model, or network integration exists. The retired workbench
is recorded in [ADR-006](ADR-006-retire-aether-studio.md).

### M32a/M32b verified-execution benchmark boundary

`aether bench` is a deliberately narrower execution entrypoint. It contains the
reviewed `welcome`, `arena-buffer`, and `task-loop` Aether sources at build time;
the operator selects only one of those names or `all`. The CLI uses the product
seed compiler once per selected workload, explicitly verifies the generated
AETH, and then times repeated calls to the normal verify-before-run VM path.
It installs no grants and has no source path, artifact path, host service,
native, process, or network input. The optional JSON report is written only to
an explicit path after all selected workloads complete. This gives local
baseline evidence without turning the CLI into an arbitrary-program timing
surface. M32b adds an opt-in profile-bound report v2 and a distinct `aether
bench compare` data path. It opens only the two explicitly selected, bounded
strict JSON reports; validates their profile/environment/workload/observable
identity; never invokes the compiler or VM; and writes a comparison only to an
explicit non-input path. Artifact identities may differ and are reported, while
source and observable output identity must match. This gives an honest
like-for-like local comparison mechanism, not a hardware/toolchain attestation
or broad speed claim; see [ADR-104](ADR-104-m32a-verified-execution-benchmarks.md)
and [ADR-105](ADR-105-m32b-profile-bound-comparisons.md).

## Bootstrap Boundary

The Rust core remains the Aether 0.37 bootstrap implementation and VM, required
to rebuild the seed artifact and diagnose invalid source. Product compilation is
seed-hosted after the narrow M23 materialization bridge where applicable. Future
language extensions must keep self-host, example, canonical-surface, and
applicable dual-compare proofs green before entering the product path.
