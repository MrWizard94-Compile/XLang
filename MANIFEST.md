# Aether Production Manifest

## Contract

The compiler accepts Aether source, returns a canonical AST, emits an AETH v2
artifact, verifies it, and runs that artifact in the Aether VM. The command line
and desktop app use the same compiler core.

## Implemented Language Boundary

Aether 0.2.0 accepts one `world` declaration and one or more named `weave`
declarations. `main` must be `weave main [] -> Whole:`. It provides `Text`,
`Whole`, and `Truth`, immutable and mutable root bindings, `revise`, `speak`,
terminal `yield`, `choose`/`otherwise`, `while`, typed calls, and shallow prefix
expressions for arithmetic, comparison, text composition, and rendering.

`Text` bindings require explicit `borrow name` or `move name`; a move makes the
local unavailable on every subsequent reachable control-flow path. `Whole` and
`Truth` are copied by ordinary name use. Nested blocks cannot introduce a
binding, which keeps slots fixed and control-flow state verifiable.

Source is UTF-8. Names and language keywords remain lowercase ASCII. Canonical
formatting uses LF, exact two-space indentation, no tabs, and no trailing
whitespace. CRLF input is accepted and formatted as LF.

The AETH v2 verifier checks headers, function metadata, local initialization and
mutability, stack types, move state, jump targets, control-flow convergence,
call signatures, UTF-8 text constants, and terminal yields before execution.
Text is bounded to 1,000,000 bytes. The VM uses Unicode scalar positions for
`measure`, `glyph`, and `cut`.

## Deliberate Boundary

Stage 1 is not a claim that Aether has explicit allocators, arenas, records,
collections, SoA layouts, typed error sets, compile-time execution, C interop,
native code generation, or a self-hosted compiler. Those are intentionally
absent until a later complete and verified language stage introduces them.

The current Rust implementation is the bootstrap compiler and VM. It is not a
target language for Aether and does not cause source to be translated to Rust,
C, JavaScript, LLVM, or another language.

## AI Boundary

Ollama assistance is optional and local-only. It is restricted to a loopback
HTTP endpoint and a conservative model-name character set. It can review source
only after a user request. It is never a compiler, evaluator, code execution
authority, or artifact signer.

## Quality Gate

The repository must pass Rust formatting, all core and desktop tests, Clippy
with warnings denied, TypeScript linting, frontend tests, production frontend
build, Windows Tauri bundle build, command-line compile and run checks for the
Stage 1 examples, a live Docker Ollama status check, and package inspection plus
launch before release.
