# Aether Core Claims Register

**Status:** Claim-control document
**Date:** 2026-08-04 (Aether 0.19 / M14 implemented; M15 comptime expansion designed)
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
| CLM-002 | Default CLI compilation is seed-hosted; the Rust core is bootstrap/diagnostic authority; the documented canonical 0.11 surface, including M2–M8 corpora (and prior canonical surface), has seed/bootstrap byte-identity proof. | **Proven now** | [SEED_PROFILE.md](SEED_PROFILE.md), [seed self-host tests](../crates/xlang-core/tests/seed_self_host.rs), resource/effect/comptime/layout/nursery/host-pilot corpus. | Extend the proof corpus before any new source form joins the product compile path. |
| CLM-003 | Default AETH execution has no ambient file, process, network, shell, or model authority; M14 I/O is only via explicit operator grants on the host session. | **Proven now (refined for M14)** | [ARCHITECTURE.md](ARCHITECTURE.md), [AETHER_0.19.md](AETHER_0.19.md), project invariants, path-jail/deny tests. | Preserve grant-empty pure run; add negatives whenever the grant surface expands. |
| CLM-004 | Aether is local-first: the active compiler toolchain has no desktop, model, or network integration, and the CLI writes only caller-selected output paths after validation. | **Proven now** | [MANIFEST.md](../MANIFEST.md), [ARCHITECTURE.md](ARCHITECTURE.md), [ADR-006](ADR-006-retire-aether-studio.md). | Require an ADR, explicit authority boundary, and negative tests before any future integration becomes active. |
| CLM-005 | Aether tooling provides versioned semantic AST, diagnostics, top-level structural edits, and M12 statement-level body edits (`aether.ast/edit/diagnostic` **v7**), including binding stage, shapes, nurseries, and host-weave nodes in addition to canonical text. | **Proven now, bounded M3–M12 authoring scope** | [AETHER_AUTHORING_PROTOCOL_v7.md](AETHER_AUTHORING_PROTOCOL_v7.md), [ADR-005](ADR-005-structural-authoring-contract.md), [ADR-016](ADR-016-m12-fine-grained-edits.md), core/CLI contract tests. | Preserve corpus/negative proof for every new node or operation. |
| CLM-006 | Aether preserves bounded M2 resources that make allocation explicit and capability-oriented rather than introducing an ambient language-level allocator. | **Proven now, bounded M2 scope** | [AETHER_0.11.md](AETHER_0.11.md), [ADR-004](ADR-004-aeth-v6-bounded-resources.md), resource behavior and hostile-artifact tests. | Specify and prove first-class outcome propagation before extending resource boundaries. |
| CLM-012 | Aether carries typed resource facts into bytecode emission and verifies Buffer/table operand provenance before VM execution. | **Proven now, bounded M2/M6 scope** | [AETHER_0.11.md](AETHER_0.11.md), [ADR-004](ADR-004-aeth-v6-bounded-resources.md), [ADR-009](ADR-009-m6-explicit-layout-shapes.md), verifier tests. | Preserve the same proof standard for any additional resource type or cross-weave result. |
| CLM-007 | Aether preserves a bounded, abortive `Error[Whole]` effect with visible handled, forwarded, and rejected routes and no hidden exception path. | **Proven now, bounded M4 scope** | [AETHER_0.11.md](AETHER_0.11.md), [M4 design](DESIGN-M4-TYPED-ERROR-EFFECTS.md), [ADR-007](ADR-007-m4-typed-error-effect.md), [M4 validation matrix](M4-VALIDATION-MATRIX.md), core/seed/authoring tests. | Preserve hostile artifact, seed-byte-identity, and total-entry proof; separately prove any resource/cancellation interaction before relaxing the clean boundary. |
| CLM-013 | Aether evaluates one explicit root-only literal `Whole` arithmetic operation at compile time under a fixed 1,024-directive budget, preserves compile-time provenance, and exposes binding stage through authoring. | **Proven now, bounded M5 scope** | [AETHER_0.11.md](AETHER_0.11.md), [M5 design](DESIGN-M5-DETERMINISTIC-COMPTIME.md), [ADR-008](ADR-008-m5-deterministic-comptime.md), [M5 validation matrix](M5-VALIDATION-MATRIX.md), bootstrap/seed/verifier/authoring tests. | M15 name chaining is the accepted next slice ([ADR-019](ADR-019-m15-comptime-expansion.md)); still require new ADRs for calls, control flow, or code generation. |
| CLM-024 | Bounded comptime expansion that allows prior root-level comptime Whole names as operands (still one op per directive, pure, no host observation) is the accepted M15 / T-CT direction. | **Accepted direction (designed; not implemented)** | [M15 design](DESIGN-M15-COMPTIME-EXPANSION.md), [ADR-019](ADR-019-m15-comptime-expansion.md), [M15 validation matrix](M15-VALIDATION-MATRIX.md). | Implement chain eval + negatives; seed dual-compare; preserve M5 corpus. |
| CLM-014 | Aether admits author-selected dual-layout Whole tables (`rows`/`columns`) with semantic equivalence and seed byte identity; no automatic layout rewrite. | **Proven now, bounded M6 scope** | [AETHER_0.11.md](AETHER_0.11.md), [M6 design](DESIGN-M6-EXPLICIT-LAYOUT-SHAPES.md), [ADR-009](ADR-009-m6-explicit-layout-shapes.md), [M6 validation matrix](M6-VALIDATION-MATRIX.md), layout harness and seed tests. | Require a new ADR before generic shapes, non-Whole fields, or automatic conversion. |
| CLM-015 | Aether admits lexical structured nurseries (`together`/`spawn`) with cooperative source-order execution, first-failure cancel of remaining unstarted spawns, and seed byte identity; no OS-thread parallelism claim. | **Proven now, bounded M7 scope** | [AETHER_0.11.md](AETHER_0.11.md), [M7 design](DESIGN-M7-STRUCTURED-CONCURRENCY.md), [ADR-010](ADR-010-m7-structured-concurrency.md), [M7 validation matrix](M7-VALIDATION-MATRIX.md), nursery and seed tests. | Require a new ADR before nested nurseries, timeouts, task handles, or parallel runtimes. |
| CLM-017 | Offline `aether.project/v1` documents with optional SHA-256 locks and CLI format/project verify provide local project integrity without a network registry or LSP. | **Proven now, bounded M9+ scope** | [AETHER_0.12.md](AETHER_0.12.md), [AETHER_0.13.md](AETHER_0.13.md), [M9 design](DESIGN-M9-PROJECT-TOOLING.md), [ADR-012](ADR-012-m9-project-tooling.md), project module tests, `examples/project`, `examples/project-multi`. | Require a new ADR before multi-package graphs, registries, or full LSP. |
| CLM-018 | Multi-unit offline projects support nested paths, full locks, independent per-unit seed compile, and `project format`, without language modules or cross-file linking. | **Proven now, bounded M10 scope** | [AETHER_0.13.md](AETHER_0.13.md), [M10 design](DESIGN-M10-MULTI-UNIT-PROJECTS.md), [ADR-013](ADR-013-m10-multi-unit-projects.md), [M10 validation matrix](M10-VALIDATION-MATRIX.md), project module tests, `examples/project-multi`. | Require implementable modules ADR (T-MOD / ADR-015 path) before cross-unit symbol resolution or lib-without-main. |
| CLM-019 | Post-M10 growth follows the ADR-014 portfolio: one track design loop at a time; default next design is language modules; host I/O and LSP require their own ADRs (host I/O requires threat-model rewrite); native/registry remain blocked without law change. | **Accepted direction (process)** | [ADR-014](ADR-014-post-m10-track-portfolio.md), [portfolio brief](DESIGN-POST-M10-TRACK-PORTFOLIO.md), [ROADMAP.md](ROADMAP.md). | Human override of order only in writing; no Proven-now claims without per-track matrix. |
| CLM-020 | Language modules (`import unit` / `export weave` / qualified calls, lib without main, `project build`) elaborate the import DAG on the host, then **seed-compile** the elaboration with bootstrap dual-compare (M11b). Single-file seed compile still rejects raw `import unit`. | **Proven now, bounded M11 (host elaborate + seed emit)** | [AETHER_0.12-MODULES.md](AETHER_0.12-MODULES.md), [ADR-015](ADR-015-m11-language-modules.md), module tests, `examples/project-modules`. | Seed does not parse multi-file sources natively; keep dual-compare on every module corpus change. |
| CLM-021 | Fine-grained structural edits replace/insert/delete whole statements under weave bodies via `aether.edit/v7` typed paths; expression-atom surgery and JSONPath are out of scope; seed-before-write preserved on CLI. | **Proven now, bounded M12** | [AETHER_AUTHORING_PROTOCOL_v7.md](AETHER_AUTHORING_PROTOCOL_v7.md), [ADR-016](ADR-016-m12-fine-grained-edits.md), authoring tests. | Keep path grammar closed; extend only with new ADR. |
| CLM-022 | A bounded offline Language Server (`aether lsp`) provides bootstrap diagnostics, symbols, format, hover, and definition over stdio; with an optional project file it resolves exported `import unit` weaves for definition/hover under the project path jail; it is not a second product compiler and does not write disk or emit AETH. | **Proven now, bounded M13a+M13b** | [DESIGN-M13-BOUNDED-LSP.md](DESIGN-M13-BOUNDED-LSP.md), [ADR-017](ADR-017-m13-bounded-lsp.md), CLI `lsp` module tests. | Keep seed as product compile authority; private weaves must not resolve. |
| CLM-023 | Capability-mediated host I/O (grant-backed read/write/env host weaves; deny-by-default; no ambient FS/shell/network) is implemented under threat model v2 (package 0.19 / M14). | **Proven now, bounded M14 scope** | [THREAT_MODEL-v2-CAPABLE-HOST.md](THREAT_MODEL-v2-CAPABLE-HOST.md), [AETHER_0.19.md](AETHER_0.19.md), [M14 design](DESIGN-M14-HOST-IO-CAPABILITIES.md), [ADR-018](ADR-018-m14-host-io-capabilities.md), [M14 validation matrix](M14-VALIDATION-MATRIX.md), core/CLI grant and path-jail tests. | Require a new ADR before network, shell, directory listing, or ambient cwd grants. |
| CLM-008 | Further generic/data-layout expansion beyond M6 dual-layout tables may use additional layouts and shape-aware folding only where equivalence and performance are demonstrated. | **Research hypothesis (M6 core proven)** | Odin study, [synthesis stop conditions](research/03-synthesis-and-evidence.md), M6 delivery evidence. | Layout/ABI extension design, expanded equivalence corpus, benchmark harness, counterexamples. |
| CLM-009 | A broader C/foreign interface beyond the M8 pure host pilot must remain narrow, typed, capability-mediated, and ownership-aware. | **Accepted direction (M8 pilot proven)** | [AETHER_0.11.md](AETHER_0.11.md), [ADR-011](ADR-011-m8-host-abi-pilot.md), pure fixture tests. | Threat model, ownership mapping, fixture libraries, and invalid-input/capability tests for any I/O or C ABI expansion. |
| CLM-016 | Aether admits capability-closed pure host weaves (`host weave` + `HOST_CALL`) with product fixtures `whole_inc` and `text_extent` without grants; missing services fail closed; no ambient I/O. | **Proven now, bounded M8 scope** | [AETHER_0.11.md](AETHER_0.11.md), [M8 design](DESIGN-M8-HOST-ABI-PILOT.md), [ADR-011](ADR-011-m8-host-abi-pilot.md), [M8 validation matrix](M8-VALIDATION-MATRIX.md), host-pilot and seed tests. | I/O-bearing hosts are covered by CLM-023/M14; still require a new ADR for C headers, libloading, or erroring host weaves. |
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
