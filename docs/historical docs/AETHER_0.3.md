# Aether 0.3 Language Specification

Status: executable Stage 2 Forge Foundation specification, 2026-07-25.

Aether 0.3.0 emits AETH v3 bytecode for the Aether VM and verifies every
artifact before it can run. It is a new language with an Aether-owned artifact
format; source is not translated to another language.

## Source Shape

An Aether source file is UTF-8 and starts with one world declaration followed by
one or more named weaves.

    world binary

    weave package [borrow source: Text] -> Bytes:
      bind encoded <- encode borrow source
      bind marked <- append move encoded 33
      bind suffix <- bytes "ff"
      bind payload <- fuse move marked move suffix
      yield move payload

    weave main [] -> Whole:
      bind payload <- call package "Aether"
      bind size <- extent borrow payload
      bind preview <- slice borrow payload 0 6
      bind decoded <- decode move preview
      speak move decoded
      yield size

The world, weave, parameter, and binding names must begin with a lowercase ASCII
letter and then use lowercase ASCII letters, digits, or underscores. Keywords
are reserved. Every program must contain exactly one main weave with the
signature weave main [] -> Whole:.

- Input may use LF or CRLF, but canonical output uses LF.
- Indentation is exactly two spaces per block level. Tabs and trailing whitespace
  are rejected.
- A weave body must end with one root-level yield whose type matches the
  declared result. Nested blocks cannot contain yield or introduce bind.
- Expressions are shallow prefix forms. Their inputs are atoms, so intermediate
  values must be bound before they are used by another expression.

## Types and Ownership

| Type | Meaning | Ordinary name use |
| --- | --- | --- |
| Text | Bounded immutable UTF-8 text | Requires borrow name or move name. |
| Whole | Signed 64-bit integer | Copies with name. |
| Truth | bright or dim | Copies with name. |
| Bytes | Bounded immutable raw bytes | Requires borrow name or move name. |

Text and Bytes are unique local values. borrow name reads the value without
consuming the local. move name consumes the local, so every later reachable
use or revision is a compile error. A move in one branch also makes the binding
unavailable after the branch because it may have been moved.

Borrow parameters are permitted only for Text and Bytes. A plain Text or Bytes
parameter is owned. Calls to an owned unique parameter require a matching
literal or move name; Whole and Truth parameters are copied.

Text and Bytes are limited to 1,000,000 bytes. The source literal
form for Bytes is bytes "...", containing an even count of ASCII hexadecimal
digits. It is decoded before execution, must be at most 1,000,000 bytes, and is
formatted with lowercase hexadecimal digits.

## Statements and Control Flow

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

- bind introduces an immutable root binding.
- bind mutable introduces a root binding that revise may replace with a value
  of the same type.
- revise is valid in roots and nested blocks for a live mutable binding.
- speak accepts only Text and appends it to deterministic stdout.
- choose requires Truth; otherwise is optional and, when present, must be
  nonempty.
- while requires Truth and must have a nonempty body.

## Expressions

| Form | Input | Result | Behavior |
| --- | --- | --- | --- |
| not value | Truth | Truth | Logical negation. |
| measure text | Text | Whole | Unicode scalar count. |
| render value | Text, Whole, Truth | Text | Bytes are rejected. |
| extent bytes | Bytes | Whole | Raw byte count. |
| encode text | Text | Bytes | UTF-8 encoding. |
| decode bytes | Bytes | Text | Rejects invalid UTF-8 at runtime. |
| sum left right | Whole, Whole | Whole | Traps on overflow. |
| difference left right | Whole, Whole | Whole | Traps on overflow. |
| product left right | Whole, Whole | Whole | Traps on overflow. |
| quotient left right | Whole, Whole | Whole | Truncates toward zero; rejects zero and overflow. |
| remainder left right | Whole, Whole | Whole | Rejects zero and overflow. |
| less left right | Whole, Whole | Truth | Numeric comparison. |
| same left right | Two equal types | Truth | Includes Bytes equality. |
| join left right | Text, Text | Text | Bounded text concatenation. |
| glyph text index | Text, Whole | Whole | Unicode scalar code point or -1. |
| cut text start end | Text, Whole, Whole | Text | Clamped scalar half-open range. |
| fuse left right | Bytes, Bytes | Bytes | Bounded byte concatenation. |
| append bytes value | Bytes, Whole | Bytes | value must be 0..=255. |
| octet bytes index | Bytes, Whole | Whole | Byte value or -1. |
| slice bytes start end | Bytes, Whole, Whole | Bytes | Clamped byte half-open range. |
| call weave args... | Declared parameter list | Declared result | Calls a named weave. |

measure, glyph, and cut operate on Unicode scalar positions, not UTF-8 byte
offsets. extent, octet, and slice operate on raw byte positions. The VM limits
calls to 1,024 nested invocations.

## Artifact and Verification

An artifact begins with ASCII AETH followed by version byte 3. It stores named
weave metadata, parameter ownership modes, result types, local descriptors, and
bytecode. Text and bytes constants are length-prefixed with a 32-bit byte length
and checked against the 1,000,000-byte runtime limit.

The verifier runs before the VM and rejects malformed headers, unsupported
versions, invalid text constants, oversized bytes constants, unknown opcodes,
invalid local slots, reads before initialization, illegal revision, use-after-
move, stack underflow, operand type mistakes, invalid calls, invalid jumps,
control-flow state disagreement, unreachable instructions, and functions without
a valid terminal yield.

AETH v2 is not accepted by the Aether 0.3 verifier or VM.

## Typed Invocation and Forge

invoke_bytecode allows a trusted host to invoke a verified named weave with
typed Text, Whole, Truth, or Bytes values. It validates the selected weave's
argument count and types, executes it, and returns stdout plus a typed result.
It does not grant file, process, network, model, or code-generation authority
to the artifact.

The stricter forge ABI is documented in FORGE_CONTRACT.md. A compiler artifact
must still contain the required runnable main weave even when the host invokes
its compile weave.

## Deliberately Not Present

Explicit allocators, arenas, records, collections, typed error sets, compile-time
execution, C interop, native code generation, package management, concurrency,
and an Aether-written compiler are not present in Aether 0.3. The Rust core is a
bootstrap implementation. Aether is not self-hosting until an Aether compiler
has compiled itself to a reproducible verified artifact.
