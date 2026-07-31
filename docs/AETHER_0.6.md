# Aether 0.6 Language Specification

Status: executable Aether 0.6 specification with Stage 7 bounded arena and
buffer emission parity, 2026-07-31.

Aether 0.6 emits deterministic AETH v6 bytecode for the Aether VM and verifies
every artifact before it runs or forge writes it. Aether source is never
translated to Rust, C, JavaScript, LLVM, or another language.

This specification supersedes [AETHER_0.5.md](AETHER_0.5.md) as the current
language contract. Aether 0.4 and 0.5 documents remain historical compatibility
references. Their AETH v4/v5 artifacts retain their byte meanings and remain
accepted by the verifier and VM.

## Compatibility surface

The scalar, text, byte, immutable-record, statement, formatting, and ordinary
ownership rules from Aether 0.5 remain valid unless this document narrows a
resource-specific operation. New compilation always emits AETH v6, including
programs that do not use a resource form. The VM still accepts verified AETH
v4 and v5 artifacts produced previously.

`Text`, `Bytes`, and immutable records remain owners: ordinary name use is
rejected, while `borrow name` reads and `move name` transfers. `Whole` and
`Truth` are copy values. Existing Text/Bytes behavior is retained as a bounded
0.5 compatibility surface; it is not silently redefined as arena allocation.

## M2 bounded-resource surface

M2 adds one explicit resource region and one fixed-capacity homogeneous
collection. The goal is a small, closed state machine that an AI author can
make locally explicit and that the bootstrap, semantic plan, verifier, VM, and
seed compiler can independently check.

### Types and declarations

| Source type/form | Meaning | Boundary rule |
| --- | --- | --- |
| `Arena` | Opaque, VM-private arena capability. | Created only by the one root `main` arena binding; used only through `access`. |
| `buffer Whole` | Unallocated mutable owner placeholder for signed 64-bit values. | Must be allocated through a closed resource outcome before borrowing. |
| `buffer Truth` | Unallocated mutable owner placeholder for `bright`/`dim` values. | Same ownership rule as `buffer Whole`. |
| `BufferWhole`, `BufferTruth` | Internal Aether weave parameter types. | A weave with one must also have exactly one `access ...: Arena` parameter; neither may be a weave result. |

An M2 program that uses resources declares exactly one named arena as a
root-level binding in `main`:

```aether
bind memory <- arena 64
bind mutable values <- buffer Whole
```

The arena capacity is a positive `Whole` literal from 1 through 1,000,000
logical bytes. `arena` cannot be declared in a helper, moved, borrowed,
returned, or stored. A helper may receive temporary exclusive authority only
through a signature such as `weave provision [access memory: Arena] -> Whole:`.

### Closed resource outcomes

Resource operations are not first-class values. Each is legal only as the
condition of the final root-level `choose` in a weave. It requires an explicit
`otherwise` arm; every arm contains exactly one `yield` or another closed
resource `choose`. This makes every success and failure path visible and
prevents a moved owner from reaching a join without restoration.

```aether
choose allocate access memory move values 2 into values:
  choose append move values 7 into values:
    choose at borrow values 0 into observed:
      yield observed
    otherwise:
      yield -3
  otherwise:
    yield -2
otherwise:
  yield -1
```

The permitted operations are:

| Form | Bright result | Dim result | Owner/region rule |
| --- | --- | --- | --- |
| `allocate access arena move buffer count into buffer` | `buffer` becomes an allocated empty buffer of `count` elements. | The original unallocated owner is restored, and arena use is unchanged. | `arena` is a live `Arena`; moved source and `into` destination are the same mutable buffer binding. |
| `append move buffer value into buffer` | The value is appended and the allocated owner is restored. | The unchanged allocated owner is restored when full. | Value type matches `Whole` or `Truth`; moved source and destination are the same binding. |
| `at borrow buffer index into target` | `target` is atomically replaced with the element. | `target` is unchanged when the index is negative or outside the current length. | `buffer` is allocated; `target` is a mutable root `Whole` or `Truth` matching the element type. |
| `count borrow buffer` | Produces a copyable `Whole` length. | Not fallible. | The buffer must already be allocated. |

`allocate` requests a count, not bytes. A negative count, multiplication
overflow, metadata overflow, or request that does not fit the remaining arena
returns the dim outcome; it never traps, reserves partial capacity, or loses the
placeholder owner. `append` never reserves more arena capacity: allocation
reserves `16 + (count * element-width)` logical bytes in advance, where a
`Whole` element width is 8 and a `Truth` element width is 1.

M2 does not provide generic allocation/append/lookup values, outcome
propagation, individual `free`/reset, user destructors, mutable element
references, resource `revise`, nested dynamic values, recursive buffers, or a
default allocator. Arena backing storage is reclaimed only when the invocation
ends. A buffer cannot be returned from a weave in 0.6: without a first-class
outcome type, a returned buffer could not prove whether allocation succeeded.

## Semantic-plan and verification contract

Before lowering AETH, the Rust bootstrap validates M2 source into a typed
semantic resource plan. Each closed operation records its lexical region name,
buffer element type, source owner/borrow place, and stable destination. Emission
consumes that plan rather than re-scanning raw syntax for resource metadata.

AETH v6 encodes one artifact-level arena capacity after the version byte:

```text
AETH | version=6:u8 | arena_capacity:u32-le | record_count:u16-le |
record table | function table
```

The new verified instructions are `ARENA`, `BUFFER`, `ACCESS`, `ALLOCATE`,
`BUFFER_APPEND`, `BUFFER_AT`, and `COUNT`. `ALLOCATE`, `BUFFER_APPEND`, and
`BUFFER_AT` carry the element tag plus destination local slot, so the verifier
can prove that a closed operation restores or atomically updates its declared
place.

For v6, the verifier requires a nonzero arena capacity exactly when resource
instructions occur, exactly one `ARENA` declaration in `main`, valid access
parameters, and no serialized access-loan local/result. Its operand-stack state
also distinguishes a buffer placeholder, a transient borrow, and a buffer owner
moved from a particular local. A crafted artifact cannot use a fresh
placeholder to satisfy an operation that promises to restore a moved owner.

The VM creates no guest-visible pointer, reference, allocator, file, process,
network, shell, or host-memory capability. It reserves the declared arena before
guest execution and executes only after normal artifact verification succeeds.

## Seed-hosted product compilation

The default compiler remains the Aether-written seed artifact. The seed source
parses and emits the canonical M2 resource forms above, including the v6 header
and direct replacement destinations. The Rust bootstrap remains the seed rebuild
and invalid-source diagnostic authority; complete invalid-source diagnostic
parity is not claimed.

The canonical M2 corpus is checked bootstrap-versus-seed byte-for-byte and run
after verification:

| Example | Proves |
| --- | --- |
| `examples/arena-buffer.ae` | Whole allocation, append, lookup, and value replacement. |
| `examples/arena-exhausted.ae` | Allocation exhaustion preserves the owner. |
| `examples/arena-full.ae` | Full append preserves the owner. |
| `examples/arena-lookup-fallback.ae` | Absent lookup leaves the copy destination unchanged. |
| `examples/arena-truth-buffer.ae` | Truth element typing and storage width. |
| `examples/arena-access-weave.ae` | Access-bound helper invocation. |

The seed matches bootstrap bytecode for all six forms. It does not imply full
seed diagnostic parity or parity for future resource extensions.

## Host and forge boundary

`Arena`, `BufferWhole`, `BufferTruth`, access loans, and closed outcomes are
never accepted by or returned through the primitive `invoke_bytecode` API. The
forge ABI remains exactly `compile [borrow source: Text] -> Bytes`; it verifies
both compiler and generated artifact before execution or write. Resource values
remain inside the verified Aether call graph.

## Related decisions

- [ADR-003](ADR-003-value-resource-semantics.md) — accepted M1 direction.
- [ADR-004](ADR-004-aeth-v6-bounded-resources.md) — executable 0.6 M2 scope,
  representation, and intentional limits.
- [SEED_PROFILE.md](SEED_PROFILE.md) — normative product compiler proof scope.
- [FORGE_CONTRACT.md](FORGE_CONTRACT.md) — host ABI and verify-before-write law.
