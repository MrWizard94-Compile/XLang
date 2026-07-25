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

The production compiler accepts only Aether 0.3.0 source. It produces a
canonical AST and deterministic AETH v3 bytecode artifact, verifies that
artifact, and runs it in the Aether VM. The command line and desktop app call
the same compiler core.

Stage 2 extends the Stage 1 language with a bounded `Bytes` type, explicit byte
ownership, binary literals and operations, deterministic checked division and
remainder, and a typed compiler invocation boundary. Legacy C-shaped,
Rust-shaped, V1, V2, and AETH v2 input remains intentionally rejected. The
repository does not transpile Aether to C, Rust, JavaScript, LLVM, or another
target language.

`aether forge` verifies a compiler artifact, requires
`compile [borrow source: Text] -> Bytes`, gives it the source text, verifies the
returned artifact bytes, and writes only a verified result. This is a host ABI,
not evidence of an Aether-written compiler or self-hosting.

## Desktop and Data Boundary

Aether Studio runs compilation locally in its Tauri process. Ollama is a
separate optional reviewer, reachable only through a validated loopback HTTP
endpoint. It cannot alter bytecode, execute code, become a compiler authority,
or participate in forge invocation. Source and model choices remain in local
WebView storage.

## Release Gate

A release requires passing Aether core, CLI, and desktop tests, Clippy with
warnings denied, frontend linting, frontend tests, production frontend build,
Windows Tauri bundle build, command-line compile and run checks, forge contract
verification, and a live Docker Ollama status check. A successful package must
be inspected and launched before it is reported as delivered.
