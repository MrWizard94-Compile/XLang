# Aether Seed Profile

Status: normative Stage 4 self-hosting subset, 2026-07-28.

This document defines the **Seed Profile**: the only language subset for which
this repository claims reproducible self-hosting. Full Aether 0.4 remains
bootstrapped by the Rust core. A Seed Profile claim is not a full-language
self-hosting claim.

## Claim

`seed/aether_seed.ae` is an Aether-written compiler that:

1. Accepts Seed Profile source as `Text`.
2. Parses statements and expressions itself (no host parser callback).
3. Emits a complete AETH **v4** artifact through ordinary `Bytes` operations.
4. Exposes the forge ABI `weave compile [borrow source: Text] -> Bytes`.
5. Rebuilds its own source byte-for-byte under `aether forge`.
6. Compiles multi-weave Seed Profile programs with `call`, matching bootstrap
   output byte-for-byte.

Evidence lives in `crates/xlang-core/tests/seed_self_host.rs` and the checked-in
artifact `seed/aether_seed.aeth`.

## Required shape

A Seed Profile program must contain:

- One `world` line (name is accepted but not used by the seed emitter).
- One or more weaves declared at indentation level zero.
- Exactly one runnable main:

      weave main [] -> Whole:

The forge-facing compiler shape used by the seed itself remains:

      weave compile [borrow source: Text] -> Bytes:

Input programs are not required to declare `compile`. Weave indices in the
emitted artifact follow declaration order (0-based). Every weave body is
compiled from source, including `main`.

## Locals and parameters

- Parameters and locals use fixed slots named `vN` where `N` is a decimal whole
  and equals the slot index (`v0` is the first parameter or local).
- The compile parameter name `source` is also accepted as slot `0`.
- Parameter lists may include multiple entries separated by commas. Optional
  `borrow` ownership is accepted for `Text` / `Bytes` parameters; owned is the
  default.
- Result types are `Text`, `Whole`, `Truth`, or `Bytes`.
- Nested blocks may `revise` existing locals but must not introduce bindings.
- `bind` / `bind mutable` establish locals; `revise` replaces a live local.

## Statements

Supported forms:

| Form | Notes |
| --- | --- |
| `bind name <- expression` | Immutable local |
| `bind mutable name <- expression` | Mutable local |
| `revise name <- expression` | Same-type replacement |
| `speak expression` | Expression must be `Text` |
| `yield expression` | Root-only; result type must match the weave |
| `choose expression:` / `otherwise:` | Condition is `Truth` |
| `while expression:` | Condition is `Truth` |

Indentation is exactly two spaces per level. Control-flow jump targets are
patched with `poke32` after structured blocks close.

## Expressions

Expressions are shallow prefix forms. Operands are atoms (literals or
`borrow`/`move` names). Intermediate results must be bound before reuse.

Supported operations (by seed emitter opcode mapping):

- Unary: `not`, `measure`, `render`, `extent`, `encode`, `decode`, `number`,
  `pack16`, `pack32`, `pack64`
- Binary: `sum`, `difference`, `product`, `less`, `same`, `join`, `glyph`,
  `quotient`, `remainder`, `fuse`, `append`, `octet`, `unpack16`, `unpack32`
- Ternary: `cut`, `slice`, `seek`, `poke`, `poke32`
- Call: `call weave_name args...` emits `OP_CALL` (21), the callee's declaration
  index as `u16`, and argument count as `u8`. Callees must be declared before
  the call site (single-pass name table). Result type is the callee weave
  result.
- Atoms: decimal `Whole` literals (optional leading `-`), `bright` / `dim`,
  text literals, `bytes "hex..."`, and `borrow` / `move` of `source` or `vN`

## Multi-weave emission

The seed keeps a function-table accumulator. On each new weave declaration and
at end of source it flushes the previous weave record:

- name length and ASCII name
- parameter count, then `(type, mode)` pairs
- result type
- local count, then `(type, mutable)` pairs (parameters occupy the leading slots)
- code length and instruction bytes

The final artifact is `AETH` + version `4` + function count + the accumulated
function table. This replaces the Stage 3 fixed two-weave (`compile` + synthetic
`main`) emitter.

## Explicit non-goals

The Seed Profile compiler does **not** claim support for:

- Forward `call` to weaves declared later in the file (bootstrap full Aether
  allows any order; the seed is single-pass)
- Nested expression trees
- Nested binding introduction
- Host I/O, networking, or model access
- CRLF source normalization (Seed Profile sources should use LF line endings)
- Full Aether diagnostic fidelity (invalid Seed Profile input may fail late or
  produce a rejectable artifact; the bootstrap compiler remains the complete
  diagnostic authority for full Aether 0.4)
- Self-hosting of the complete language surface

## Reproducibility procedure

From the repository root:

```powershell
cargo run -p aether-cli -- compile .\seed\aether_seed.ae --output .\target\aether_seed.aeth
cargo run -p aether-cli -- forge .\target\aether_seed.aeth .\seed\aether_seed.ae --output .\target\aether_seed.forged.aeth
Get-FileHash .\target\aether_seed.aeth, .\target\aether_seed.forged.aeth, .\seed\aether_seed.aeth
```

All three SHA-256 digests must match. The regression test also forges a nearby
source variant and requires a different verified artifact so the compiler cannot
return a fixed stored payload. A second regression forges a multi-weave program
with `call` and requires byte identity with bootstrap plus a successful run.

## Authority

- Full language: [AETHER_0.4.md](AETHER_0.4.md)
- Host forge ABI: [FORGE_CONTRACT.md](FORGE_CONTRACT.md)
- Architecture: [ARCHITECTURE.md](ARCHITECTURE.md)
- Product gate: [../MANIFEST.md](../MANIFEST.md)
