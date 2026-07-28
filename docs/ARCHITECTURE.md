# Architecture

~~~mermaid
flowchart LR
    Editor["Aether Studio editor"] -->|"Tauri command"| Core["Aether 0.4 bootstrap core"]
    CLI["aether CLI"] --> Core
    Core --> Source["Canonical AST"]
    Core --> Artifact["Verified AETH v4 artifact"]
    Artifact --> VM["Aether VM"]
    VM --> Result["stdout and exit value"]
    SeedSrc["seed/aether_seed.ae"] -->|"bootstrap compile"| SeedArt["seed compiler AETH v4"]
    Forge["aether forge"] -->|"verify compiler artifact"| SeedArt
    Forge -->|"source Text"| SeedArt
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

AETH versions prior to v4 are intentionally rejected by the Aether 0.4 VM.

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

## Seed-Profile Self-Hosting Boundary

Stage 3 adds ordinary language primitives (`seek`, `number`, packing, unpacking,
and poke forms) that any Aether program may use. The Aether-written seed
compiler in `seed/aether_seed.ae` uses those primitives to parse the Seed
Profile and construct AETH v4 bytes. Self-hosting is claimed only for that
profile: bootstrap compile, forge rebuild, and second-generation forge must all
match byte-for-byte, and a distinct source variant must produce a different
verified artifact.

Full Aether 0.4 remains bootstrap-compiled. See [SEED_PROFILE.md](SEED_PROFILE.md).

## Desktop Boundary

Studio is a Tauri 2 desktop app. The React renderer owns the active document.
The native command layer compiles it, runs only verified AETH output, and returns
bounded artifact metadata plus VM output. Source persistence uses `aether.source`
and model persistence uses `aether.model` in local WebView storage.

## Local Model Selection

The compiler requires no AI. The optional reviewer queries Docker-hosted Ollama
at a credential-free loopback HTTP base URL only. The status command reads
`/api/tags`, fills the selector with installed models, and a review request uses
`/api/chat` with streaming disabled and a bounded source payload. The reviewer
receives source only after the user explicitly invokes it and never participates
in compilation, forge invocation, or execution.

## Bootstrap Boundary

The Rust core remains the complete Aether 0.4 bootstrap implementation and VM.
The seed compiler is evidence of Seed-Profile self-hosting only. Expanding that
profile until it covers the full language is future work and must repeat the same
reproducible artifact comparison standard.
