# M4 Design: typed abortive error effect

**Status:** Implemented Aether 0.7 / M4 vertical slice. The bounded source,
AETH v7, VM, seed-hosted compiler, and v2 authoring contracts are exercised by
bootstrap-versus-seed proof tests.

**Date:** 2026-08-01

**Decision record:** [ADR-007](ADR-007-m4-typed-error-effect.md)

**Validation record:** [M4 validation matrix](M4-VALIDATION-MATRIX.md)

## Purpose and boundary

M4 establishes the smallest error/effect model that can demonstrate three
facts without adding a hidden exception channel:

1. a weave can explicitly declare a typed failure path;
2. a caller can explicitly handle it or explicitly forward it; and
3. a total caller is rejected when it tries to let that failure escape.

The model is deliberately an *abortive* error effect, not a general algebraic
effect system. It does not capture or resume a continuation, infer an effect
row, add asynchronous behavior, create a host capability, or weaken AETH
verification. The bounded choice is informed by Koka's explicit effect types
and handlers, while avoiding its general effect-row and resumption surface in
this first Aether increment. OCaml's documented unhandled-effect and
one-shot-continuation behavior reinforces why resumptions and linear resources
must not be introduced together without a separate proof.

Primary research sources, accessed 2026-08-01:

- [Koka language book — effect types and handlers](https://koka-lang.github.io/koka/doc/book.html)
- [OCaml effect-handler manual](https://ocaml.org/manual/effects.html)

The current executable product is Aether 0.7. Its runnable reference model is
`crates/xlang-core/tests/m4_error_effect_semantics.rs`; the product tests add
source, AETH, VM, seed, and authoring evidence around that model.

## Core claim and invariants

The sole M4 effect is `Error[Whole]`. A weave is either total or may abort with
one signed 64-bit error code. Its semantic result is:

```text
Outcome[T] = return(T) | error(Whole)
```

The following invariants are normative for the M4 product slice.

| ID | Invariant |
| --- | --- |
| M4-INV-001 | An error route is visible in the weave signature, `raise`, `forward`, or `handle` syntax. An ordinary `call` never transports `Error[Whole]`. |
| M4-INV-002 | `raise` is abortive and terminal. It has no captured continuation, resume operation, ambient handler lookup, or host-language exception fallback. |
| M4-INV-003 | A total weave may invoke an erroring weave only through an explicit terminal `handle`; an unhandled or undeclared route is a source error. |
| M4-INV-004 | A forwarding weave must declare exactly the error effect it forwards. A handler discharges the callee's error into a normal result. |
| M4-INV-005 | An error boundary may not contain a live owner, `borrow`/`access` loan, `Arena`, `Buffer`, or M2 closed resource outcome. This first slice does not cross ownership/resource state. |
| M4-INV-006 | Every AETH effect instruction is v7-only and is rejected unless its function signature, operand stack, branch slots, targets, and clean boundary state agree. v4/v5/v6 bytes retain their meanings. |
| M4-INV-007 | `main`, Forge's `compile`, and primitive host invocation remain total. An AETH artifact has no route to report an unhandled guest error through a host exception or host capability. |

## Proposed source surface

This accepted Aether 0.7 grammar favors explicit, machine-readable intent over
punctuation-heavy shorthand.

```text
weave-header ::= "weave" name parameters "->" result [ "raises Whole" ] ":"
statement    ::= existing-statement | raise | forward | handle
raise        ::= "raise" whole-atom
forward      ::= "forward call" name { atom }
handle       ::= "handle call" name { atom } "into" destination
                 "otherwise error into" destination
```

The signature phrase `raises Whole` denotes the single `Error[Whole]` effect.
It is not a generic type constructor, an inferred effect row, or an open
exception set. The explicit upper-case `Whole` fixes the payload to a copyable,
verifier-friendly value for M4.

### Handled path

```aether
weave parse_digit [value: Whole] -> Whole raises Whole:
  raise 10

weave main [] -> Whole:
  bind mutable parsed <- 0
  bind mutable code <- 0
  handle call parse_digit -1 into parsed otherwise error into code
```

`parse_digit` visibly permits `Error[Whole]`. `main` is total because the
handler turns either exit into an ordinary `Whole` result. The normal binding
(`parsed`) and error binding (`code`) are existing distinct mutable root
bindings. M4 immediately yields the selected binding, so there is no later
control-flow join.

### Forwarded path

```aether
weave parse_digit [value: Whole] -> Whole raises Whole:
  raise 10

weave parse_forwarded [value: Whole] -> Whole raises Whole:
  forward call parse_digit value
```

`forward` is terminal. A normal callee result becomes the caller's result; an
error becomes the caller's `Error[Whole]` result. There is no ordinary result
value after a forwarded call and no implicit propagation through `bind`,
`revise`, `yield`, or a normal expression.

### Rejected path

```aether
weave invalid [value: Whole] -> Whole:
  bind parsed <- call parse_digit value
  yield parsed
```

This is rejected with `AE-EFFECT-001`: `call parse_digit` may produce
`Error[Whole]`, while `invalid` is total and uses neither `handle` nor
`forward`. The compiler never lowers this shape into a hidden exception route.

## Static rules

### Function signatures and calls

1. A M4 erroring weave writes `raises Whole` exactly once. A total weave omits
   it.
2. `raises Whole` is permitted only on a non-`main` weave whose result is
   `Whole` and whose parameters are ordinary `Whole` or `Truth` copy values.
   This
   confines M4's first cross-weave error boundary to copy values.
3. A weave that reaches `raise` or `forward` must declare `raises Whole`.
   The annotation is a conservative may-error contract, so an erroring weave
   may also yield normally. A weave whose only effectful call is terminally
   handled remains total and must omit the declaration.
4. An ordinary `call` can target only a total weave. A `forward call` must
   target an erroring weave with the same result type as its caller. A `handle
   call` must target an erroring weave.
5. `raise`, `forward`, and `handle` are root-level terminal statements. They
   cannot be nested in `choose`/`while`, followed by another statement, or used
   as an expression.
6. A `handle` is one terminal statement, not a user-authored branch block. Its
   normal and error destinations are distinct existing mutable root `Whole`
   bindings; the VM yields the selected destination immediately. Both the
   caller and callee return `Whole` in M4.

### Ownership, arenas, and M2 outcomes

M4 intentionally proves *separation* before it proves generalized interaction:

- A weave containing `arena`, `buffer`, `access`, `allocate`, resource
  `append`, `at`, or a resource terminal `choose` cannot use `raises Whole`,
  `raise`, `forward`, or `handle`.
- At any M4 control boundary, every unique value (`Text`, `Bytes`, immutable
  record, `BufferWhole`, `BufferTruth`), arena capability, and ephemeral loan
  must be absent or already consumed. A remaining active owner/loan is
  `AE-EFFECT-003`.
- M4 function signatures intentionally exclude owners, loans, arenas, buffers,
  records, and `Text`/`Bytes`. The VM therefore never needs to decide whether a
  pending error owns a guest resource, and no error payload can cross the host
  boundary.

These restrictions are a correctness boundary, not an assertion that errors
and resources are fundamentally incompatible. A later extension may relax them
only after it specifies logical destruction, branch joins, cancellation, and
verifier state together.

### Diagnostics and AI authoring

M4 product work introduces `aether.diagnostic/v2` rather than silently adding
an incompatible code to v1. The minimum new stable diagnostics are:

| Code | Meaning |
| --- | --- |
| `AE-EFFECT-001` | An error route is unhandled or undeclared. |
| `AE-EFFECT-002` | A `raise`, `forward`, or `handle` form is malformed or violates its terminal/signature rule. |
| `AE-EFFECT-003` | An effect boundary crosses a live owner, loan, arena, buffer, or M2 resource outcome. |
| `AE-EFFECT-004` | An error payload, handler binding, or effect signature has the wrong type. |

Likewise, the complete semantic AST and edit payload must become
`aether.ast/v2` and `aether.edit/v2`. v1 remains a documented Aether 0.6
contract; it must not be reinterpreted as if it knew M4 nodes.

## Operational model

The semantic kernel uses only two exits:

```text
evaluate(yield value)     = return(value)
evaluate(raise code)      = error(code)
evaluate(forward call f)  = evaluate(f)
evaluate(handle call f)   = match evaluate(f) with
                            | return(value) -> return(value)
                            | error(code)   -> return(code)
```

No handler searches dynamically through arbitrary frames. `handle` names the
callee at the source and AETH instruction site; it is a bounded two-exit call.
This makes ownership rejection and verifier control-flow state finite, and it
avoids resumable-continuation semantics before structured concurrency exists.

## AETH v7 implementation contract

A completed M4 product increment will emit AETH v7. The v7 function table adds
an `effect_tag:u8` immediately after the existing result type:

```text
parameter descriptors | result type | effect_tag | local descriptors | code

effect_tag = 0  total
effect_tag = 1  Error[Whole]
```

The v7-only instructions are deliberately explicit:

| Opcode | Encoding | Verifier rule |
| --- | --- | --- |
| `RAISE` (53) | no immediate operands; pops `Whole` | Function tag is `Error[Whole]`; operand stack is otherwise empty; the effect boundary is clean; no successor exists. |
| `FORWARD_CALL` (54) | `function:u16`, `argument_count:u8` | Target tag is `Error[Whole]`; caller has the same result/effect; arguments are copy values; boundary is clean; no successor exists. |
| `HANDLE_CALL` (55) | `function:u16`, `argument_count:u8`, `success_slot:u16`, `error_slot:u16`, `success_target:u32`, `error_target:u32` | Target tag is `Error[Whole]`; caller/callee and slots are `Whole`; targets are valid; each target receives its designated initialized mutable root local; boundary is clean. |

Artifact parsing must preserve v4/v5/v6 layout and behavior byte-for-byte.
Only v7 decodes an effect tag or these opcodes. Unknown tags, malformed targets,
uninitialized handler slots, a nonempty stack at an abortive exit, a resource
state at an effect boundary, or an effect opcode in an older version must be
rejected before execution or forge write.

The VM represents a frame exit as `Return(RuntimeValue)` or `ErrorWhole(i64)`.
It does not use Rust `panic`, a host exception, host I/O, an ambient handler,
or a resumable guest continuation. `main`, forge `compile`, and host `invoke`
remain total; the verifier rejects an artifact that could let `Error[Whole]`
escape one of those boundaries.

## Seed, tooling, and compatibility proof

M4 became default product compilation only after all of these were delivered
as one atomic vertical slice:

1. Bootstrap parser, typed semantic plan, formatter, encoder/decoder,
   verifier, VM, forge review, and hostile-artifact tests for AETH v7.
2. The Aether-written seed parses and emits every canonical M4 form byte-for-
   byte identically to bootstrap, including its own v7 rebuild and forge proof.
3. `aether.ast/v2`, `aether.edit/v2`, and `aether.diagnostic/v2` represent the
   new signature and terminal statements without weakening v1 validation.
4. The current canonical 0.6 corpus remains v7-emittable without changed
   source semantics, and v4/v5/v6 artifact compatibility tests remain green.
5. The full M4 matrix, including handled, forwarded, rejected, ownership,
   resource, malformed-artifact, and no-host-escape cases, passes under the
   constitution gate.

## Deliberate non-goals

M4 does not add effect inference, rows, polymorphism, multiple error kinds,
user-declared effects, `Error[Text]`, checked result unions, generic handlers,
resumption, cleanup/finally handlers, asynchronous effects, cancellation,
tasks, host effects, resource/effect interleaving, FFI, or a native backend.
Those remain separately gated research decisions.

## Falsification and stop conditions

Do not implement this design as product surface if any of these becomes true:

1. handled, forwarded, and rejected paths cannot be explained from source and
   function metadata alone;
2. the verifier cannot prove effect targets, slots, stack state, and clean
   boundaries without trusting source compilation;
3. the seed cannot reproduce canonical effect artifacts byte-for-byte;
4. an effect boundary needs a hidden exception, host callback, ambient handler,
   resumable continuation, or special async coloring; or
5. a meaningful resource use case requires an unsound exception to
   M4-INV-005 rather than a separately specified ownership/cancellation model.

The executable semantic kernel, product tests, self-host proof, and authoring
contracts are the evidence point for this bounded Aether 0.7 feature. They do
not claim diagnostic parity for invalid M4 source or parity for future effects.
