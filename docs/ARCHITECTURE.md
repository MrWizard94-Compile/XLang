# Architecture

~~~mermaid
flowchart LR
    Editor["Aether Studio editor"] -->|"Tauri command"| Core["Aether 0.2 compiler core"]
    CLI["aether CLI"] --> Core
    Core --> Source["Canonical AST"]
    Core --> Artifact["Verified AETH v2 artifact"]
    Artifact --> VM["Aether VM"]
    VM --> Result["stdout and exit value"]
    Editor -->|"optional review request"| Guard["loopback and model validation"]
    Guard --> Ollama["Docker Ollama :11434"]
    Ollama --> Review["review text"]
    Review --> Editor
~~~

## Compiler Boundary

The compiler core is a dependency-free Rust bootstrap crate. It parses UTF-8
Aether source, validates names, types, mutation, Text move state, and structured
control flow, serializes a deterministic AST, and emits AETH v2 bytecode. The
bytecode verifier runs before the VM. The compiler does not call a model,
evaluate JavaScript, contact a network service, or persist source.

Each source file declares one \`world\` and one or more named \`weave\`s. The source
validator creates a complete function-signature table before checking bodies,
which supports cross-weave calls while preserving exact argument arity, types,
and Text ownership modes. Root bindings receive fixed local slots; nested blocks
can revise but cannot introduce bindings. This keeps bytecode slots and
control-flow state deterministic.

## Artifact Boundary

\`AETH\` is an Aether-owned binary format, not generated source for another
language. Version 2 stores named weave metadata, parameter ownership modes,
result types, local descriptors, and bytecode. Its instruction set represents
Text, Whole, and Truth values; immutable and mutable locals; moves; calls;
control-flow jumps; bounded text primitives; stdout output; and a typed yield.

The verifier rejects malformed headers, invalid UTF-8 text constants, invalid
metadata, unknown opcodes, invalid local slots, reads before initialization,
illegal revisions, use-after-move, stack underflow, bad operand types, invalid
call signatures, invalid jump targets, non-convergent control-flow states, and
paths that do not terminate in \`yield\` before execution.

The VM has no file, process, network, host-language evaluation, or allocator API
surface. Arithmetic detects \`Whole\` overflow. Text is capped at 1,000,000 bytes;
\`measure\`, \`glyph\`, and \`cut\` use Unicode scalar positions so UTF-8 cannot cause
slice-boundary panics.

## Desktop Boundary

Studio is a Tauri 2 desktop app. The React renderer owns the active document.
The native command layer compiles it, runs only verified AETH output, and returns
bounded artifact metadata plus VM output. Source persistence uses \`aether.source\`
and model persistence uses \`aether.model\` in local WebView storage.

## Local Model Selection

The compiler requires no AI. The optional reviewer queries Docker-hosted Ollama
at a credential-free loopback HTTP base URL only. The status command reads
\`/api/tags\`, fills the selector with installed models, and a review request uses
\`/api/chat\` with streaming disabled and a bounded source payload. The reviewer
receives source only after the user explicitly invokes it and never participates
in compilation or execution.

## Bootstrap Boundary

The Rust core is the Stage 1 bootstrap implementation. Aether still needs an
Aether-written compiler and a reproducible artifact comparison before it can be
called self-hosting. The present artifact format and VM are intentionally
Aether-owned and remain independent of Rust, C, JavaScript, LLVM, or another
target language.
