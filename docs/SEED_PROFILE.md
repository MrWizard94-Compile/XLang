# Aether Seed Profile

Status: normative Stage 5 Seed Profile (complete canonical 0.4 source surface + product compile path), 2026-07-28.

This document defines the **Seed Profile** implemented by `seed/aether_seed.ae`.
It covers the complete documented canonical Aether 0.4 source surface. Product
`compile` uses this profile via the seed artifact. Rust bootstrap remains for
seed rebuild, `check` AST, and dual-compare proofs. A Seed Profile claim is not
a claim of full bootstrap diagnostic parity for invalid input.

Here, *canonical* means the Aether 0.4 grammar and formatting constraints in
[AETHER_0.4.md](AETHER_0.4.md): shallow prefix expressions, exact indentation,
and root-only bindings. The profile does not expand that language surface.

## Claim

`seed/aether_seed.ae` is an Aether-written compiler that:

1. Accepts complete canonical Aether 0.4 source as `Text`.
2. Parses statements and expressions itself (no host parser callback).
3. Emits a complete AETH **v4** artifact through ordinary `Bytes` operations.
4. Exposes the forge ABI `weave compile [borrow source: Text] -> Bytes`.
5. Rebuilds its own source byte-for-byte under `aether forge`.
6. Compiles every documented canonical statement, shallow expression, literal,
   ownership mode, and multi-weave program (including forward calls to weaves
   declared later), matching bootstrap output byte-for-byte.
7. Accepts CRLF or LF line endings, including a valid final source line without
   a line terminator; canonical emission is independent of host newline style.
8. Decodes the Aether text escapes `\\`, `\"`, `\n`, `\r`, and `\t` before
   recording UTF-8 byte lengths.

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

- Parameters and locals may use ordinary lowercase names. The seed assigns slots
  in declaration order and resolves names through a per-weave name map.
- The compile parameter name `source` remains slot `0` when present.
- `vN` names still work when bound/declared that way (the seed source itself uses
  them heavily).
- Parameter lists may include multiple entries separated by commas. Optional
  `borrow` ownership is accepted for `Text` / `Bytes` parameters; owned is the
  default.
- Result types are `Text`, `Whole`, `Truth`, or `Bytes`.
- Nested blocks may `revise` existing locals but must not introduce bindings.
- `bind` / `bind mutable` establish locals; `revise` replaces a live local.
- Hex `bytes "ff…"` literals decode to raw bytes. Text literals decode the five
  defined escapes (`\\`, `\"`, `\n`, `\r`, `\t`) and record **byte** length of
  UTF-8 content after decoding (not scalar count).

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
  index as `u16`, and argument count as `u8`. A first pass records every weave
  name and result type so forward calls are allowed. Result type is the callee
  weave result.
- Atoms: decimal `Whole` literals (optional leading `-`), `bright` / `dim`,
  text literals, `bytes "hex..."`, ordinary names for copyable values, and
  `borrow` / `move` of declared `Text` or `Bytes` locals and parameters

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

Seed parity does not change Aether 0.4 language rules. In particular, nested
expression trees and nested binding introduction remain outside the language
grammar rather than Seed Profile exclusions. The Seed Profile compiler does
**not** claim support for:

- Host I/O, networking, or model access
- Full Aether diagnostic fidelity (invalid Seed Profile input may fail late or
  produce a rejectable artifact; the bootstrap compiler remains the complete
  diagnostic authority for invalid Aether 0.4 input)
- Future Aether language extensions until they meet the same byte-identity proof

## Reproducibility procedure

From the repository root:

```powershell
cargo run -p aether-cli -- compile .\seed\aether_seed.ae --output .\target\aether_seed.aeth
cargo run -p aether-cli -- forge .\target\aether_seed.aeth .\seed\aether_seed.ae --output .\target\aether_seed.forged.aeth
Get-FileHash .\target\aether_seed.aeth, .\target\aether_seed.forged.aeth, .\seed\aether_seed.aeth
```

All three SHA-256 digests must match. The regression tests also forge:

1. A nearby source variant (different verified artifact — not a fixed payload).
2. A multi-weave program with `call` (byte identity with bootstrap + run).
3. A forward-call program (callee after caller) and a CRLF multi-weave source.
4. Every shipped `examples/*.ae` file seed-compiles byte-identically to bootstrap
   (`compile_with_seed`).
5. A canonical-surface corpus covering every statement, expression, ownership
   mode, literal mode, and final-line termination behavior.

## Authority

- Full language: [AETHER_0.4.md](AETHER_0.4.md)
- Host forge ABI: [FORGE_CONTRACT.md](FORGE_CONTRACT.md)
- Architecture: [ARCHITECTURE.md](ARCHITECTURE.md)
- Product gate: [../MANIFEST.md](../MANIFEST.md)
