# Aether North Star

**Status:** Human-approved product direction; not a language-version contract  
**Date:** 2026-08-11 (pins refreshed for package 0.37; M25 local package publication is product)
**Current executable contract:** [MANIFEST.md](../../MANIFEST.md) and
[AETHER_0.37.md](AETHER_0.37.md) (language keyword surface remains **0.11** forms plus bounded M19e task syntax)

## Vision

Aether is intended to become a local-first systems language and toolchain
designed for code that is primarily generated, transformed, and
audited by AI while remaining understandable and controllable by people.

The goal is not to make an unprovable claim that one language is universally
“better than all others.” The goal is to create a system that earns specific,
measurable advantages for AI-assisted systems work: canonical structure,
explicit resource behavior, deterministic artifacts, verification before
execution, and capability-constrained integration.

## Two truth levels

| Level | What it means | Authoritative documents |
| --- | --- | --- |
| **Implemented Aether (package 0.37)** | The language/compiler/VM users can run and the seed-hosted proof covers. | [MANIFEST.md](../../MANIFEST.md), [AETHER_0.37.md](AETHER_0.37.md), [AETHER_0.11.md](AETHER_0.11.md), [SEED_PROFILE.md](SEED_PROFILE.md), [ARCHITECTURE.md](ARCHITECTURE.md), [PROGRESS_REPORT-FULL-PROJECT.md](PROGRESS_REPORT-FULL-PROJECT.md) |
| **North-star direction** | A planned design hypothesis and ordered research program. It is not an implementation claim or syntax promise. | This document, [CORE_CLAIMS.md](CORE_CLAIMS.md), [ROADMAP.md](ROADMAP.md), [ROADMAP-MAINSTREAM-MATURITY.md](../historical%20docs/ROADMAP-MAINSTREAM-MATURITY.md), and [research/](../historical%20docs/research/) |

No future-facing paragraph in this document changes the executable contract. A
feature becomes part of the product only when its specification, ADR where
material, implementation, seed proof, tests, documentation, and constitution
gate all agree.

## Target users and primary journeys

| User | Need | Desired journey |
| --- | --- | --- |
| AI coding agent | Generate and revise code without fragile line-oriented patches or hidden authority. | Read a versioned project/AST contract → submit validated edits → receive deterministic diagnostics → compile only through the verifier boundary. |
| Human director or engineer | Review an agent's intent, resource effects, and artifact provenance quickly. | Inspect canonical source/structure → see explicit ownership, allocation, effects, and capabilities → reproduce a build locally. |
| Systems programmer | Control resources and layout without sacrificing understandable semantics. | Choose ownership/allocator/layout policy explicitly → prove behavior with tests and benchmarks → package a verified artifact. |
| Local-first team | Keep source and build authority on controlled machines. | Use the compiler without cloud authority → retain deterministic local artifacts. |

## Design principles

1. **One semantic shape, two authoring views.** Human-readable canonical text
   and a versioned machine-readable structure must represent the same program;
   no textual macro expansion may hide meaning from tools.
2. **Visible resource behavior.** Ownership transfer, borrowing, allocation,
   failure/effect behavior, and external capabilities should be explicit in the
   source and semantic representation that an agent and reviewer inspect.
3. **Verifier-first execution.** An artifact is decoded and verified before it
   can run or be written by forge. Future features strengthen, rather than
   bypass, this boundary.
4. **Capabilities over ambient authority.** Code receives only the host
   authority that an explicit, typed boundary grants. Any future model adviser
   is outside the compiler trust boundary; it never becomes parser, compiler,
   verifier, or execution authority.
5. **Data-oriented performance must be proved.** Layout control and any
   shape-based specialization must preserve semantics and demonstrate benefits
   on published workloads, not merely promise optimizer magic.
6. **Normal code for compile-time work.** M5 uses constrained, typed Aether
   semantics rather than unconstrained text macros or host evaluation; any
   expansion must preserve that authority boundary.
7. **Design for change without ambiguity.** Artifact versions, AST schemas,
   interfaces, package identities, and diagnostics must have explicit version
   and compatibility rules.

## Intended technical shape

The desired end state is a coherent stack, sequenced by dependencies rather
than implemented all at once:

~~~mermaid
flowchart LR
    Human["Human-readable canonical source"] <--> Structure["Validated AI structural edits"]
    Human --> Semantic["Typed semantic model: values, resources, effects"]
    Structure --> Semantic
    Semantic --> Artifact["Verified AETH artifact"]
    Artifact --> Vm["Capability-constrained Aether VM"]
    Semantic --> Tooling["Formatter, diagnostics, LSP, build/package metadata"]
    Semantic -. future, separately approved .-> Interop["Narrow foreign/host interfaces"]
~~~

The **implemented product (package 0.37)** covers a much larger portion of the
stack than early pilots, while remaining deliberately bounded:

- Seed-hosted product compile + dual-compare for the documented corpus  
- AETH **v11** emit for non-task source and **v12** task-frame emit; verified
  **v4–v11** compatibility inputs
- Bounded resources (multi-weave total arenas under M19d), dual-layout tables,
  cooperative nurseries, `Error[Whole]`, literal + chained comptime  
- M19e deterministic active-frame cancellation at explicit verifier-checked task
  checkpoints; private lanes, reverse-slot teardown, and no external effects
- Pure host pilot, grant-backed host I/O (M14), Whole-only foreign pilot (M21)  
- Authoring `aether.ast/edit/diagnostic` **v8**; offline project/workspace/test/LSP
- Stdlib layer 1; offline cross-package import  
- Transparent locked-project local package pack/verify/publish/install/cache flow

See [AETHER_0.37.md](AETHER_0.37.md), [AETHER_AUTHORING_PROTOCOL_v8.md](AETHER_AUTHORING_PROTOCOL_v8.md),
and [PROGRESS_REPORT-FULL-PROJECT.md](PROGRESS_REPORT-FULL-PROJECT.md).

It does **not** yet provide general typed effects/resumptions, OS-thread
parallelism, generic type parameters, automatic AoS→SoA rewriting, C-header
ingestion, a general or bundled native backend, first-class resource outcomes, Buffer weave
results, general mid-frame cancellation beyond the M19e closed subset, network
registries, or ambient guest I/O.

[M19e's active-frame cancellation design](../historical%20docs/DESIGN-M19E-T-RX-ACTIVE-FRAME-CANCEL.md)
and [ADR-042](../historical%20docs/ADR-042-m19e-active-frame-cancel.md) specify the implemented v12
exception: an explicit task may park at a verifier-checked checkpoint and be
cancelled by an eligible later sibling failure. It does not add general task
handles, timeouts, manual cancellation, parallelism, or external-effect rollback.

## Explicit non-goals and constraints

- Do not claim universal language superiority without a scoped scorecard and
  reproducible evidence.
- Do not transpile Aether source to C, Rust, JavaScript, LLVM, or another
  language under the current project law.
- Do not grant AETH artifacts file, process, network, or shell capability.
- Do not use cloud AI or an AI model as a compiler authority.
- Do not adopt macros that evade canonical parsing, schema validation, or
  tool-aware diagnostics.
- Do not treat `legacy/` as a production input; it is historical reference
  material only.

## How success will be measured

North-star success is a set of narrow, falsifiable claims rather than a slogan:

| Dimension | Evidence standard |
| --- | --- |
| AI authorability | Published corpus with deterministic structural-edit acceptance/rejection and parser/formatter round trips. |
| Resource predictability | Static negative tests, allocation traces, and documented ownership/destruction rules. |
| Artifact trust | Verifier negative corpus/fuzzing, version compatibility tests, and verify-before-run/write evidence. |
| Reproducibility | Byte-identical seed/bootstrap output for a stated surface and reproducible build provenance. |
| Performance | Workload-specific benchmarks with pinned hardware, baselines, flags, variance, and counterexamples. |
| Interoperability | Narrow typed ABI fixtures that prove ownership and capability behavior; no ambient host access. |

The claim register defines the exact proof burden in
[CORE_CLAIMS.md](CORE_CLAIMS.md). The implementation order and stop conditions
are in [ROADMAP.md](ROADMAP.md).

## Reading path

1. [MANIFEST.md](../../MANIFEST.md) — what works today.
2. [research/01-reference-systems.md](../historical%20docs/research/01-reference-systems.md) — the
   official-source study.
3. [research/02-component-decomposition.md](../historical%20docs/research/02-component-decomposition.md)
   and [03-synthesis-and-evidence.md](../historical%20docs/research/03-synthesis-and-evidence.md) —
   the dependency model and design rationale.
4. [CORE_CLAIMS.md](CORE_CLAIMS.md) — what may and may not be claimed.
5. [ROADMAP.md](ROADMAP.md) — the approved order for future work.
