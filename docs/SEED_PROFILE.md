# Aether Seed Profile

Status: normative Stage 7 Seed Profile (complete canonical 0.6 source surface +
product compile path), 2026-07-31.

This document defines the **Seed Profile** implemented by `seed/aether_seed.ae`.
It covers the complete documented canonical Aether 0.6 source surface. Product
`compile` uses this profile via the seed artifact. Rust bootstrap remains for
seed rebuild, `check` AST, and dual-compare proofs. A Seed Profile claim is not
a claim of full bootstrap diagnostic parity for invalid input.

Here, *canonical* means the Aether 0.6 grammar and formatting constraints in
[AETHER_0.6.md](AETHER_0.6.md): shallow prefix expressions, exact indentation,
root-only bindings, bounded immutable records, and the closed bounded-resource
forms. The profile does not expand that language surface.

## Claim

`seed/aether_seed.ae` is an Aether-written compiler that:

1. Accepts complete canonical Aether 0.6 source as `Text`.
2. Parses statements and expressions itself (no host parser callback).
3. Emits a complete AETH **v6** artifact with a resource-capacity header and
   optional record table through ordinary `Bytes` operations.
4. Exposes the forge ABI `weave compile [borrow source: Text] -> Bytes`.
5. Rebuilds its own source byte-for-byte under `aether forge`.
6. Compiles every documented canonical statement, shallow expression, literal,
   ownership mode, and multi-weave program (including forward calls to weaves
   declared later), matching bootstrap output byte-for-byte.
7. Accepts CRLF or LF line endings, including a valid final source line without
   a line terminator; canonical emission is independent of host newline style.
8. Decodes the Aether text escapes `\\`, `\"`, `\n`, `\r`, and `\t` before
   recording UTF-8 byte lengths.
9. Parses bounded immutable record declarations, constructors, and explicit
   borrowed field projections, plus `arena`, Whole/Truth `buffer`, `access`,
   `count`, and closed allocation/append/lookup outcomes; it matches bootstrap
   v6 output byte-for-byte for the documented canonical corpus.

Evidence lives in `crates/xlang-core/tests/seed_self_host.rs` and the checked-in
artifact `seed/aether_seed.aeth`.

## Required shape

A Seed Profile program must contain:

- One `world` line (name is accepted but not used by the seed emitter).
- Zero or more `record` declarations after `world` and before the first weave.
  Each record has one to 64 primitive fields in declaration order.
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
  `borrow` ownership is accepted for owners, `access` is accepted only for an
  `Arena` parameter, and owned is the default. A Buffer parameter requires
  exactly one access Arena parameter in the same weave.
- Result types are `Text`, `Whole`, `Truth`, `Bytes`, or a declared record name.
  `Arena`, access loans, `BufferWhole`, and `BufferTruth` are not result types.
- Nested blocks may `revise` existing locals but must not introduce bindings.
- `bind` / `bind mutable` establish locals; `revise` replaces a live local.
- Hex `bytes "ff…"` literals decode to raw bytes. Text literals decode the five
  defined escapes (`\\`, `\"`, `\n`, `\r`, `\t`) and record **byte** length of
  UTF-8 content after decoding (not scalar count).
- `arena N` appears only in `main` and becomes the one v6 resource-plan
  capacity. `buffer Whole` and `buffer Truth` establish unallocated owner
  placeholders. Resource owner replacement uses only closed outcomes, never
  `revise`.

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
  `pack16`, `pack32`, `pack64`, `count borrow buffer`
- Binary: `sum`, `difference`, `product`, `less`, `same`, `join`, `glyph`,
  `quotient`, `remainder`, `fuse`, `append`, `octet`, `unpack16`, `unpack32`
- Ternary: `cut`, `slice`, `seek`, `poke`, `poke32`
- Records: `make record-name fields...` constructs every declared field in
  order; `field borrow record-binding field-name` projects a cloned immutable
  field.
- Call: `call weave_name args...` emits `OP_CALL` (21), the callee's declaration
  index as `u16`, and argument count as `u8`. A first pass records every weave
  name and result type so forward calls are allowed. Result type is the callee
  weave result.
- Atoms: decimal `Whole` literals (optional leading `-`), `bright` / `dim`,
  text literals, `bytes "hex..."`, ordinary names for copyable values, and
  `borrow` / `move` of owner locals and parameters, plus `access` of a live
  Arena only in a resource operation or access-parameter call.
- Resources: `allocate access arena move buffer count into buffer`,
  `append move buffer value into buffer`, and `at borrow buffer index into
  target`. Each is emitted only from a terminal resource `choose`, has an
  explicit `otherwise` branch, and lowers to direct v6 replacement/update
  instructions.

## Multi-weave emission

The seed keeps a function-table accumulator. On each new weave declaration and
at end of source it flushes the previous weave record:

- name length and ASCII name
- parameter count, then `(type, mode)` pairs
- result type
- local count, then `(type, mutable)` pairs (parameters occupy the leading slots)
- code length and instruction bytes

The final artifact is `AETH` + version `6` + arena capacity (`u32` little
endian) + bounded record schema table + function table. Record type descriptors
retain tag `5` plus a record identifier; v6 also uses Arena/Buffer type tags,
access parameter mode, and the closed resource instruction payloads. This
replaces the Stage 3 fixed two-weave (`compile` + synthetic `main`) emitter.

## Explicit non-goals

Seed parity does not expand Aether 0.6 language rules. In particular, nested
expression trees, nested binding introduction, nested record fields, record
mutation, host record invocation, first-class resource outcomes, Buffer weave
results, and resource-owner `revise` remain outside the language grammar rather
than Seed Profile exclusions. The Seed Profile compiler does **not** claim
support for:

- Host I/O, networking, or model access
- Full Aether diagnostic fidelity (invalid Seed Profile input may fail late or
  produce a rejectable artifact; the bootstrap compiler remains the complete
  diagnostic authority for invalid Aether 0.6 input)
- Future Aether language extensions until they meet the same byte-identity proof

## Reproducibility procedure

From the repository root:

```powershell
cargo run -p aether-cli -- compile .\seed\aether_seed.ae --output .\target\aether_seed.aeth --bootstrap
cargo run -p aether-cli -- forge .\target\aether_seed.aeth .\seed\aether_seed.ae --output .\target\aether_seed.forged.aeth
Get-FileHash .\target\aether_seed.aeth, .\target\aether_seed.forged.aeth, .\seed\aether_seed.aeth
```

All three SHA-256 digests must match. The regression tests also forge:

1. A nearby source variant (different verified artifact — not a fixed payload).
2. A multi-weave program with `call` (byte identity with bootstrap + run).
3. A forward-call program (callee after caller) and a CRLF multi-weave source.
4. Every shipped `examples/*.ae` file seed-compiles byte-identically to bootstrap
   (`compile_with_seed`).
5. A prior canonical-surface corpus covering every statement, expression,
   ownership mode, literal mode, record operation, and final-line termination
   behavior.
6. The canonical M2 arena/buffer corpus: Whole allocation/append/lookup,
   allocation exhaustion, append full, lookup fallback, Truth elements, and an
   access-bound helper weave.

## Authority

- Full language: [AETHER_0.6.md](AETHER_0.6.md)
- Host forge ABI: [FORGE_CONTRACT.md](FORGE_CONTRACT.md)
- Architecture: [ARCHITECTURE.md](ARCHITECTURE.md)
- Product gate: [../MANIFEST.md](../MANIFEST.md)
