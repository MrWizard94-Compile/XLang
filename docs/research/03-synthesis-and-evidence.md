# Aether Synthesis and Evidence Plan

**Status:** SOP phases 3 and 4 design synthesis
**Date:** 2026-07-28; current-contract update: 2026-08-01
**Inputs:** [reference systems](01-reference-systems.md) and
[component decomposition](02-component-decomposition.md)

## Evidence vocabulary

Aether's long-range design must not blur an aspiration into an implementation
claim. Every statement in the design set uses one of these labels:

| Label | Meaning | Required proof before it can advance |
| --- | --- | --- |
| **Implemented** | Present in the current Aether 0.8 product contract and covered by repository evidence. | Code, specifications, behavior tests, and applicable seed proof already exist. |
| **Accepted direction** | A human-approved direction for future design, not source syntax or runtime behavior. | A feature ADR/specification, security and compatibility review, implementation plan, and normal gates. |
| **Research hypothesis** | A promising claim suggested by sources or the original brief. | A falsifiable experiment, comparison criteria, and a decision record. |
| **Rejected / prohibited** | Incompatible with Aether's current law or trust model. | It remains out of scope unless a human-approved law/ADR change resolves the conflict. |

## Synthesis: seven design propositions

| Proposition | Status | Evidence and reason | Required boundary |
| --- | --- | --- | --- |
| Verified, local-first artifacts are the product foundation. | **Implemented** | AETH is verified before run/write; forge verifies compiler output; the active toolchain has no model authority or network integration. | Preserve this below every future source feature and backend discussion. |
| Values should make ownership and resource transfer locally visible. | **Implemented, bounded M2 scope** | Rust, Swift, Hylo, and Zig provide contrasting evidence that memory/resource behavior should be explicit. Aether 0.6 adds closed arena/Buffer owner behavior. | Preserve the verified closed outcome boundary before adding cross-weave results or effects. |
| Allocation should be explicit and capability-oriented. | **Implemented, bounded M2 scope** | Zig and Odin demonstrate allocator-visible APIs; Aether 0.6 provides one named bounded arena with no ambient allocator. | Specify outcome propagation, extended lifetime, and ABI behavior before expansion. |
| Macro-free compile-time execution should use ordinary, type-checked Aether forms. | **Implemented, bounded M5 scope** | Zig demonstrates explicit compile-time evaluation and Rust demonstrates a restricted constant-evaluation subset; [ADR-008](../ADR-008-m5-deterministic-comptime.md) yields one fixed-budget literal `Whole` evaluator. | Preserve no host authority, no macro expansion, fixed fuel, v8 provenance, and seed identity before expanding comptime. |
| Errors and environmental behavior should be typed rather than hidden. | **Implemented, bounded M4 scope** | Koka and OCaml effect/handler research plus [ADR-007](../ADR-007-m4-typed-error-effect.md) yielded one verified abortive `Error[Whole]` route. | Preserve the clean ownership/resource boundary, total host entry points, v3 diagnostics/structure, and seed identity before expanding the effect surface. |
| Data-layout choice and generic specialization should be explicit or proven semantics-preserving. | **Research hypothesis** | Odin provides visible SoA facilities; shape-based folding from the original brief remains an untested idea. | No automatic layout rewrite without semantic-equivalence tests and a benchmark protocol. |
| AI authoring should target validated structure, while retaining canonical human-readable text. | **Implemented, bounded M3/M4/M5 scope** | `aether.ast/v3`, `aether.edit/v3`, code/span diagnostics, explicit binding stage, and strict local validation expose the current grammar without granting unbounded edits. | Preserve the exact-base, typed top-level-edit boundary; a stable schema and validated operations still precede any “syntax-error-proof” claim. |

## Current-law reconciliation

The historical `Aether.md` brief considered LLVM or a native backend as a
possible long-term architecture. Project-local law currently says something
narrower and binding: Aether parses Aether, emits AETH, verifies AETH, and does
not translate Aether source to LLVM, C, Rust, JavaScript, or another language.

Therefore:

1. AETH remains the only current execution artifact and the Aether VM remains
   the only current execution target.
2. No roadmap item silently introduces a native or LLVM backend.
3. A future backend discussion requires explicit human direction, an ADR,
   revised project law where necessary, an artifact/security model, and a
   reproducible proof that it does not weaken verification or capabilities.
4. C ABI research is not permission to parse arbitrary headers, execute foreign
   code, or expose ambient host authority from an AETH artifact.

This keeps the original research valuable without allowing an old exploratory
brief to override the project constitution.

## AI-first design requirements

“Primarily generated by AI” is a design constraint, not a waiver of language
precision. A future Aether authoring interface must make the following
properties testable:

1. **Canonical text:** equivalent valid source has one formatter-owned layout.
2. **Stable structure:** a versioned AST schema can represent every accepted
   source construct without hidden text expansions.
3. **Validated edits:** an agent submits operations against node IDs/schema
   versions; the compiler rejects stale, malformed, or semantically invalid
   edits before source is written.
4. **Deterministic feedback:** diagnostics identify the failing structured
   construct and stable source span, with machine-readable error codes.
5. **Capability transparency:** generated code makes ownership, allocation,
   effects, and external authority visible in the typed representation.
6. **Offline authority:** any future local model may advise only; parsing,
   formatting, type-checking, compilation, verification, and execution remain
   deterministic compiler/VM decisions.

The first partial foundations existed in 0.5 (canonical formatting and a
canonical AST output). M3 established the bounded schema, edit protocol,
stable diagnostics, and local tooling contract; M4/M5 extend that contract in
Aether 0.8 with effect-aware and stage-aware v3 nodes. Its evidence and limits are
[AETHER_AUTHORING_PROTOCOL_v3.md](../AETHER_AUTHORING_PROTOCOL_v3.md);
fine-grained arbitrary-node edits remain future work.

## Falsifiable research spikes

Before a hypothesis becomes a language commitment, run a small bounded spike
with an explicit stop condition instead of building a broad incomplete feature.

| Topic | Smallest useful experiment | Success evidence | Stop condition |
| --- | --- | --- | --- |
| Mutable value semantics | Dynamic aggregate with borrow, move, revise, and destruction paths. | All alias/use-after-move paths rejected; seed/bootstrap artifacts match; rules are explainable from local source. | Semantics require hidden global lifetime inference or unsound escape exceptions. |
| Explicit allocators | One arena-backed collection with injected failing allocator. | No ambient allocation in its Aether API; deterministic OOM behavior; cleanup/escape rules hold. | Library API needs an implicit global allocator or cannot explain ownership across calls. |
| Compile-time evaluation | **Met in M5:** pure fixed-budget evaluator over five literal Whole operations. | Deterministic byte-identical v8 output, 1,024-directive cap, diagnostics, and no host I/O. | It needs textual macro expansion, runtime host capabilities, or unbounded compiler execution. |
| Typed effects | **Met in M4:** one abortive `Error[Whole]` operation with handled, forwarded, and rejected paths. | `raises Whole`, `raise`, `forward`, and `handle` have visible two-exit semantics; clean boundaries reject owners/loans/resources. | The feature reintroduces hidden exception flow, resumable continuation, resource crossing, or special async coloring. |
| Structural edits | Schema plus typed top-level insert/replace/delete operations over a canonical corpus. | **Met in M3:** valid edits round-trip through formatter/parser and seed validation; invalid/stale/duplicate/malformed edits are rejected deterministically. | Future tooling relies on line-number text replacement or can write semantically unchecked source. |
| SoA / shape analysis | A layout-explicit collection and a candidate same-shape specialization. | Observable semantics and ABI are preserved; benchmark harness reports a reproducible workload. | Performance benefit depends on undocumented layout changes or unsound type erasure. |
| Structured concurrency | Lexical task group with join, failure, and cancellation tests. | No task survives its scope; cleanup/cancellation is deterministic; effects describe blocking. | Detached background work or hidden scheduler ownership is necessary. |

## What “better” is allowed to mean

Aether must not claim universal superiority. It can earn narrow, reproducible
claims along dimensions that matter to its target users:

- fewer invalid AI edit attempts on a published structural-edit corpus;
- explicit, auditable ownership/allocation/effect information in generated
  code;
- verifier rejection of malformed artifacts before execution;
- deterministic builds and byte-for-byte seed/bootstrap agreement for a stated
  language surface;
- reproducible workload-specific memory-layout or compile-latency comparisons;
- capability confinement tested against file, process, network, and shell
  access attempts.

Each comparison must declare baseline versions, hardware, workloads, compiler
flags, variance, and counterexamples. The exact claim register and acceptance
tests are in [../CORE_CLAIMS.md](../CORE_CLAIMS.md).

## Design outcome

The coherent direction is a local-first, verifier-centered systems language
whose human-readable source and machine-readable structure converge on the
same canonical semantics. Resource behavior is intended to be explicit;
effects, data-layout transformations, interop, and concurrency are sequenced
behind that foundation rather than added as disconnected features. The ordered,
approval-gated path is [../ROADMAP.md](../ROADMAP.md).
