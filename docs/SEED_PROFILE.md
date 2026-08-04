# Aether Seed Profile

Status: normative Stage 7/M4–M7 Seed Profile (complete canonical 0.10 source
surface + product compile path), 2026-08-04.

This document defines the **Seed Profile** implemented by `seed/aether_seed.ae`.
It covers the complete documented canonical Aether 0.10 source surface. Product
`compile` uses this profile via the seed artifact. Rust bootstrap remains for
seed rebuild, `check` AST, and dual-compare proofs. A Seed Profile claim is not
a claim of full bootstrap diagnostic parity for invalid input.

Here, *canonical* means the Aether 0.10 grammar and formatting constraints in
[AETHER_0.10.md](AETHER_0.10.md): shallow prefix expressions, exact indentation,
root-only bindings, bounded immutable records, closed bounded-resource forms,
dual-layout tables, bounded terminal effect forms, literal `comptime bind`, and
structured nurseries. The profile does not expand that language surface.

## Claim

`seed/aether_seed.ae` is an Aether-written compiler that:

1. Accepts complete canonical Aether 0.10 source as `Text`.
2. Parses statements and expressions itself (no host parser callback).
3. Emits a complete AETH **v10** artifact with a resource-capacity header,
   optional record table, shape table, function effect metadata, table opcodes,
   and nursery opcodes through ordinary `Bytes` operations.
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
   output byte-for-byte for the documented canonical corpus.
10. Parses `raises Whole`, terminal `raise`, `forward call`, and one-line
    terminal `handle call ... into success otherwise error into code`; it emits
    `RAISE` (53), `FORWARD_CALL` (54), and `HANDLE_CALL` (55) byte-for-byte
    like the bootstrap.
11. Parses root-only immutable `comptime bind` with one `sum`, `difference`,
    `product`, `quotient`, or `remainder` operation over literal `Whole`
    operands, evaluates it with checked Aether arithmetic, and emits
    `COMPTIME_WHOLE` (56) byte-for-byte like the bootstrap.
12. Parses `shape` declarations and `table Shape layout rows|columns` with
    closed allocate/store/load; emits table opcodes (57–61) byte-for-byte like
    the bootstrap for the documented M6 corpus.
13. Parses lexical `together:` nurseries with `spawn call ... into` lines;
    emits nursery opcodes (62–64) byte-for-byte like the bootstrap for the
    documented M7 corpus.

Evidence lives in `crates/xlang-core/tests/seed_self_host.rs` and the checked-in
artifact `seed/aether_seed.aeth`.

## Required shape

A Seed Profile program must contain:

- One `world` line (name is accepted but not used by the seed emitter).
- Zero or more `record` and `shape` declarations after `world` and before the
  first weave.
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

## Explicit non-claims

- Full invalid-source diagnostic parity with the Rust bootstrap is **not**
  claimed.
- Parallel OS-thread execution is **not** claimed for M7 nurseries.
- Automatic layout rewriting and general generics are **not** claimed for M6.
