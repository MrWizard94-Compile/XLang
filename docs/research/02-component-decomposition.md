# Aether Component Decomposition Matrix

**Status:** SOP phase 2 comparison matrix
**Date:** 2026-07-28; current-contract update: 2026-08-01
**Companion:** [01-reference-systems.md](01-reference-systems.md)

## Applying the SOP comparison lens to a programming language

The SOP's required product categories remain useful for language design when
translated to the language/toolchain domain. This prevents the research from
focusing only on syntax or only on VM internals.

| SOP category | Aether-equivalent component | Core question |
| --- | --- | --- |
| UI | Human source, canonical formatting, editor/LSP, and M3 AI structural editing | Can humans and agents express the same intent without ambiguous text surgery? |
| Workflow logic | Parse, resolve, type-check, compile, verify, run, and package | Is every transformation deterministic, inspectable, and testable? |
| Data model | Source AST, semantic IR, value/ownership model, artifacts, and package metadata | Which facts are explicit at every boundary, and which are inferred? |
| Integrations | Host invoke/forge ABI, future C and package interfaces | Can a boundary be narrow, typed, and independently verified? |
| Permissions | Artifact imports/capabilities, file/process/network authority, local-data rules | Does a program receive only explicitly granted authority? |
| Analytics | Diagnostics, compiler traces, reproducible benchmarks, and artifact provenance | Can decisions be measured without ambient telemetry or source exfiltration? |
| Administration | Versioning, dependency identity, release/rebuild workflow, governance | Can a project reproduce and audit a toolchain result over time? |

## Reference-to-component matrix

The shorthand in this matrix records the component each reference makes most
useful to study. It is not a feature-parity claim.

| Reference | UI / authoring | Workflow logic | Data model | Integrations | Permissions | Analytics | Administration |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Rust | Diagnostics and API docs | Cargo and compiler stages | ownership, traits, `Result` | crates / FFI lessons | unsafe boundary | compiler checks | reproducible package workflow |
| Zig | direct source and `comptime` | build-integrated compilation | explicit allocator | C-facing build paths | explicit resource APIs | test allocators | single-tool workflow |
| Swift | readable typed declarations | compiler ownership checking | value/reference types, consume/borrow | platform ABI lessons | ownership enforcement | diagnostics | language evolution discipline |
| Hylo | value-oriented source model | generic compilation | mutable values and projections | prospective C research | safe defaults / unsafe opt-in | IR specification | experimental-design evidence |
| Koka | concise effect annotations | effect inference and handlers | effect rows | handler-defined environment | declared effects | inferred effect reporting | research compiler |
| OCaml 5 | handler syntax | resumable control flow | effects and continuations | libraries can define control abstractions | handler boundary | semantic manual | stable language manual |
| Roc | AI-readable project material | application/platform split | pure data plus abilities | platform interface | host-owned effects | application boundary | platform packaging model |
| Go | compact concurrent source | goroutines/channels | shared memory plus channels | standard network/process ecosystem | runtime-mediated behavior | race-detector lesson | simple deployment model |
| Odin | data-oriented source forms | native compilation | AoS/SoA layouts, allocators | C-friendly types | explicit low-level controls | performance-oriented layout | compact toolchain |
| WebAssembly | text/binary correspondence | validate then instantiate | typed binary modules/components | interface types and ABI | import-limited authority | validation algorithm | portable binary distribution |

## Candidate Aether components and dependencies

The current product already contains a parser, canonical formatter, bootstrap
semantic checks, AETH verifier/VM, forge bridge, and seed-hosted compiler. The
following map is a design dependency model for future work; boxes marked
“candidate” are not implemented features.

~~~mermaid
flowchart TD
    Source["Canonical Aether source"] --> Ast["Versioned semantic AST"]
    Ast --> Semantics["Ownership, types, diagnostics"]
    Semantics --> Ir["Candidate typed semantic IR"]
    Ir --> Aeth["AETH emission and verifier"]
    Aeth --> Vm["Capability-free Aether VM"]

    Ast --> Ai["M3 validated structural edit protocol"]
    Semantics --> Resources["Candidate allocator and resource rules"]
    Resources --> Abi["Candidate FFI / host ABI"]
    Semantics --> Effects["Candidate typed effects"]
    Effects --> Tasks["Candidate structured concurrency"]
    Semantics --> Shapes["Candidate generic shape analysis"]
    Shapes --> Layout["Candidate explicit / proven layout lowering"]
~~~

### Component contracts

| Component | Present boundary | Candidate future contract | Evidence needed before implementation |
| --- | --- | --- | --- |
| Canonical source and AST | Aether 0.7 M3/M4 has deterministic formatting, `aether.ast/v2`, `aether.diagnostic/v2`, and bounded `aether.edit/v2` top-level operations, including the effect nodes. | Fine-grained structural edits only after a separately versioned safety design. | Preserve round-trip, malformed/stale rejection, schema/version, and core/CLI contract tests for each extension. |
| Ownership and values | `borrow` and `move` protect bounded scalar, byte, and immutable-record use. | A complete value/borrow/mutation/escape model for dynamic aggregates and resources. | Formal rules; negative compile tests; seed-emission proof; soundness review. |
| Allocation | VM intentionally exposes no allocator API. | Explicit capability-passed allocators or arenas, with no ambient default allocation in the language contract. | Allocation trace tests; OOM behavior; ownership/destruction rules; ABI interaction design. |
| Effects and errors | Aether 0.7 implements one typed, abortive `Error[Whole]` effect with terminal `raise`, `forward`, and `handle`; no handler can resume. | General typed, inspectable error propagation with no hidden exception path. | Preserve the v7 source/AETH/VM/seed proof, v2 diagnostics/authoring contracts, hostile-artifact corpus, and a separately specified ownership/cancellation expansion before relaxing the clean boundary. |
| Concurrency | No concurrent Aether execution model exists. | Lexically scoped tasks, cancellation, joins, and effect-mediated blocking/async handlers. | Deterministic scheduler model; orphan-task property tests; cancellation and resource cleanup tests. |
| Generics and layout | No generic/SoA language surface exists. | Shape-aware specialization and explicit or provably safe layout selection. | ABI/layout rules; benchmark protocol; semantic-equivalence tests across layouts. |
| Interop | Forge invokes only a verified Aether compiler with a narrow primitive ABI. | A separately designed capability-mediated FFI, potentially including C ABI support. | Ownership mapping; fixture libraries; malformed-header/input handling; no ambient host authority. |
| Artifact and runtime | AETH v4/v5/v6 compatibility and v7 output are verified before write or execution. | Evolve only through versioned verifier rules and compatibility tests. | Decoder fuzzing; old-artifact compatibility or deliberate rejection; seed and bootstrap parity. |
| Toolchain and administration | CLI, formatter, seed rebuild, and manual release gates exist. | Unified project metadata, dependency identity, LSP, and reproducible package workflow. | Threat model; offline reproducibility proof; upgrade/rollback plan; no implicit cloud state. |

## Design constraints derived from dependencies

1. Resource and ownership semantics precede dynamic aggregates, FFI ownership,
   and safe task cancellation. A C-facing surface cannot responsibly define
   ownership before this model exists.
2. Typed effects precede colorless blocking/async behavior. Adding a scheduler
   before an effect contract would hide important behavior behind runtime magic.
3. M3 proves that a stable semantic AST precedes a structural AI-edit protocol.
   A formatter alone cannot make arbitrary tree edits valid; v1 therefore
   permits typed top-level declarations only.
4. Generic shape analysis precedes any automatic data-layout transformation.
   Layout changes must preserve observable semantics or be explicit in source.
5. The AETH verifier remains below every future compiler path. New source
   features do not bypass verification, forge validation, or host-capability
   limits.

## Review questions for every component proposal

- What exact source and artifact invariants change?
- Which ownership, allocation, and capability facts are explicit to an AI
  generator and to a human reviewer?
- What invalid input is rejected, at which boundary, and with which diagnostic?
- How will bootstrap and seed behavior be compared for the new canonical
  surface?
- What reproducible test, benchmark, or formal property can falsify the claim?

The synthesis answers these questions at the design level in
[03-synthesis-and-evidence.md](03-synthesis-and-evidence.md); the ordered
implementation plan is [../ROADMAP.md](../ROADMAP.md).
