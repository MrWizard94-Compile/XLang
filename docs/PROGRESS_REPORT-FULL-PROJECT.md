# Aether / XLang — Comprehensive Project Progress Report

```text
Document: Comprehensive Project Progress Report
Status: Current, evidence-led Level 4 product record
Date: 2026-08-10
Repository: XLang (Aether product workspace)
Source snapshot HEAD: dd61bdec74f06d36ec285c0433e429c1eccb932b
Branch: codex/xlang-local-first-studio (pushed, tracking origin)
Current product package: aether-core / aether-cli 0.36.0
Language contract: Aether 0.11 plus bounded M11–M23 / M19a–M19e surfaces
Artifact contract: AETH v4–v12 input; v11 or v12 current output
Authoring contract: aether.ast/v8, aether.edit/v8, aether.diagnostic/v8
Preview channel: local folder only, UNLICENSED; not a public release
Authority: AGENTS Constitution, MANIFEST.md, CORE_CLAIMS.md, ADRs, matrices
Related rules: CONST-GATE-001, CONST-DONE-001, CONST-COMPLETE-001,
  ENG-WARN-001, TEST-BEHAVIOR-001, DOC-SYNC-001, SEC-INPUT-001,
  RND-INVAR-001, REL-PACKAGE-001, REV-PACK-001
Live audit: docs/AUDIT_REPORT-2026-08-10-FULL-PROJECT.md
  (GATE PASS mode=release)
Supersedes: 2026-08-08 snapshot of this report (pre BARP ADR-064–069)
```

---

## Executive position

**Aether is a real, locally usable compiler-and-VM toolchain pilot—not a
general-purpose 1.0 language and not a replacement claim for Rust, C++, Go,
Java, Python, TypeScript, or any other language.** It parses Aether source,
emits Aether-owned **AETH** bytecode, verifies the bytecode, and executes it in
the Aether VM. Guest **source** is not transpiled to Rust, C, JavaScript, or
LLVM. Optional **verified AETH → ISO C** is an authorized F-NATIVE pilot only
(M35a/b pure Whole subset).

Package **0.36.0** combines:

1. **Seed-hosted product compilation** as the default CLI path.  
2. **BARP** (Bootstrap Authority Reduction Program, ADR-043–069): product owns
   default check/format/structure, LSP diagnostics/symbols/hover/definition,
   seed rebuild, and nearly all structural-edit ops on product units.  
3. **Rust bootstrap as recovery/oracle only** (`--bootstrap`, dual-compare,
   nested body-list AST residual, full `aether.ast/v8`).  
4. Bounded language surface through M19e/M21/M23; offline projects, workspaces,
   tests, stdlib, LSP; local technical-preview packaging.  
5. Authorized law-fork pilots: **F-NATIVE** (AETH→C), **F-REGISTRY** (offline
   digest cache only).  

The project is deliberately strongest where it differentiates:

| Strength | Evidence posture |
| --- | --- |
| Verifier-first AETH | Verify before run and before write |
| Explicit authority | No ambient guest FS/process/shell/network/model |
| Evidence-led self-host | Seed≡bootstrap dual-compare on claimed corpora; product seed rebuild identity |
| AI-oriented authoring without model authority | Versioned AST/edit/diagnostic protocol; product accept gates |
| Honest residual claims | Nested body AST, seed packets, multi-file forge, network registry still open |

It remains early relative to mature ecosystems: no general generics, public
registry, broad FFI, parallel runtime, large stdlib, public license channel, or
consumer IDE product beyond bounded offline LSP.

### One-page truth table (2026-08-10)

| Surface | Current truth | Important boundary |
| --- | --- | --- |
| Aether source → AETH → verify → VM | **Implemented** | Aether-only VM default; no source transpile |
| Product default compile / check / format / structure | **Proven (ADR-064)** | Recovery via `--bootstrap` |
| Product seed rebuild (no `--bootstrap`) | **Proven (ADR-067)** | Dual-compare still uses bootstrap emit |
| Structural top-level weave + weave-body stmt + primitive record | **Proven product path (ADR-065/068/069)** | Nested choose/while body lists residual bootstrap |
| LSP diagnostics / symbols / hover / definition | **Product-primary (ADR-058/063/066)** | Not a second product AETH emitter |
| Multi-module / workspace | **Host elaborate + seed emit (ADR-056)** | Seed-native multi-file = false |
| M23 pure comptime calls | **Seed-native D2a (ADR-043 Phase 1)** | No nested calls / control-flow callees |
| M19e task-frame cancel | **AETH v12 implemented** | Checkpoint-only; no handles/timeouts/parallelism |
| M21 foreign weave | **Whole-only pilot** | Native lib not sandboxed |
| F-NATIVE M35a/b | **Verified AETH→C pure Whole pilot** | Not default; SPEAK/Text multi-weave later |
| F-REGISTRY M24a | **Offline pin/verify cache** | No network fetch |
| Technical-preview package | **Local release-gate verified** | `UNLICENSED`; not a public release |
| “Better than all languages” | **Prohibited** | Scoped claims only with evidence |

---

## 1. Snapshot control and authority

### 1.1 Snapshot basis

| Field | Value |
| --- | --- |
| HEAD | `dd61bde` (`feat(barp): ADR-069 product weave-body statements and primitive records`) |
| Branch | `codex/xlang-local-first-studio` @ origin |
| Gate | `aether-gate -Mode release` → **GATE PASS** (2026-08-10) |
| Seed SHA-256 | `A654F7FEF7DEC5E2B75F146CCCD491321AD8702FE6BA25947C0D409D7C686AE5` |
| Packaged `aether.exe` SHA-256 | `1121616B701A419117D71FF6B26F05A9EDAED6EF929BF7769B6A08E6BB5C5F32` |
| Preview root | `dist/aether-0.36.0-tp/` (generated; 339 files + SHA-256SUMS) |

A pushed branch and a local package are **not** a public binary release.

### 1.2 Source-of-truth hierarchy

| Priority | Artifact |
| --- | --- |
| 1 | AGENTS Constitution (quality/safety process) |
| 2 | [MANIFEST.md](../MANIFEST.md) |
| 3 | [AETHER_0.36.md](AETHER_0.36.md) (+ prior package contracts) |
| 4 | [CORE_CLAIMS.md](CORE_CLAIMS.md) |
| 5 | ADRs, validation matrices, delivery reports |
| 6 | Historical docs / roadmaps (context only) |

### 1.3 Product identity

| Field | Current value |
| --- | --- |
| Product name | Aether |
| Core | `aether-core` 0.36.0 at `crates/xlang-core` |
| CLI | `aether` via `aether-cli` 0.36.0 |
| FFI pilot lib | `aether-ffi-pilot` 0.36.0 (`publish = false`) |
| Rust | edition 2021, `rust-version = "1.88"` |
| Product compile authority | Seed forge (`compile_product_bytecode`) |
| Bootstrap authority | Recovery/oracle only (BARP ADR-064–069) |
| License | `UNLICENSED` |

---

## 2. Vision actually implemented

Local-first, verifier-first, AI-primary **systems-language toolchain**:

- Explicit ownership and closed effects without ambient allocation or hidden
  exceptions.  
- Capability host I/O and foreign load only under operator grants.  
- Structured authoring data (`aether.ast/edit/diagnostic` v8) without giving a
  model compile or write authority.  
- Offline projects, workspaces, locks, tests, and bounded LSP.  

North star: [NORTH_STAR.md](NORTH_STAR.md). Executable boundary: MANIFEST.

---

## 3. Language and runtime capability map

### 3.1 Core language (0.11 base + package increments)

| Area | Status | Boundary |
| --- | --- | --- |
| World, weaves, records, shallow expressions | Implemented | Canonical grammar; no legacy C/Rust syntax |
| Ownership modes own/borrow/access/move | Implemented | Explicit; fail closed |
| M2 arenas/buffers | Implemented | Bounded capacities |
| M4 `Error[Whole]` | Implemented | Abortive; no hidden exceptions |
| M5/M15/M23 comptime | Implemented | Root-only; M23 seed-native D2a body subset |
| M6 dual-layout tables | Implemented | Whole fields; no auto rewrite |
| M7 nurseries | Implemented | Cooperative source-order; not OS threads |
| M8 pure host weaves | Implemented | Fixture services; no ambient I/O |
| M14 grant-backed I/O | Implemented | Explicit `--grant-*` roots/names |
| M16 resource + handle | Implemented | Terminal handle over resource-free errors |
| M19a release | Implemented | Explicit `release` |
| M19b/d nursery×resource / spawn arenas | Implemented | Policy A+; dual-compare where claimed |
| M19c Policy B (v11 cooperative) | Implemented | Unstarted cancel; mid-frame not claimed on v11 |
| M19e task frames (v12) | Implemented | Checkpoint cancel only |
| M21 foreign weave | Implemented pilot | Whole-only; residual native risk |
| M23 pure comptime calls | Implemented | Seed-native D2a |

### 3.2 Artifact / VM

- Magic AETH; versions v4–v12 as compatibility inputs where claimed.  
- Default product emit v11; v12 when task frames present.  
- Verify-before-run and verify-before-write are project invariants.  
- RTP-001 ASCII Text fast path (0.34): representation only; no AETH change.

---

## 4. Compiler architecture and BARP maturity

### 4.1 Two engines, one product default

```text
User source
    │
    ├─ Product path (DEFAULT) ── seed forge ── verify ── AETH
    │     compile / check / format / structure / LSP / seed rebuild
    │     structural product ops (ADR-065/068/069)
    │
    └─ Bootstrap path (RECOVERY/ORACLE)
          --bootstrap check/format/structure/compile
          dual-compare proofs
          nested body-list structural AST residual
          full aether.ast/v8
```

### 4.2 BARP ledger (ADR-043 → ADR-069)

| ADR | Outcome |
| --- | --- |
| 043–050 | Program + Phase 1 M23 seed-native; forge-first product; preflights AE-SEED |
| 051–054 | Bootstrap-free `compile_with_seed`; product check/format/structure/project format |
| 055–058 | Product diagnostic ABI; multi-module honesty; edit base gate; LSP product diagnostics |
| 059–060 | F-NATIVE M35a; F-REGISTRY M24a authorized pilots |
| 061 | Direction only: seed error packets + multi-file forge (**not implemented**) |
| 062–064 | M35b locals; product default CLI toolchain |
| 065–067 | Product weave replace; LSP surface hover/def; product seed rebuild |
| 068–069 | Product top-level weave insert/delete; weave-body statements; primitive records |

### 4.3 Residual Rust bootstrap (honest)

1. Dual-compare oracle emit (`compile --bootstrap` in tests/gate).  
2. Recovery flags: `check|format|structure|project format --bootstrap`.  
3. Nested structural body lists (choose/while paths).  
4. Full authoring tree `aether.ast/v8` (product structure is envelope).  
5. Bootstrap remains the **proof** authority; product is the **default product**
   authority.

---

## 5. Tooling surface

| Tool | Status | Notes |
| --- | --- | --- |
| `aether compile` | Product default seed | `--bootstrap` oracle; `--native-c` F-NATIVE |
| `aether check` / `format` / `structure` | Product default | Recovery AST with `--bootstrap` |
| `aether apply-edit` | Product accept + product ops subset | Nested residual bootstrap base |
| `aether project` / `workspace` | Offline verify/lock/build/test/format | PKG-001 locks |
| `aether test` / `project test` | Offline discovery + optional grants/reports | Exit 0 = pass |
| `aether lsp` | Offline stdio M13a+b | Product diagnostics/symbols/hover/def |
| `aether registry` | Offline pin-local / verify-cache | No network |
| `aether forge` / `run` | Host forge ABI; grant-backed run | Verify first |

---

## 6. Security and threat posture

| Model | Scope |
| --- | --- |
| TP / 0.36 threat model | Local preview; no network product path |
| v2 capable host | M14 grants; path jail |
| v3 foreign ABI | M21 residual native risk accepted |
| v4 native backend | F-NATIVE AETH→C only (authorized) |
| v5 package registry | F-REGISTRY offline cache (authorized); network later |

Invariants: guest AETH has no ambient FS/process/shell/network/model authority;
CLI writes only caller-selected paths after validation; forge/VM verify first.

---

## 7. Proof and measurement

### 7.1 Dual-compare and seed identity (live)

- 32 top-level examples seed≡bootstrap under gate.  
- Seed self-host tests: multi-weave, M2–M8, M14–M16, M19a/b/e, M21, M23, full
  surface, product seed rebuild identity.  
- Gate seed pin: bootstrap ≡ product ≡ forged ≡ checked-in  
  `A654F7FE…686AE5`.

### 7.2 Performance evidence

| ID | Claim | Boundary |
| --- | --- | --- |
| CLM-037 / RTP-001 | Local self-host median improvement recorded in AETHER_0.34 | One workload/hardware class; not a general language speed claim |

### 7.3 Automated gate

```powershell
pwsh -NoProfile -File tools/aether-gate.ps1 -Mode release
```

Live result 2026-08-10: **GATE PASS mode=release** (fmt, clippy, full tests,
dual-compare, seed identity, package + consumer verify).

---

## 8. Package and distribution

| Item | State |
| --- | --- |
| Local technical-preview package | `dist/aether-0.36.0-tp/` |
| Integrity | Exact SHA-256SUMS; unlisted files rejected |
| License | `UNLICENSED` |
| Public release / installer / signed channel | **Not claimed** |

---

## 9. Maturity assessment

### 9.1 What is mature enough for local technical use

- Compile/run pure Aether programs on the seed path.  
- Offline project/workspace integrity.  
- Grant-backed I/O experiments.  
- Bounded LSP for diagnostics/navigation.  
- Structural editing for product units (top-level + weave-body + records).  
- Local TP packaging for consumers on the same machine class.

### 9.2 What is not mature

| Gap | Why it matters |
| --- | --- |
| Nested product structural body paths | Still bootstrap for choose/while lists |
| Seed-internal structured error packets | Product diagnostics are host AE-SEED classification |
| Seed-native multi-file forge | Host still elaborates modules |
| Broad FFI / memory-safe foreign | Pilot Whole-only; native residual risk |
| Network registry | Explicitly not implemented |
| Parallel / timed / preemptive tasks | Outside M19e |
| Public 1.0 ecosystem | License, packaging channel, stdlib breadth |

### 9.3 Maturity scorecard (qualitative)

| Dimension | Score (1–5) | Comment |
| --- | --- | --- |
| Core compile/run correctness | 4 | Dual-compare + verifier heavy |
| Product independence from bootstrap | 4 | BARP through 069; residual nested AST/oracle |
| Security model honesty | 5 | Explicit grants and residuals |
| Tooling completeness | 3.5 | Strong offline CLI; no full IDE product |
| Ecosystem / packaging | 2 | Local TP only |
| Comparative superiority claims | 0 | Prohibited without scoped evidence |

---

## 10. Risks

| Risk | Mitigation |
| --- | --- |
| Silent seed/bootstrap drift | Dual-compare tests + gate identity |
| Overclaiming self-host / independence | Trackers + residual lists in ADRs/claims |
| Native residual risk (M21/F-NATIVE) | Explicit authorize docs; not default path |
| DOC-SYNC lag after BARP speed | This audit + progress refresh |
| Scope creep to “full language 1.0” | MANIFEST non-goals + CORE_CLAIMS status labels |

---

## 11. Recommended next work (lawful order)

1. **BARP residual:** nested body-list product structural paths; then ADR-061
   seed error packets / multi-file forge direction as vertical slices.  
2. **F-NATIVE M35c+:** SPEAK/Text, multi-weave, dual-run with host `cc` when
   available (new ADR + matrix).  
3. **F-REGISTRY M24b:** signed fetch only with explicit network + threat update.  
4. **Broader task model:** only with new ADR (handles/timeouts/parallelism).  
5. **Public channel / license:** human decision only; not implied by TP package.

---

## 12. Milestone ledger (compressed)

| Era | Outcome |
| --- | --- |
| 0.1–0.11 | Language core through M8 host pilot |
| 0.12–0.18 | Projects, modules, fine edits, LSP |
| 0.19–0.24 | Host I/O, comptime chain, resource+handle, tests, workspaces, stdlib, xpkg |
| 0.25–0.33 | M19a–d, M21, M23 |
| 0.34–0.36 | RTP-001, PKG-001, M19e v12 |
| Post-0.36 BARP (this HEAD) | ADR-043–069 product toolchain independence; F-NATIVE/F-REGISTRY pilots |

---

## 13. How to verify this report

```powershell
# Full release audit
pwsh -NoProfile -File tools/aether-gate.ps1 -Mode release
# Expected: GATE PASS mode=release

# Spot-check seed pin
Get-FileHash -Algorithm SHA256 .\seed\aether_seed.aeth
# Expected: A654F7FEF7DEC5E2B75F146CCCD491321AD8702FE6BA25947C0D409D7C686AE5
```

Companion audit: [AUDIT_REPORT-2026-08-10-FULL-PROJECT.md](AUDIT_REPORT-2026-08-10-FULL-PROJECT.md).

---

## 14. Closing statement

Aether 0.36 at HEAD `dd61bde` is a **credible local technical-preview systems
toolchain** with strong verifier discipline, growing seed independence under
BARP, and carefully bounded capability expansions. It is **not** a mainstream
1.0 language, **not** a public product channel, and **not** free of residual
Rust bootstrap roles—but those residuals are now narrow, documented, and
intentionally held for proof and recovery rather than default product authority.

Progress is real. Claims stay evidence-led.

---

*End of PROGRESS_REPORT-FULL-PROJECT.md (2026-08-10).*
