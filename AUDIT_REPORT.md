# Aether Migration Audit

Date: 2026-07-25

## Legacy Intake

| Area | Result | Aether use |
| --- | --- | --- |
| `legacy/xlang-v1-prototype` | Experimental Rust implementations with known compile failures. | Preserve source spans, diagnostic discipline, and failure cases as reference; do not compile or emit its syntax. |
| `legacy/xlang-v2-snapshot` | A former parser and type checker with a focused frontend test suite. | Preserve parser, semantic-check, and test-design lessons; do not promote its grammar or token stream. |
| `legacy/aether-genesis-ai-studio` | Historical AI Studio material with stale cloud metadata and JavaScript evaluation behavior. | Preserve as historical material; do not evaluate generated JavaScript or use cloud AI. |
| Docker Ollama | Local loopback AI runtime. | Keep it optional and limited to user-requested source review. |

## Production Boundary

The production compiler accepts only Aether 0.2.0 source. It produces a
canonical AST and deterministic AETH v2 bytecode artifact, verifies that
artifact, and runs it in the Aether VM. The command line and desktop app call
the same compiler core.

Stage 1 extends the initial kernel with named weaves, typed calls, three value
types, structured control flow, mutable root slots, explicit local Text access,
and Unicode-safe bounded text primitives. Legacy C-shaped, Rust-shaped, and V2
source remains intentionally rejected. The repository does not transpile Aether
to C, Rust, JavaScript, LLVM, or another target language.

## Desktop and Data Boundary

Aether Studio runs compilation locally in its Tauri process. Ollama is a
separate optional reviewer, reachable only through a validated loopback HTTP
endpoint. It cannot alter bytecode, execute code, or become a compiler
authority. Source and model choices remain in local WebView storage.

## Release Gate

A release requires passing Aether core tests, Clippy with warnings denied,
frontend linting, frontend tests, production frontend build, Windows Tauri
bundle build, command-line compile and run checks, and a live Docker Ollama
status check. A successful package must be inspected and launched before it is
reported as delivered.
