# Aether / XLang — Full Project Progress Report

```
Document: Full Project Progress Report
Status: Honest institutional record (Level 4 product docs)
Authority: AGENTS Constitution + current MANIFEST / claims / ADR set
Date: 2026-08-08
Branch: codex/xlang-local-first-studio
HEAD base: 0adb49f0a3924bed1e7d32bdf62e89825841875e (M23 design/ADR); M23, RTP-001, and PKG-001 are uncommitted working-tree deliveries
Package: aether-core / aether-cli 0.36.0 (working tree)
Language surface: 0.11 keyword forms plus bounded M19e task syntax / AETH v11 and v12
Related Rule IDs: CONST-GATE-001, CONST-DONE-001, CONST-COMPLETE-001,
  CONST-DEP-001, DOC-SYNC-001, ENG-WARN-001, TEST-BEHAVIOR-001,
  SEC-INPUT-001, REV-PACK-001, REL-PACKAGE-001, SOP-PHASE-001, SOP-GATE-001
Supersedes: prior progress report pinned at package 0.12 / HEAD 1ab0be1
```

---

## 1. Executive summary

**Aether** is a **local-first** programming language and CLI toolchain hosted in
this repository (historically labeled XLang). It parses only Aether source,
emits verified **AETH** bytecode, and runs that bytecode in the Aether VM. It
does **not** transpile to Rust, C, JavaScript, LLVM, or any other language
(CLM-010 / project law).

As of **2026-08-08**, package **0.36.0**:

1. **Language pilot M0–M8** is product: surface **0.11** forms / AETH **v11**
   with seed-hosted default compile and dual-compare for the documented corpus.  
2. **Tooling portfolio M9–M18, M20–M22, M17b–d** is product: multi-unit
   projects, modules, structural edits, bounded LSP, grant host I/O, tests,
   workspaces, stdlib layer 1, cross-package import.  
3. **T-RX depth M19a–e** is product within honest bounds: `release`, nursery×
    resource Policy A+, multi-weave arenas, v11 **cooperative** Policy B, and
    v12 deterministic active-frame cancellation at explicit task checkpoints.
4. **M21 foreign ABI pilot** is product after human residual-risk acceptance:
   Whole-only `foreign weave`, `--grant-lib`, seed≡bootstrap for foreign corpus.  
5. **M23 pure comptime calls** are product within their bounded total-guest
   Whole-only contract; bootstrap materializes accepted calls before seed emission
   and the result dual-compares to bootstrap.
6. **RTP-001 runtime Text** caches ASCII provenance privately and restores a
   practical full debug self-host gate; its measured result is local evidence,
   not a general VM performance claim.
7. **PKG-001 offline workspace locks** provide optional local package-identity
   pins, explicit project/workspace lock refresh, project-manifest confinement,
   and locked-build integrity preflight; no registry or network authority exists.
8. **TP-1 integrity** and the **TP-2 local technical preview** are verified
   under AGENTS Constitution: the version-derived 0.36 package has exact
   checksums, consumer behavior checks, and an unlisted-file rejection probe.

**This is not a 1.0 general systems language.** It is a high-trust, bounded
systems pilot with strong verification and capability discipline. Claims below
are evidence-linked and deliberately non-marketing (CLM-011 prohibited).

### 1.1 One-page truth table

| Layer | Truth (0.36) |
| --- | --- |
| Seed-hosted compile + verify + run | **Product-usable** |
| Dual-compare seed≡bootstrap | **Proven** for documented corpus (not full diagnostic parity) |
| AI structural authoring (ast/edit/diagnostic **v8**) | **Product-usable** (bounded) |
| Offline project / workspace / test / LSP | **Product-usable** (bounded, offline; locked workspace integrity is opt-in) |
| Grant host I/O (M14) | **Product-usable** (deny-by-default) |
| Foreign weave pilot (M21) | **Product-usable** (Whole-only; not sandboxed) |
| Multi-weave arenas / resourceful total spawn (M19d) | **Product-usable** |
| Pure comptime helper calls (M23) | **Product-usable, bounded** (no control/recursion/host/effect/resource) |
| Runtime Text scalar operations (RTP-001) | **Product-usable, scoped performance evidence** (ASCII fast path; Unicode scalar fallback) |
| Offline workspace package identity (PKG-001) | **Product-usable, bounded** (local name/path/project identity and nested unit locks; no registry) |
| Cooperative Policy B (M19c) | **Bounded claim only** (no mid-frame cancel) |
| M19e active-frame cancellation | **Product-usable, bounded** — v12 task/checkpoint frames only; no handles, timeouts, preemption, or external effects |
| Network registry / ambient I/O / native backend | **Blocked** — F-NATIVE/F-REGISTRY decision packages ready; need human §3 authorize |
| “Better than all languages” | **Prohibited claim** |

### 1.2 What changed since the prior full report (0.12)

The previous institutional report (HEAD `1ab0be1`, package **0.12**) recorded
M0–M9 product and M10 **design only**. Since then the project shipped:

| Package band | Highlights |
| --- | --- |
| 0.13–0.18 | M10 multi-unit projects; M11 modules (+ seed dual-compare); M12 fine edits; M13a/b LSP |
| 0.19–0.24 | M14 grant I/O; M15 comptime chain; M16 resource+handle; M17 test; M18 workspace; M20/M22 stdlib + xpkg |
| 0.25–0.30 | M19a `release`; M19b Policy A; M20b stdlib L1; M17b–d project test / grants / reports |
| 0.31–0.33 | M21 foreign pilot (+ seed dual-compare); M19d multi-weave arenas + cooperative Policy B; M23 pure comptime calls |
| 0.34 | RTP-001 cached ASCII VM Text fast path; no language or AETH surface change |
| 0.35 | PKG-001 local workspace locks; no language or AETH surface change |
| 0.36 | M19e bounded active-frame cancellation: task/checkpoint source, AETH v12, seed/authoring parity |
| 0.35 | PKG-001 optional offline workspace locks; no language, AETH, VM, capability, registry, or network change |

---

## 2. Governing law (AGENTS Constitution)

### 2.1 Authority stack

| Level | Artifact | Role |
| --- | --- | --- |
| 1–3 | AGENTS Constitution pack | Universal quality + process law |
| 2 | Pack `SOP.md` | Research → design → implement → audit → harden → deliver |
| 1 | Pack Definition of Done | Machine-checkable Done |
| 4 | Current manifest / claims / version contract | Aether mission, invariants, stack pins |
| 4 | `MANIFEST.md` | Executable product contract |
| 4 | `docs/CORE_CLAIMS.md` | Claim register (Proven / Direction / Hypothesis / Prohibited) |
| 4 | ADRs + validation matrices | Per-track decisions and evidence |

### 2.2 Non-negotiable project invariants (Level 4)

1. **Seed-hosted product compile** — default CLI uses the Aether-written seed;
   Rust bootstrap is rebuild / diagnostic / dual-compare authority.  
2. **Verify before run / write** — supported AETH **v4–v11** only.  
3. **No ambient host capability leak** — pure fixtures by default; M14 I/O and
   M21 libraries require explicit operator grants; no shell/network ambient.  
4. **Honest self-host claims** — dual-compare only where proven; **no** full
   seed diagnostic parity claim.  
5. **CLI authority boundary** — local files, explicit outputs, no model/network
   as compiler authority.  
6. **Archived legacy material is not a production build input** — the current
   checkout intentionally contains no active legacy implementation tree.
7. **Zero-warning gate** — Clippy `all = "deny"`; `unsafe_code = "deny"` with
   scoped `allow` only for authorized M21 FFI load path.

### 2.3 Increment recipe (every feature)

```text
design → implementable ADR → validation matrix → vertical slice
→ gates (fmt, clippy -D warnings, tests, dual-compare) → DOC-SYNC
→ delivery report → atomic commit
```

Blocked tracks are not “almost done”: mid-frame Policy B, expanded FFI,
native/LLVM, and network registry require new ADRs and/or law forks.

---

## 3. Product pin (current baseline)

| Layer | Value |
| --- | --- |
| Product name | Aether |
| Repository | XLang workspace (`C:\WPAI\Software\XLang` or equivalent) |
| Branch | `codex/xlang-local-first-studio` |
| Delivery state | M23 + RTP-001 + PKG-001 + M19e package 0.36 working tree; commit intentionally not assumed by this report |
| Core crate | `aether-core` @ `crates/xlang-core` **0.36.0** |
| CLI binary | `aether` via `apps/xlang-cli` / package `aether-cli` **0.36.0** |
| Pilot FFI lib | `aether-ffi-pilot` **0.36.0** (cdylib; not a product dependency of guest code) |
| Language surface | **0.11** keyword forms plus bounded `task weave` / `checkpoint` (records, arena/buffer, effects, comptime, shapes, nurseries, host/foreign weaves, `release`) |
| Artifact | AETH **v11** for non-task source; AETH **v12** for valid task source; **v4–v11** verified compatibility inputs |
| Product compile | Seed-emitted (`compile_with_seed` / embedded `SEED_COMPILER_ARTIFACT`); M23 is bootstrap-materialized before forge |
| Bootstrap | `compile --bootstrap`, `check` AST, seed rebuild, M23 validation/materialization |
| Seed sources | `seed/aether_seed.ae` (~2637 lines) + `seed/aether_seed.aeth` (~31 229 bytes) |
| Rust edition / MSRV | edition 2021, `rust-version = "1.88"` |
| Authoring protocols | `aether.ast/v8`, `aether.edit/v8`, `aether.diagnostic/v8` |

### 3.1 Runtime / host surface (honest)

| Surface | Default | Requires |
| --- | --- | --- |
| Pure host fixtures (`whole_inc`, `text_extent`) | Installed | Nothing |
| Grant I/O host weaves (M14) | Absent | `--grant-read` / `--grant-write` / `--grant-env` |
| Foreign weave load (M21) | Fail closed | `--grant-lib KEY=PATH` (file path; no PATH search) |
| Network / shell / ambient FS | **Never** | Law change + ADR |

---

## 4. Architecture (implemented)

```text
┌─────────────────────────────────────────────────────────────┐
│  CLI `aether` (local files only; explicit --output paths)   │
├─────────────────────────────────────────────────────────────┤
│  Bootstrap (Rust aether-core)                               │
│    parse → validate → semantic resource plan → emit AETH    │
│    check AST / format / structure / apply-edit / LSP diag   │
│    seed rebuild (--bootstrap) / dual-compare authority      │
├─────────────────────────────────────────────────────────────┤
│  Product compile path                                       │
│    bootstrap validate (where required) → forge via seed     │
│    AETH artifact → verify → VM run (optional grants)        │
├─────────────────────────────────────────────────────────────┤
│  Seed Profile (Aether-written)                              │
│    seed/aether_seed.ae → seed/aether_seed.aeth (checked in) │
│    forge ABI: weave compile [borrow source: Text] -> Bytes  │
└─────────────────────────────────────────────────────────────┘
```

**Invariants:** no source-to-other-language transpile; verify-before-run/write;
guest code untrusted; forge host owns I/O after verification.

Authoritative architecture write-up: [ARCHITECTURE.md](ARCHITECTURE.md).  
Forge ABI: [FORGE_CONTRACT.md](FORGE_CONTRACT.md).  
Seed claims: [SEED_PROFILE.md](SEED_PROFILE.md).

---

## 5. Milestone ledger (status at 0.35)

Statuses are **product-honest**: Implemented means matrix + delivery evidence
exist; Design-only means no product claim.

| ID | Milestone | Package (approx.) | Status |
| --- | --- | --- | --- |
| M0 | Research / north star / claims | foundation | **Complete** |
| M1 | Value/resource semantics | ADR-003 direction | **Accepted direction** |
| M2 | Bounded arena + Buffer | 0.6 / v6 | **Implemented** (expanded multi-weave by M19d) |
| M3 | Structural authoring | v1→**v7** | **Implemented** |
| M4 | `Error[Whole]` | 0.7 / v7 | **Implemented** |
| M5 | Literal comptime | 0.8 / v8 | **Implemented** |
| M6 | Dual-layout tables | 0.9 / v9 | **Implemented** |
| M7 | Structured nurseries | 0.10 / v10 | **Implemented** (cooperative only) |
| M8 | Pure host pilot | 0.11 / v11 | **Implemented** |
| M9 | Offline project verify/format | 0.12 | **Implemented** |
| M10 | Multi-unit projects | 0.13 | **Implemented** |
| M11 | Language modules (M11a+b) | 0.14–0.15 | **Implemented** |
| M12 | Fine-grained edits | 0.16 | **Implemented** |
| M13 | Bounded LSP (a+b) | 0.17–0.18 | **Implemented** |
| M14 | Grant host I/O | 0.19 | **Implemented** |
| M15 | Comptime name chaining | 0.20 | **Implemented** |
| M16 | Resource ↔ handle | 0.21 | **Implemented** |
| M17 | Offline `aether test` | 0.22 | **Implemented** |
| M17b | Project `role: test` | 0.28 | **Implemented** |
| M17c | Grants in tests | 0.29 | **Implemented** |
| M17d | Structured test reports | 0.30 | **Implemented** |
| M18 | Offline workspace | 0.23 | **Implemented** |
| M19 | Deeper T-RX umbrella | ADR-023 | **Direction** (sliced) |
| M19a | Explicit `release` | 0.25 | **Implemented** + seed dual-compare |
| M19b | Nursery×resource Policy A | 0.26 | **Implemented** + seed dual-compare |
| M19c | Policy B cooperative | 0.32 | **Bounded product** (ADR-036) |
| M19d | Multi-weave arenas / Policy A+ | 0.32 | **Implemented** + seed dual-compare |
| M19e | Active-frame cancel + destroy | 0.36 / v12 | **Implemented, bounded** ([ADR-042](ADR-042-m19e-active-frame-cancel.md)); task/checkpoint source, private lanes, deterministic teardown, seed/authoring parity, and hostile-artifact evidence [recorded](DELIVERY_REPORT-2026-08-08-M19E-T-RX-IMPLEMENTATION.md) |
| M20 / M20b | Stdlib layer 0/1 | 0.24 / 0.27 | **Implemented** |
| M21 | Foreign ABI pilot | 0.31 | **Implemented** + seed dual-compare |
| M22 | Cross-package import | 0.24 | **Implemented** |
| M23 | Pure comptime weave calls | 0.33 | **Implemented** + bootstrap-materialized seed-emission dual-compare |
| RTP-001 | Runtime Text ASCII fast path | 0.34 | **Implemented**; no language or AETH surface change |
| PKG-001 | Offline workspace locks | 0.35 | **Implemented**; local package identity pins and locked-build preflight; no registry |

### 5.1 Package timeline (toolchain)

| Version | Theme |
| --- | --- |
| **0.11** | Language surface pin (M2–M8 on AETH v11) |
| **0.12–0.13** | Project tooling M9–M10 |
| **0.14–0.18** | Modules, edits, LSP |
| **0.19–0.24** | Host I/O, comptime, resource+handle, test, workspace, stdlib, xpkg |
| **0.25–0.30** | `release`, nursery×resource, stdlib L1, project test/grants/reports |
| **0.31** | Foreign weave pilot + human residual-risk accept + seed dual-compare |
| **0.32** | Multi-weave arenas, Policy A+, cooperative Policy B |
| **0.33** | M23 pure comptime calls; bounded bootstrap materialization before seed emission |
| **0.34** | RTP-001 private cached-ASCII VM Text fast path; local exact self-host evidence |
| **0.35** | PKG-001 optional local workspace locks, explicit lock refresh, and locked-build preflight |

Exact contract text: [MANIFEST.md](../MANIFEST.md), [AETHER_0.35.md](AETHER_0.35.md).

---

## 6. Language surface (0.11 forms + later semantics)

The **keyword grammar** remains the Stage-7 / 0.11 shallow-prefix family.
Later packages add **toolchain semantics** and carefully scoped forms
(`release`, `foreign weave`, multi-weave `arena`) without abandoning AETH-only
execution.

### 6.1 Core language (M0–M8)

| Area | Capability |
| --- | --- |
| Values | `Whole`, `Truth`, `Text`, `Bytes`; copy vs unique owners; `borrow` / `move` |
| Records | Immutable nominal records; `make` / `field borrow` |
| Resources (M2) | `arena N`, `buffer Whole|Truth`, `access`, closed `allocate`/`append`/`at` |
| Effects (M4) | `raises Whole`, `raise`, `forward call`, terminal `handle` |
| Comptime (M5/M15) | Root `comptime bind`; one pure Whole op; name chaining (M15) |
| Layout (M6) | `shape`, `table … layout rows|columns` |
| Concurrency (M7) | `together` / `spawn call … into` (cooperative, ≤8 spawns) |
| Host (M8) | Body-less `host weave`; pure fixtures without grants |

### 6.2 Later language-adjacent product forms

| Form | Package | Notes |
| --- | --- | --- |
| `release name` | 0.25 | OP_RELEASE (66); clean-boundary raise after cleanup |
| Parent resources + nursery (Policy A) | 0.26 | Pure spawn callees; no access across `together` |
| Multi-weave total arenas | 0.32 | Header capacity = **sum** of arenas |
| Resourceful total spawn callees (A+) | 0.32 | Self-owned resources; no resource spawn **args** |
| Cooperative Policy B | 0.32 | Unstarted cancel / return-end only |
| `foreign weave … from "k" symbol "s"` | 0.31 | Whole-only pilot; grant-lib required at run |

### 6.3 Explicit non-goals (current law)

- Transpile to C/Rust/JS/LLVM  
- Ambient guest file/process/network/shell  
- Network package registry  
- General mid-frame cancellation beyond the M19e verifier-checked task/checkpoint subset
- Free-on-raise of live parent owners  
- Full seed diagnostic parity with bootstrap  
- Memory-safety claims for foreign libraries  
- Nested nurseries, OS-thread parallelism  
- General generics / automatic layout rewrite  

---

## 7. Tooling and CLI surface

### 7.1 Commands (product)

| Command | Role |
| --- | --- |
| `aether check` | Bootstrap AST / diagnostics |
| `aether structure` | `aether.ast/v8` JSON |
| `aether apply-edit` | `aether.edit/v8` + seed-before-write |
| `aether format` | Canonical source |
| `aether compile` | Default **seed**; optional `--bootstrap` |
| `aether forge` | Run compiler artifact on source |
| `aether run` | Verify + VM; optional `--grant-*` / `--grant-lib` |
| `aether test` | Discover `*_test.ae`; optional grants/reports |
| `aether project verify|format|build|test` | Offline project integrity / build / tests |
| `aether workspace verify|build` | Multi-package offline graph |
| `aether lsp` | Offline stdio LSP (bootstrap diag; optional project) |
| `aether version` | Package version string |

### 7.2 Offline project / workspace

- `aether.project/v1` — units, optional locks, nested path jail  
- Multi-unit + modules (`import unit` / `export weave`) via host elaboration then seed emit (M11b)  
- `aether.workspace/v1` — packages, acyclic `depends_on`, nested verify  
- M22 — `import unit "…" from package name as alias` under depends_on  

### 7.3 Authoring (AI-first north star slice)

- Semantic AST + diagnostics + structural edits **v7**  
- Statement-level body ops (M12); not expression-atom surgery  
- LSP is **not** a second product compiler and does not emit AETH  

### 7.4 Stdlib

Offline pure modules under `stdlib/`:

| Module | Role |
| --- | --- |
| `whole.ae` | Pure Whole helpers (L0/L1) |
| `truth.ae` | Pure Truth helpers |
| `text.ae` | Pure Text helpers |
| `main.ae` / tests | Demos and pure tests |

No host I/O in stdlib; no registry distribution.

---

## 8. Seed Profile and self-host honesty

### 8.1 Proven

- Seed self-hosts: forge of `seed/aether_seed.ae` matches checked-in
  `seed/aether_seed.aeth` (gate / seed_self_host tests).  
- Product default compile uses the embedded seed artifact.  
- **Byte identity** seed≡bootstrap for the documented corpus, including:  
  shipped seed-path examples, M2–M8 fixtures, M14 host-io *declarations*,
  M15 chain, M16 resource-handle, M19a release, M19b nursery-resource,
  M19d spawn-arena, M21 foreign-pilot/sum, M23 comptime-calls after bootstrap
  materialization, and the multi-module elaboration path.

### 8.2 Not proven / not claimed

- Full diagnostic parity for invalid input (bootstrap remains diagnostic
  authority).  
- Seed native multi-file parse (modules are host-elaborated then seed-emitted).  
- Direct raw-M23 seed evaluation (0.33 deliberately uses the documented
  bootstrap-materialization bridge).
- Dual-compare for undocumenteds / future syntax without a new matrix.  

Evidence: `crates/xlang-core/tests/seed_self_host.rs`, [SEED_PROFILE.md](SEED_PROFILE.md).

---

## 9. Security and threat models

| Document | Scope |
| --- | --- |
| [THREAT_MODEL-TECHNICAL-PREVIEW.md](THREAT_MODEL-TECHNICAL-PREVIEW.md) | TP baseline (offline local tool) |
| [THREAT_MODEL-v2-CAPABLE-HOST.md](THREAT_MODEL-v2-CAPABLE-HOST.md) | Grant-backed host I/O (M14) |
| [THREAT_MODEL-v3-FOREIGN-ABI.md](THREAT_MODEL-v3-FOREIGN-ABI.md) | Foreign library residual risk (M21) |
| [HUMAN-AUTHORIZE-FFI.md](HUMAN-AUTHORIZE-FFI.md) | Human residual-risk accept for M21 |

**Honest residual (M21):** operator-granted native code runs in-process; Aether
does **not** sandbox foreign libraries and does **not** claim their memory safety.

---

## 10. Claims register (summary)

Full table: [CORE_CLAIMS.md](CORE_CLAIMS.md).

| Status class | Representative claims |
| --- | --- |
| **Proven now** | CLM-001–007, 012–018, 020–033, 036–039 (bounded scopes as stated) |
| **Accepted direction / process** | CLM-009 broader FFI ownership, CLM-019 portfolio process, and CLM-034/035 law forks |
| **Research hypothesis** | CLM-008 further layout/generics |
| **Prohibited** | CLM-010 transpile; CLM-011 “better than all languages” |

---

## 11. Evidence inventory

### 11.1 Documentation volume (approx.)

| Kind | Count (repo `docs/`) |
| --- | --- |
| ADRs | 42 (`ADR-001` … `ADR-042`) |
| Design docs | ~36 |
| Validation matrices | ~32 |
| Delivery reports | ~38 |
| Language/toolchain contracts | `AETHER_0.1`…`0.35` (+ modules/authoring) |

### 11.2 Examples (`examples/*.ae`)

Representative fixtures (28 top-level programs), including:

| Example | Proves |
| --- | --- |
| `welcome`, `weaves`, `control-flow`, `unicode` | Baseline seed path |
| `arena-*` | M2 resource outcomes |
| `error-effect`, `release-raise` | M4 / M19a |
| `comptime`, `comptime-chain` | M5 / M15 |
| `layout-table` | M6 |
| `nursery-*`, `nursery-resource`, `spawn-arena` | M7 / M19b / M19d |
| `host-pilot`, `host-io-*` | M8 / M14 |
| `resource-handle` | M16 |
| `foreign-pilot`, `foreign-sum` | M21 |
| `project/`, `project-multi/`, `project-modules/`, `workspace/` | M9–M11, M18, M22 |
| `tests/` | M17 offline tests |

### 11.3 Quality gates

```powershell
pwsh -File .\tools\aether-gate.ps1 -Mode quick   # day-to-day
pwsh -File .\tools\aether-gate.ps1 -Mode full    # includes seed forge identity
pwsh -File .\tools\aether-gate.ps1 -Mode release # full proof + local package
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

TP packaging helpers: `tools/package-preview.ps1`, `tools/verify-preview.ps1`.
The channel remains local-only and `UNLICENSED`; no public release is implied.

As of HEAD `c23f6dd`, **quick gate PASS** was recorded for the M19d delivery
cycle (fmt, clippy, dual-compare examples including `spawn-arena.ae`).

---

## 12. Recent delivery arc (selected commits)

Newest first on `codex/xlang-local-first-studio`:

| Commit | Summary |
| --- | --- |
| `c23f6dd` | M19d multi-weave arenas + cooperative Policy B (0.32) |
| `c55a436` | M21 foreign-sum dual-compare + DOC-SYNC |
| `8b56a1d` | M21 seed foreign weave dual-compare |
| `0c4e9b1` | M21 foreign ABI pilot after human authorize (0.31) |
| `365b824` | M17d structured test reports (0.30) |
| `52198e2` | M17c grants-in-tests; M19c design-only hygiene (0.29) |
| `786f092` | M17b project test (0.28) |
| `b648d9d` | M20b stdlib layer 1 (0.27) |
| `1a31468` | M19b nursery×resource Policy A (0.26) |
| `e142fb8` / `0533d57` | M19a `release` seed then bootstrap (0.25) |
| `6fa846d` | M20 stdlib + M22 xpkg; design M19/M21 (0.24) |
| … | M14–M18, M11–M13, M10, TP-1/TP-2 (see full `git log`) |

---

## 13. Maturity self-assessment (honest)

Against [ROADMAP-MAINSTREAM-MATURITY.md](ROADMAP-MAINSTREAM-MATURITY.md) pillars
(0–5 scale, informal):

| Pillar | ~Score now | Notes |
| --- | --- | --- |
| Multi-file / modules | **3** | M11 + M18 + M22 offline; no registry |
| Type / language depth | **2–3** | Small typed surface; deliberate limits |
| Resource model | **3** | Bounded arenas; multi-weave; no reclaim mid-run |
| Concurrency | **2–3** | Cooperative nurseries only |
| Effects | **2–3** | Single `Error[Whole]` |
| Comptime | **2–3** | Literal + name chain + restricted pure call; no control/recursion |
| Host I/O | **3** | Grant-mediated; no ambient |
| FFI | **2** | Whole-only pilot; residual risk accepted |
| Tooling (test/LSP/edit) | **3** | Offline professional subset |
| Packages / ecosystem | **2** | Offline path graphs; tiny stdlib |
| Verification strength | **4–5** | Aether differentiator — preserve |
| AI structural authoring | **3** | v7 structure/edits; not full agent IDE |
| Local-first authority | **5** | Preserve |

**Interpretation:** credible **niche systems + AI-first toolchain pilot**, not
Rust/Go/Java ecosystem class. Path to peer class remains multi-epoch (E1–E7)
and law-gated.

---

## 14. Open residuals and lawful next work

### 14.1 Blocked without new ADR / human law

| Item | Why blocked |
| --- | --- |
| Broader task cancellation | M19e implements only verifier-checked parked task-frame cancellation. Handles, timeouts, manual cancellation, nested task nurseries, parallelism, and external-effect cancellation require a new ADR. |
| Expanded FFI (Text/Bytes, headers, callbacks) | New threat + ADR |
| Broader runtime / Unicode performance evidence | RTP-001 proves one local ASCII-heavy self-host workload only; other workloads and Unicode need declared measurements. |
| Native / LLVM backend | Law fork F-NATIVE — [HUMAN-AUTHORIZE-NATIVE.md](HUMAN-AUTHORIZE-NATIVE.md) |
| Network registry | Law fork F-REGISTRY — [HUMAN-AUTHORIZE-REGISTRY.md](HUMAN-AUTHORIZE-REGISTRY.md) |
| Ambient guest capabilities | Rejected under current philosophy |
| Free-on-raise | Forbidden (M19-INV-004) |

### 14.2 Lawful next design candidates

Pick **one track** at a time (ADR-014):

1. **Further offline package / stdlib polish** — only through a new scoped
   design and matrix; PKG-001 is complete and does not authorize a registry.
2. **Deeper T-CT** — direct seed evaluation or comptime control/recursion only
   under a new fuel/DoS ADR.
3. **Human law-fork authorize** — post F-NATIVE and/or F-REGISTRY §3 phrase, then
   vertical ADR (not free-form implement).  
4. **Authoring/LSP depth** — still offline; seed remains compile authority.

### 14.3 DOC-SYNC debt

Primary product pins were refreshed in the 2026-08-05 DOC-SYNC honesty pass and
again for M23, RTP-001, PKG-001, and M19e on 2026-08-08. M19e's implementation
delivery updates the parser, verifier, VM, seed, authoring, compatibility, and
evidence boundaries together.
**Product truth** remains:

1. `MANIFEST.md`  
2. `CORE_CLAIMS.md`
3. Latest `AETHER_0.36.md` + relevant ADR/matrix
4. `SEED_PROFILE.md` for the exact product-emission boundary
5. This progress report  

Any remaining historical AETHER_0.x “current package” footers on older contracts
are intentional history; current package is always the highest `AETHER_0.y`.

---

## 15. How to verify this report’s product pin

From repository root:

```powershell
# Version
cargo run -p aether-cli --release -- version
# Expect: Aether 0.36.0

# PKG-001: preview then verify the checked-in locked workspace
cargo run -p aether-cli --release -- workspace lock examples/workspace/aether.workspace.json
cargo run -p aether-cli --release -- workspace verify examples/workspace/aether.workspace.json
cargo run -p aether-cli --release -- workspace build examples/workspace/aether.workspace.json --package app --output target/workspace-app.aeth
# Run target/workspace-app.aeth → exit 42

# Seed-emitted dual-compare sample (M23)
cargo run -p aether-cli --release -- compile examples/comptime-calls.ae --output target/m23.seed.aeth
cargo run -p aether-cli --release -- compile examples/comptime-calls.ae --output target/m23.boot.aeth --bootstrap
# Byte-compare artifacts; run seed-emitted path → exit 512

# Scoped local release self-host measurement (build time excluded)
pwsh -NoProfile -File .\tools\measure-seed-self-host.ps1

# Foreign pilot (M21) — build pilot lib first
cargo build -p aether-ffi-pilot --release
cargo run -p aether-cli --release -- compile examples/foreign-pilot.ae --output target/fp.aeth
cargo run -p aether-cli --release -- run target/fp.aeth --grant-lib pilot=target/release/aether_ffi_pilot.dll
# Expect exit 42

# Offline gate
pwsh -File .\tools\aether-gate.ps1 -Mode quick
```

---

## 16. Rule ID self-audit (this document)

| Rule ID | Status | Notes |
| --- | --- | --- |
| `DOC-SYNC-001` | Pass for report scope | Report matches MANIFEST 0.36 / M19e + M23 + RTP-001 + PKG-001 delivery working tree; historical soft spots called out |
| `CONST-CONTRACT-001` | Pass | Scope = institutional progress record, not a fake 1.0 |
| `RND-INVAR-001` | Pass | Dual-compare / residual-risk honesty preserved |
| `SEC-INPUT-001` | Pass | Grant/foreign residual risk documented |
| `REV-PACK-001` | Pass | Evidence commands, residual list, next actions |
| `CLM-011` | Pass | No superiority marketing |

---

## 17. Document control

| Field | Value |
| --- | --- |
| Path | `docs/PROGRESS_REPORT-FULL-PROJECT.md` |
| Audience | Human directors, auditors, agents under Constitution |
| Update trigger | Package bump, milestone Done, law fork, or HEAD pin for audits |
| Related | `MANIFEST.md`, `CORE_CLAIMS.md`, `ROADMAP.md`, `ROADMAP-MAINSTREAM-MATURITY.md`, delivery reports under `docs/` |

---

## 18. Bottom line

Aether **0.36** is a **complete offline toolchain pilot** for a small, verified,
capability-disciplined language: seed-emitted product compile, rich offline
tooling, bounded resources and effects, cooperative nurseries with multi-weave
arenas, deterministic checkpointed active-frame cancellation, grant I/O, a
human-authorized Whole-only foreign pilot, and restricted
pure compile-time helpers. Its current VM includes a measured local ASCII Text
fast path without changing source or artifact semantics. PKG-001 adds optional
locked local workspaces that bind nested project identity and unit locks before
verification or locked artifact output.

It is **ready for local technical-preview use** within those contracts.

It is **not** ready to claim mainstream systems-language peer status, general
FFI safety, parallel concurrency, or network package ecosystems—those require
further design, evidence, and in some cases explicit human law changes.

---

*End of Full Project Progress Report (updated 2026-08-08 / package 0.36.0).*
