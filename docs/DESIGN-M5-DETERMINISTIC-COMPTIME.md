# M5 Design: deterministic bounded compile-time evaluation

**Status:** Accepted implementation design; product implementation is in
progress.

**Date:** 2026-08-01

**Decision record:** [ADR-008](ADR-008-m5-deterministic-comptime.md)

**Validation record:** [M5 validation matrix](M5-VALIDATION-MATRIX.md)

## Purpose and boundary

M5 proves that Aether can evaluate ordinary Aether arithmetic during
compilation without introducing a macro expander, a second source language, a
host callback, or unbounded compiler work. This is intentionally a small,
complete first evaluator rather than a disguised general metaprogramming
system.

Zig demonstrates an explicit compile-time context and a branch quota; Rust
demonstrates that a compile-time context should admit a restricted expression
set. Aether borrows the explicitness and restriction, but does not expose a
user-adjustable quota: AI-generated input must not be able to raise the
compiler's resource budget.

Primary research sources, accessed 2026-08-01:

- [Zig language reference: comptime and evaluation quota](https://ziglang.org/documentation/master/)
- [Rust Reference: constant evaluation](https://doc.rust-lang.org/stable/reference/const_eval.html)

## Core claim and invariants

The M5 product slice makes one compile-time result both executable and
auditable. Its source form is normal Aether arithmetic marked with an explicit
stage; its artifact form is a v8 provenance instruction; its runtime meaning is
the ordinary `Whole` value.

| ID | Invariant |
| --- | --- |
| M5-INV-001 | `comptime bind` is explicit, root-only, immutable, and produces a normal immutable `Whole` local for later runtime use. |
| M5-INV-002 | The accepted evaluator grammar contains only a single literal `Whole` arithmetic operation: `sum`, `difference`, `product`, `quotient`, or `remainder`. It accepts no names, calls, loops, records, owners, buffers, arenas, effects, text, or bytes. |
| M5-INV-003 | Each directive consumes exactly one fixed evaluation unit. At most 1,024 directives occur in a source program, so compile-time work is bounded independently of a user-controlled setting. |
| M5-INV-004 | Arithmetic uses the same signed-64-bit checked semantics as the Aether VM. Overflow and division by zero are compilation errors, never host panics or undefined values. |
| M5-INV-005 | M5 evaluation is pure: it reads only parsed literal operands, has no file, process, network, shell, model, guest allocator, or forge-writing authority. |
| M5-INV-006 | AETH v8 represents each evaluated result with `COMPTIME_WHOLE`, so the artifact preserves the source-stage provenance instead of silently looking like a runtime expression. v4-v7 keep their immutable meanings. |
| M5-INV-007 | The Aether-written seed evaluates and emits the exact M5 subset byte-for-byte like the bootstrap before M5 joins default product compilation. |
| M5-INV-008 | A structural document and edit request name the binding stage explicitly in v3; no tool infers whether an arithmetic binding was intended to be evaluated early. |

## Source surface

```text
comptime-bind     ::= "comptime bind" name "<-" whole-arithmetic
whole-arithmetic  ::= arithmetic-op whole-literal whole-literal
arithmetic-op     ::= "sum" | "difference" | "product" | "quotient" | "remainder"
whole-literal     ::= existing signed decimal Whole literal
```

Example:

```aether
world comptime_math

weave main [] -> Whole:
  comptime bind table_width <- product 16 8
  comptime bind header_size <- sum 12 4
  yield sum table_width header_size
```

The result is a regular immutable `Whole` local. It is not a compile-time-only
name, an implicit constant fold, or a user-defined macro. Existing runtime
`bind` syntax remains runtime syntax; the new statement starts with
`comptime`, which was not a previously valid statement form.

The explicit rejection surface is equally important:

```aether
comptime bind invalid <- sum table_width 4
comptime bind invalid <- less 1 2
comptime bind invalid <- join "a" "b"
```

All three are outside M5. Dependency chaining, boolean values, dynamic data,
ordinary weave calls, compile-time control flow, and code generation remain
future research decisions. This first slice proves a bounded evaluator kernel
before any of those interaction surfaces are admitted.

## Evaluation and diagnostics

The evaluator has no heap-facing Aether values and no recursive execution. It
accepts the two parsed literals, applies the specified checked operation, and
returns one signed `Whole` result. Because each accepted form is exactly one
operation, the directive count is the complete fuel model.

| Code | Meaning |
| --- | --- |
| `AE-COMPTIME-001` | The statement is misplaced, mutable, or outside the M5 literal arithmetic subset. |
| `AE-COMPTIME-002` | A compile-time arithmetic operation overflows or divides by zero. |
| `AE-COMPTIME-003` | The source exceeds the fixed 1,024-directive evaluation budget. |

The bootstrap produces these source diagnostics before the seed is invoked.
The Seed Profile remains intentionally honest: it must emit valid canonical M5
programs byte-identically, but it does not claim complete invalid-source
diagnostic parity.

## AETH v8 provenance contract

New compilation emits AETH v8. Its header and function metadata retain the v7
arena-capacity, record-table, and effect-tag layout exactly:

```text
AETH | version=8:u8 | arena_capacity:u32-le | record_count:u16-le |
record table | function table
```

The v8-only instruction is:

| Opcode | Encoding | Verified and runtime contract |
| --- | --- | --- |
| `COMPTIME_WHOLE` (56) | `value:i64-le` | Pushes a `Whole` whose source was evaluated by an accepted M5 directive. It is valid only in AETH v8 and is type-checked as a `Whole` stack value. |

`COMPTIME_WHOLE` deliberately has the same VM value semantics as a normal
`PUSH_WHOLE`, but it is not an optimizer hint. It is the bounded provenance
marker that lets an artifact inspection distinguish explicit compile-time work
from ordinary runtime arithmetic. The verifier does not claim cryptographic
proof of original source text; it enforces version, encoding, stack type, and
artifact integrity before execution or forge write.

## Seed and authoring contract

The seed recognizes canonical `comptime bind` statements, evaluates their
literal operands with ordinary Aether `Whole` arithmetic, discards the temporary
runtime-expression emission, and emits `COMPTIME_WHOLE` plus the normal local
store. Its own compiler source need not use M5 directives, so the self-host
proof remains a direct test of the new parser/emitter rather than a privileged
bootstrap path.

Structural authoring advances to `aether.ast/v3`, `aether.edit/v3`, and
`aether.diagnostic/v3`. A `Bind` node has a required `stage` property whose
value is `runtime` or `comptime`; a v3 edit payload must state that property.
v1 and v2 remain historical contracts and are rejected rather than silently
reinterpreted.

## Falsification and stop conditions

Do not promote this design into the default product path if any of the
following becomes necessary:

1. textual token or AST expansion that bypasses ordinary parsing and v3
   structure;
2. a host file, process, network, model, shell, allocator, or callback access;
3. a user-adjustable or unbounded evaluation quota;
4. unchecked arithmetic, a Rust panic, or different bootstrap/seed results;
5. artifact provenance that cannot survive bytecode verification; or
6. a seed implementation that cannot reproduce every canonical M5 artifact
   byte-for-byte.

The deliberately small M5 evaluator does not settle future compile-time calls,
control flow, type-level computation, layout rewriting, package scripts, or
code generation. Each requires a separate bounded design and proof.

