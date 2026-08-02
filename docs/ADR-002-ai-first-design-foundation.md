# ADR-002: AI-first design foundation and evidence-gated evolution

**Status:** Accepted
**Date:** 2026-07-28
**Decision makers:** WPAI product direction, with explicit human approval
**Related Rule IDs:** DOC-ADR-001, SOP-PHASE-001, RND-INVAR-001, RND-CORE-001,
RND-DOC-001, IP-INVENTION-001, DOC-SYNC-001

**Implementation update (2026-08-01):** The 0.5 boundary described below is
the historical decision context. The current executable contract is Aether 0.8,
including the bounded resource core in [ADR-004](ADR-004-aeth-v6-bounded-resources.md)
the bounded `Error[Whole]` effect in [ADR-007](ADR-007-m4-typed-error-effect.md),
and the bounded deterministic M5 evaluator in [ADR-008](ADR-008-m5-deterministic-comptime.md).

## Context

The original Aether brief describes an ambitious AI-first systems language:
mutable value semantics, explicit allocators, compile-time execution without
macros, typed errors/effects, data-oriented layout, interop, integrated tooling,
and structural AI editing. At this ADR's decision time, the executable product
had a smaller, strong Aether 0.5 boundary: seed-hosted compilation, verified
AETH v4/v5 artifacts, a capability-free VM, and bounded value semantics.
Aether 0.6 extended that foundation with verified AETH v6 bounded arenas and
buffers; Aether 0.7 adds verified AETH v7 effect metadata and a bounded
`Error[Whole]` route; Aether 0.8 adds v8 `COMPTIME_WHOLE` provenance for one
fixed-budget literal evaluator. See [ADR-004](ADR-004-aeth-v6-bounded-resources.md),
[ADR-007](ADR-007-m4-typed-error-effect.md), and
[ADR-008](ADR-008-m5-deterministic-comptime.md).

The repository lacked an Aether-specific SOP research set that decomposed the
reference systems, recorded current-vs-future truth, defined measurable claims,
and ordered the dependencies. The historical brief also considered a native/LLVM
backend, while current project law is AETH-only. Without an explicit decision,
future work could either overclaim the existing implementation or quietly
violate the current artifact and security boundary.

## Decision

Adopt an evidence-gated AI-first language development foundation composed of:

1. a primary-source study of ten relevant language/toolchain systems;
2. a component-decomposition matrix and synthesis with falsifiable research
   spikes;
3. a north-star document that separates the current executable contract from
   planned direction;
4. a claim register that makes proof burden and prohibited claims explicit; and
5. an ordered roadmap that places value/resource semantics before allocators,
   FFI, effects, concurrency, and layout transformations.

Preserve AETH-only execution, verify-before-run/write, capability confinement,
local-first data handling, and seed-hosted proof requirements. A future semantic
feature is not admitted to default product compilation until its specification,
tests, verifier/VM/forge behavior, seed/bootstrap proof, documentation, and
constitution gate are complete.

The historical Aether brief and legacy material remain research inputs only.
They do not authorize Aether-to-LLVM/native translation, foreign host authority,
or unproven performance/safety claims.

## Consequences

The project can pursue the complete ambition without confusing aspiration with
current behavior. Research findings become traceable to primary sources;
milestones have dependencies and stop conditions; and “better” becomes a
collection of scoped, reproducible claims instead of a blanket assertion.

This adds documentation and review obligations before large semantic work. It
also deliberately leaves several material choices open: the exact resource
model, allocator form, effect system, generic/layout semantics, concurrency
model, and any foreign-interface scope. The bounded structural-authoring choice
is now decided by [ADR-005](ADR-005-structural-authoring-contract.md); its
fine-grained edit vocabulary remains future work.

## Alternatives considered

| Option | Pros | Cons |
| --- | --- | --- |
| Keep the vision only in the historical brief and scattered docs | No new documentation work. | Conflates old exploration with current law; offers no evidence plan or dependency order. |
| Immediately implement a large set of language features | Appears to move quickly. | Violates dependency-first design and would create seed, verifier, semantic, and documentation gaps. |
| Adopt a reference language's model wholesale | Faster initial decisions. | Imports mismatched constraints and does not establish Aether-specific proof or AI-authoring advantages. |
| Evidence-gated Aether foundation (chosen) | Preserves ambition, local law, traceability, and measurable milestones. | Requires deliberate research, ADRs, and proof before feature implementation. |

## Links

* Related ADRs: [ADR-001: Immutable nominal records and AETH v5](ADR-001-records-and-aeth-v5.md).
* Related design: [NORTH_STAR.md](NORTH_STAR.md), [CORE_CLAIMS.md](CORE_CLAIMS.md), [ROADMAP.md](ROADMAP.md).
* Related research: [01-reference-systems.md](research/01-reference-systems.md), [02-component-decomposition.md](research/02-component-decomposition.md), [03-synthesis-and-evidence.md](research/03-synthesis-and-evidence.md).
* Related product contract: [MANIFEST.md](../MANIFEST.md), [ARCHITECTURE.md](ARCHITECTURE.md).
