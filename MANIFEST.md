# Aether Production Manifest

## Contract

Aether **0.5.0** accepts Aether source, returns a canonical AST from the Rust
bootstrap for tooling, and **emits AETH v4 bytecode primarily through the
Aether-written seed compiler** (forge ABI). The CLI and desktop app use the same
seed-hosted compile path. Source is never translated to an existing language.

## Implemented Language Boundary

Aether 0.5.0 language surface remains the 0.4.0 types and operations: `Text`,
`Whole`, `Truth`, and bounded `Bytes` with explicit `borrow`/`move` for unique
values. Root bindings receive fixed slots; nested blocks may revise but cannot
introduce bindings.

Bounded facilities include `encode`, `decode`, `extent`, `octet`, `slice`,
`fuse`, `append`, `seek`, `number`, `pack16`, `pack32`, `pack64`, `unpack16`,
`unpack32`, `poke`, and `poke32`.

Source is UTF-8. Names and keywords are lowercase ASCII. Canonical formatting
uses LF, exact two-space indentation, no tabs, and no trailing whitespace. CRLF
input is accepted by bootstrap formatters and by the seed line scanner; a valid
final source line need not end in a terminal LF.

AETH v4 only. Earlier versions are rejected.

## Compile Path Boundary

| Path | Role |
|------|------|
| **Seed (default)** | `compile_with_seed` / CLI `compile` / Studio build — Aether-written compiler |
| **Bootstrap** | `compile_to_bytecode` / CLI `compile --bootstrap` / `check` AST — rebuild seed, diagnostics |
| **Forge** | Host ABI only: `compile [borrow source: Text] -> Bytes` |

The seed artifact is checked in at `seed/aether_seed.aeth` and embedded as
`SEED_COMPILER_ARTIFACT` for offline deterministic product builds.

## Seed-Profile Self Hosting

`seed/aether_seed.ae` parses the complete documented canonical Aether 0.4
surface: all statement and shallow expression forms, named locals/params,
`borrow`/`move`, multi-weave `call` including forward callees, hex bytes
literals, UTF-8 text constants with the five defined escapes, and LF/CRLF input
with or without a final line terminator. It emits AETH v4 through ordinary
`Bytes` operations with no host parser callback.

Proofs in `crates/xlang-core/tests/seed_self_host.rs`:

1. Multi-generation self-host identity of the seed
2. Distinct source variant yields a different artifact
3. Multi-weave, forward-call, and CRLF fixtures match bootstrap
4. **All shipped `examples/*.ae` seed-compile byte-identically to bootstrap**
5. A complete canonical-surface corpus covering every statement, expression,
   ownership mode, literal mode, and accepted line termination matches bootstrap

This is full canonical Aether 0.4 source-emission parity, self-hosting of the
seed, and seed-hosted compilation of the shipped example corpus. It is **not** a
claim of full invalid-source diagnostic parity or of parity for future language
features without the same proof.

## AI Boundary

Ollama is optional, loopback-only, user-triggered review. Never compiler authority.

## Quality Gate

Rust fmt, Clippy `-D warnings`, core/CLI/seed tests, Studio lint/tests/build,
CLI seed-compile of examples, forge self-host hash check. Release also requires
Tauri bundle, optional live Ollama check, and package inspection.
