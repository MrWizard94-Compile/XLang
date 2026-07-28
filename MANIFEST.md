# Aether Production Manifest

## Contract

The bootstrap compiler accepts Aether 0.4 source, returns a canonical AST,
emits an AETH v4 artifact, verifies it, and runs it in the Aether VM. The CLI
and desktop app use the same core. Source is never translated to an existing
language.

## Implemented Language Boundary

Aether 0.4.0 has `Text`, `Whole`, `Truth`, and bounded `Bytes`. `Text` and
`Bytes` require explicit `borrow name` or `move name`; `Whole` and `Truth` copy
by ordinary name use. Root bindings receive fixed slots and nested blocks may
revise but cannot introduce bindings.

The bounded binary and text facilities are `encode`, `decode`, `extent`,
`octet`, `slice`, `fuse`, `append`, `seek`, `number`, `pack16`, `pack32`,
`pack64`, `unpack16`, `unpack32`, `poke`, and `poke32`. Range violations,
invalid numeric text, invalid UTF-8, and invalid fixed-width byte access fail
deterministically at runtime.

Source is UTF-8. Names and keywords are lowercase ASCII. Canonical formatting
uses LF, exact two-space indentation, no tabs, and no trailing whitespace.
CRLF input is accepted and formats to LF.

The AETH v4 verifier checks headers, function metadata, local initialization
and mutability, stack types, move state, jump targets, control-flow convergence,
call signatures, text and byte limits, and terminal yields before execution.
Versions prior to v4 are intentionally rejected.

## Forge Boundary

`aether forge <compiler-artifact> <source-file> --output <artifact-file>` is a
strict local bridge. It verifies the compiler artifact, requires the exact
`compile [borrow source: Text] -> Bytes` ABI, supplies source as the only host
argument, verifies the yielded bytes as AETH, and only then writes an artifact.
The invoked compiler has no file, process, network, model, shell, or direct
artifact-writing authority.

## Seed-Profile Self Hosting

`seed/aether_seed.ae` is source in Aether itself. It parses the documented
Seed Profile (Stage 4: multi-weave programs with `call`, including forward
calls, plus CRLF input), derives local descriptors and instruction bytes per
weave, patches structured-control-flow offsets using `poke32`, and constructs an
AETH v4 function table through normal `Bytes` operations. It does not call a
host parser, compiler, source generator, or fixed-artifact lookup.

The checked-in `seed/aether_seed.aeth` is reproduced by both the Rust bootstrap
compiler and the Aether seed compiler. `crates/xlang-core/tests/seed_self_host.rs`
proves multi-generation self-host identity, a distinct valid source variant,
byte-identical multi-weave + `call` forge (including forward callees), and CRLF
input parity with LF bootstrap. This supports a self-hosting claim only for the
Seed Profile. Full Aether remains bootstrap-compiled until a complete Aether
compiler has the same proof.

## AI Boundary

Ollama assistance is optional and local-only. It is restricted to a loopback
HTTP endpoint and validated model names. It may review source after a user
request, but is never a compiler, evaluator, execution authority, or artifact
signer.

## Quality Gate

The release gate requires Rust formatting, workspace tests including the
self-hosting proof, Clippy with warnings denied, frontend linting and tests,
production frontend build, Windows Tauri bundle build, CLI compile/forge/run
checks, Docker Ollama status verification, and package inspection plus launch.
