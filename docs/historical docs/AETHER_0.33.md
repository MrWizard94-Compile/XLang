# Aether 0.33 Toolchain Contract (M23 pure comptime weave calls)

**Status:** Historical package contract — language **0.11** surface forms plus
M23 T-CT semantics; AETH **v11** unchanged; current package contract is
[AETHER_0.35.md](AETHER_0.35.md)
**Depends on:** ADR-008, ADR-019, ADR-039
**Validation:** [M23 validation matrix](../Current%20state/M23-VALIDATION-MATRIX.md)

## Surface

A root-level immutable directive may now call one earlier, pure helper:

```aether
comptime bind name <- call weave_name whole-argument...
```

The target must be a prior **guest** weave that is total, returns `Whole`, and
accepts only owned `Whole` parameters. Its M23 body is restricted to root
`bind`/`revise` of Whole locals and one terminal Whole `yield`; its expressions
are Whole atoms or one checked arithmetic operation. Each argument is a Whole
literal or a prior root-level comptime Whole name in the calling weave.

Every accepted call folds at compile time to the existing `COMPTIME_WHOLE` (56)
immediate. There is no new AETH opcode, VM instruction, host authority, or
runtime call edge. The program-wide 1,024 `comptime bind` budget is unchanged.

`examples/comptime-calls.ae` demonstrates an arithmetic bind followed by two
pure helper calls and exits 512.

## Product path

The Rust bootstrap remains the parser, semantic validator, and diagnostic
authority. For M23 source, it evaluates only the accepted restricted call and
materializes that call into an equivalent M5 literal comptime directive before
passing the program to the checked-in Aether seed compiler. The seed then emits
the normal v11 `COMPTIME_WHOLE` artifact. The product result must be
byte-identical to direct bootstrap emission.

This is a deliberately explicit seed bridge, not a claim that the checked-in
seed independently interprets raw M23 call syntax. The public `aether compile`
path remains seed-emitted after bootstrap validation/materialization.

## Rejections

M23 rejects host or foreign targets, `raises Whole` targets, non-Whole or
borrow/access parameters, non-Whole results, future targets, runtime arguments,
wrong arity, nested calls, `choose`/`while`, resource/effect/nursery forms, and
checked arithmetic failure in the helper. These fail closed with
`AE-COMPTIME-001` or `AE-COMPTIME-002` as appropriate.

## Non-goals

Recursion, mutual recursion, comptime control flow, nested expression trees,
Text/Bytes/Truth comptime values, source generation, macros, user-settable
fuel, host observation, foreign loading, and general metaprogramming remain
outside Aether 0.33.

*End of AETHER_0.33.md*
