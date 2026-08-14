# Aether 0.7 Language Specification

**Status:** Historical Aether 0.7 specification. It records the M4 contract
that emitted deterministic AETH v7, verified every artifact before run or forge
write, and used the Aether-written seed compiler by default. The current
contract is [AETHER_0.8.md](AETHER_0.8.md).

**Date:** 2026-08-01

Aether source is never translated to Rust, C, JavaScript, LLVM, or another
language. Source is parsed as Aether, lowered to AETH, verified, and executed
only by the Aether VM.

This document superseded [AETHER_0.6.md](AETHER_0.6.md) as the then-current
language contract. Aether 0.4, 0.5, and 0.6 documents remain historical compatibility
references. Verified AETH v4/v5/v6 artifacts keep their original byte meanings
and remain accepted; new source compilation emits v7.

## Compatibility surface

The scalar, text, byte, immutable-record, ownership, control-flow, M2 arena,
and bounded `Buffer` rules from Aether 0.6 remain valid unless this document
adds a tighter M4 rule. `Text`, `Bytes`, immutable records, buffers, and arenas
remain unique/resource values. `Whole` and `Truth` remain copy values. All
source is UTF-8; canonical formatting uses LF, two-space indentation, no tabs,
and no trailing whitespace. Bootstrap and seed accept CRLF input and an optional
final line feed.

M2 remains a separate closed resource state machine: one root `Arena` in
`main`, `buffer Whole`/`buffer Truth`, access-only arena use, and terminal
resource `choose` forms. A resource owner cannot cross the host/forge ABI or be
a weave result. See [AETHER_0.6.md](AETHER_0.6.md) and
[ADR-004](ADR-004-aeth-v6-bounded-resources.md) for the unchanged M2 rules.

## M4: bounded `Error[Whole]`

M4 adds one explicit, abortive effect:

```text
Outcome[Whole] = return(Whole) | error(Whole)
```

It is not a general exception system, algebraic effect system, result-union
surface, effect row, dynamic handler lookup, resumption mechanism, host
exception route, or resource-cleanup mechanism.

### Source grammar

```text
weave-header ::= "weave" name parameters "->" result [ "raises Whole" ] ":"
raise        ::= "raise" whole-atom
forward      ::= "forward call" name { copy-atom }
handle       ::= "handle call" name { copy-atom } "into" destination
                 "otherwise error into" destination
```

The canonical M4 example is:

```aether
world error_effect

weave leaf [value: Whole] -> Whole raises Whole:
  raise value

weave forwarded [value: Whole] -> Whole raises Whole:
  forward call leaf value

weave main [] -> Whole:
  bind mutable success <- 0
  bind mutable code <- 0
  handle call forwarded 17 into success otherwise error into code
```

### Static rules

1. `raises Whole` declares `Error[Whole]`; a total weave omits it. Only a
   non-`main` weave may declare it.
2. An erroring weave returns `Whole` and accepts only ordinary owned `Whole` or
   `Truth` parameters. Its declaration is conservative: it may normally
   `yield` or terminally error.
3. `raise` takes exactly one `Whole` literal or copy binding and is the final
   root statement of an erroring weave.
4. `forward call` targets another erroring weave with the same `Whole` result
   and is the final root statement of an erroring weave.
5. Ordinary `call` targets only a total weave. A source attempt to call an
   erroring weave that way is `AE-EFFECT-001`.
6. `handle call` is the final root statement of a total `Whole` weave. It
   targets an erroring `Whole` weave and names two distinct existing mutable
   root `Whole` destinations. A normal exit overwrites the success destination
   and yields it; an error exit overwrites the error destination and yields it.
7. M4 control (`raise`, `forward`, or `handle`) cannot occur in a weave that
   uses an M2 resource form. At an M4 boundary there can be no live `Text`,
   `Bytes`, record, arena, buffer, borrow/access loan, or resource outcome.
   The failure is `AE-EFFECT-003`.

These restrictions make the two exits visible without requiring a hidden
cleanup path or a cross-frame owner transfer. M4 does not claim that resources
and effects are universally incompatible; it deliberately postpones that
interaction until destruction, joins, cancellation, and verifier state are
specified together.

### Diagnostics

M4 adds stable v2 diagnostic categories:

| Code | Meaning |
| --- | --- |
| `AE-EFFECT-001` | Error route is undeclared, unhandled, or crosses the total entry boundary. |
| `AE-EFFECT-002` | Effect target, terminal form, or result relationship is invalid. |
| `AE-EFFECT-003` | Effect control crosses an owner, loan, arena, buffer, or M2 boundary. |
| `AE-EFFECT-004` | Error code or destination is not a permitted live `Whole` copy value. |

The full rules and proof scope are
[DESIGN-M4-TYPED-ERROR-EFFECTS.md](DESIGN-M4-TYPED-ERROR-EFFECTS.md),
[ADR-007](ADR-007-m4-typed-error-effect.md), and
[M4-VALIDATION-MATRIX.md](M4-VALIDATION-MATRIX.md).

## AETH v7

New compilation emits:

```text
AETH | version=7:u8 | arena_capacity:u32-le | record_count:u16-le |
record table | function table
```

A v7 function metadata entry adds `effect_tag:u8` immediately after its result
type (including a record result descriptor if applicable):

```text
name | parameter descriptors | result type | effect_tag | local descriptors | code

effect_tag = 0  Total
effect_tag = 1  Error[Whole]
```

The v7-only instructions are:

| Opcode | Encoding | Verified contract |
| --- | --- | --- |
| `RAISE` (53) | no immediate operands | Erroring function; consumes `Whole`; empty remaining stack and clean effect boundary; no successor. |
| `FORWARD_CALL` (54) | `function:u16`, `argument_count:u8` | Erroring caller and callee; matching `Whole` result; copy-only arguments; clean boundary; no successor. |
| `HANDLE_CALL` (55) | `function:u16`, `argument_count:u8`, `success_slot:u16`, `error_slot:u16`, `success_target:u32`, `error_target:u32` | Total `Whole` caller, erroring `Whole` callee, live distinct mutable `Whole` slots, valid instruction targets, empty stack after arguments, and clean boundary. |

Only v7 reads an effect tag or decodes these opcodes. A v4/v5/v6 artifact that
contains their bytes in code is rejected as an unknown/unsupported instruction;
v4/v5/v6 metadata layout is not reinterpreted. The verifier rejects malformed
tags, invalid target slots, nonempty terminal stacks, owner/resource crossings,
unreachable code, and a non-total `main` before VM execution or forge write.

The VM represents a frame exit internally as `Return(RuntimeValue)` or
`ErrorWhole(i64)`. It does not use Rust panic, host exceptions, file/network/
process authority, ambient handlers, or resumable continuations. `main`, the
primitive invoke boundary, and the Forge `compile` ABI remain total.

## Structural authoring v2

`aether.ast/v2`, `aether.edit/v2`, and `aether.diagnostic/v2` are the current
local authoring contracts. The AST exposes each weave's `effect` and the
`Raise`, `Forward`, and terminal `Handle` nodes. A v2 edit includes `effect` in
an editable weave declaration and is re-rendered, bootstrap-validated,
seed-compiled, and AETH-verified before CLI output is written.

v1 remains a historical Aether 0.6 contract. The v2 endpoint rejects it rather
than silently guessing an upgrade. See
[AETHER_AUTHORING_PROTOCOL_v2.md](AETHER_AUTHORING_PROTOCOL_v2.md).

## Seed-hosted product compilation

The checked-in `seed/aether_seed.ae` source and `seed/aether_seed.aeth` artifact
parse and emit all documented canonical Aether 0.7 forms, including v7 effect
metadata and `RAISE`, `FORWARD_CALL`, and `HANDLE_CALL`. `compile_with_seed`
and CLI `compile` remain the product path. The Rust bootstrap remains the seed
rebuild and full invalid-source diagnostic authority.

The seed/bootstrap proof covers self-host identity, shipped examples (including
`examples/error-effect.ae`), the prior canonical surface, and M2 resource
fixtures byte-for-byte. It does not claim complete invalid-source diagnostic
parity for the seed or parity for unimplemented language extensions.

## Host and forge boundary

The primitive invoke API accepts and returns only primitive values. Records,
arenas, buffers, loans, resource outcomes, and `ErrorWhole` exits never cross
it. Forge accepts only a verified compiler artifact with
`compile [borrow source: Text] -> Bytes`, verifies the compiler and generated
artifact, then writes only to the explicit CLI output path.

## Deliberate non-goals

Aether 0.7 does not add effect inference, effect polymorphism, multiple error
kinds, `Error[Text]`, generic handlers, continuation resumption, recovery and
continue, cleanup/finally, async effects, cancellation, tasks, host effects,
resource/effect composition, FFI, native code generation, or a desktop/model/
network integration.

Any extension must keep the AETH-only execution model, verify-before-run/write,
seed-hosted product compile proof, zero-warning gate, and the Constitution's
research, documentation, and security obligations.
