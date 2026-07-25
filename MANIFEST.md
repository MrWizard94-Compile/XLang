# Aether Production Manifest

## Contract

The compiler accepts Aether source, returns a canonical AST, emits an AETH
artifact, verifies it, and runs that artifact in the Aether VM. The command line
and desktop app use the same compiler core.

## Implemented Language Boundary

Aether 0.1 supports one world declaration, the main weave, immutable bind
statements, Text and Whole values, speak, and terminal yield. It rejects
noncanonical indentation, trailing whitespace, shadowing, use-before-bind,
implicit conversion, malformed text escapes, legacy declarations, and source
outside its exact grammar.

The planned MVS, arena, forge, choose, match, and move terms from Aether.md are
not accepted as partial syntax. They remain design constraints until a complete,
tested language stage introduces them.

## AI Boundary

Ollama assistance is optional and local-only. It is restricted to a loopback
HTTP endpoint and a conservative model-name character set. It can review source
only after a user request. It is never a compiler, evaluator, or code execution
authority.

## Quality Gate

The repository must pass Rust formatting, Aether core and desktop tests, Clippy
with warnings denied, TypeScript linting, frontend tests, production frontend
build, Windows Tauri bundle build, command-line artifact compile and run checks,
and a live Docker Ollama status check before release.
