# Architecture

~~~mermaid
flowchart LR
    Editor["Aether Studio editor"] -->|"Tauri command"| Core["Aether compiler core"]
    Core --> Artifact["Verified AETH artifact"]
    Artifact --> VM["Aether VM"]
    VM --> Result["stdout and exit code"]
    Editor -->|"optional review request"| Guard["loopback and model validation"]
    Guard --> Ollama["Docker Ollama :11434"]
    Ollama --> Review["review text"]
    Review --> Editor
~~~

## Compiler Boundary

The compiler core is a dependency-free Rust bootstrap crate. It parses Aether
source, validates binding and effect types, serializes a deterministic AST, and
emits AETH bytecode. The bytecode verifier runs before the VM. The compiler
does not call a model, evaluate JavaScript, contact a network service, or persist
source.

## Artifact Boundary

AETH is an Aether-owned binary format, not generated source for another
language. The current instruction set represents Text and Whole values, immutable
local slots, stdout output, and a Whole exit code. The verifier rejects malformed
headers, invalid operands, unknown slots, invalid stack type flow, and any code
after yield before execution.

## Desktop Boundary

Studio is a Tauri 2 desktop app. The React renderer owns the active document.
The native command layer compiles it, runs only the verified AETH output, and
returns bounded artifact metadata and VM output. Source persistence uses
aether.source and model persistence uses aether.model in local WebView storage.

## Local Model Selection

The compiler requires no AI. The optional reviewer queries Docker-hosted Ollama
at a credential-free loopback HTTP base URL only. The status command reads
/api/tags, fills the selector with installed models, and a review request uses
/api/chat with streaming disabled and a bounded source payload.
