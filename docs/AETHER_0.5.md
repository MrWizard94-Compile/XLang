# Aether 0.5 Language Specification

Status: executable Aether 0.5 specification with Stage 6 immutable-record
emission parity, 2026-07-28.

Aether 0.5 emits deterministic AETH bytecode for the Aether VM and verifies
every artifact before it runs. It is a new language with an Aether-owned
artifact format; source is not translated to another language.

## Source Shape

An Aether source file begins with one `world` declaration, followed by zero or
more `record` declarations, then one or more named `weave` declarations.
Records must all appear before the first weave. `main` must be
`weave main [] -> Whole:`. Bodies use exact two-space indentation and finish
with one root-level `yield`.

Expressions are shallow prefix forms: their operands are atoms. Bind an
intermediate value before passing it to another operation.

## Types and Ownership

| Type | Meaning | Ordinary name use |
| --- | --- | --- |
| `Text` | Bounded immutable UTF-8 text | Requires `borrow name` or `move name`. |
| `Whole` | Signed 64-bit integer | Copies with name. |
| `Truth` | `bright` or `dim` | Copies with name. |
| `Bytes` | Bounded immutable raw bytes | Requires `borrow name` or `move name`. |
| declared record name | Immutable nominal aggregate | Requires `borrow name` or `move name`. |

`Text` and `Bytes` are limited to 1,000,000 bytes. A record has at most 64
fields and its aggregate runtime payload is limited to 1,000,000 bytes. A move
consumes a binding on every reachable path. `borrow` keeps it readable. `bind
mutable` permits `revise` with a same-type replacement while the binding remains
live.

## Immutable Records

Declare a record immediately after `world`:

    record card [label: Text, score: Whole, payload: Bytes, active: Truth]

Fields are declared in constructor order, must have unique lowercase names, and
may use only `Text`, `Whole`, `Truth`, or `Bytes`. Records cannot contain other
records in 0.5; this deliberately bounds aggregate layout and avoids recursive
or partial-move semantics.

Construct a record with every field in declaration order:

    bind value <- make card "Aether" 7 bytes "0102" bright

Project a field only by explicitly borrowing the live record:

    bind label <- field borrow value label

`field` clones the selected immutable field and does not move or mutate the
record. Whole-record values may be passed, returned, rebound, compared with
`same`, or moved under the normal ownership rules. `same` is structural for two
values of the same declared record type. `render` does not accept records;
project a field first.

Internal Aether calls support record parameters and results. The host invocation
API deliberately accepts and returns primitive values only; a host caller must
invoke a weave that projects its required primitive result.

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

The 0.4 arithmetic, text, byte, slice, call, and control-flow forms remain
unchanged. Aether 0.5 adds `make record field...` and
`field borrow record field-name` as described above.

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

Programs without records emit AETH v4 exactly as before. Record-bearing programs
emit AETH v5. A v5 artifact begins with `AETH`, version byte `5`, a bounded
record table, then the normal function table. Each record table entry stores its
ASCII name and primitive field schema. Function and local type descriptors use
tag `5` plus a little-endian record identifier for nominal record types.

v5 adds two verified instructions: `MAKE_RECORD record-id:u16` and
`FIELD record-id:u16 field-index:u8`. The verifier rejects malformed record
tables, duplicate or invalid names, invalid type identifiers, invalid field
references, record opcodes in v4, invalid stack shapes, and all existing header,
control-flow, move-state, and local-access failures. Earlier AETH versions are
rejected; v4 remains accepted for compatibility.

## Bootstrap and Seed Status

The Rust core remains the complete Aether 0.5 bootstrap compiler and VM.
`seed/aether_seed.ae` emits the documented 0.5 record surface and the prior 0.4
surface byte-identically to Rust, with reproducible self-compilation proof.
Rust remains the invalid-source diagnostic authority and seed rebuild path; full
diagnostic parity is not claimed. The profile boundary is normative in
[SEED_PROFILE.md](SEED_PROFILE.md).

[AETHER_0.4.md](AETHER_0.4.md) is retained as the historical pre-record surface
specification. The record-format decision is recorded in
[ADR-001](ADR-001-records-and-aeth-v5.md).
