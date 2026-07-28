# Aether Language Development Roadmap

**Status:** Human-approved planning foundation
**Date:** 2026-07-28
**Scope:** This orders future design and engineering work. It does not grant
blanket authorization to implement every milestone without its own approved
specification and constitution gate.

## Current baseline

Stage 6 is complete: Aether 0.5 has a seed-hosted product compile path,
deterministic verified AETH v4/v5 artifacts, bounded value/byte behavior, and
immutable nominal records. Its exact executable scope is
[MANIFEST.md](../MANIFEST.md). The long-range direction is
[NORTH_STAR.md](NORTH_STAR.md); it must not be mistaken for current behavior.

## Ordering principles

1. Nail value/ownership and resource semantics before FFI, dynamic collections,
   task cancellation, or layout rewriting.
2. Design a stable semantic representation before promising structural AI
   editing.
3. Specify effects before adding colorless concurrency or compile-time work
   that can observe the environment.
4. Prove semantics and seed parity before a feature enters default product
   compilation.
5. Preserve current AETH-only execution and capability constraints unless an
   explicit human-approved law/ADR change says otherwise.

## Milestones

| ID | Milestone | Dependency | Primary deliverable | Done only when | Status |
| --- | --- | --- | --- | --- | --- |
| M0 | Research, design foundation, and evidence register | None | Reference study, component matrix, north star, claims, ADR, and this roadmap. | Cross-links/claims are consistent, source-backed, and current-vs-future boundaries are explicit. | **This delivery** |
| M1 | Value and resource semantic specification | M0 | A precise ownership, borrow, move, mutation, escape, dynamic-aggregate, and destruction design; typed semantic-IR proposal. | Rules have counterexamples, negative compile cases, verifier consequences, seed feasibility plan, and human approval of the chosen model. | **Decision package delivered — awaiting human approval** |
| M2 | Explicit allocation and bounded dynamic aggregates | M1 | Allocator/arena capability API plus one representative dynamic collection. | No ambient allocation in the language API; deterministic OOM/cleanup behavior; ownership/escape tests; seed/bootstrap parity; docs/ADR synchronized. | Planned |
| M3 | Versioned structural authoring contract | M1 | Machine-readable semantic AST schema, diagnostic code/span contract, and a small validated edit protocol. | Round-trip corpus, stale/malformed-edit rejection, canonical formatting, local-only storage behavior, and CLI/Studio contract tests. | Planned |
| M4 | Typed errors and effects | M1 | Error/effect semantics with one bounded handled/forwarded/rejected capability. | The type/effect checker, diagnostics, ownership interactions, verifier representation, and seed proof agree; no hidden exception route. | Research-gated |
| M5 | Deterministic compile-time execution | M1 and M4 design decision | A pure, resource-bounded compile-time subset using ordinary Aether forms. | Determinism, limits, no host I/O, diagnostics, and artifact provenance are tested; no macro/text expansion bypass exists. | Research-gated |
| M6 | Generic shapes and data-layout experiment | M1 and M2 | Explicit-layout collection plus a constrained shape-analysis prototype. | Layout/ABI rules, semantic-equivalence tests, and reproducible performance methodology demonstrate a scoped benefit. | Research-gated |
| M7 | Structured concurrency | M2 and M4 | Lexical task-group model with join, failure, cancellation, and effect-mediated blocking. | No orphan task/property tests, deterministic cleanup, capability rules, diagnostics, and seed parity pass. | Research-gated |
| M8 | Foreign/host interface pilot | M1 and M2 | Narrow, typed, ownership-aware ABI fixture; C-facing design only if the threat model supports it. | Invalid-input, ownership transfer, capability denial, ABI compatibility, and local reproducibility tests pass. | Research-gated |
| M9 | Integrated project/tooling evolution | M3 plus stable package/ABI decisions | Reproducible project metadata, dependency identity, formatter/LSP integration, and release workflow proposal. | Security, offline reproducibility, upgrade/rollback, and package verification criteria are approved and tested. | Deferred |

## Milestone detail

### M1 — value and resource semantics first

This is the next design decision because every original pillar depends on it.
The work is a specification and bounded proof plan, not a broad code rewrite.
The proposed package is [DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md](DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md),
[ADR-003](ADR-003-value-resource-semantics.md),
[M1 validation matrix](M1-VALIDATION-MATRIX.md), and
[value/resource research](research/04-value-resource-models.md). It recommends
owned values, non-escaping `borrow`/`access` loans, explicit bounded arenas,
closed allocation outcomes, and logical destruction without user code. The
model is **Proposed**, not accepted or implemented.

Human approval must affirm or alter the following proposed rules:

- `Whole`/`Truth` copy; owners use explicit `borrow`/`move`; `access` is the
  only non-escaping exclusive capability loan; and `revise` is failure-atomic;
- M2 dynamic storage is one fixed-capacity, named arena and a Copy-element
  `Buffer`, with no recursive values, dynamic record fields, manual free, or
  host ABI crossing;
- allocation has a closed `allocated`/`exhausted` outcome and no ambient
  allocator, fallback, panic, or host exception path; and
- source validator, typed semantic IR, new AETH version/verifier, VM, seed,
  forge boundary, and proof corpus enforce the same ownership/region contract.

**M1 stop-ship conditions:** reliance on untracked global lifetime inference,
implicit allocation, an ambiguous destruction rule, host capability leakage, or
a model the seed cannot represent/prove.

### M2 and M3 — two complementary foundations

M2 validates resource behavior with a real, small dynamic collection. M3 gives
AI/human tools a stable semantic contract. They may be developed in separate
approved increments after M1, but neither may silently redefine source
ownership or artifact semantics.

### M4 through M8 — controlled experiments, not feature pile-on

Effects, compile-time execution, layouts/generics, structured concurrency, and
foreign interfaces have large interaction surfaces. Each begins with the
falsifiable spike described in
[research/03-synthesis-and-evidence.md](research/03-synthesis-and-evidence.md).
No milestone advances merely because its happy path works.

## Decision backlog requiring human approval

| Decision | Why it cannot be guessed | Earliest milestone |
| --- | --- | --- |
| Exact mutable-value / borrowing / copying vocabulary | It determines source compatibility, diagnostics, IR, and every future ownership guarantee. | M1 |
| Allocator capability form and OOM contract | It governs every dynamic collection, FFI boundary, and resource cleanup. | M1/M2 |
| Semantic AST schema/versioning and edit authorization model | It determines whether AI edits are safe, portable, and auditable. | M3 |
| Error/effect representation and inference boundary | It affects function types, handlers, cancellation, and compile-time rules. | M4 |
| Compile-time evaluator limits | It affects determinism, denial-of-service resistance, and host authority. | M5 |
| Generic shape and layout semantics | It affects ABI, correctness, performance claims, and debuggability. | M6 |
| Task model and cancellation semantics | It affects resource lifetime, scheduler behavior, and failure propagation. | M7 |
| Foreign interface scope, including any C-header strategy | It affects ownership, hostile input handling, portability, and host capability boundaries. | M8 |
| Any native/LLVM backend | It conflicts with current AETH-only project law and therefore requires explicit law/ADR change before design work. | Outside this roadmap unless approved |

## Universal acceptance gate for a language milestone

Every implementation increment must provide, as applicable:

1. A specification, updated architecture/manifest, and ADR for a material
   decision (`DOC-SYNC-001`, `DOC-ADR-001`).
2. Tests written for intended behavior, including malformed source/artifacts,
   error paths, and compatibility (`TEST-BEHAVIOR-001`, `SEC-INPUT-001`).
3. Seed-hosted emission and byte-identity proof for the documented canonical
   source surface before default product compilation uses the feature.
4. Verifier/VM/forge updates that preserve verify-before-run/write and no host
   capability leak.
5. Clean formatting, tests, linting, Clippy/warnings, and all applicable
   constitution gates (`ENG-WARN-001`, `CONST-GATE-001`).
6. A compact evidence package: exact commands, results, known limits, and
   changed-document manifest (`REV-PACK-001`).

## Risks and mitigations

| Risk | Mitigation |
| --- | --- |
| Feature ambition outruns the small verified core. | One bounded milestone at a time; no new default compile feature without seed proof. |
| AI-first becomes an untestable marketing claim. | Claim register, schema/edit corpus, deterministic diagnostics, and published measurements. |
| Performance goals cause unsound or invisible layout rewrites. | Require explicit semantics, equivalence tests, baseline comparisons, and counterexamples. |
| FFI or future backend weakens local security. | Keep AETH verifier/VM boundary; require capability threat model and explicit human approval. |
| Legacy ideas override current project law. | Treat legacy and `Aether.md` as research input; AGENTS law, ADRs, and current specs control implementation. |

## Immediate next action

Review and approve, reject, or refine the M1 model in
[ADR-003](ADR-003-value-resource-semantics.md). No semantic implementation
begins until that human decision is recorded. If accepted, M2 starts as one
complete bounded increment: explicit arena resource plan, one copy-element
buffer, source/IR/verifier/VM/seed proof, hostile-artifact tests, and
documentation synchronization.
