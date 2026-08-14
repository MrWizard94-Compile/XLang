# Aether Language Development Roadmap

**Status:** M0–M23 product: M19a–e (M19e bounded active-frame cancellation),
M20/M20b, M17b–d, M21, M22, and M23 pure comptime calls; RTP-001 runtime
performance increment; PKG-001 offline workspace locks; M32a/M32b
verified-execution evidence tooling; and M25 local source-package publication.
Package **0.37** remains current. M19e
is implemented as the verifier-checked v12 task-frame slice
(ADR-042); TP-1/TP-2 delivered; portfolio ADR-014
**Date:** 2026-08-14 (BARP through ADR-115; product authority reduction active)
**Scope:** This orders language design and engineering work. Each future
milestone still requires its own versioned specification, evidence, and
constitution gate.

**Mainstream maturity program (multi-epoch):** see
[ROADMAP-MAINSTREAM-MATURITY.md](../historical%20docs/ROADMAP-MAINSTREAM-MATURITY.md) for the full
plan to reach Rust/C++/Go/Java/Python/TS **class** (niche systems + AI-first),
including epochs E0–E7, M11–M40, gates, risks, and law forks. This file remains
the near-term milestone ledger.

## Completion targets (P0 freeze, 2026-08-04)

Human-approved under AGENTS Constitution + SOP:

| Target | Definition | Delivery |
| --- | --- | --- |
| **TP-1** Integrity Complete | Claim/docs sync, formal audit, automated offline gate, clean product tree | Done (audit report) |
| **TP-2** Technical Preview | TP-1 + threat model + local `dist/` package + SHA-256SUMS + consumer verify | Current 0.37 local-only package; no public release or license grant |
| **P4.1** Multi-unit offline projects | Extend M9 after TP-2; full SOP design → ADR → matrix before code | Feature track (not 1.0) |

Current local-package threat model: [THREAT_MODEL-0.37-LOCAL-PACKAGES.md](THREAT_MODEL-0.37-LOCAL-PACKAGES.md).

Historical pure-surface freeze: [THREAT_MODEL-TECHNICAL-PREVIEW.md](../historical%20docs/THREAT_MODEL-TECHNICAL-PREVIEW.md).
Package helpers: `tools/package-preview.ps1`, `tools/verify-preview.ps1`, and
`tools/aether-gate.ps1 -Mode release`.

**Out of scope unless new ADR:** expanded FFI beyond M21 Whole pilot, full
LLVM/object native, registry network fetch, ambient guest I/O, mid-frame cancel
destroy. (Bounded LSP, grant I/O, Whole foreign pilot, M35a AETH→C, M24a offline
registry cache are product under authorized bounds.)

## Current baseline

Stage 7 and M3–M9 are complete: package **0.12** ships language surface **0.11**
/ AETH **v11** with seed-hosted product compile, deterministic verified output
while retaining verified AETH v4–v10 compatibility, bounded value/byte behavior,
immutable nominal records, one explicit bounded arena with Whole/Truth buffers,
dual-layout Whole tables, structured nurseries, one bounded `Error[Whole]`
effect, explicit literal compile-time `Whole` evaluation, a capability-closed
pure host ABI pilot, and offline project verify/format. Authoring provides
`aether.ast/v6`, `aether.edit/v6`, and `aether.diagnostic/v6`. Exact executable
scope is [MANIFEST.md](../../MANIFEST.md), [AETHER_0.12.md](../historical%20docs/AETHER_0.12.md),
[AETHER_0.11.md](AETHER_0.11.md), and
[AETHER_AUTHORING_PROTOCOL_v6.md](../historical%20docs/AETHER_AUTHORING_PROTOCOL_v6.md). The
long-range direction is [NORTH_STAR.md](NORTH_STAR.md); it must not be mistaken
for current behavior. M8 admits pure host weaves only; M9 is offline project
integrity only. Neither claims C headers, libloading, ambient I/O, general
generics, automatic layout rewriting, network registries, full LSP, or seed
invalid-source diagnostic parity.

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
| M0 | Research, design foundation, and evidence register | None | Reference study, component matrix, north star, claims, ADR, and this roadmap. | Cross-links/claims are consistent, source-backed, and current-vs-future boundaries are explicit. | Complete |
| M1 | Value and resource semantic specification | M0 | A precise ownership, borrow, move, mutation, escape, dynamic-aggregate, and destruction design; typed semantic-IR proposal. | Rules have counterexamples, negative compile cases, verifier consequences, seed feasibility plan, and human approval of the chosen model. | **Accepted direction; ADR-003** |
| M2 | Explicit allocation and bounded dynamic aggregates | M1 | Allocator/arena capability API plus one representative dynamic collection. | No ambient allocation in the language API; deterministic OOM/cleanup behavior; ownership/escape tests; seed/bootstrap parity; docs/ADR synchronized. | **Implemented in Aether 0.6: closed one-arena Whole/Truth Buffer core** |
| M3 | Versioned structural authoring contract | M1 | Machine-readable semantic AST schema, diagnostic code/span contract, and a small validated edit protocol. | Round-trip corpus, stale/malformed-edit rejection, canonical formatting, explicit local CLI I/O, and core/CLI contract tests. | **Implemented: v1–v5 historical; v6 current (0.11 surface nodes)** |
| M4 | Typed errors and effects | M1 | One bounded handled/forwarded/rejected `Error[Whole]` capability. | Source, v7 verifier/VM, seed, authoring, and byte identity agree; no hidden exception route. | **Implemented in Aether 0.7; preserved by Aether 0.8/AETH v8** |
| M5 | Deterministic compile-time execution | M1 and M4 design decision | A pure, resource-bounded compile-time subset using ordinary Aether forms. | Determinism, fixed limits, no host I/O, diagnostics, v8 provenance, v3 stage contract, and seed byte identity agree; no macro/text expansion bypass exists. | **Implemented in Aether 0.8/AETH v8** |
| M6 | Generic shapes and data-layout experiment | M1 and M2 | Explicit-layout collection plus a constrained shape-analysis prototype. | Layout/ABI rules, semantic-equivalence tests, and reproducible performance methodology demonstrate a scoped benefit. | **Implemented in Aether 0.9/AETH v9: dual-layout Whole tables** |
| M7 | Structured concurrency | M2 and M4 | Lexical task-group model with join, failure, cancellation, and effect-mediated blocking. | No orphan task/property tests, deterministic cleanup, capability rules, diagnostics, and seed parity pass. | **Implemented in Aether 0.10/AETH v10: structured nurseries** |
| M8 | Foreign/host interface pilot | M1 and M2 | Narrow, typed, ownership-aware pure host ABI fixture (`host weave` / `HOST_CALL`); C-facing design only if a later threat model supports it. | Invalid-input, ownership transfer, capability denial, ABI compatibility, and local reproducibility tests pass. | **Implemented in Aether 0.11/AETH v11: pure host pilot** |
| M9 | Integrated project/tooling evolution | M3 plus stable package/ABI decisions | Reproducible project metadata, dependency identity, formatter/LSP integration, and release workflow proposal. | Security, offline reproducibility, upgrade/rollback, and package verification criteria are approved and tested. | **Implemented pilot in 0.12: offline project verify + format; LSP/registry deferred** |
| M10 | Multi-unit offline projects | M9 | Nested multi-unit `aether.project/v1`, locks, independent per-unit seed compile, `project format`; no language modules. | Path/lock negatives, multi-unit example, independence documented, matrix green, no registry/import. | **Implemented in package 0.13 (language surface still 0.11)** |
| M11 | Language modules | M10 | `import unit` / `export weave` / qualified calls; lib without main; project build; phased seed proof. | Matrix green; bootstrap multi-module (M11a); seed dual-compare before seed authority (M11b). | **Implemented (0.15): M11a+M11b** |
| M12 | Fine-grained structural edits | M3, M11 | `aether.edit/v7` statement-level replace/insert/delete under weave bodies; seed-before-write. | Matrix green; no expression-atom paths; v7 protocol. | **Implemented in package 0.16** |
| M13 | Bounded offline LSP | M12 | `aether lsp` stdio: diagnostics, symbols, format, hover, definition; project-aware imports (M13b). | Matrix green; bootstrap diagnostics honesty; no AETH from LSP. | **Implemented 0.18 (M13a+M13b)** |
| M14 | Capability host I/O | M8 + threat v2 | Grant-backed read/write/env host weaves; deny-by-default; no shell/network. | Threat model v2; matrix path-jail negatives; pure run unchanged. | **Implemented in package 0.19 (ADR-018)** |
| M15 | Comptime expansion (T-CT) | M5 | Prior root-level comptime Whole **name chaining** as operands; still one op/directive, 1,024 fuel, pure, no calls/control/host. | Matrix green; seed dual-compare chain corpus; M5 programs unchanged. | **Implemented in package 0.20 (ADR-019)** |
| M16 | Resource ↔ handle (T-RX) | M2, M4 | Total weave may own arena/buffer/table and terminal-`handle` pure Error[Whole]; abortive raise/forward and nurseries stay resource-free. | Matrix green; seed dual-compare mix example; access cannot span handle. | **Implemented in package 0.21 (ADR-020)** |
| M17 | Offline test runner (T-TEST) | M9, seed compile | `aether test` discovers `*_test.ae`, seed-compiles, pure-runs; pass = exit 0. | Matrix green; empty discovery fails closed; examples/tests pass. | **Implemented in package 0.22 (ADR-021)** |
| M17b | Project test units | M17, M11 | `role: test` + `aether project test` elaborates each test entry with libs. | Matrix green; zero tests fail closed; stdlib import test. | **Implemented in package 0.28 (ADR-030)** |
| M17c | Grants-in-tests | M17, M14 | Optional `--grant-*` on `aether test` / `project test`; default pure. | Matrix green; host-io grant test. | **Implemented in package 0.29 (ADR-031)** |
| M17d | Structured test reports | M17 | Optional `--report` JSON + `--report-junit` XML. | Matrix green; CLI unit tests. | **Implemented in package 0.30 (ADR-033)** |
| M18 | Offline workspace (T-PKG) | M9–M11 | `aether.workspace/v1` multi-package path graph + `workspace verify`; acyclic `depends_on`; nested project verify. | Matrix green; cycle/escape negatives; no registry/linking claim. | **Implemented in package 0.23 (ADR-022)** |
| M19 | Deeper T-RX | M16 | Destruction model before abortive+resource / nursery+resource. | Design only. | **Direction ADR-023** |
| M19a | Explicit `release` | M19 | `release name` + OP_RELEASE; raise-after-cleanup. | Matrix green; seed≡bootstrap for release corpus. | **Implemented in package 0.25 (ADR-027)** |
| M19b | Nursery × resource Policy A | M19a, M7 | Parent may own resources with pure spawn callees; nursery site allows live owners, not access loans. | Matrix green; seed≡bootstrap mix corpus. | **Implemented in package 0.26 (ADR-028)** |
| M19c | Nursery Policy B cooperative | M19d, M7 | Unstarted cancel clean; total resourceful spawns end owners at return. | Matrix green; no mid-frame cancel claim. | **Bounded product in package 0.32 (ADR-036)** |
| M19d | Multi-weave / spawn arenas | M19b, M2 | Total weaves declare self-owned arenas; capacity sum; Policy A+ spawn. | Matrix green; seed≡bootstrap spawn-arena. | **Implemented in package 0.32 (ADR-035)** |
| M19e | Active-frame cancel + destroy | M19a–d, M7 | Versioned `task weave` / `checkpoint`, v12 frames, private lanes, deterministic teardown. | Full matrix: source, v12 verifier/VM, seed, authoring, hostile artifacts, v4–v11 compatibility, and zero-warning gates. | **Implemented in package 0.36 (ADR-042); bounded task-frame scope only** |
| M20 | Stdlib layer 0 | M11 | Pure Whole helper modules under `stdlib/`. | Project build + tests. | **Implemented in package 0.24 (ADR-024)** |
| M20b | Stdlib layer 1 | M20 | Expand pure Whole helpers; add Truth + Text modules; multi-import demo. | Project build dual-compare; `aether test`; exit 42 demo. | **Implemented in package 0.27 (ADR-029)** |
| M21 | Foreign ABI pilot (T-FFI) | M8, threat v3 | Narrow foreign weave + explicit library path grant. | Matrix green; human residual-risk accepted; seed≡bootstrap foreign corpus. | **Pilot implemented in package 0.31 (ADR-025); seed dual-compare proven** |
| M22 | Cross-package import | M11, M18 | `import unit "…" from package name as alias` + `workspace build --package`. | Matrix green; depends_on jail. | **Implemented in package 0.24 (ADR-026)** |
| M23 | Pure comptime weave calls (T-CT) | M15 | `comptime bind <- call` pure total Whole helpers; COMPTIME_WHOLE fold. | Matrix green; seed-native D2a eval (BARP Phase 1); product dual-compare ≡ bootstrap; no host/effect/resource callees. | **Implemented in package 0.33 (ADR-039); BARP Phase 1 seed-native (ADR-043)** |
| RTP-001 | Runtime Text ASCII fast path | Current VM | Private cached ASCII provenance for scalar-equivalent Text operations; no source or AETH change. | Unicode boundary tests, full debug/release suites, and scoped local self-host measurement. | **Implemented in package 0.34 (ADR-040)** |
| PKG-001 | Offline workspace locks | M9, M18, M22 | Explicit project/workspace lock refresh; complete local package identity pins; locked build preflight. | Matrix green; project-manifest + nested unit lock checks; no-registry boundary preserved. | **Implemented in package 0.35 (ADR-041)** |
| M25 | Local package publication | PKG-001, M18, M22 | Transparent locked source bundle, explicit local cache, verify/publish/install commands. | Deterministic bundle identity; hostile metadata/tree/path/cache negatives; two independent M22 consumers; no resolver/network/guest authority. | **Implemented in package 0.37 (ADR-107)** |
| M32a | Verified-execution benchmark suite | Current seed compile + verified VM | Closed embedded `welcome`/`arena-buffer`/`task-loop` corpus; `aether bench`; explicit local JSON report. | Product-seed compile once, explicit verify before samples, empty grants, bounded warmup/iteration counts, behavior-stability checks, no broad performance claim. | **Implemented post-0.36; full gate PASS (ADR-104)** |
| M32b | Profile-bound benchmark comparison | M32a | Opt-in v2 profile/environment/checked-output report plus strict `aether bench compare` data-only comparison. | v1 compatibility; 256 KiB strict inputs; equal profile/environment/workload/source/behavior required; artifact change explicit; no performance claim. | **Implemented post-0.36; full gate PASS (ADR-105)** |

## Milestone detail

### M1 — accepted value and resource direction

This was the foundational design decision because every original pillar depends
on it. The accepted package is [DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md](../historical%20docs/DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md),
[ADR-003](../historical%20docs/ADR-003-value-resource-semantics.md),
[M1 validation matrix](../historical%20docs/M1-VALIDATION-MATRIX.md), and
[value/resource research](../historical%20docs/research/04-value-resource-models.md). It recommends
owned values, non-escaping `borrow`/`access` loans, explicit bounded arenas,
closed allocation outcomes, and logical destruction without user code. The
direction is accepted; Aether 0.6 implements the deliberately closed first M2
surface in [ADR-004](../historical%20docs/ADR-004-aeth-v6-bounded-resources.md).

The accepted rules are:

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

### M2 and M3 — completed bounded resource core and authoring foundation

M2 validates resource behavior with a real, small dynamic collection. Aether
0.6 completes the one-arena Whole/Truth Buffer core: a typed semantic plan,
closed outcomes, v6 verifier/VM behavior, hostile-artifact defense, seed proof,
and synchronized documentation. First-class outcome propagation, Buffer results,
and resource-owner `revise` remain deliberately out of scope and cannot be
added without their own specification and proof.

M3–M8 authoring gives AI/human tools a stable machine-readable semantic AST,
diagnostic code/span contract, and validated edit protocol. Its exact-base stale
guard, strict typed JSON payloads, canonical reparse, and seed compile before
explicit CLI output are documented in
[AETHER_AUTHORING_PROTOCOL_v6.md](../historical%20docs/AETHER_AUTHORING_PROTOCOL_v6.md). The operation
surface remains intentionally limited to top-level declaration
insert/replace/delete; it does not bypass source validation or AETH
verification.

### M4 — implemented bounded error effect

M4's accepted design is [DESIGN-M4-TYPED-ERROR-EFFECTS.md](../historical%20docs/DESIGN-M4-TYPED-ERROR-EFFECTS.md),
[ADR-007](../historical%20docs/ADR-007-m4-typed-error-effect.md), and
[M4 validation matrix](../historical%20docs/M4-VALIDATION-MATRIX.md). It selects one abortive,
typed `Error[Whole]` capability with visible `raises Whole`, `raise`,
terminal `forward call`, and terminal one-line `handle call` forms. The pure
reference model plus source, verifier, VM, seed, and v2 authoring tests prove
the handled, forwarded, rejected, clean-boundary, and total-entry claims.

The design deliberately prohibits continuation capture/resumption, effect
inference, generic error payloads, owner/loan/resource crossings, resource
outcome mixing, host error exits, and a v6 reinterpretation. The M4 increment
remains Aether 0.7/AETH v7 semantics; Aether 0.11 preserves it inside AETH v11
plus `aether.ast/v6`, `aether.edit/v6`, and `aether.diagnostic/v6`. Broader
effects still require a new decision and proof.

### M5 — implemented deterministic compile-time evaluator

M5 is specified by [DESIGN-M5-DETERMINISTIC-COMPTIME.md](../historical%20docs/DESIGN-M5-DETERMINISTIC-COMPTIME.md),
[ADR-008](../historical%20docs/ADR-008-m5-deterministic-comptime.md), and
[M5 validation matrix](../historical%20docs/M5-VALIDATION-MATRIX.md). Aether admits only
root-level immutable `comptime bind` with one literal signed-`Whole` `sum`,
`difference`, `product`, `quotient`, or `remainder`. It has one fixed
evaluation unit per directive, a program-wide 1,024-directive cap, checked VM
arithmetic, no host authority, and `COMPTIME_WHOLE` provenance in current AETH
output.

The bootstrap parser/validator/emitter/verifier/VM, Aether-written seed,
authoring contracts, pure M5 reference model, hostile source/artifact tests,
self-host rebuild, and shipped example dual-compare prove the bounded slice.
M5 does not settle compile-time names, calls, control flow, type computation,
code generation, macros, or build scripts; each remains a new research decision.

### M6 — implemented dual-layout tables

M6 is specified by [DESIGN-M6-EXPLICIT-LAYOUT-SHAPES.md](../historical%20docs/DESIGN-M6-EXPLICIT-LAYOUT-SHAPES.md),
[ADR-009](../historical%20docs/ADR-009-m6-explicit-layout-shapes.md), and
[M6 validation matrix](../historical%20docs/M6-VALIDATION-MATRIX.md). Aether 0.9/0.10 admit Whole-only
`shape` declarations and `table Shape layout rows|columns` with closed allocate/
store/load, semantic equivalence, and a published layout harness.

### M7 — implemented structured nurseries

M7 is specified by [DESIGN-M7-STRUCTURED-CONCURRENCY.md](../historical%20docs/DESIGN-M7-STRUCTURED-CONCURRENCY.md),
[ADR-010](../historical%20docs/ADR-010-m7-structured-concurrency.md), and
[M7 validation matrix](../historical%20docs/M7-VALIDATION-MATRIX.md). Aether 0.10 admits lexical
`together`/`spawn` nurseries with cooperative source-order execution and
first-failure cancel of remaining unstarted spawns. No OS-thread parallelism is
claimed.

### M8 — implemented pure host ABI pilot

M8 is specified by [DESIGN-M8-HOST-ABI-PILOT.md](../historical%20docs/DESIGN-M8-HOST-ABI-PILOT.md),
[ADR-011](../historical%20docs/ADR-011-m8-host-abi-pilot.md), and
[M8 validation matrix](../historical%20docs/M8-VALIDATION-MATRIX.md). Aether 0.11 admits body-less
`host weave` declarations and `HOST_CALL` with a pure product fixture only.

### M9 — implemented offline project tooling pilot

M9 is specified by [DESIGN-M9-PROJECT-TOOLING.md](../historical%20docs/DESIGN-M9-PROJECT-TOOLING.md),
[ADR-012](../historical%20docs/ADR-012-m9-project-tooling.md), and
[M9 validation matrix](../historical%20docs/M9-VALIDATION-MATRIX.md). Package 0.12 adds
`aether.project/v1`, `project verify`, and `format`. Full LSP and network
package registries remain deferred.

### M10 — multi-unit offline projects (implemented)

M10 is specified by [DESIGN-M10-MULTI-UNIT-PROJECTS.md](../historical%20docs/DESIGN-M10-MULTI-UNIT-PROJECTS.md),
[ADR-013](../historical%20docs/ADR-013-m10-multi-unit-projects.md), and
[M10 validation matrix](../historical%20docs/M10-VALIDATION-MATRIX.md). Package **0.13** extends M9
with nested paths and real multi-unit integrity while keeping **independent
compilation units** (no import/module system). See
[AETHER_0.13.md](../historical%20docs/AETHER_0.13.md) and
[DELIVERY_REPORT-2026-08-04-M10-MULTI-UNIT-PROJECTS.md](../historical%20docs/DELIVERY_REPORT-2026-08-04-M10-MULTI-UNIT-PROJECTS.md).

### M11 — language modules (implemented)

M11 is specified by [DESIGN-M11-LANGUAGE-MODULES.md](../historical%20docs/DESIGN-M11-LANGUAGE-MODULES.md),
[ADR-015](../historical%20docs/ADR-015-m11-language-modules.md), and
[M11 validation matrix](../historical%20docs/M11-VALIDATION-MATRIX.md). Package **0.15** elaborates
import graphs then seed-compiles with dual-compare.

### M12 — fine-grained structural edits (implemented)

M12 is specified by [DESIGN-M12-FINE-GRAINED-EDITS.md](../historical%20docs/DESIGN-M12-FINE-GRAINED-EDITS.md),
[ADR-016](../historical%20docs/ADR-016-m12-fine-grained-edits.md), and
[M12 validation matrix](../historical%20docs/M12-VALIDATION-MATRIX.md). Package **0.16** ships
`aether.edit/v7` statement-level ops.

### M13 — bounded offline LSP (designed)

M13 is specified by [DESIGN-M13-BOUNDED-LSP.md](../historical%20docs/DESIGN-M13-BOUNDED-LSP.md),
[ADR-017](../historical%20docs/ADR-017-m13-bounded-lsp.md), and
[M13 validation matrix](../historical%20docs/M13-VALIDATION-MATRIX.md). **M13a:** `aether lsp` stdio
with diagnostics/symbols/format/hover/definition. **M13b:** project-aware
imports. Implementation is the next engineering increment.

### M14 — capability host I/O (designed)

M14 is specified by [THREAT_MODEL-v2-CAPABLE-HOST.md](THREAT_MODEL-v2-CAPABLE-HOST.md),
[DESIGN-M14-HOST-IO-CAPABILITIES.md](../historical%20docs/DESIGN-M14-HOST-IO-CAPABILITIES.md),
[ADR-018](../historical%20docs/ADR-018-m14-host-io-capabilities.md), and
[M14 validation matrix](../historical%20docs/M14-VALIDATION-MATRIX.md). Implementation is the next
engineering increment after this design freeze.

### Remaining research / completion program

Post-M10 growth is governed by [ADR-014](../historical%20docs/ADR-014-post-m10-track-portfolio.md).
Mainstream multi-epoch plan:
[ROADMAP-MAINSTREAM-MATURITY.md](../historical%20docs/ROADMAP-MAINSTREAM-MATURITY.md).

**Default next:** the 0.37 M25 local-publication increment is the current
package contract and passed full/release delivery gates. Select one new scoped,
ADR-backed design. Further stdlib depth remains a separate scoped design; FFI
only after its existing human-authorized residual boundary.

Expansion beyond the currently authorized F-NATIVE and F-REGISTRY ADR tracks
remains blocked without a new human decision.

Each track begins with the falsifiable spike discipline in
[research/03-synthesis-and-evidence.md](../historical%20docs/research/03-synthesis-and-evidence.md).
No milestone advances merely because its happy path works.

## Decision backlog requiring human approval

| Decision | Why it cannot be guessed | Earliest milestone |
| --- | --- | --- |
| Exact mutable-value / borrowing / copying vocabulary | It determines source compatibility, diagnostics, IR, and every future ownership guarantee. | M1 |
| Allocator capability form and OOM contract | It governs every dynamic collection, FFI boundary, and resource cleanup. | M1/M2 |
| Fine-grained structural-edit vocabulary beyond top-level declarations | It must preserve transparent ownership/resource invariants without turning JSON paths into a second unsafe language. | Post-M3 / **T-EDIT** |
| General error/effect representation and inference boundary beyond `Error[Whole]` | It affects function types, handlers, cancellation, and compile-time rules. The M4 initial abortive representation is decided in ADR-007; generalization remains open. | Post-M4 |
| Any expansion beyond M5 literal comptime | Calls, control flow, type computation, or source generation would change determinism, denial-of-service resistance, and host authority. | **M15 / T-CT** name chaining designed ([ADR-019](../historical%20docs/ADR-019-m15-comptime-expansion.md)); further slices still need new ADRs |
| Generic shape and layout semantics | It affects ABI, correctness, performance claims, and debuggability. | M6 |
| Task model and cancellation semantics | M7/v11 retains unstarted-only cancellation; M19e implements the bounded active-frame v12 model. Broader task semantics still require a new ADR. | M7 / M19e |
| Foreign interface scope, including any C-header strategy | It affects ownership, hostile input handling, portability, and host capability boundaries. | M8 / **T-FFI** (after pure host; ADR-014) |
| Any expansion beyond authorized F-NATIVE | It can alter portability, toolchain trust, and verifier/VM boundaries; existing F-NATIVE M35a–j is bounded, while new scope needs an ADR. | After M35j / **T-NATIVE** |
| Language modules / cross-file weave resolution | Changes seed surface and ownership/diagnostics; cannot be implied by multi-unit project files alone. | **M11 / T-MOD** — designed ([ADR-015](../historical%20docs/ADR-015-m11-language-modules.md)); implement M11a then M11b |
| Lib units without standalone `main` | Requires language + seed definition of non-entry units. | With **T-MOD** |
| Host I/O beyond pure fixtures | Expands guest-visible host authority; TP threat model must be revised first. | **T-HOST** (ADR-014; not default next) |
| Bounded LSP | Must not become second compiler authority; offline-first. | **T-LSP** (after T-EDIT preferred) |
| Post-M10 portfolio order override | Only human may reorder tracks; agents must not invent parallel mega-features. | [ADR-014](../historical%20docs/ADR-014-post-m10-track-portfolio.md) |

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

Package **0.37** is the current executable contract: v11 compatibility, M19e
v12 task frames and explicit checkpoints, RTP-001's internal ASCII Text fast
path, PKG-001's optional fully local workspace locks, and M25 transparent
locked source-package publication. M32a/M32b are post-0.36 tooling increments
that add no source, AETH, verifier, VM, or
capability change: they provide a closed pure corpus, optional profiled v2
reports, and strict local data-only comparison. The M19e
vertical slice is complete across parser/semantic model, verifier/VM, seed,
authoring, hostile artifacts, compatibility, and its documented quality gate.
Its design and implementation records are
[DESIGN-M19E-T-RX-ACTIVE-FRAME-CANCEL.md](../historical%20docs/DESIGN-M19E-T-RX-ACTIVE-FRAME-CANCEL.md),
[ADR-042](../historical%20docs/ADR-042-m19e-active-frame-cancel.md),
[M19E-VALIDATION-MATRIX.md](M19E-VALIDATION-MATRIX.md), and the
[implementation delivery report](../historical%20docs/DELIVERY_REPORT-2026-08-08-M19E-T-RX-IMPLEMENTATION.md).

The 0.36 local-preview release/stabilization increment is complete: the release
gate builds, stages, consumer-verifies, and integrity-probes the package
(`GATE PASS mode=release` re-verified 2026-08-10 at HEAD `dd61bde`).

**Active program:** [ADR-043](../historical%20docs/ADR-043-bootstrap-authority-reduction.md) BARP —
reduce Rust bootstrap product-path authority. **Through ADR-115:** seed SPEAK
pilot (003/004/005/006/007/010/011/012/013/014/015), including bounded lexical tab /
top-level `fn ` detection, canonical reserved-task prefixes, exact canonical
task-checkpoint detection, and canonical ordinary-Whole Text-literal / exact
Truth-literal yield detection plus canonical ordinary-Whole `choose same` / `choose less` /
exact `choose bright:` / exact `choose dim:` / exact `choose not bright:` /
exact `choose not dim:` nested-yield detection,
canonical ordinary-Whole direct-bind/root-yield unknown-call detection with
forward-header support, multi-source unit digests, and forge SPEAK merge.
Bootstrap is recovery/oracle only.
**Next:** broader `AE-SEED-011` / `AE-SEED-013` cases and seed-native
multi-file elaboration.
Residual bootstrap: dual-compare oracle emit, recovery `--bootstrap` flags, full
`aether.ast/v8`.

**Parallel maturity evidence:** [ADR-104](../historical%20docs/ADR-104-m32a-verified-execution-benchmarks.md)
and [ADR-105](../historical%20docs/ADR-105-m32b-profile-bound-comparisons.md) define M32a/M32b's
bounded `aether bench` interface. It measures only verified AETH execution
(`verify + decode + execute`) across an embedded pure workload corpus; profiled
v2 reports can be compared only when strict local data identities match. The
implementation passed the full gate and does not change BARP priority, the 0.36
historical M19e semantics, or law-fork authority.

**Law forks (reaffirmed 2026-08-11):** F-NATIVE through **M35j** (cross-compile
target matrix, ADR-059–099); F-REGISTRY through **M24i** X.509-lite CA store
(ADR-060–100). M21 FFI pilot remains product.

**Still needing new ADR / matrix:** expanded FFI signatures, full RFC 5280 X.509 DER,
seed SPEAK full matrix, seed-native multi-file, task handles/timeouts/parallel
runtime (ADR-081; checkpoint required ADR-101; surface ADR-085/093/097).

**Human backlog (updated 2026-08-11):**
1. **BARP** — seed SPEAK full matrix + seed-native multi-file forge ABI.
2. F-NATIVE M35k+ (bundled hermetic toolchain / sysroot).
3. F-REGISTRY full X.509 DER if authorized beyond lite store.
4. Task handles/timeouts/parallel **runtime** ADRs (beyond AE-SEED-015 checkpoint).

**Full project audit (2026-08-11):** HEAD `12a2181` — pack verify PASS;
`aether-gate -Mode full` **GATE PASS**; seed dual-compare identity green.
See [AUDIT_REPORT-2026-08-11-FULL-PROJECT.md](../historical%20docs/AUDIT_REPORT-2026-08-11-FULL-PROJECT.md)
and [PROGRESS_REPORT-FULL-PROJECT.md](PROGRESS_REPORT-FULL-PROJECT.md).
