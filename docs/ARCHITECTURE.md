# Architecture

~~~mermaid
flowchart LR
    CLI["aether CLI"] -->|"compile"| SeedPath["compile_with_seed"]
    CLI -->|"structure / apply-edit"| Authoring["aether.ast/v1 + aether.edit/v1"]
    Authoring -->|"canonical validated source"| SeedPath
    SeedPath --> SeedArt["embedded seed AETH v6 compiler"]
    SeedArt --> Artifact["Verified AETH v4, v5, or v6 artifact"]
    Artifact --> VM["Aether VM"]
    VM --> Result["stdout and exit value"]
    Check["aether check"] --> Bootstrap["Rust bootstrap AST"]
    SeedSrc["seed/aether_seed.ae"] -->|"compile --bootstrap"| SeedArtFile["seed/aether_seed.aeth"]
    Forge["aether forge"] -->|"verify + invoke"| SeedArt
    SeedArt --> Candidate["candidate AETH Bytes"]
    Candidate -->|"verify before write"| Forge
~~~

## Scope and Future-Design Boundary

This document describes the implemented Aether 0.6 architecture. The broader
AI-first systems-language direction is documented separately in
[NORTH_STAR.md](NORTH_STAR.md), [CORE_CLAIMS.md](CORE_CLAIMS.md), and
[ROADMAP.md](ROADMAP.md). The accepted M1 direction is
[DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md](DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md);
the executable bounded M2 subset is [ADR-004](ADR-004-aeth-v6-bounded-resources.md).
In particular, Aether 0.6 has no typed effects, structured concurrency, generic
shape folding, SoA lowering, C-header ingestion, fine-grained arbitrary-node
structural edits, or native backend. M3 provides a bounded top-level structural
authoring protocol described in [AETHER_AUTHORING_PROTOCOL_v1.md](AETHER_AUTHORING_PROTOCOL_v1.md).
Aether source
continues to emit AETH only; future designs may not bypass verifier, forge, or
host-capability boundaries.

## Compiler Boundary

The Rust bootstrap core uses pinned `serde`/`serde_json` only for strict local
M3 JSON parsing and deterministic structural-document serialization. It parses
UTF-8 Aether source, validates names, types, mutation, `Text` and `Bytes` move state, and
structured control flow, serializes a deterministic AST, builds a typed M2
resource semantic plan, and emits AETH v6 bytecode. The bytecode verifier runs
before the VM. The compiler does not call a
model, evaluate JavaScript, contact a network service, or persist source.

Each source file declares one `world` and one or more named `weave`s. The source
validator creates a complete function-signature table before checking bodies,
which supports cross-weave calls while preserving exact argument arity, types,
and ownership modes. Root bindings receive fixed local slots; nested blocks can
revise but cannot introduce bindings. This keeps bytecode slots and control-flow
state deterministic. The resource plan records a lexical region name, element
type, source owner/borrow place, and destination for every closed resource
operation; lowering consumes it rather than re-scanning raw source syntax.

## Artifact Boundary

`AETH` is an Aether-owned binary format, not generated source for another
language. Version 4 stores named weave metadata, parameter ownership modes,
result types, local descriptors, and bytecode. Version 5 adds a bounded nominal
record table before the same function table, record-aware type descriptors, and
verified `MAKE_RECORD` / `FIELD` instructions. Version 6 adds a bounded
arena-capacity field before the record table and verified `ARENA`, `BUFFER`,
`ACCESS`, `ALLOCATE`, `BUFFER_APPEND`, `BUFFER_AT`, and `COUNT` instructions.
Its instruction set represents `Text`, `Whole`, `Truth`, bounded `Bytes`,
immutable records, opaque arena capability state, fixed Copy-element buffers,
immutable and mutable locals; moves; calls; control-flow jumps; text and byte
primitives including search and fixed-width packing/patching; stdout output;
and a typed yield.

The verifier rejects malformed headers, invalid UTF-8 text constants, oversized
bytes constants, invalid metadata, unknown opcodes, invalid local slots, reads
before initialization, illegal revisions, use-after-move, stack underflow, bad
operand types, invalid calls, invalid jump targets, non-convergent control-flow
states, and paths that do not terminate in `yield` before execution. For v6 it
also requires exactly one main arena declaration for a nonzero resource plan,
checks resource type/destination relations, rejects persistent access loans, and
tracks whether a Buffer operand is a placeholder, a borrow, or the exact moved
local owner.

The VM has no file, process, network, host-language evaluation, or guest
allocator API surface. Arithmetic detects `Whole` overflow. Text and bytes are capped at
1,000,000 bytes. `measure`, `glyph`, `cut`, and `seek` use Unicode scalar
positions; `extent`, `octet`, `slice`, `unpack*`, and `poke*` use bounded raw
byte positions.

AETH v4 remains valid for historical record-free programs and v5 for historical
record-bearing programs. New 0.6 compilation emits v6. Versions prior to v4
and unknown future versions are intentionally rejected.

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

Stage 7 keeps the Aether-written seed as the **default product compiler** and
extends its proven surface with bounded arenas and Copy-element buffers:

- `compile_with_seed` embeds `SEED_COMPILER_ARTIFACT` and forges user source.
- CLI `aether compile` uses that path.
- CLI `compile --bootstrap` and `check` still use the Rust bootstrap for seed
  rebuild and AST diagnostics.

The Seed Profile emits the complete documented canonical Aether 0.6 source
surface: all statement and shallow expression families, named locals/params,
`borrow`/`move`/`access`, multi-weave `call` (including forward callees), hex
`bytes` literals, UTF-8 text constants with all defined escapes, LF/CRLF input
with or without a final line terminator, immutable records, and closed M2
resource forms. It emits v6 with an arena capacity field and optional record
table. Self-host, shipped-example, prior canonical-surface, and M2
dual-compare proofs live in `seed_self_host.rs` and the core resource corpus.
See
[SEED_PROFILE.md](SEED_PROFILE.md).

Bootstrap is not gone: it rebuilds the seed, supplies the full invalid-source
diagnostic path, and dual-checks proofs. Product bytecode is seed-produced.

## Structural Authoring Boundary

M3 has two local wire contracts: `aether.ast/v1` exports the fully validated,
formatter-canonical semantic tree, and `aether.edit/v1` carries an exact
canonical base source plus typed top-level Record/Weave `replace`,
`insertAfter`, or `delete` operations. Strict parsing rejects duplicate JSON
keys, unknown fields, unsupported versions, invalid shapes, excessive depth,
and bounded-input violations before an edit can affect a Program. Existing
record type references are reindexed by nominal name when records move, so a
record insertion cannot silently retarget a weave signature.

The core edit operation is pure: it formats and bootstrap-validates candidate
source but does not persist, execute, or invoke a model. The CLI then
seed-compiles the returned canonical source, which verifies the AETH artifact,
before writing it to the caller-selected output path. The protocol is not a
guest capability, does not modify Forge, and cannot become compiler authority
for an AI model.

## CLI Authority Boundary

The CLI owns caller-selected local file I/O. `structure` writes only the
semantic document to stdout. `compile`, `forge`, and `apply-edit` write only to
their explicit output paths; `compile` and `forge` verify AETH before writing,
and `apply-edit` seed-compiles before writing canonical source. No active
desktop, WebView, model, or network integration exists. The retired workbench
is recorded in [ADR-006](ADR-006-retire-aether-studio.md).

## Bootstrap Boundary

The Rust core remains the Aether 0.6 bootstrap implementation and VM, required to
rebuild the seed artifact and diagnose invalid source. Product compilation is
seed-hosted. Future language extensions must keep self-host, example, and
canonical-surface dual-compare proofs green before entering the product path.
