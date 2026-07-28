# Aether Migration Audit

Date: 2026-07-28 (Stage 4 update)

## Legacy Intake

| Area | Result | Aether use |
| --- | --- | --- |
| `legacy/xlang-v1-prototype` | Experimental Rust implementations with known compile failures. | Preserve source spans, diagnostic discipline, and failure cases as reference; do not compile or emit its syntax. |
| `legacy/xlang-v2-snapshot` | A former parser and type checker with a focused frontend test suite. | Preserve parser, semantic-check, and test-design lessons; do not promote its grammar or token stream. |
| `legacy/aether-genesis-ai-studio` | Historical AI Studio material with stale cloud metadata and JavaScript evaluation behavior. | Preserve as historical material; do not evaluate generated JavaScript or use cloud AI. |
| Docker Ollama | Local loopback AI runtime. | Keep it optional and limited to user-requested source review. |

## Production Boundary

The production bootstrap compiler accepts Aether 0.4.0 source. It produces a
canonical AST and deterministic AETH v4 bytecode artifact, verifies that
artifact, and runs it in the Aether VM. The command line and desktop app call
the same compiler core.

Stage 4 extends the Stage 3 Seed Profile compiler with multi-weave emission and
`call`, while retaining the Stage 3 bounded text/binary primitives (`seek`,
`number`, `pack*`, `unpack*`, `poke*`), AETH v4, and reproducible
self-compilation proof. Legacy C-shaped, Rust-shaped, V1, V2, and pre-v4 AETH
input remains intentionally rejected. The repository does not transpile Aether
to C, Rust, JavaScript, LLVM, or another target language.

`aether forge` verifies a compiler artifact, requires
`compile [borrow source: Text] -> Bytes`, gives it the source text, verifies the
returned artifact bytes, and writes only a verified result.

## Seed-Profile Self-Hosting Boundary

`seed/aether_seed.ae` is source in Aether. It parses the documented Seed Profile
(including multi-weave programs with `call`) and emits AETH v4 through ordinary
language operations. The regression tests prove bootstrap, first forge, and
second forge match the checked-in artifact; a distinct source variant produces a
different verified artifact; and multi-weave + `call` forge matches bootstrap.

This is self-hosting for the Seed Profile only. Full Aether 0.4 remains
bootstrap-hosted. Scope: [docs/SEED_PROFILE.md](docs/SEED_PROFILE.md).

## Desktop and Data Boundary

Aether Studio runs compilation locally in its Tauri process. Ollama is a
separate optional reviewer, reachable only through a validated loopback HTTP
endpoint. It cannot alter bytecode, execute code, become a compiler authority,
or participate in forge invocation. Source and model choices remain in local
WebView storage.

## Release Gate

A release requires passing Aether core, CLI, and desktop tests (including the
seed self-host proof), Clippy with warnings denied, frontend linting, frontend
tests, production frontend build, Windows Tauri bundle build, command-line
compile/forge/run checks, and a live Docker Ollama status check when claiming AI
integration. A successful package must be inspected and launched before it is
reported as delivered.
