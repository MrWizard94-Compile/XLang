# Value and Resource Model Research

**Status:** Primary-source evidence record for M1
**Date:** 2026-07-28
**Scope:** Ownership, borrowing, destruction, explicit allocation, and bounded
dynamic storage for a future Aether surface. This document does not change the
executable Aether 0.5 contract.

## Question

What is the smallest resource model that gives AI-authored Aether programs
visible ownership, allocation, failure, and cleanup boundaries without adding
a general reference/lifetime language, ambient allocation, user-defined
destructors, or host authority?

The answer must fit Aether's existing constraints: source emits AETH only,
artifacts are verified before execution or forge write, the VM has no file,
process, network, shell, or model capability, and new default-product syntax
requires bootstrap/seed byte-identity proof.

## Evidence examined

| System | Primary source | Relevant established mechanism | Useful lesson for Aether | Not imported wholesale |
| --- | --- | --- | --- | --- |
| Rust | [Reference: destructors](https://doc.rust-lang.org/reference/destructors.html) and [core `Drop`](https://doc.rust-lang.org/stable/core/ops/trait.Drop.html) | Values have deterministic scope-based destruction; `Copy` and custom destruction have deliberately different contracts. | Make ownership transfer and destruction explicit in semantic state. | General references, lifetime parameters, trait-driven destructors, arbitrary cleanup code. |
| Swift | [Declarations: ownership](https://docs.swift.org/swift-book/documentation/the-swift-programming-language/declarations/) and [Generics: noncopyable types](https://docs.swift.org/swift-book/documentation/the-swift-programming-language/generics/) | Borrowing, consuming, and exclusive `inout` access distinguish noncopyable ownership from temporary access. | A third, explicit exclusive access mode is useful for a mutable allocator capability. | ARC, class identity, implicit copies, and broad language surface. |
| Zig | [Language reference](https://ziglang.org/documentation/master/) | Allocators are passed explicitly; allocation failure participates in the ordinary error model rather than becoming an invisible global policy. | Allocation authority and failure must appear at the call boundary. | Manual allocator protocol everywhere, unrestricted pointers, and direct host-memory model. |
| Hylo | [Language specification](https://hylo-lang.org/docs/reference/specification/) | Ownership-aware values, projections, dynamic lifetimes, and allocation/deallocation operations make storage provenance meaningful. | Region provenance can prevent an allocated value from outliving its allocator. | The full capability, projection, and generic lifetime system. |

Sources were accessed on 2026-07-28. They are design inputs, not claims that
Aether implements any referenced mechanism today.

## Findings

### 1. Copy, ownership transfer, and destruction must be separate concepts

The research consistently separates a harmless duplicate of a small value from
transfer of responsibility for a resource-bearing value. Aether 0.5 already
has this source-level distinction: `Whole` and `Truth` read by value, while
`Text`, `Bytes`, and records require explicit `borrow` or `move`. M1 should
preserve that distinction and make its next-version state transitions precise.

The selected model must not equate immutable data with freely copyable data.
An immutable `Text` can still represent a bounded, resource-bearing value whose
duplication must be explicit or deliberately specified.

### 2. Mutable capability access needs an explicit, non-escaping mode

An allocator changes resource accounting even when an ordinary data binding is
not replaced. Passing it as an ordinary shared borrow would obscure that fact;
passing it by ownership would make every allocation noisy and difficult to
compose. Swift's distinction between borrowing/consuming and exclusive mutable
access motivates a small Aether-specific third parameter mode: `access`.

`access` is deliberately narrower than a general mutable reference. It is
valid only for an opaque capability, lasts for one operation or call, cannot be
stored, returned, yielded, captured, or kept across a control-flow edge, and
does not introduce reference-valued user types.

### 3. Allocation authority and allocation failure belong in the program

Zig's allocator and error conventions show why a collection API that silently
chooses a heap cannot support credible resource reasoning. The proposed Aether
model therefore requires a named bounded arena at each dynamic allocation
site, plus a closed allocation outcome instead of a VM panic, host exception,
or fallback allocator.

This is a semantic rule, not a claim that Aether 0.5 currently provides it.
Current text/byte transformations use bounded VM values and have no explicit
arena API; M2 must not silently recast them as already-explicit allocation.

### 4. Provenance must be lexical and visible, not globally inferred

Hylo demonstrates that storage provenance matters when a value may outlive the
operation that created it. Aether needs that safety property, but not a general
lifetime language. The recommended restriction is lexical region identity:
each future `Buffer` carries the name of the `Arena` that owns its storage, and
a function signature names that arena whenever it receives or returns a
region-backed value. No caller or compiler may invent an unnamed outlives
relationship across a public boundary.

### 5. User-defined destruction is the wrong first capability

Rust demonstrates useful deterministic destruction, but running arbitrary
destructor code would introduce hidden ordering, effects, and potentially host
authority before Aether has an effects model. The first dynamic-storage slice
should have logical destruction only: the language invalidates an owner at its
defined end point, while the bounded arena reclaims backing storage when the
arena ends. No user callback runs.

## Aether-specific synthesis

The research supports the proposed **owned values, ephemeral loans, and
bounded arenas** model specified in
[DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md](../DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md).
It combines four deliberately narrow pieces:

1. copyable scalar values versus explicit ownership transfer;
2. non-escaping read loans (`borrow`) and exclusive capability loans
   (`access`), neither represented as general reference values;
3. fixed-capacity, named arenas as the only authority for new dynamic storage;
   and
4. closed, explicit allocation outcomes with failure atomicity.

This is intentionally not a claim of originality, a Rust/Swift/Zig/Hylo
replacement, or a performance result. Its Aether-specific contribution is the
combination of visible resource boundaries with a compact verifier-friendly
state machine, an AETH-only execution boundary, and an implementation plan
that the seed compiler can prove one source form at a time.

## Rejected starting points

| Starting point | Reason rejected for M1 |
| --- | --- |
| A general borrowed-reference and lifetime type system | It would add inference and public-boundary complexity before Aether has dynamic aggregates, effects, or a stable semantic IR. |
| A global/default allocator | It hides the authority and failure policy that M1 exists to expose. |
| Immediate manual `free` or general individual deallocation | It introduces alias, double-free, and use-after-free obligations before a small safe collection is proven. |
| User-defined destructors | It creates untyped hidden effects and ordering questions before M4. |
| Recursive/cyclic aggregates in M2 | It makes escape, layout, and destruction proof much larger than the first bounded experiment. |
| Reinterpreting AETH v4 or v5 instructions | It would undermine artifact compatibility and verifier clarity. |

## Evidence limits and required follow-up

This study establishes a design rationale only. It supplies no benchmark,
memory-allocation measurement, compatibility proof, or language-feature
implementation. The proposed model remains subject to a human decision in
[ADR-003](../ADR-003-value-resource-semantics.md). If accepted, M2 must prove
the rules through source, semantic IR, a new AETH version, verifier, VM, seed,
and negative-artifact tests described in
[M1-VALIDATION-MATRIX.md](../M1-VALIDATION-MATRIX.md).
