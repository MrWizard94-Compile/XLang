# XLang Production Manifest

## Contract

The compiler accepts a source string, produces an AST when the bootstrap language
is valid, and reports a precise lex, parse, or type error otherwise. The command
line and desktop app call the same `xlang-core::compile_source` entry point.

## Current Language Boundary

The bootstrap supports monomorphic functions, typed and inferred `let` bindings,
primitive `Int`, `Bool`, `Str`, and `Void` values, calls, expressions, `return`,
`while`, and counted `for` loops. It intentionally does not yet implement
conditionals, assignment after declaration, arrays, references, structs, enums,
patterns, generics, or named-type resolution. These limits are compiler behavior,
not claims of future support.

## AI Boundary

Ollama assistance is optional and local-only. It is restricted to a loopback HTTP
endpoint and a conservative model-name character set. It can review source and
reported compiler diagnostics, but it is never treated as a compiler, evaluator,
or code execution authority.

## Quality Gate

The repository baseline must pass Rust tests, Clippy with warnings denied,
TypeScript linting, frontend tests, production frontend build, the Windows Tauri
bundle build, and a live Docker Ollama status and chat check before release.
