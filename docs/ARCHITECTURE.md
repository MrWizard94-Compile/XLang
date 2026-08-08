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
    SeedSrc["seed/aether_seed.ae"] -->|"compile --bootstrap"| SeedArtFile["seed/aether_seed.aeth"]
    Forge["aether forge"] -->|"verify + invoke"| SeedArt
    SeedArt --> Candidate["candidate AETH Bytes"]
    Candidate -->|"verify before write"| Forge
~~~

## Scope and product boundary

This document describes the implemented **Aether 0.36.0** architecture: the
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

Current product capabilities include verifier-first AETH v11/v12 emission,
seed-hosted compilation, bounded resources/effects/nurseries, grant-mediated
host I/O, the narrow foreign pilot, offline project/workspace tooling with
optional local workspace locks, deterministic checkpointed task frames, and
versioned structural authoring v8. M23 adds
a deliberately narrow pure-Whole helper-call form at comptime; it does not add
an AETH instruction or runtime authority. General effects, OS-thread parallelism, generic type parameters,
C-header ingestion, arbitrary-node structural edits, native code generation,
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
record-bearing programs. Package 0.36 emits v11 for non-task source and v12 for
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
Seed Profile. The bootstrap validates the source first; this retains complete
diagnostics and is also the deliberate M23 enforcement point:

- `compile_with_seed` embeds `SEED_COMPILER_ARTIFACT` and forges validated user
  source.
- CLI `aether compile` uses that path.
- CLI `compile --bootstrap` and `check` still use the Rust bootstrap for seed
  rebuild and AST diagnostics.

The Seed Profile directly emits the documented canonical 0.11 source surface
and later covered product forms into AETH v11 or v12 as task-frame metadata
requires. It directly accepts M5/M15
literal/name-chain comptime arithmetic. For an accepted M23 call directive,
the bootstrap evaluates the restricted pure helper and materializes only that
directive into equivalent literal M5 source before it reaches the seed. The
seed therefore emits the product artifact, but the checked-in seed does not
claim to independently parse raw M23 calls. Byte identity with direct bootstrap
emission is required. See [SEED_PROFILE.md](SEED_PROFILE.md) and
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

## CLI Authority Boundary

The CLI owns caller-selected local file I/O. `structure` writes only the
semantic document to stdout. `compile`, `forge`, and `apply-edit` write only to
their explicit output paths; `compile` and `forge` verify AETH before writing,
and `apply-edit` seed-compiles before writing canonical source. No active
desktop, WebView, model, or network integration exists. The retired workbench
is recorded in [ADR-006](ADR-006-retire-aether-studio.md).

## Bootstrap Boundary

The Rust core remains the Aether 0.36 bootstrap implementation and VM, required
to rebuild the seed artifact and diagnose invalid source. Product compilation is
seed-hosted after the narrow M23 materialization bridge where applicable. Future
language extensions must keep self-host, example, canonical-surface, and
applicable dual-compare proofs green before entering the product path.
