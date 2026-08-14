# Full Project Audit — Aether / XLang

**Date:** 2026-08-11  
**HEAD:** `12a2181` (`codex/xlang-local-first-studio`)  
**Package:** Aether **0.36.0** (language surface 0.11 + M19e v12 + BARP/F-NATIVE/F-REGISTRY maturity)  
**Constitution pack:** AGENTS Constitution **5.0.1** (`CONST-GATE-001`, `GOV-INT-001`)  
**Auditor mode:** AGENTS Constitution enforce + gate  

---

## 1. Executive result

| Check | Result |
| --- | --- |
| Working tree clean at audit start | **PASS** (tip `12a2181` matched origin) |
| Pack integrity `verify-pack.ps1` | **PASS** (`GOV-INT-001`) |
| `aether-gate -Mode full` | **PASS** |
| Seed bootstrap ≡ product ≡ forge ≡ checked-in | **PASS** (SHA-256 `3B85696292FD5287F8778E708D095131D55094B04AB402209F8E77767B28E1A4`) |
| Claim honesty (residuals documented) | **PASS** after DOC-SYNC |
| Overall | **GREEN** |

---

## 2. Gate evidence (mode=full)

Command:

```powershell
pwsh -File tools/aether-gate.ps1 -Mode full
```

Observed steps (summary):

1. Pack verify — PASS  
2. `cargo fmt --check` — PASS  
3. `cargo clippy --workspace --all-targets -D warnings` — PASS  
4. `cargo test --workspace` (includes `seed_self_host`) — PASS  
5. Example dual-compare corpus (32 top-level examples) — PASS  
6. Host-pilot compile+run exit 48 — PASS  
7. Project verify/format/build examples — PASS  
8. Seed bootstrap + product + forge hash identity — PASS  

Log: `target/audit-gate-full.log` (local; not committed).

---

## 3. Section 0 self-audit (CONST-GATE-001)

| # | Item | Score | Notes |
| --- | --- | --- | --- |
| 1 | Spec / contract present | PASS | MANIFEST, AETHER_0.36, CORE_CLAIMS |
| 2 | Implementation complete for claimed surface | PASS | Trackers + ADRs through 101 |
| 3 | Dependency-first | PASS | BARP before residual bootstrap claims |
| 4 | Tests against behavior | PASS | Full gate + dual-compare |
| 5 | Zero warnings | PASS | Clippy deny-all |
| 6 | Docs match code | PASS | DOC-SYNC this delivery |
| 7 | Security input hygiene | PASS | Fail-closed AE-SEED/AE-NATIVE/AE-REG codes |
| 8 | Secrets hygiene | PASS | No secrets in tree |
| 9 | Verify before run/write | PASS | Verifier + forge contract |
| 10 | No ambient guest capability | PASS | Grant-empty pure fixtures |
| 11 | Honest self-host / dual-compare | PASS | Seed hash identity |
| 12 | Residual risk recorded | PASS | Honesty trackers `false` |
| 13 | Multi-agent N/A | N/A | Single-auditor delivery |
| 14 | IP / invention N/A | N/A | No new invention disclosure package |
| 15 | Delivery packaging | PASS | This report + progress report |

---

## 4. Honesty trackers still `false`

| Tracker | Meaning |
| --- | --- |
| `seed_native_multi_module_elaboration()` | Multi-file is host elaborate + seed emit |
| `seed_internal_error_packets()` | Full seed-binary packet matrix not claimed |
| `seed_speak_emit_conformance_complete()` | SPEAK pilot codes only (004/005/006/012) |
| `product_path_requires_bootstrap_dual_compare()` | Product path does not require bootstrap gate |
| `product_multi_module_invokes_bootstrap()` | Multi-module product does not invoke bootstrap |
| `compile_with_seed_invokes_bootstrap()` | Product seed compile does not invoke bootstrap |

---

## 5. Residual risk / non-claims

1. **Seed SPEAK full matrix** — not complete; host preflight still primary for many codes.  
2. **Seed-native multi-file forge ABI** — not implemented.  
3. **F-NATIVE** — host toolchain required; cross-compile best-effort; no bundled sysroot.  
4. **F-REGISTRY** — X.509-lite only; not RFC 5280 DER; fetch is explicit CLI only.  
5. **Task model** — checkpoint required (AE-SEED-015); **no** handles/timeouts/parallel runtime.  
6. **M21 FFI** — Whole-only pilot; native code not sandboxed (human residual acceptance).  
7. **Package version** remains **0.36.0**; BARP/F-NATIVE/F-REGISTRY are maturity on that contract, not a version bump.

---

## 6. DOC-SYNC performed after green gate

| Document | Sync |
| --- | --- |
| README.md | Default path honesty; F-NATIVE/F-REGISTRY bounded claims; audit links |
| AGENTS.md | BARP through ADR-101; F-NATIVE M35j; F-REGISTRY M24i; audit links |
| CORE_CLAIMS.md | Date tip; CLM-004/040 refine; **CLM-041/042** law-fork claims |
| DESIGN-BARP-001 | Seed SPEAK multi-code residual language |
| This audit + PROGRESS_REPORT-FULL-PROJECT | New/updated |

---

## 7. Residual doubt

None material for green ship of the **audit + doc-sync** package. Optional future
`aether-gate -Mode release` packaging re-stamp was not re-run in this audit
(last recorded release-mode stamp: 2026-08-10 at historical HEAD; full mode is green at tip).

---

*End of AUDIT_REPORT-2026-08-11-FULL-PROJECT.md*
