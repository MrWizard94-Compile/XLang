# Aether Mainstream Maturity Roadmap

**Document ID:** ROADMAP-MAINSTREAM-001  
**Status:** Strategic plan (Level 4 product doc) — **not** a promise of schedule or superiority  
**Date:** 2026-08-04  
**Baseline product:** package **0.13.0** / language surface **0.11** / AETH **v11**  
**Branch pin (at authoring):** `codex/xlang-local-first-studio`  
**Binding law:** AGENTS Constitution + project `AGENTS.md` + `MANIFEST.md`  
**Process law:** SOP phases 1–10 per increment (`SOP-PHASE-001`, `SOP-GATE-001`)  
**Portfolio law:** [ADR-014](ADR-014-post-m10-track-portfolio.md)  
**Claims law:** [CORE_CLAIMS.md](CORE_CLAIMS.md) — **CLM-011 prohibits** “better than all languages”  
**North star:** [NORTH_STAR.md](NORTH_STAR.md) (direction ≠ current behavior)

---

## 0. How to read this document

### 0.1 What “Rust / C++ / Go / Java / Python / TS level” means here

It does **not** mean “identical features” or “wins benchmarks.” It means Aether
reaches **comparable maturity class** on dimensions that make a language
usable for real teams:

| Maturity pillar | Informal meaning |
| --- | --- |
| **P1 Language completeness** | Express real multi-file programs without extreme contortions |
| **P2 Runtime & systems surface** | Do useful I/O and integration under explicit capabilities |
| **P3 Tooling & DX** | Edit, diagnose, navigate, format, test, package at professional quality |
| **P4 Ecosystem & packages** | Share and pin reusable libraries offline-first (registry only if law allows) |
| **P5 Performance & deploy** | Competitive enough for stated workloads; ship binary/runtime story |
| **P6 Trust & verification** | Preserve (and improve) Aether’s verifier-first, local-first advantages |
| **P7 Community & governance** | Spec stability, versioning, docs, contribution model |
| **P8 Evidence & honesty** | Every public claim scorecard-backed (`CORE_CLAIMS`) |

**Target outcome (end of this plan):** Aether is a **credible local-first systems
language + AI-first toolchain** that a team could choose for new projects in
scoped domains — not a toy pilot — **without** abandoning AETH verification or
claiming universal superiority.

### 0.2 What this plan is not

- Not a commitment to calendar dates (effort bands only).  
- Not authorization to implement any track without per-track ADR + matrix.  
- Not permission to add LLVM/native transpile or ambient guest I/O silently.  
- Not a claim that Aether will match C++ template metaprogramming, Java’s
  enterprise ecosystem, or Python’s package count.

### 0.3 Universal increment recipe (every work item)

```text
1. Human scope freeze (CONST-CONTRACT)
2. Research / falsifiable spike + stop condition
3. Design doc
4. Implementable ADR (material decisions)
5. Validation matrix (positive + hostile/negative)
6. Vertical slice implementation (CONST-COMPLETE)
7. Gates: fmt, clippy -D warnings, tests, dual-compare as applicable
8. DOC-SYNC (MANIFEST, AETHER_x.y, claims, ROADMAP)
9. Delivery report + Rule ID self-audit
10. Atomic commit; push only if human directs
```

### 0.4 Effort legend

| Band | Meaning |
| --- | --- |
| **S** | Days–2 weeks focused engineering |
| **M** | ~2–8 weeks |
| **L** | ~2–6 months |
| **XL** | Multi-quarter program |
| **XXL** | Multi-year platform program |

Bands assume a small effective team (1–3 strong engineers + agents under
Constitution). Calendar time expands with review rigor and seed self-host cost.

---

## 1. Baseline: where Aether stands today (honest)

### 1.1 Product pin

| Layer | Today |
| --- | --- |
| Language surface | **0.11** |
| Toolchain package | **0.13.0** |
| Artifact | AETH **v11** default; v4–v10 compatibility inputs |
| Compile path | Seed-hosted product; Rust bootstrap rebuild/diagnostics |
| Runtime | Aether VM; pure host fixtures only |
| Projects | Multi-unit offline integrity; **independent** units (no modules) |
| Authoring | Top-level structural AST/edit/diagnostic **v6** |
| Distribution | Local technical preview package + checksums |
| Ecosystem | None outside repo |

### 1.2 Maturity scorecard vs mainstream (0–5 scale)

Scale: **0** absent · **1** research/spike · **2** pilot proven · **3** usable for
small real projects · **4** competitive for domain · **5** mainstream-class.

| Dimension | Rust | C++ | Go | Java | Python | TS | **Aether now** | Target end-state |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Multi-file modules | 5 | 5 | 5 | 5 | 5 | 5 | **1** (files only) | **4–5** |
| Type system depth | 5 | 5 | 3 | 4 | 2–3 | 4 | **2** (small typed surface) | **4** |
| Memory/resource model | 5 | 3–4 | 2 | 2 | 1 | 1 | **2** (bounded pilot) | **4** |
| Concurrency | 5 | 4 | 5 | 4 | 3 | 3 | **2** (cooperative nursery) | **3–4** |
| Error/effect model | 4 | 3 | 3 | 3 | 2 | 3 | **2** (Error[Whole]) | **3–4** |
| Comptime/meta | 4 | 5 | 2 | 2 | 2 | 3 | **1–2** (literal only) | **3** |
| Host I/O | 5 | 5 | 5 | 5 | 5 | 5 | **0–1** (pure fixtures) | **4** |
| FFI / interop | 5 | 5 | 4 | 4 | 4 | 3 | **1** | **3–4** |
| Package ecosystem | 5 | 4 | 5 | 5 | 5 | 5 | **0** | **3–4** offline-first |
| LSP / IDE | 5 | 4 | 5 | 5 | 5 | 5 | **0–1** | **4** |
| Test tooling | 5 | 3 | 5 | 5 | 5 | 5 | **1** (cargo-side) | **4** |
| Build/package graph | 5 | 4 | 5 | 5 | 5 | 5 | **2** (offline project) | **4** |
| Perf / deploy | 5 | 5 | 4 | 4 | 2 | 3 | **1** (small VM) | **3–4** *or* 5 if law allows native |
| Spec stability | 4 | 3 | 4 | 5 | 3 | 4 | **2** | **4** |
| Artifact verification | 2–3 | 1–2 | 2 | 2 | 1 | 2 | **4–5** (Aether strength) | **5** (keep lead) |
| AI structural authoring | 1–2 | 1 | 1 | 1 | 1 | 2 | **3** (top-level) | **5** (north star) |
| Local-first / no cloud authority | 3 | 3 | 3 | 2 | 2 | 2 | **5** | **5** (keep) |

**Interpretation:** Aether today is a **high-trust, low-surface pilot**. The
journey to “mainstream level” is primarily **surface, systems, tooling, and
ecosystem** expansion while **preserving** verification and capability discipline.

### 1.3 Asymmetric advantages to preserve (never trade away)

1. Verify-before-run/write for AETH  
2. Seed-hosted product compile with dual-compare discipline  
3. Local-first CLI (no model/network as compiler authority)  
4. Explicit, capability-oriented resources (even when expanded)  
5. Canonical text + versioned structure for AI/human dual authoring  
6. Honest claims register (under-claim over hype)

### 1.4 Law forks (human must decide later)

| Fork | If yes | If no (default law) |
| --- | --- | --- |
| **F-NATIVE** | AETH + optional native/LLVM backend after law rewrite | Stay AETH-VM forever; performance via better VM/JIT still allowed if not “transpile source” |
| **F-REGISTRY** | Signed offline-first registry + threat model | Offline path/URL-none packages only |
| **F-AMBIENT** | Rejected under current philosophy | Keep deny-by-default host grants |

This plan assumes **default law** unless a section is marked **Law fork required**.

---

## 2. Reference maturity targets by language family

Use these as **baselines for gaps**, not feature copy lists.

### 2.1 Rust-class

| Expectation | Gap for Aether |
| --- | --- |
| Ownership that scales to real crates | Pilot ownership; no crate graph |
| Trait/impl polymorphism | Absent |
| Cargo + crates.io culture | Offline projects only |
| rustc + rust-analyzer | No modules compiler path; no LSP |
| Fearless concurrency narrative | Cooperative nurseries only |
| FFI + bindgen ecosystem | Pure host pilot only |

### 2.2 C++-class

| Expectation | Gap |
| --- | --- |
| Zero-cost abstraction culture | No optimizing backend story |
| Templates / concepts depth | No generics |
| Multi-decade ABI/layout control | Dual-layout Whole tables only |
| Ubiquitous interop | No C ABI product |

### 2.3 Go-class

| Expectation | Gap |
| --- | --- |
| Batteries-included stdlib | None |
| Goroutines + channels simplicity | Nurseries ≠ runtime scheduler |
| `go mod` + single binary deploy | No modules; VM deploy only |
| Fast compile narrative | Seed forge is slow for full self-host |

### 2.4 Java-class

| Expectation | Gap |
| --- | --- |
| Huge stdlib + enterprise patterns | None |
| JVM ecosystem / tooling | Custom VM only |
| Package namespaces + strong versioning | Project units only |
| Observability / GC ops | Different model entirely |

### 2.5 Python-class

| Expectation | Gap |
| --- | --- |
| Extreme approachability + REPL culture | No REPL product |
| PyPI scale | No packages |
| Dynamic productivity | Aether is static/small surface by design |

### 2.6 TypeScript-class

| Expectation | Gap |
| --- | --- |
| Gradual typing + JS interop | No host language interop |
| First-class editor experience | No LSP |
| npm monorepos | Offline multi-unit without linking |

**Strategic choice:** Aether should aim for a **Rust/Zig-like systems niche +
AI-first authoring**, not Python’s library count or TS’s web monopoly. “Mainstream
level” = **professional completeness in that niche**.

---

## 3. Capability dependency graph (critical path)

```text
                    ┌─────────────────────┐
                    │  M0–M10 BASELINE    │
                    │  0.11 lang / 0.13   │
                    └──────────┬──────────┘
                               │
              ┌────────────────┼────────────────┐
              ▼                ▼                ▼
        ┌──────────┐    ┌──────────┐    ┌──────────────┐
        │ T-MOD    │    │ T-EDIT   │    │ Threat model │
        │ modules  │    │ fine edit│    │ evolution    │
        └────┬─────┘    └────┬─────┘    └──────┬───────┘
             │               │                 │
             ▼               ▼                 ▼
        ┌──────────┐    ┌──────────┐    ┌──────────────┐
        │ T-PKG    │    │ T-LSP    │    │ T-HOST I/O   │
        │ packages │    │ editor   │    │ capabilities │
        └────┬─────┘    └──────────┘    └──────┬───────┘
             │                                 │
             ▼                                 ▼
        ┌──────────┐                    ┌──────────────┐
        │ Stdlib   │                    │ T-FFI        │
        │ layers   │                    │ C/foreign    │
        └────┬─────┘                    └──────────────┘
             │
     ┌───────┴────────┬─────────────┐
     ▼                ▼             ▼
┌─────────┐    ┌──────────┐  ┌────────────┐
│ T-CT    │    │ T-RX     │  │ Concurrency│
│ comptime│    │ res+eff  │  │ expansion  │
└─────────┘    └──────────┘  └────────────┘
     │
     ▼
┌─────────────────────────────────────────┐
│ Performance: VM → optional JIT → FORK   │
│ native only if F-NATIVE law rewrite     │
└─────────────────────────────────────────┘
     │
     ▼
┌─────────────────────────────────────────┐
│ Ecosystem, docs, 1.0 governance, LTS    │
└─────────────────────────────────────────┘
```

**Critical path to “real programs”:**  
`T-MOD → T-HOST → stdlib I/O → T-PKG → test runner → LSP`.

**Critical path to “AI-first mainstream DX”:**  
`T-EDIT → T-LSP → project intelligence → package identity`.

Both paths should advance; **do not** serialize entirely if staffing allows
**two** tracks max (language vs tooling), never unbounded parallel.

---

## 4. Epoch plan (program of programs)

Epochs are **portfolio phases**. Each epoch ends with a **maturity gate** that
is evidence-based. Skipping gates reintroduces pilot debt.

| Epoch | Name | Goal | Exit = “mainstream-ish” progress |
| --- | --- | --- | --- |
| **E0** | Foundation (DONE) | Pilots M0–M10, TP-1/TP-2 | Local-first verified core |
| **E1** | Multi-file language | Modules + linked programs | Real multi-file apps *in-language* |
| **E2** | Authoring platform | Fine edits + LSP + diagnostics parity | Professional editor + AI structure |
| **E3** | Capable host | Capability I/O + stdlib | Programs that touch the world safely |
| **E4** | Interop & packages | FFI + offline packages | Reuse + integration |
| **E5** | Language depth | Generics/effects/comptime/concurrency depth | Expressive power class |
| **E6** | Performance & deploy | VM/JIT/(optional native) + ship story | Competitive deploy |
| **E7** | Ecosystem & 1.0 | Stdlib breadth, LTS, governance | “Choose Aether” credibility |

**Rough effort:** E1–E2 **XL**; E3–E4 **XL–XXL**; E5–E6 **XXL**; E7 continuous.

---

## 5. Epoch E0 — Foundation (complete)

| Item | Status |
| --- | --- |
| Research / north star / claims | Done |
| Records, resources, authoring, effects, comptime pilot, layout, nurseries, host pure pilot | Done |
| Offline projects multi-unit | Done (M10) |
| Integrity + local TP package | Done |
| Portfolio ADR-014 | Done |

**Do not re-litigate E0** except DOC-SYNC and gate automation.

---

## 6. Epoch E1 — Multi-file language (T-MOD first)

**Objective:** Make multi-unit projects **semantically meaningful**.

### 6.1 Milestone M11 — Language modules (primary)

| Field | Content |
| --- | --- |
| Track | T-MOD |
| Effort | **L–XL** |
| Design deliverables | `DESIGN-M11-LANGUAGE-MODULES.md`, `ADR-015`, `M11-VALIDATION-MATRIX.md` |
| Product docs | `AETHER_0.x` language bump likely; AETH version if link format changes |

#### 6.1.1 Decisions to make in design (not pre-decided here)

1. **Import syntax** — path-based vs name-based vs project-graph only  
2. **Export surface** — explicit export list vs world-public  
3. **Compilation model**  
   - A) Single linked AETH artifact  
   - B) Multi-artifact dynamic link at run (harder)  
   - C) Host concatenating modules (rejected by ADR-013 honesty)  
4. **Cycles** — forbid vs layered  
5. **Lib without `main`** — required for real modules  
6. **Seed strategy** — full dual-compare for module corpus before product default  
7. **Visibility vs ownership** — imported values/weaves and resource rules  

#### 6.1.2 Validation matrix themes

| Class | Examples |
| --- | --- |
| Positive | Two-module call, export/import, project multi-unit with link |
| Negative | Missing export, cycle (if forbidden), path escape import, private weave |
| Seed | Byte identity for valid multi-module programs |
| Hostile AETH | Malformed module tables / link metadata |

#### 6.1.3 Done when

- [ ] Human accepts ADR-015  
- [ ] Matrix green  
- [ ] Seed dual-compare for documented module corpus  
- [ ] MANIFEST + claims updated; CLM-018 extended honestly  
- [ ] Delivery report  

#### 6.1.4 Stop conditions

Seed unprovable; ambient globals; import path escapes project root; fake linking
via silent concatenation marketed as modules.

### 6.2 Milestone M12 — Linked project compile entry

| Field | Content |
| --- | --- |
| Depends | M11 |
| Effort | **M** |
| Work | `project build` / `project compile` produces runnable artifact for `main` unit graph |
| Done | One command builds multi-module example end-to-end |

### 6.3 Milestone M13 — Module-aware authoring schema

| Field | Content |
| --- | --- |
| Depends | M11 |
| Effort | **M** |
| Work | `aether.ast` / `edit` / `diagnostic` version bump for module nodes |
| Done | Structural insert/delete of imports/exports with stale-base rules |

### 6.4 E1 maturity gate

| Criterion | Evidence |
| --- | --- |
| Real multi-file program | Shipped example using imports (not independent dual mains only) |
| Project tooling | verify + build for module graph |
| Trust preserved | Verify-before-run; no ambient I/O |
| Scorecard | Modules maturity **≥ 4** internal rating |

---

## 7. Epoch E2 — Authoring platform (AI + human DX)

**Objective:** Match “TS/Rust analyzer class” **for Aether’s model** (structure-first).

### 7.1 Milestone M14 — Fine-grained structural edits (T-EDIT)

| Effort | **L** |
| Depends | Stable post-M11 AST preferred (can start design in parallel carefully) |

| Work | Body-level replace/insert with ownership-preserving reparse; never JSON-as-unsafe-language |
| Negatives | Stale base, illegal partial node, effect/resource break |
| Done | Versioned edit protocol vN; corpus of AI-style patches |

### 7.2 Milestone M15 — Diagnostic parity program

| Effort | **L** |
| Work | Seed/bootstrap diagnostic convergence on invalid corpus; code/span stability |
| Done | Published parity % with explicit non-claims for remainder |

### 7.3 Milestone M16 — Bounded LSP (T-LSP)

| Effort | **L–XL** |
| Depends | M14 recommended; diagnostics stable |

| Scope | hover/definition/refs (module-aware), diagnostics publish, format range |
| Non-goals | Second compiler; silent writes; network |
| Authority | LSP **reads** via bootstrap/seed APIs; **writes** only through explicit edit/apply paths |
| Done | VS Code (or generic LSP) extension offline; golden tests |

### 7.4 Milestone M17 — Formatter/project intelligence

| Effort | **M** |
| Work | Workspace symbols, project-wide format, lock refresh helpers |
| Done | `aether project` suite feels like a small `cargo` subset |

### 7.5 E2 maturity gate

| Criterion | Evidence |
| --- | --- |
| Editor usable daily | LSP offline + format + diagnostics |
| AI authoring | Fine-grained edit corpus success rate published |
| Scorecard | AI authoring **≥ 4**, LSP **≥ 4** |

---

## 8. Epoch E3 — Capable host world (systems usefulness)

**Objective:** Programs that read/write/env under **explicit grants** — Go/Python
usefulness class without ambient authority.

### 8.0 Prerequisite — Threat model 2.0

| Effort | **M** |
| Deliverable | `THREAT_MODEL-v2-CAPABLE-HOST.md` superseding TP freeze for I/O tracks |
| Required before | Any I/O-bearing host weave |

Contents: grant model, path roots, ambient deny, multi-tenant non-goal, audit
logging optional, residual risk table.

### 8.1 Milestone M18 — Capability host I/O pilot (T-HOST)

| Effort | **L–XL** |
| Work | Catalog: e.g. `read_bytes`, `write_bytes`, `env_get` with grant handles |
| Rules | Missing grant fail closed; no shell; path jail |
| Tests | Escape, symlink (policy), oversized I/O, missing file |
| Done | Example “copy file” or “read config” under grants |

### 8.2 Milestone M19 — Stdlib layer 0 (core)

| Effort | **L** |
| Modules | text/bytes utils, result helpers, small collections **if** language supports |
| Depends | M11 modules; may need M20 generics for real stdlib |

### 8.3 Milestone M20 — Error/result ergonomics expansion

| Effort | **L** |
| Work | Beyond single `Error[Whole]` toward usable Result-like patterns **or** justified expansion ADR |
| Stop | Hidden exceptions; effect inference soup |

### 8.4 Milestone M21 — Test runner (`aether test`)

| Effort | **M–L** |
| Work | Discover tests in project, run in VM, report JUnit-like optional |
| Done | Stdlib and examples tested via product CLI |

### 8.5 E3 maturity gate

| Criterion | Evidence |
| --- | --- |
| Real utility programs | ≥3 end-to-end demos with I/O grants |
| Security | Threat model v2 + negative corpus green |
| Scorecard | Host I/O **≥ 4**, test tooling **≥ 3** |

---

## 9. Epoch E4 — Interop and packages

### 9.1 Milestone M22 — Narrow C ABI / FFI (T-FFI)

| Effort | **XL** |
| Depends | M18 lessons strongly recommended |
| Work | Explicit extern signatures; ownership mapping; no full header parser in v1 |
| Stop | libloading chaos; unsound raw pointers as default |

### 9.2 Milestone M23 — Multi-package offline graphs (T-PKG)

| Effort | **L** |
| Work | Path dependencies across roots; lock digests; version pins |
| Non-goal | Network fetch (unless F-REGISTRY) |

### 9.3 Milestone M24 — Optional registry (F-REGISTRY law fork)

| Effort | **XL** |
| Gate | Human law + threat model for remote trust, signatures, mirrors |
| Work | `aether registry` offline cache, signature verify, reproducible fetch |
| If rejected | Stay path-dep only; still can hit maturity 3–4 for teams with monorepos |

### 9.4 Milestone M25 — Package publish workflow (local)

| Effort | **M** |
| Work | Pack source+metadata+lock; verify; install from path/cache |

### 9.5 E4 maturity gate

| Criterion | Evidence |
| --- | --- |
| Third-party style reuse | ≥1 internal package consumed by ≥2 projects |
| FFI | One maintained C library binding fixture |
| Scorecard | Packages **≥ 3–4**, FFI **≥ 3** |

---

## 10. Epoch E5 — Language depth (expressive power)

Order inside E5 is flexible after E1; prefer after modules.

### 10.1 Milestone M26 — Generics / interfaces (controlled)

| Effort | **XL** |
| Models | Traits-like vs type params only; start monomorphization-only |
| Seed | Dual-compare mandatory |
| Stop | Unbounded compile-time explosion without fuel accounting |

### 10.2 Milestone M27 — Algebraic data / richer records

| Effort | **L–XL** |
| Work | Sum types / enums; nested records if sound |
| Depends | Ownership rules for variants |

### 10.3 Milestone M28 — Comptime expansion (T-CT)

| Effort | **L–XL** |
| Work | Pure calls/control within fuel; still no host I/O at comptime |
| Stop | Macro-text generation that bypasses structure |

### 10.4 Milestone M29 — Resource ↔ effect (T-RX)

| Effort | **L–XL** |
| Work | Safe interaction of errors/cancel with arenas/buffers |
| Stop | Leaks, double-free, cancel unsoundness |

### 10.5 Milestone M30 — Concurrency expansion

| Effort | **XL** |
| Options | Nested nurseries, channels, timeouts — **not** claiming Go runtime parity early |
| Stop | Data races as undefined; orphan tasks |

### 10.6 Milestone M31 — Layout/generics performance program

| Effort | **L** |
| Work | Expand M6; prove locality claims with CORE_CLAIMS scorecard |

### 10.7 E5 maturity gate

| Criterion | Evidence |
| --- | --- |
| Expressiveness | Port non-trivial algorithms/libs without absurd verbosity |
| Type system | Generics maturity **≥ 3–4** |
| Safety interactions | RX matrix green |

---

## 11. Epoch E6 — Performance and deployment

### 11.1 Milestone M32 — VM performance engineering

| Effort | **L–XL** |
| Work | Hot path ops, allocation, dispatch; benchmarks vs baseline Aether versions |
| Method | Pinned hardware, medians, workloads in repo |

### 11.2 Milestone M33 — Optional JIT (still AETH)

| Effort | **XL** |
| Work | Tiered compilation of verified AETH; **not** source-to-LLVM |
| Law | Compatible with no-transpile if JIT consumes verified bytecode only |

### 11.3 Milestone M34 — Single-file / embeddable runtime distribution

| Effort | **M** |
| Work | Runtime + stdlib packaging; versioned runtime ABI |

### 11.4 Milestone M35 — Native backend (F-NATIVE only)

| Effort | **XXL** |
| Law fork | Explicit rewrite of Level-4 invariants + ADR + security model |
| Work | Lowering strategy, retain verification story or dual pipeline honesty |
| If rejected | Declare AETH-VM+JIT as the performance path and optimize there |

### 11.5 E6 maturity gate

| Criterion | Evidence |
| --- | --- |
| Perf claims | Scorecard-backed vs **prior Aether** and optionally vs C baseline on microbenches |
| Deploy | Documented production runbook |
| Scorecard | Perf/deploy **≥ 3** (or **4–5** with native fork) |

---

## 12. Epoch E7 — Ecosystem, stdlib breadth, and 1.0 governance

### 12.1 Stdlib roadmap (layered)

| Layer | Contents | Depends |
| --- | --- | --- |
| **L0** | primitives sugar, bytes/text, result | E1–E3 |
| **L1** | collections, sorting, hashing, json/toml subset | Generics helpful |
| **L2** | path, fs (capability), process (strict grants) | T-HOST |
| **L3** | net (capability, threat model++) | Separate ADR |
| **L4** | crypto, compress, encoding | FFI or pure impl |
| **L5** | async/http frameworks | Only after concurrency+net |

### 12.2 Milestone M36 — Documentation site & tutorial path

| Effort | **M–L** |
| Work | Learn Aether in one day path; book; API refs generated from structure |

### 12.3 Milestone M37 — Spec freeze process

| Effort | **M** |
| Work | RFC process, semver for language vs package, deprecation windows |

### 12.4 Milestone M38 — Compatibility lab

| Effort | **L** |
| Work | Artifact compatibility suite across AETH versions; upgrade tools |

### 12.5 Milestone M39 — Security program

| Effort | Continuous |
| Work | Fuzz verifier/parser/project; supply chain for seed; release signing optional |

### 12.6 Milestone M40 — 1.0 release candidate program

| Effort | **XL** |
| Requirements | All E1–E4 gates + selected E5; full constitution audit; threat model current; no Critical findings |
| Non-requirement | Matching PyPI size or C++ template power |

### 12.7 E7 / 1.0 maturity gate (definition of “mainstream-class Aether”)

Aether may be called **1.0 / mainstream-class for its niche** only when **all** hold:

1. **Multi-file modules** with seed proof (E1)  
2. **Editor-grade LSP** offline (E2)  
3. **Capability I/O stdlib** sufficient for CLI tools (E3)  
4. **Offline packages** path (E4; registry optional)  
5. **Test runner** + CI story for consumers  
6. **Perf story** documented (VM/JIT; native only if forked)  
7. **Claims register** has no overstated Proven-now items  
8. **Human** accepts 1.0 under Constitution audit (SOP 8–10)  
9. Explicit **non-claims** published (not a JVM, not a browser language, etc.)

---

## 13. Work breakdown structure (WBS) — track catalog

| ID | Track | Epoch | Effort | Law notes |
| --- | --- | --- | --- | --- |
| T-MOD | Language modules | E1 | L–XL | Default next design |
| T-LINK | Project linked build | E1 | M | After T-MOD |
| T-EDIT | Fine-grained edits | E2 | L | AI north star |
| T-DIAG | Diagnostic parity | E2 | L | Seed honesty |
| T-LSP | Bounded LSP | E2 | L–XL | Not second compiler |
| T-TM2 | Threat model v2 | E3 | M | Before host I/O |
| T-HOST | Host I/O capabilities | E3 | L–XL | Grants only |
| T-STD0 | Stdlib L0–L1 | E3–E5 | L–XL | Continuous |
| T-TEST | `aether test` | E3 | M–L | |
| T-FFI | C/foreign ABI | E4 | XL | After host lessons |
| T-PKG | Multi-package offline | E4 | L | |
| T-REG | Registry | E4 | XL | **Law fork** |
| T-GEN | Generics/interfaces | E5 | XL | |
| T-ADT | Sum types / richer data | E5 | L–XL | |
| T-CT | Comptime expansion | E5 | L–XL | No host at comptime |
| T-RX | Resource×effect | E5 | L–XL | |
| T-CONC | Concurrency depth | E5 | XL | |
| T-PERF | VM perf | E6 | L–XL | |
| T-JIT | JIT | E6 | XL | Bytecode only |
| T-NATIVE | Native/LLVM | E6 | XXL | **Law fork** |
| T-DOC | Docs/site | E7 | M–L | |
| T-GOV | RFC/semver/1.0 | E7 | L–XL | |

---

## 14. Parallelization model (how to run the program)

### 14.1 Lanes

| Lane | Owns | Notes |
| --- | --- | --- |
| **L-Lang** | modules, types, effects, comptime, concurrency | Seed dual-compare heavy |
| **L-Host** | threat model, I/O, FFI, stdlib fs | Security reviews mandatory |
| **L-Tool** | edits, LSP, project CLI, test runner | Must not fork semantics |
| **L-Perf** | VM/JIT/benchmarks | After language stability windows |
| **L-Gov** | docs, RFC, release, claims | Continuous |

**Rule:** Max **two** active implementation tracks; unlimited design drafting.  
Seed rebuild is a global bottleneck — schedule full gates.

### 14.2 Cadence

| Cadence | Activity |
| --- | --- |
| Per PR | `aether-gate -Mode quick` |
| Per milestone | `-Mode full` + delivery report |
| Per epoch | Formal audit + maturity scorecard update |
| Continuous | DOC-SYNC, claim register hygiene |

### 14.3 Roles (Constitution-adapted)

| Role | Responsibility |
| --- | --- |
| Human director | Law forks, epoch exit, 1.0 accept, reorder tracks |
| Architect | ADRs, threat models |
| Language eng | Parser/seed/VM |
| Tooling eng | LSP/edits/CLI |
| Security reviewer | Host/FFI/registry |
| Claim auditor | CORE_CLAIMS honesty |

---

## 15. Versioning strategy (suggested)

| Stream | What bumps |
| --- | --- |
| Language surface `0.x` | Syntax/semantics users write |
| AETH `vN` | Bytecode/shape; compatibility windows documented |
| Toolchain package | CLI/project/LSP even if language stable |
| Authoring schema `aether.ast/vN` | Structure nodes |
| Project schema | `aether.project/vN` if graph model changes |

**Policy:** Prefer **toolchain-only** bumps when possible; language bumps require
seed proof. Never silent AETH meaning changes for old versions.

---

## 16. Risk register (program-level)

| Risk | Likelihood | Impact | Mitigation |
| --- | --- | --- | --- |
| Scope thrash (everything at once) | High | High | ADR-014; two-track cap |
| Seed forge time kills iteration | High | Med | quick/full gates; corpus subsetting with honesty |
| Host I/O becomes ambient | Med | Critical | Threat model v2; deny-by-default tests |
| LSP second compiler | Med | High | Authority ADR; no silent compile path |
| Generics compile bombs | Med | High | Fuel limits; monomorph caps |
| Ecosystem emptiness after 1.0 | High | High | Stdlib + offline packages before “1.0” marketing |
| Native fork splits community | Med | High | Only with explicit law rewrite |
| Overclaim marketing | High | High | CORE_CLAIMS gate on every release note |
| Diagnostic debt | High | Med | Explicit parity program M15 |
| Resource×effect unsound | Med | Critical | T-RX before concurrent I/O heavy stdlib |

---

## 17. Evidence & claim policy for “mainstream” marketing

Before any public statement like “production-ready” or “comparable to Go for CLI tools”:

1. Name **scope** (CLI tools / embedded / teaching).  
2. Name **baseline** (which language/version).  
3. Publish **metrics** (CORE_CLAIMS scorecard).  
4. Publish **counterexamples** (what still hurts).  
5. Update claims register statuses.  

**Forbidden:** “Aether replaces Rust/C++/…”.  
**Allowed:** “Aether 1.0 targets local-first verified CLI tools with capability I/O; modules and offline packages included; not a general OS kernel language.”

---

## 18. First 12 milestones (near-term execution order)

Executable queue under current law (design before code each time):

| Step | Milestone | Type | Output |
| --- | --- | --- | --- |
| 1 | M11 design package | Design | DESIGN-M11, ADR-015, matrix |
| 2 | M11 implement modules | Impl | Language + seed proof |
| 3 | M12 project linked build | Impl | `project build` |
| 4 | M14 fine-grained edits design+impl | Design→Impl | Authoring vN |
| 5 | M15 diagnostic parity program | Impl | Parity report |
| 6 | M16 LSP design+impl | Design→Impl | Offline LSP |
| 7 | Threat model v2 | Design | Host I/O unblocked |
| 8 | M18 host I/O pilot | Impl | Granted I/O |
| 9 | M21 test runner | Impl | `aether test` |
| 10 | M19/M25 stdlib L0 + package path | Impl | Reuse story |
| 11 | M23 multi-package offline | Impl | Graphs |
| 12 | Epoch E1–E3 gate review | Audit | Maturity scorecard refresh |

---

## 19. Long-range milestone map (M11–M40 summary table)

| ID | Title | Epoch | Depends | Effort |
| --- | --- | --- | --- | --- |
| M11 | Language modules | E1 | M10 | L–XL |
| M12 | Linked project build | E1 | M11 | M |
| M13 | Module authoring schema | E1 | M11 | M |
| M14 | Fine-grained structural edits | E2 | M3/M11 | L |
| M15 | Diagnostic parity program | E2 | Seed | L |
| M16 | Bounded LSP | E2 | M14–15 | L–XL |
| M17 | Project intelligence CLI | E2 | M12 | M |
| M18 | Host I/O capabilities | E3 | Threat v2 | L–XL |
| M19 | Stdlib L0 | E3 | M11 | L |
| M20 | Error ergonomics expansion | E3 | M4 | L |
| M21 | Test runner | E3 | M12 | M–L |
| M22 | Narrow FFI | E4 | M18 | XL |
| M23 | Multi-package offline | E4 | M11 | L |
| M24 | Registry (optional fork) | E4 | Law | XL |
| M25 | Local package publish | E4 | M23 | M |
| M26 | Generics/interfaces | E5 | M11 | XL |
| M27 | ADTs / richer data | E5 | M11 | L–XL |
| M28 | Comptime expansion | E5 | M5 | L–XL |
| M29 | Resource×effect | E5 | M2+M4 | L–XL |
| M30 | Concurrency depth | E5 | M7 | XL |
| M31 | Layout performance program | E5 | M6 | L |
| M32 | VM performance | E6 | Stable IR | L–XL |
| M33 | JIT | E6 | M32 | XL |
| M34 | Embeddable runtime distro | E6 | M18 | M |
| M35 | Native backend (fork) | E6 | Law | XXL |
| M36 | Docs site / book | E7 | Ongoing | M–L |
| M37 | RFC + semver process | E7 | Community | M |
| M38 | Compatibility lab | E7 | AETH history | L |
| M39 | Security/fuzz program | E7 | Continuous | L |
| M40 | 1.0 RC program | E7 | E1–E4+ | XL |

---

## 20. Stdlib & “batteries” detailed outline

### 20.1 L0 — always capability-free

- Integer/text/bytes helpers  
- Option/Result-like patterns (language-dependent)  
- Test assertions  

### 20.2 L1 — pure data

- Vec/Map if generics exist; otherwise fixed modules  
- Sorting, searching  
- JSON subset encode/decode (no network)  

### 20.3 L2 — capability data plane

- `fs` with grant  
- `env` with grant  
- `clock` with grant (determinism policy for tests)  

### 20.4 L3 — network (separate threat epoch)

- TCP/HTTP only after dedicated ADR  
- Default deny; explicit net capability  

### 20.5 Quality bar per stdlib module

- Spec section  
- Property/tests  
- Capability table  
- No hidden global allocator beyond language model  
- Seed-compilable examples  

---

## 21. Tooling suite end-state (`aether` CLI vision)

| Command family | Purpose | Epoch |
| --- | --- | --- |
| `check` / `structure` / `apply-edit` / `format` | Authoring | Now→E2 |
| `compile` / `run` / `forge` | Core | Now |
| `project verify|format|build|test` | Workspace | E1–E3 |
| `test` | Runner | E3 |
| `pkg` / `lock` / `vendor` | Packages | E4 |
| `lsp` | Editor server | E2 |
| `bench` | Perf harness | E6 |
| `doc` | Generate docs | E7 |
| `registry` | Optional | E4 fork |

---

## 22. AI-first excellence program (north-star specific)

Mainstream languages are weak here; Aether can **lead** if disciplined.

| Program | Work | Metric |
| --- | --- | --- |
| Structural corpus | 1000+ edit cases | Accept/reject deterministic |
| Agent playbooks | Documented tool loops | Human review time ↓ |
| Provenance | Edit → source → AETH hash chain | Auditable builds |
| No model authority | Forever | Threat model invariant |
| Semantic queries | “show resources crossing boundary” | Static queries tests |

**Success:** AI agents fail closed more often than they corrupt programs — measurable.

---

## 23. Security roadmap (parallel to features)

| Phase | Focus |
| --- | --- |
| Now | Verifier negatives, project path jail, pure host fail-closed |
| E3 | Grant model, path roots, audit events optional |
| E4 | FFI ownership, signed packages if registry |
| E6 | JIT spray/hardening, runtime isolation options |
| Continuous | Fuzz parsers, schemas, AETH decoder |

---

## 24. Resourcing scenarios

### 24.1 Solo + agents (Constitution-bound)

- One epoch focus; expect **years** to E7  
- Prefer E1→E2→E3 before deep E5  

### 24.2 Small team (3–5)

- L-Lang + L-Tool parallel after M11 design freeze  
- E1–E4 in **multi-year** but plausible  

### 24.3 Funded product team

- Add L-Host security engineer early (E3)  
- E7 1.0 with stdlib L2 and packages offline  

**This document does not staff the project**; it only sizes the mountain.

---

## 25. Decision log template (for human overrides)

When reordering or forking law, append to ROADMAP or a short ADR:

```text
Date:
Override:
Rationale:
Tracks deferred:
Law forks touched (F-NATIVE/F-REGISTRY/none):
Accepted by:
```

---

## 26. Immediate next actions (this week / next sprint)

1. Mainstream maturity roadmap committed.  
2. **M11–M13 complete** (modules, edits, LSP through 0.18).  
3. **M14 host I/O implemented** (package 0.19; threat model v2 + ADR-018 grants).  
4. Human chooses whether **F-NATIVE** or **F-REGISTRY** will ever be on the table (can wait).

---

## 27. One-page scorecard: journey summary

| Now | After E1–E3 | After E4–E7 (niche mainstream) |
| --- | --- | --- |
| Pilot language | Real multi-file + editor + I/O CLIs | Packages, depth, perf story, 1.0 governance |
| Strong verification | Still strong | Still strong (differentiation) |
| Weak ecosystem | Growing stdlib | Credible offline ecosystem |
| No modules | Modules | Generics + stdlib |
| Pure host only | Capability I/O | Optional FFI/registry per law |

---

## 28. Document control

| Field | Value |
| --- | --- |
| Location | `docs/ROADMAP-MAINSTREAM-MATURITY.md` |
| Related | `ROADMAP.md` (near-term milestones), ADR-014 (portfolio), NORTH_STAR, CORE_CLAIMS |
| Update trigger | Epoch exit, law fork, or major track reorder |
| Honesty rule | Under-claim; never mark mainstream-complete without §12.7 checklist |

---

*End of mainstream maturity roadmap. AGENTS Constitution remains law: every
step is design-gated, evidence-backed, and capability-honest. Reaching Rust /
C++ / Go / Java / Python / TypeScript **class** is a multi-epoch program —
Aether wins by being the best **verified, local-first, AI-structured systems
language**, not by cloning every feature of every peer.*
