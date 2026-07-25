# Aether 0.2 Language Specification

## Status

Aether 0.2.0 is the executable Stage 1 language. Its compiler emits AETH v2
bytecode for the Aether VM and verifies every artifact before execution. This
spec describes the implemented boundary, not the full design vision in
`Aether.md`.

## Program Shape

An Aether source file is UTF-8 and starts with one world declaration followed by
one or more weave declarations.

```aether
world genesis

weave format_count [borrow label: Text, count: Whole] -> Text:
  bind count_text <- render count
  yield join borrow label borrow count_text

weave main [] -> Whole:
  bind label <- "Aether Stage 1: "
  bind message <- call format_count borrow label 2
  speak borrow message
  yield 0
```

A program must declare exactly one `main` weave with the exact signature:

```aether
weave main [] -> Whole:
```

The artifact can hold at most 256 weaves. A weave name, world name, parameter
name, and binding name begins with a lowercase ASCII letter and then uses only
lowercase ASCII letters, digits, or underscores. Keywords cannot be names.

## Canonical Source

- Aether accepts UTF-8 source. Identifiers and keywords are ASCII by design.
- Canonical source uses LF line endings. CRLF input is accepted and formats to LF.
- Each block level uses exactly two spaces. Tabs are invalid.
- Trailing whitespace is invalid. A blank line may not contain whitespace.
- Semicolons, braces, C-style declarations, `fn`, `let`, and legacy XLang forms
  are not Aether syntax.
- Text literals use double quotes and support only `\\`, `\"`, `\n`, `\r`,
  and `\t` escapes. A text literal is limited to 1,000,000 UTF-8 bytes.

## Types and Local Access

Stage 1 has exactly three value types.

| Type | Meaning | Ordinary name use |
| --- | --- | --- |
| `Text` | Bounded immutable UTF-8 text | Requires explicit `borrow name` or `move name`. |
| `Whole` | Signed 64-bit integer | Copies with `name`. |
| `Truth` | `bright` or `dim` | Copies with `name`. |

`borrow name` reads a bound local without consuming it. `move name` consumes
that local. Any later use or revision of a moved local is a compile error. A
move inside one branch also marks the local unavailable after the branch, because
it may have been moved.

This is the Stage 1 local MVS check. The VM stores bounded immutable text values,
but Aether does not yet expose allocator lifetime, arena, or destructor
semantics. Do not treat this stage as the final explicit-allocation design.

## Weaves and Calls

A weave has a result type and an optional comma-separated parameter list.

```aether
weave echo [borrow value: Text] -> Text:
  yield borrow value

weave decorate [value: Text] -> Text:
  bind decorated <- join "> " move value
  yield move decorated
```

A parameter written as `borrow name: Text` cannot consume the caller's Text.
A plain `name: Text` is owned. Calls to an owned Text parameter require either
a Text literal or `move name`. `Whole` and `Truth` parameters are copied.

A call is a shallow expression:

```aether
call weave_name atom atom
```

All arguments must match the declared count and type. Calls can target any
declared weave, including a weave declared later in the file. Runtime recursion
is bounded to a depth of 1,024 calls.

## Statements and Control Flow

Only a weave root can introduce bindings. This gives every binding a fixed local
slot and prevents conditional declaration shapes.

```aether
bind name <- expression
bind mutable name <- expression
revise name <- expression
speak expression
yield expression
choose expression:
  statements
otherwise:
  statements
while expression:
  statements
```

- `bind` introduces an immutable root binding.
- `bind mutable` introduces a root binding that `revise` may replace with a
  value of the same type.
- `revise` is allowed in roots and nested blocks, but only for a live mutable
  binding.
- `speak` requires `Text` and appends it to deterministic stdout.
- `choose` requires `Truth`; `otherwise` is optional but, when present, must
  be nonempty.
- `while` requires `Truth` and must have a nonempty body.
- A weave must end with one root-level `yield` whose value matches the weave
  result type. `yield` is not valid inside a nested block.

## Expressions

Expressions intentionally use a shallow prefix form. Operation inputs and call
arguments are atoms, not nested expressions. Bind an intermediate value before
using another operation.

| Form | Input | Result |
| --- | --- | --- |
| `not value` | Truth | Truth |
| `measure text` | Text | Whole scalar count |
| `render value` | Text, Whole, or Truth | Text |
| `sum left right` | Whole, Whole | Whole |
| `difference left right` | Whole, Whole | Whole |
| `product left right` | Whole, Whole | Whole |
| `less left right` | Whole, Whole | Truth |
| `same left right` | Two equal value types | Truth |
| `join left right` | Text, Text | Text |
| `glyph text index` | Text, Whole | Whole Unicode scalar code point or `-1` |
| `cut text start end` | Text, Whole, Whole | Text |
| `call weave args...` | Declared parameter list | Declared result |

`cut` clamps its scalar positions to the valid range and returns the half-open
range `[start, end)`. `measure`, `glyph`, and `cut` use Unicode scalar
positions, not raw UTF-8 byte offsets. `render bright` produces `"bright"` and
`render dim` produces `"dim"`. Whole arithmetic traps deterministically on
overflow.

## Artifact and Verification

A compiled artifact begins with the ASCII magic `AETH` followed by version byte
`2`. It contains function metadata, parameter ownership modes, local
descriptors, and bytecode. Text constants are length-prefixed UTF-8 with a
32-bit byte length and retain the 1,000,000-byte safety limit.

The verifier runs before the VM and rejects malformed headers, invalid UTF-8,
unknown opcodes, invalid local slots, reads before initialization, illegal
revision, use-after-move, stack underflow, incorrect operand types, invalid
calls, invalid jumps, control-flow state disagreement, and functions without a
valid terminal yield.

The Aether VM has no file, process, network, host-language evaluation, or cloud
AI capabilities. Its only Stage 1 observable effects are stdout and the
`main` whole exit value.

## Deliberately Not Present

The following design terms remain future work and are rejected rather than
partially accepted: explicit allocators, arenas, forge, records, arrays,
Struct-of-Arrays layouts, typed error sets, compile-time execution, C interop,
native code generation, package management, concurrency, and a compiler written
in Aether.

The current Rust core is a bootstrap implementation. Aether will be self-hosting
only when an Aether compiler is written in Aether, compiles to verified AETH, and
recompiles its own source to a reproducible artifact.
