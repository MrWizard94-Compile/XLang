# Aether North Star

**Status:** Human-approved product direction; not a language-version contract
**Date:** 2026-08-01 (CLI product-boundary update)
**Current executable contract:** [MANIFEST.md](../MANIFEST.md) and
[AETHER_0.6.md](AETHER_0.6.md)

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
| **Implemented Aether 0.6** | The language/compiler/VM users can run and the seed-hosted proof covers. | [MANIFEST.md](../MANIFEST.md), [AETHER_0.6.md](AETHER_0.6.md), [SEED_PROFILE.md](SEED_PROFILE.md), [ARCHITECTURE.md](ARCHITECTURE.md) |
| **North-star direction** | A planned design hypothesis and ordered research program. It is not an implementation claim or syntax promise. | This document, [CORE_CLAIMS.md](CORE_CLAIMS.md), [ROADMAP.md](ROADMAP.md), and [research/](research/) |

No future-facing paragraph in this document changes Aether 0.6 behavior. A
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
6. **Normal code for compile-time work.** If compile-time execution is adopted,
   it uses constrained, typed Aether semantics rather than unconstrained text
   macros or host evaluation.
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

The current 0.6 implementation covers a deliberately small portion of this:
canonical source formatting, bounded value semantics, a bootstrap canonical-AST
path, AETH v4/v5 compatibility and v6 verification, seed-hosted product
compilation, a narrow forge ABI, a capability-free VM, and one explicit bounded
arena with closed Whole/Truth buffer outcomes. M3 now adds local `aether.ast/v1`
structure, diagnostic codes/spans, and bounded top-level `aether.edit/v1`
operations; see [AETHER_AUTHORING_PROTOCOL_v1.md](AETHER_AUTHORING_PROTOCOL_v1.md).
It does **not** yet provide typed effects, structured concurrency, generic shape
folding, SoA lowering, C-header ingestion, a native backend, first-class
resource outcomes, Buffer weave results, or fine-grained arbitrary-node edits.

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

1. [MANIFEST.md](../MANIFEST.md) — what works today.
2. [research/01-reference-systems.md](research/01-reference-systems.md) — the
   official-source study.
3. [research/02-component-decomposition.md](research/02-component-decomposition.md)
   and [03-synthesis-and-evidence.md](research/03-synthesis-and-evidence.md) —
   the dependency model and design rationale.
4. [CORE_CLAIMS.md](CORE_CLAIMS.md) — what may and may not be claimed.
5. [ROADMAP.md](ROADMAP.md) — the approved order for future work.
