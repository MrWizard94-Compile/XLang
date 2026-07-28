# Architecture

~~~mermaid
flowchart LR
    Editor["Aether Studio editor"] -->|"Tauri command"| SeedPath["compile_with_seed"]
    CLI["aether compile"] --> SeedPath
    SeedPath --> SeedArt["embedded seed AETH v4"]
    SeedArt --> Artifact["Verified AETH v4 artifact"]
    Artifact --> VM["Aether VM"]
    VM --> Result["stdout and exit value"]
    Check["aether check"] --> Bootstrap["Rust bootstrap AST"]
    SeedSrc["seed/aether_seed.ae"] -->|"compile --bootstrap"| SeedArtFile["seed/aether_seed.aeth"]
    Forge["aether forge"] -->|"verify + invoke"| SeedArt
    SeedArt --> Candidate["candidate AETH Bytes"]
    Candidate -->|"verify before write"| Forge
    Editor -->|"optional review request"| Guard["loopback and model validation"]
    Guard --> Ollama["Docker Ollama :11434"]
    Ollama --> Review["review text"]
    Review --> Editor
~~~

## Compiler Boundary

The Rust bootstrap core is a dependency-free crate. It parses UTF-8 Aether
source, validates names, types, mutation, `Text` and `Bytes` move state, and
structured control flow, serializes a deterministic AST, and emits AETH v4
bytecode. The bytecode verifier runs before the VM. The compiler does not call a
model, evaluate JavaScript, contact a network service, or persist source.

Each source file declares one `world` and one or more named `weave`s. The source
validator creates a complete function-signature table before checking bodies,
which supports cross-weave calls while preserving exact argument arity, types,
and ownership modes. Root bindings receive fixed local slots; nested blocks can
revise but cannot introduce bindings. This keeps bytecode slots and control-flow
state deterministic.

## Artifact Boundary

`AETH` is an Aether-owned binary format, not generated source for another
language. Version 4 stores named weave metadata, parameter ownership modes,
result types, local descriptors, and bytecode. Its instruction set represents
`Text`, `Whole`, `Truth`, and bounded `Bytes`; immutable and mutable locals;
moves; calls; control-flow jumps; text and byte primitives including search and
fixed-width packing/patching; stdout output; and a typed yield.

The verifier rejects malformed headers, invalid UTF-8 text constants, oversized
bytes constants, invalid metadata, unknown opcodes, invalid local slots, reads
before initialization, illegal revisions, use-after-move, stack underflow, bad
operand types, invalid calls, invalid jump targets, non-convergent control-flow
states, and paths that do not terminate in `yield` before execution.

The VM has no file, process, network, host-language evaluation, or allocator API
surface. Arithmetic detects `Whole` overflow. Text and bytes are capped at
1,000,000 bytes. `measure`, `glyph`, `cut`, and `seek` use Unicode scalar
positions; `extent`, `octet`, `slice`, `unpack*`, and `poke*` use bounded raw
byte positions.

AETH versions prior to v4 are intentionally rejected by the Aether 0.5 VM.

## Forge Boundary

The generic `invoke_bytecode` API invokes verified named weaves with checked
host values. The forge-specific API is narrower: it accepts only a verified
artifact exposing `compile [borrow source: Text] -> Bytes`, passes source text
as the sole value, receives the resulting bytes, and verifies those bytes before
the CLI writes them. The host bridge does not parse, translate, or alter the
source supplied to `compile`.

Every compiler artifact must still contain a valid zero-argument `main` weave
returning `Whole`, because all AETH artifacts remain independently verifiable
and runnable. The fixed compile ABI makes self-hosting proofs inspectable
without granting an artifact host capabilities.

## Seed-Hosted Product Compile Boundary

Stage 5 makes the Aether-written seed the **default product compiler**:

- `compile_with_seed` embeds `SEED_COMPILER_ARTIFACT` and forges user source.
- CLI `aether compile` and Studio use that path.
- CLI `compile --bootstrap` and `check` still use the Rust bootstrap for seed
  rebuild and AST diagnostics.

The Seed Profile emits the complete documented canonical Aether 0.4 source
surface: all statement and shallow expression families, named locals/params,
`borrow`/`move`, multi-weave `call` (including forward callees), hex `bytes`
literals, UTF-8 text constants with all defined escapes, and LF/CRLF input with
or without a final line terminator. Self-host, shipped-example, and
canonical-surface dual-compare proofs live in `seed_self_host.rs`. See
[SEED_PROFILE.md](SEED_PROFILE.md).

Bootstrap is not gone: it rebuilds the seed, supplies the full invalid-source
diagnostic path, and dual-checks proofs. Product bytecode is seed-produced.

## Desktop Boundary

Studio is a Tauri 2 desktop app. The React renderer owns the active document.
The native command layer **seed-compiles** the document, runs only verified AETH
output, and returns bounded artifact metadata plus VM output. Source persistence
uses `aether.source` and model persistence uses `aether.model` in local WebView
storage.

## Local Model Selection

The compiler requires no AI. The optional reviewer queries Docker-hosted Ollama
at a credential-free loopback HTTP base URL only. The status command reads
`/api/tags`, fills the selector with installed models, and a review request uses
`/api/chat` with streaming disabled and a bounded source payload. The reviewer
receives source only after the user explicitly invokes it and never participates
in compilation, forge invocation, or execution.

## Bootstrap Boundary

The Rust core remains the Aether 0.5 bootstrap implementation and VM, required to
rebuild the seed artifact and diagnose invalid source. Product compilation is
seed-hosted. Future language extensions must keep self-host, example, and
canonical-surface dual-compare proofs green before entering the product path.
