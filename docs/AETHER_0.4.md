# Aether 0.4 Language Specification

Status: historical executable Aether 0.4 specification (pre-record surface),
superseded by [AETHER_0.5.md](AETHER_0.5.md), 2026-07-28.

Aether 0.4.0 emits AETH v4 bytecode for the Aether VM and verifies every
artifact before it runs. It is a new language with an Aether-owned artifact
format; source is not translated to another language.

## Source Shape

An Aether source file begins with one `world` declaration and one or more named
`weave` declarations. `main` must be `weave main [] -> Whole:`. Bodies use
exact two-space indentation and finish with one root-level `yield`.

Expressions are shallow prefix forms: their operands are atoms. Bind an
intermediate value before passing it to another operation.

## Types and Ownership

| Type | Meaning | Ordinary name use |
| --- | --- | --- |
| `Text` | Bounded immutable UTF-8 text | Requires `borrow name` or `move name`. |
| `Whole` | Signed 64-bit integer | Copies with name. |
| `Truth` | `bright` or `dim` | Copies with name. |
| `Bytes` | Bounded immutable raw bytes | Requires `borrow name` or `move name`. |

`Text` and `Bytes` are limited to 1,000,000 bytes. A move consumes a binding on
every reachable path. `borrow` keeps it readable. `bind mutable` permits
`revise` with a same-type replacement while the binding remains live.

## Statements

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

Nested blocks cannot introduce bindings or yield. `choose` and `while` require
`Truth`; `speak` requires `Text`.

## Expressions

Existing arithmetic, text, byte, slice, call, and control-flow behavior remains
as specified in the Stage 2 archive. Aether 0.4 adds these executable forms:

| Form | Input | Result | Behavior |
| --- | --- | --- | --- |
| `seek text needle start` | Text, Text, Whole | Whole | Scalar index of the first match at or after clamped `start`, or `-1`. |
| `number text` | Text | Whole | Parses one canonical signed Whole spelling. |
| `pack16 value` | Whole | Bytes | Two little-endian bytes; accepts `0..=65535`. |
| `pack32 value` | Whole | Bytes | Four little-endian bytes; accepts `0..=4294967295`. |
| `pack64 value` | Whole | Bytes | Eight little-endian two's-complement bytes. |
| `unpack16 bytes start` | Bytes, Whole | Whole | Reads two little-endian bytes at a strict in-range offset. |
| `unpack32 bytes start` | Bytes, Whole | Whole | Reads four little-endian bytes at a strict in-range offset. |
| `poke bytes index value` | Bytes, Whole, Whole | Bytes | Replaces one in-range byte; value must be `0..=255`. |
| `poke32 bytes index value` | Bytes, Whole, Whole | Bytes | Replaces four in-range bytes with a little-endian `u32`. |

`seek` is scalar-aware like `measure`, `glyph`, and `cut`. Fixed-width binary
operations do not clamp: malformed offsets and values fail at runtime.

## Artifact and Verification

An artifact begins with ASCII `AETH` followed by version byte `4`. It stores
weave names, parameter ownership modes, result types, local descriptors, and
bytecode. Constants are length-prefixed and verified before execution.

The verifier rejects malformed headers, unsupported versions, invalid UTF-8,
oversized constants, invalid local access, illegal revision, use-after-move,
stack and type errors, invalid calls or jumps, unreachable instructions, and
functions without a reachable terminal `yield`.

## Bootstrap Status

The Rust core remains the complete Aether 0.4 bootstrap compiler and VM.
`seed/aether_seed.ae` emits the complete documented canonical Aether 0.4 source
surface byte-identically to Rust, with reproducible self-compilation proof. Rust
remains the invalid-source diagnostic authority and seed rebuild path; full
diagnostic parity is not claimed. The profile boundary is normative in
[SEED_PROFILE.md](SEED_PROFILE.md).

`AETHER_0.3.md` is retained as a historical Stage 2 specification and is
superseded by this document.
