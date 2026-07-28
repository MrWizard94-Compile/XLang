# Aether Core Claims Register

**Status:** Claim-control document
**Date:** 2026-07-28
**Purpose:** Keep implemented facts, accepted directions, and research hypotheses
separate so the project can be ambitious without overstating evidence.

## Claim status rules

| Status | May be described as | May not be described as |
| --- | --- | --- |
| **Proven now** | current behavior, within the linked contract and tests | a guarantee outside the stated version/surface |
| **Accepted direction** | planned product direction | an implemented feature, syntax, ABI, benchmark result, or schedule commitment |
| **Research hypothesis** | a candidate worth testing | a design decision, performance result, or safety proof |
| **Prohibited now** | a non-goal or constraint | a roadmap promise without explicit human decision and law alignment |

## Register

| ID | Claim | Status | Evidence / boundary | Required next evidence |
| --- | --- | --- | --- | --- |
| CLM-001 | Aether source compiles to Aether-owned AETH artifacts that are verified before run or forge write. | **Proven now** | [MANIFEST.md](../MANIFEST.md), [ARCHITECTURE.md](ARCHITECTURE.md), verifier/VM tests. | Preserve negative verifier tests for each artifact evolution. |
| CLM-002 | Default CLI/Studio compilation is seed-hosted; the Rust core is bootstrap/diagnostic authority; the documented canonical 0.5 surface has seed/bootstrap byte-identity proof. | **Proven now** | [SEED_PROFILE.md](SEED_PROFILE.md), [seed self-host tests](../crates/xlang-core/tests/seed_self_host.rs). | Extend the proof corpus before any new source form joins the product compile path. |
| CLM-003 | AETH execution has no file, process, network, shell, or model authority. | **Proven now** | [ARCHITECTURE.md](ARCHITECTURE.md), project invariants, forge boundary. | Add a negative capability test whenever an embedding/ABI boundary changes. |
| CLM-004 | Aether is local-first: optional Ollama review is loopback-only and never compiler authority. | **Proven now** | [MANIFEST.md](../MANIFEST.md), [ARCHITECTURE.md](ARCHITECTURE.md). | Preserve input bounds and local-only validation when Studio integration evolves. |
| CLM-005 | A future Aether authoring surface should offer a versioned semantic AST and validated structural edits in addition to canonical text. | **Accepted direction** | [NORTH_STAR.md](NORTH_STAR.md), [research synthesis](research/03-synthesis-and-evidence.md). | Schema, edit protocol, stale/malformed-edit behavior, round-trip corpus, and security review. |
| CLM-006 | A future resource model should make allocation explicit and capability-oriented rather than introducing ambient language-level allocation. | **Accepted direction** | [reference study](research/01-reference-systems.md), [roadmap](ROADMAP.md); the concrete M1 model is [proposed and awaiting approval](DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md). | Human approval of ADR-003, then allocation/ownership implementation, OOM behavior, test allocator, and ABI design. |
| CLM-007 | A future error/effect model should make failure and environmental behavior typed and inspectable instead of hiding it in exceptions or async coloring. | **Research hypothesis** | Koka/OCaml/Roc study in [research](research/01-reference-systems.md). | A small semantics spike with handled/forwarded/rejected paths and ownership/cancellation proof. |
| CLM-008 | A future generic/data-layout subsystem may use explicit layouts and shape-aware folding only where equivalence and performance are demonstrated. | **Research hypothesis** | Odin study and [synthesis stop conditions](research/03-synthesis-and-evidence.md). | Layout/ABI specification, equivalence corpus, benchmark harness, counterexamples. |
| CLM-009 | A future C/foreign interface must be narrow, typed, capability-mediated, and ownership-aware. | **Accepted direction** | Existing narrow forge ABI and [component matrix](research/02-component-decomposition.md). | Threat model, ownership mapping, fixture libraries, and invalid-input/capability tests. |
| CLM-010 | Aether source is not translated to C, Rust, JavaScript, LLVM, or another language. | **Prohibited now** | Project-local AGENTS law and [NORTH_STAR.md](NORTH_STAR.md). | Explicit human direction plus project-law/ADR/security/artifact-model change before reconsideration. |
| CLM-011 | Aether is “better than all other languages.” | **Prohibited as a blanket claim** | No finite comparative evidence can establish this. | Replace only with scoped claims that pass the scorecard below. |

## Claim scorecard for scoped comparisons

Any future public comparative statement must identify its scope, baseline,
method, and counterexamples. The following is the minimum accepted scorecard.

| Proposed advantage | Minimum metric | Test evidence | Disallowed shortcut |
| --- | --- | --- | --- |
| Fewer AI authoring failures | Structural-edit success/failure rate on a versioned corpus; parser/formatter round-trip rate. | Corpus, schema version, deterministic expected results, tool versions. | Counting hand-picked examples or accepting raw text edits as structural proof. |
| More explicit resource behavior | Static query showing ownership/allocation/effects at each public boundary. | Negative compile tests, trace fixtures, and documented error/OOM behavior. | Claiming visibility while an implicit allocator or exception path exists. |
| Stronger artifact trust | Malformed-artifact rejection before execution or write. | Decoder/verifier negative corpus, fuzzing method, version tests. | Running artifacts first or treating checksum-only validation as semantic verification. |
| Faster compilation or execution | Median and distribution across declared workloads. | Pinned toolchain, hardware, flags, warm/cold policy, raw data, baseline version. | Single anecdotal timing or changing workloads between systems. |
| Better data locality | Memory-layout and throughput metrics for a stated workload. | ABI/layout contract plus correctness equivalence tests and benchmark data. | Assuming SoA is always faster or silently changing observable layout. |
| Safer integration | Capability-denial and ownership-transfer fixtures. | Host/FFI threat model and tests for invalid/hostile inputs. | Calling FFI “safe” because a wrapper exists. |

## Advancement procedure

To promote an accepted direction or research hypothesis into an implemented
claim, the delivery must provide all applicable items:

1. A current language/architecture specification and an ADR for a material
   decision (`DOC-ADR-001`, `DOC-SYNC-001`).
2. A dependency-first implementation with no unresolved boundary or seed gaps
   (`CONST-DEP-001`, `CONST-COMPLETE-001`).
3. Intended-behavior tests, invalid-input tests, and seed/bootstrap parity for
   every new canonical source form (`TEST-BEHAVIOR-001`).
4. Security and capability review at each input/host boundary
   (`SEC-INPUT-001`).
5. Clean format, build, lint, and warning gates (`ENG-WARN-001`).
6. Reproducible evidence sufficient for the exact sentence used in the claim.

The future-milestone order is [ROADMAP.md](ROADMAP.md). The full research basis
is [research/](research/).
