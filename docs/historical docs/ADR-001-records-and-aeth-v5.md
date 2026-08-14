# ADR-001: Immutable nominal records and AETH v5

**Status:** Accepted
**Date:** 2026-07-28
**Decision makers:** WPAI product direction, with explicit human approval
**Related Rule IDs:** DOC-ADR-001, RND-INVAR-001, RND-CORE-001, SEC-INPUT-001

## Context

Aether 0.4 had only scalar and byte/text values. The language needed a first
aggregate value without compromising deterministic artifacts, the seed-hosted
product compiler, explicit ownership, or the AETH v4 artifacts already in use.

## Decision

Add immutable nominal `record` declarations before weave declarations. A record
contains one to 64 primitive fields (`Text`, `Whole`, `Truth`, or `Bytes`), is
constructed with `make`, and is projected only by `field borrow`. Records are
unique values, so ordinary use requires `borrow` or `move`; projection clones
the selected immutable field. Records can be passed and returned by internal
Aether calls, but the host invocation boundary remains primitive-only.

Keep v4 valid and byte-stable for programs without records. Emit AETH v5 only
for record-bearing programs. v5 adds a bounded record table, variable-width
record type descriptors, `MAKE_RECORD`, and `FIELD`. The verifier validates the
schema, type identifiers, field references, stack shapes, and v5-only opcode
use before execution. Aggregate runtime payloads are capped at 1,000,000 bytes.

The Aether-written seed compiler must emit the same v5 artifacts byte-for-byte
as the Rust bootstrap before records are available through the default compile
path.

## Consequences

Programs gain an auditable aggregate abstraction without mutation, recursive
layout, partial moves, hidden host serialization, or a compatibility break for
v4 artifacts. The compiler, verifier, VM, seed emitter, tests, and artifact
documentation must all evolve together. Host integrations cannot directly
inject or receive records until a separately designed ABI extension is accepted.

## Alternatives considered

| Option | Pros | Cons |
| --- | --- | --- |
| Break v4 and put records in v5-only artifacts | Simpler compatibility story for new code | Invalidates existing valid artifacts and violates the compatibility goal. |
| Keep v4 and overload existing encodings | Avoids a version number | Makes type decoding ambiguous and weakens artifact verification. |
| Mutable or recursively nested records | More expressiveness | Requires new aliasing, size, and partial-move rules beyond this bounded milestone. |
| Host-only records | Low compiler work | Breaks the local, Aether-owned value and bytecode model. |

## Links

* Related ADRs: none.
* Related code: `crates/xlang-core/src/lib.rs`, `seed/aether_seed.ae`, `examples/records.ae`.
* Related specification: [AETHER_0.5.md](AETHER_0.5.md).
