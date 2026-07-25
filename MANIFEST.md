# Aether Production Manifest

## Contract

The compiler accepts Aether 0.3 source, returns a canonical AST, emits an AETH
v3 artifact, verifies it, and runs that artifact in the Aether VM. The command
line and desktop app use the same compiler core. Source is never translated to
Rust, C, JavaScript, LLVM, or another language.

## Implemented Language Boundary

Aether 0.3.0 accepts one `world` declaration and one or more named `weave`
declarations. `main` must be `weave main [] -> Whole:`. The type set is `Text`,
`Whole`, `Truth`, and bounded `Bytes`.

`Text` and `Bytes` bindings require explicit `borrow name` or `move name`; a
move makes the local unavailable on every subsequent reachable control-flow
path. `Whole` and `Truth` are copied by ordinary name use. Nested blocks cannot
introduce a binding, which keeps slots fixed and control-flow state verifiable.

`Bytes` values use lowercase or uppercase hexadecimal literals in the form
`bytes "0011aaff"`, are capped at 1,000,000 decoded bytes, and are manipulated
only through bounded byte operations. `decode` rejects invalid UTF-8 at runtime;
`append` rejects a value outside `0..=255`; `slice` clamps byte offsets; and
`octet` returns `-1` for an out-of-range index. `quotient` and `remainder`
reject division by zero and signed overflow.

Source is UTF-8. Names and language keywords remain lowercase ASCII. Canonical
formatting uses LF, exact two-space indentation, no tabs, and no trailing
whitespace. CRLF input is accepted and formatted as LF.

The AETH v3 verifier checks headers, function metadata, local initialization and
mutability, stack types, move state, jump targets, control-flow convergence,
call signatures, text and bytes constant limits, and terminal yields before
execution. AETH v2 artifacts are intentionally rejected.

## Forge Boundary

`aether forge <compiler-artifact> <source-file> --output <artifact-file>` is a
strict local bridge. It verifies the compiler artifact, requires the exact
compiler ABI `compile [borrow source: Text] -> Bytes`, supplies the source as
the only host argument, verifies the returned bytes as AETH, and only then writes
the output file. The compiler weave may emit diagnostic text, but it has no host
file, process, network, model, or artifact-writing authority.

Stage 2 is not a claim that an Aether compiler has been written in Aether or
that Aether is self-hosting. The current Rust implementation remains the
bootstrap compiler and VM. Self-hosting requires an Aether compiler source,
verified compiler artifact, and reproducible recompilation evidence.

## AI Boundary

Ollama assistance is optional and local-only. It is restricted to a loopback
HTTP endpoint and a conservative model-name character set. It can review source
only after a user request. It is never a compiler, evaluator, code execution
authority, or artifact signer.

## Quality Gate

The repository must pass Rust formatting, all core, CLI, and desktop tests,
Clippy with warnings denied, TypeScript linting, frontend tests, production
frontend build, Windows Tauri bundle build, command-line checks for Stage 2
examples, forge contract verification, a live Docker Ollama status check, and
package inspection plus launch before release.
