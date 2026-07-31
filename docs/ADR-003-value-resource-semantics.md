# ADR-003: Owned values, ephemeral loans, and bounded arenas

**Status:** Accepted — 2026-07-31 implementation direction
**Date:** 2026-07-28
**Decision makers:** WPAI product direction, with human approval recorded on
2026-07-31
**Related Rule IDs:** DOC-ADR-001, RND-INVAR-001, RND-CORE-001,
RND-DOC-001, IP-INVENTION-001, SEC-INPUT-001, DOC-SYNC-001,
TEST-BEHAVIOR-001

## Context

At the original 0.5 decision boundary, Aether had useful bounded source
ownership rules: `Whole` and `Truth` copy by ordinary use; `Text`, `Bytes`,
and immutable records require explicit `borrow` or `move`; mutable root
bindings support `revise`; and the verifier tracks local initialization and
move state. It did not yet have dynamic aggregates, an allocator capability,
resource provenance, a non-escaping exclusive capability mode, an
allocation-outcome type, or a user-visible destruction contract.

The long-range direction accepted in ADR-002 calls for explicit,
capability-oriented allocation, but deliberately left the exact resource model
open. The choice cannot be deferred into an implementation because it controls
source compatibility, semantic IR, artifact format, verifier state, VM failure
behavior, seed feasibility, host boundaries, future typed effects, and every
possible FFI/concurrency design.

The historical Aether brief and reference systems are research input, not
authority to copy an entire ownership or allocator model. Current project law
also forbids a host-capability leak, requires verified AETH execution, and
requires seed/bootstrap proof before new source reaches default product
compilation.

## Accepted decision

Aether uses the following first resource model as the direction for M2 and
later ownership/effect work:

1. **Owned values:** `Text`, `Bytes`, records, and future buffers are source
   owners. They transfer only through explicit `move`; copyable `Whole` and
   `Truth` retain ordinary copy use.
2. **Ephemeral read loans:** `borrow` creates a non-escaping, operation-scoped
   read loan. It is not a general reference value and may not be returned,
   stored, yielded, captured, or cross a control-flow edge.
3. **Ephemeral exclusive capability loans:** new `access` grants one operation
   or call exclusive mutable use of an opaque `Arena` capability. It also may
   not escape and is not a general mutable reference.
4. **Named bounded arenas:** dynamic storage must come from a named,
   fixed-capacity, VM-private arena recorded in a new versioned AETH resource
   plan. M2 starts with one arena per invocation and a 1,000,000 logical-byte
   upper bound.
5. **Closed allocation outcomes:** allocation yields
   `Allocation[T] = allocated(T) | exhausted`, never an ambient allocator
   fallback, VM panic, or host exception. Fallible operations are failure-atomic:
   an `exhausted` result consumes neither caller owners nor arena capacity.
   Fixed-capacity buffer operations use equally closed owner-preserving outcomes:
   `Append[T] = appended(T) | full(T)` and
   `Lookup[T] = found(T) | absent`.
6. **Logical destruction without user code:** successful replacement and lexical
   scope exit destroy owner values logically. Arena backing storage is reclaimed
   only at arena end in M2. There are no user destructors, callbacks, finalizers,
   or implicit host effects.
7. **Narrow dynamic experiment:** M2 adds only a homogeneous
   `Buffer[Whole @ arena]` or `Buffer[Truth @ arena]`; no recursive values,
   owned elements, dynamic record fields, manual free/reset, or host ABI
   crossing.
8. **Explicit typed semantic IR:** new source lowers through an ownership/loan/
   region-aware semantic IR before it can emit a new AETH version. v4/v5 remain
   byte-compatible and are not reinterpreted.

The complete design vocabulary, counterexamples, and layer map are in
[DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md](DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md).
The executable Aether 0.6 subset deliberately narrows generic outcome
propagation and buffer-returning boundaries while it establishes the verifier
and seed proof. Those implementation choices are normative in
[ADR-004](ADR-004-aeth-v6-bounded-resources.md), not an implicit weakening of
the accepted direction.

## Consequences

### Positive

- Dynamic allocation authority, capacity, failure, ownership transfer, and
  storage provenance become visible at source and artifact boundaries.
- The compiler and verifier receive a compact finite state machine rather than
  a general implicit lifetime system.
- No new resource feature can grant file, network, process, shell, model, or
  arbitrary host-memory authority.
- The bounded first collection is small enough for behavior tests, hostile
  artifact tests, and seed/bootstrap byte-identity proof.
- The model preserves a clean path for future effects: an allocation fact is
  already explicit, but M1 does not pretend to have designed a general effect
  language.

### Costs and deliberate limits

- A buffer cannot share mutable elements, recursively contain owners, or free
  individual allocations in M2.
- Arena retention lasts until lexical arena end; capacity use is predictable but
  early reuse is intentionally deferred.
- A caller must handle an explicit outcome, increasing source verbosity in
  exchange for clear failure behavior.
- Existing Aether 0.5 Text/Bytes runtime allocations remain a bounded
  compatibility behavior; they are not silently converted to the new model.
- The broader accepted direction remains future work outside the deliberately
  closed Aether 0.6 subset. Any expansion still requires complete bootstrap,
  verifier, VM, seed, documentation, and test evidence before it becomes
  product surface area.

## Alternatives considered

| Alternative | Why not chosen |
| --- | --- |
| General borrowed references with lifetime inference | Too much hidden state and proof complexity for the first dynamic-resource increment. |
| Global/default allocator | Hides allocation authority and failure policy, contrary to the accepted direction. |
| General manual free | Adds double-free, use-after-free, and alias problems before a safe bounded kernel exists. |
| Reference counting or garbage collection as the base model | Makes cost/destruction less inspectable and does not deliver explicit allocation boundaries. |
| Ownership-only arena argument | Makes ordinary allocation composition awkward while still requiring temporary access semantics. |
| Borrowed arena with hidden mutation | Misdescribes the resource state change; `access` makes it explicit. |
| Proposed model | Keeps the first proof finite, verifier-friendly, resource-bounded, and compatible with AETH-only execution. |

## Security and compatibility constraints

- AETH v4/v5 behavior and byte interpretation remain unchanged.
- A new resource representation requires a new artifact version, decoder, and
  verifier rules; unknown versions/instructions remain rejected before run/write.
- Forge and primitive invoke APIs remain unchanged; arena/buffer/outcome values
  cannot cross to the host.
- The VM must reserve capacity using a fallible path before guest execution and
  treat resource admission failure as a non-executing failure.
- Seed parity remains canonical-source emission parity; full invalid-source
  diagnostic parity is not claimed.

## Approval and implementation record

Human approval was recorded on 2026-07-31 before the bounded M2 implementation
began. It authorized a complete implementation, not a waiver of the M2 ADR,
dependency-first delivery, hostile-input tests, seed proof, zero-warning gate,
or AETH-only/capability constraints.

The implementation is Aether 0.6 / AETH v6. It carries a typed semantic
resource plan into emission, verifies operand-stack buffer provenance, and
proves the documented canonical resource corpus seed-identical to bootstrap.
It intentionally does not claim that the broader M1 design has already gained
generic outcome propagation, buffer results, or a typed effects language.

## Links

- [M1 design specification](DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md)
- [M1 validation matrix](M1-VALIDATION-MATRIX.md)
- [ADR-004: AETH v6 bounded resources](ADR-004-aeth-v6-bounded-resources.md)
- [Value/resource research](research/04-value-resource-models.md)
- [ADR-002: AI-first design foundation](ADR-002-ai-first-design-foundation.md)
- [Roadmap](ROADMAP.md)
- [Current Aether 0.6 specification](AETHER_0.6.md)
