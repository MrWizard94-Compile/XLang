# Aether / XLang — Comprehensive Project Progress Report

**Date:** 2026-08-11  
**HEAD:** `12a2181` on `codex/xlang-local-first-studio`  
**Package contract:** **0.36.0** (language **0.11** + M19e AETH v12 + post-0.36 maturity program)  
**Audit:** [AUDIT_REPORT-2026-08-11-FULL-PROJECT.md](AUDIT_REPORT-2026-08-11-FULL-PROJECT.md) — **GREEN**  
**Constitution:** AGENTS Constitution 5.0.1  

---

## 1. One-sentence status

Aether is a **local-first, seed-hosted product compiler** with verified AETH
execution, dual-compare self-host proofs, and human-authorized **F-NATIVE** /
**F-REGISTRY** pilots — still **0.36.0** package contract, with BARP having moved
product authority off the Rust bootstrap for default toolchain paths.

---

## 2. What the product is today

### 2.1 Language and runtime

| Layer | Status |
| --- | --- |
| Core language surface | Canonical **0.11** (M2 resources, M4 Error[Whole], M5/M15/M23 comptime, M6 layout, M7 nurseries, M8 pure host) |
| Toolchain packages | M9–M18 projects/modules/LSP/tests/workspaces/stdlib; M19a–e resource/task; M21 FFI pilot; M22 cross-package; M23 seed-native |
| AETH | **v11** default; **v12** for M19e `task weave` + `checkpoint` |
| Default compile | **Seed forge** (`seed/aether_seed.aeth`) — not Rust bootstrap |
| Execution | Verify-before-run VM; grant-empty pure fixtures; optional `--grant-*` / `--grant-lib` |

### 2.2 Product toolchain (BARP)

Bootstrap Authority Reduction Program ([DESIGN-BARP-001](DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md), ADR-043–101):

| Capability | Authority |
| --- | --- |
| `compile` / seed rebuild | Product seed (ADR-067) |
| `check` / `format` / `structure` / project format | Product default; `--bootstrap` recovery |
| LSP diagnostics / symbols / hover / definition / format | Product-primary |
| Structural edits | Product weave/body/record paths |
| Multi-module / multi-source | Host elaborate + seed emit + unit digests |
| Diagnostics | `AE-SEED-*` preflights + SPEAK packets; seed SPEAK pilot codes |
| Bootstrap residual | Dual-compare oracle, recovery flags, full `aether.ast/v8` |

### 2.3 Law forks (human-authorized)

| Fork | Through | Highlights |
| --- | --- | --- |
| **F-NATIVE** | **M35j** (ADR-059–099) | AETH→C / object / LLVM IR / LLVM object / exe; host dual-run; probe + hermetic env; cross-target matrix (`AE-NATIVE-007`) |
| **F-REGISTRY** | **M24i** (ADR-060–100) | Offline pin/verify; HMAC/Ed25519; explicit fetch; rotation; multi-root policy; multi-level key certs; X.509-lite + CA store (`AE-REG-012/013`) |

### 2.4 Task model

| Item | Status |
| --- | --- |
| M19e active-frame cancel | **Proven** (v12, checkpoints) |
| Task frame surface / checkpoint density / inventory | **Proven** tooling (ADR-085/093/097) |
| Reserved future surface | Fail-closed **AE-SEED-014** (ADR-089) |
| Task weave requires checkpoint | Fail-closed **AE-SEED-015** (ADR-101) |
| Handles / timeouts / parallel runtime | **Not implemented** (ADR-081 design only) |

---

## 3. Architecture snapshot

```
Aether source (.ae)
    │
    ├─ product path (default)
    │     host preflights (AE-SEED-*)
    │     multi-source? → host elaborate
    │     forge seed/aether_seed.aeth  → AETH bytes
    │     seed SPEAK merge on failure (pilot codes)
    │     verify_bytecode
    │
    ├─ recovery path (--bootstrap)
    │     Rust parser/validate/emit (oracle + AST)
    │
    └─ optional lowers (F-NATIVE, verified AETH only)
          → C / object / LLVM / native exe (host tools)

Verified AETH
    ├─ aether run  (VM; pure or grants)
    ├─ aether forge (compiler ABI only)
    └─ dual-compare tests (product ≡ bootstrap where claimed)
```

---

## 4. Evidence ladder (how we know it works)

| Evidence class | Mechanism |
| --- | --- |
| Unit / integration tests | `cargo test --workspace` |
| Seed self-host | `seed_self_host` dual-compare + multi-generation |
| Example corpus | Gate dual-compare of shipped examples |
| Quality gate | `tools/aether-gate.ps1` (quick / full / release) |
| Claim control | [CORE_CLAIMS.md](CORE_CLAIMS.md) status rules |
| ADR trail | 100+ ADRs under `docs/ADR-*.md` |
| Matrices | M* / M19E / M23 / M35A / M24A validation matrices |
| Constitution | Project Level-4 pointer → pack AGENTS.md Section 0 |

**Latest full-gate stamp:** 2026-08-11 HEAD `12a2181` — **GATE PASS mode=full**  
**Seed identity:** `3B85696292FD5287F8778E708D095131D55094B04AB402209F8E77767B28E1A4`

---

## 5. Recent maturity program (post–0.36 release)

The 0.36 package contract (M19e) is the executable language baseline. Since then,
work has been **independence and infrastructure maturity** without a package bump:

### 5.1 BARP highlights (ADR-043 → 098/101)

- Product-default CLI toolchain; bootstrap recovery only  
- Multi-source envelope + multi-file host forge + unit digests  
- SPEAK protocol `AETHER_SEED_ERROR:` + host packet ABI  
- Seed SPEAK multi-code pilot: **AE-SEED-004/005/006/012** (dual-compare rebuild)  
- Forge SPEAK capture on failure and verify merge  
- Yield-in-truth-choose fail-closed (AE-SEED-013)  
- Task checkpoint required (AE-SEED-015)  

### 5.2 F-NATIVE (M35a → M35j)

C pilot → locals → SPEAK/multi-weave → host-cc dual-exec → object → LLVM IR →
LLVM object → native exe → toolchain probe/hermetic → **cross-compile target matrix**

### 5.3 F-REGISTRY (M24a → M24i)

Offline pin → HMAC signed → Ed25519/HTTPS → rotation → multi-root policy →
root certs → multi-level chains → X.509-lite → **CA store + chain verify**

### 5.4 Recent tip commits (illustrative)

| Commit | Slice |
| --- | --- |
| `12a2181` | ADR-098–101 multi-code SPEAK, targets, CA store, checkpoint |
| `1e2fa31` | ADR-094–097 SPEAK empty pilot, native probe, X.509-lite, task inventory |
| `b5305cf` | ADR-090–093 forge SPEAK capture, native exe, multi-level certs, checkpoints |
| `f03e310` | ADR-086–089 SPEAK matrix, LLVM object, root certs, task reserve |

---

## 6. Maturity scorecard

| Domain | Maturity | Notes |
| --- | --- | --- |
| Seed self-host | **High** | Byte-identical multi-generation |
| Product vs bootstrap independence | **High** | Default product; residual oracle documented |
| Diagnostics honesty | **Medium–High** | Host strong; seed SPEAK pilot expanding |
| Multi-module | **Medium** | Host elaborate works; not seed-native |
| Native lower | **Medium** | Useful pilot; host-dependent |
| Registry | **Medium** | Offline + signed + lite CA; not full PKI |
| Task concurrency | **Medium** | M19e solid; no handles/timeouts/parallel |
| Foreign ABI | **Low–Medium** | Bounded pilot only |
| Package versioning | **Stable** | Still 0.36.0 contract |

---

## 7. Explicit non-goals (current law)

- Ambient guest network / shell / model calls  
- OS-thread parallel runtime as product claim  
- Task handles / timeouts as implemented features  
- Full RFC 5280 X.509 DER  
- Bundled hermetic cross-compile sysroot  
- Claiming seed SPEAK complete for every AE-SEED code  
- Claiming seed-native multi-file forge  

---

## 8. Recommended next increments

Ordered for dependency honesty (`CONST-DEP-001`):

1. **BARP:** expand seed SPEAK to remaining conformance codes; design seed-native multi-file forge ABI  
2. **F-NATIVE M35k+:** optional bundled/hermetic tool path when operators need reproducibility  
3. **F-REGISTRY:** RFC 5280-shaped DER only if human re-authorizes beyond X.509-lite  
4. **Task runtime:** implementable ADRs for handles and/or timeouts under ADR-081 invariants  
5. **Optional package 0.37** when language surface or AETH version actually expands  

---

## 9. How to re-verify

```powershell
# Constitution pack
pwsh -File "..\..\AGENTS Constitution\tools\verify-pack.ps1"

# Full project gate
pwsh -File tools\aether-gate.ps1 -Mode full
# expected: GATE PASS mode=full

# Optional packaging gate
pwsh -File tools\aether-gate.ps1 -Mode release
```

---

## 10. Document map

| Doc | Role |
| --- | --- |
| [MANIFEST.md](../MANIFEST.md) | Executable product contract |
| [CORE_CLAIMS.md](CORE_CLAIMS.md) | Proven vs residual claims |
| [ROADMAP.md](ROADMAP.md) | Track order + human backlog |
| [SEED_PROFILE.md](SEED_PROFILE.md) | Seed emission honesty |
| [FORGE_CONTRACT.md](FORGE_CONTRACT.md) | Host forge ABI |
| [DESIGN-BARP-001](DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md) | BARP program |
| [AUDIT_REPORT-2026-08-11-FULL-PROJECT.md](AUDIT_REPORT-2026-08-11-FULL-PROJECT.md) | This audit stamp |
| ADRs 043–101 | Maturity decision trail |

---

*End of PROGRESS_REPORT-FULL-PROJECT.md*
