# Aether 0.8 Language Contract

**Status:** Current executable product contract — M5 deterministic bounded
compile-time evaluation

**Artifact output:** AETH v8

**Compatibility input:** verified AETH v4, v5, v6, v7, and v8 artifacts

## Purpose

Aether 0.8 is a local-first, deterministic language for AI-primary authorship.
It parses only Aether source, emits only AETH bytecode, verifies every artifact
before execution or forge output, and runs artifacts only in the Aether VM. It
does not translate source into Rust, C, JavaScript, LLVM, or another language.

This version preserves Aether 0.7's scalar, byte, record, bounded-resource,
and bounded `Error[Whole]` effect contracts, then adds one explicit and
resource-bounded compile-time evaluation form. It is deliberately not a macro
system, a build-scripting system, or general compile-time execution. The
implemented M5 slice and the dependency-ordered next milestones are documented
in [ROADMAP.md](ROADMAP.md); future extensions beyond this bounded surface must
follow that roadmap and the constitution gates before they enter the product
contract.

## New M5 statement

Only a weave root may contain this immutable statement:

```text
comptime bind <name> <- <operation> <whole-literal> <whole-literal>
```

`<operation>` is exactly one of `sum`, `difference`, `product`, `quotient`, or
`remainder`. Both operands must be signed decimal `Whole` literals. The result
is a normal immutable `Whole` local available to later runtime statements.

```aether
world comptime_math

weave main [] -> Whole:
  comptime bind table_width <- product 16 8
  comptime bind header_size <- sum 12 4
  bind total <- sum table_width header_size
  yield total
```

The formatter prints this form exactly as shown. Existing `bind` and `bind
mutable` forms remain runtime bindings; no stage is inferred from the value.

## Compile-time semantics and limits

Every accepted directive evaluates one literal operation with Aether's checked
signed 64-bit `Whole` arithmetic. Addition, subtraction, multiplication,
division, and remainder use the same overflow behavior as the VM. Division or
remainder by zero and all overflow are source errors.

At most **1,024** `comptime bind` directives may occur in one source program.
This fixed program-wide budget is not configurable from source, a CLI flag, or
an artifact. There is no recursion, loop, call, named dependency, branch,
text, bytes, record, arena, buffer, effect, host callback, file, process,
network, shell, model, or allocator access in the M5 evaluator.

| Diagnostic | Meaning |
| --- | --- |
| `AE-COMPTIME-001` | The directive is mutable, nested, malformed, or outside the literal Whole arithmetic subset. |
| `AE-COMPTIME-002` | The operation overflows or divides/remainders by zero. |
| `AE-COMPTIME-003` | The program exceeds the fixed 1,024-directive budget. |

The Rust bootstrap is the invalid-source diagnostic authority. The seed must
match it byte-for-byte for valid documented M5 sources; full invalid-source
diagnostic parity is not claimed.

## Preserved language surface

The Aether 0.7 contract remains in force:

- `Text`, `Whole`, `Truth`, and bounded `Bytes` use explicit ownership modes.
- Immutable nominal records have primitive non-recursive fields and remain
  inside Aether's primitive-only host boundary.
- `main` owns the one bounded arena capability; `buffer Whole` and `buffer
  Truth` use closed terminal allocation, append, and lookup outcomes.
- `raises Whole`, `raise`, `forward call`, and terminal one-line `handle call`
  are the sole M4 effect forms. Effect control cannot cross owners, loans,
  arenas, buffers, or resource outcomes.
- Root bindings receive deterministic slots. Nested blocks can `revise` but
  cannot introduce bindings.

See [AETHER_0.7.md](AETHER_0.7.md) for the historical 0.7 specification and
[DESIGN-M5-DETERMINISTIC-COMPTIME.md](DESIGN-M5-DETERMINISTIC-COMPTIME.md) for
the M5 design and stop conditions.

## AETH v8

New compilation writes AETH v8:

```text
AETH | version=8:u8 | arena_capacity:u32-le | record_count:u16-le |
record table | function table
```

The v8 header and function `effect_tag` layout are unchanged from v7. It adds
one v8-only instruction:

| Opcode | Encoding | Contract |
| --- | --- | --- |
| `COMPTIME_WHOLE` (56) | `value:i64-le` | Pushes a verified `Whole` from an accepted `comptime bind`. |

`COMPTIME_WHOLE` has the runtime value behavior of `PUSH_WHOLE`, but preserves
explicit source-stage provenance for artifact inspection. It is valid only in
v8. The marker is not a cryptographic proof of original source text.

v4 through v7 retain their original format meanings and can be verified and
run as compatibility inputs. A v8 opcode in any earlier version, an earlier
opcode used outside its version, malformed metadata, invalid control flow, or
an invalid stack state is rejected before VM execution or forge write.

## Seed and authoring contracts

Default `aether compile` invokes the checked-in Aether-written seed compiler.
The seed parses canonical `comptime bind`, computes the five accepted literal
operations using ordinary Aether arithmetic, and emits `COMPTIME_WHOLE` plus a
normal local store. Seed and bootstrap output for the shipped M5 example are
byte-identical.

Structural tooling is versioned as `aether.ast/v3`, `aether.edit/v3`, and
`aether.diagnostic/v3`. Every `Bind` node has a required `stage` property with
the exact value `runtime` or `comptime`. v1 and v2 documents are historical and
are rejected rather than guessed or silently upgraded. See
[AETHER_AUTHORING_PROTOCOL_v3.md](AETHER_AUTHORING_PROTOCOL_v3.md).

## Explicit non-goals

0.8 does not implement macros, generated text or AST expansion, user-defined
compile-time functions, compile-time control flow, generic type computation,
layout rewriting, build scripts, package execution, general effects,
concurrency, foreign interfaces, or a native backend. Each requires a separate
decision, threat model, seed proof, and AETH/version contract if it changes the
product surface.
