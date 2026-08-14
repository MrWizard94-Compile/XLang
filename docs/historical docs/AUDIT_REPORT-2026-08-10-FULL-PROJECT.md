# Full project audit — 2026-08-10

**Branch:** `codex/xlang-local-first-studio`  
**HEAD:** `dd61bdec74f06d36ec285c0433e429c1eccb932b`  
**Package:** aether-core / aether-cli **0.36.0**  
**Pack:** AGENTS Constitution 5.0.1 — `verify-pack.ps1` **PASS** (`GOV-INT-001`)  
**Rule IDs:** `CONST-GATE-001`, `CONST-DONE-001`, `ENG-WARN-001`, `DOC-SYNC-001`,
`TEST-BEHAVIOR-001`, `SEC-INPUT-001`, `RND-INVAR-001`, `REL-PACKAGE-001`

## Scope

Full local quality + release packaging audit of the Aether product workspace.
Claim honesty vs Level-4 `AGENTS.md`, `MANIFEST.md`, `CORE_CLAIMS.md`, BARP
through ADR-069, F-NATIVE M35a/b, F-REGISTRY M24a. No public publish. No network
fetch.

## Gate results

| Gate | Result |
| --- | --- |
| Pack verify (`GOV-INT-001`) | **PASS** |
| `cargo fmt --check` | **PASS** |
| `cargo clippy --workspace --all-targets -D warnings` | **PASS** |
| `cargo test --workspace` (incl. seed_self_host) | **PASS** (113 lib + 27 seed + 23 CLI + matrices) |
| Example dual-compare seed≡bootstrap (32 top-level) | **PASS** |
| host-pilot run exit 48 | **PASS** |
| project verify / multi / modules build | **PASS** |
| Seed identity bootstrap≡product≡forged≡checked-in | **PASS** SHA-256 `A654F7FEF7DEC5E2B75F146CCCD491321AD8702FE6BA25947C0D409D7C686AE5` |
| Release build + package-preview | **PASS** `dist/aether-0.36.0-tp` (339 files + SHA-256SUMS) |
| Consumer preview verify | **PASS** |
| Unlisted package file reject | **PASS** |
| **`aether-gate -Mode release`** | **GATE PASS mode=release** |

### Release package pins (this run)

| Artifact | SHA-256 |
| --- | --- |
| Checked-in `seed/aether_seed.aeth` | `A654F7FEF7DEC5E2B75F146CCCD491321AD8702FE6BA25947C0D409D7C686AE5` |
| Packaged `aether.exe` | `1121616B701A419117D71FF6B26F05A9EDAED6EF929BF7769B6A08E6BB5C5F32` |

## Findings

### Defects (stop-ship)

**None.** Gate green; no fmt/clippy/test/package failures.

### DOC-SYNC performed after green gate

| ID | Severity | Finding | Fix |
| --- | --- | --- | --- |
| D1 | Medium | `PROGRESS_REPORT-FULL-PROJECT.md` snapshot dated 2026-08-08; still described bootstrap as seed-rebuild authority | Full rewrite to HEAD `dd61bde` / BARP ADR-064–069 |
| D2 | Low | MANIFEST bootstrap residual still said “outside product weave-replace” | Updated to nested body-list residual + product-default toolchain intro |
| D3 | Low | ROADMAP release stamp still 2026-08-08 only | Re-stamp release gate at 2026-08-10 / `dd61bde` |
| D4 | Info | No dated full-project audit for post-BARP maturity | This file |

### Accepted residual (honest, not stop-ship)

| ID | Severity | Finding | Disposition |
| --- | --- | --- | --- |
| R1 | Product | Nested choose/while statement paths still bootstrap AST | Documented ADR-069 residual; product owns weave-body + top-level weaves/records |
| R2 | Product | Full `aether.ast/v8` structure requires `--bootstrap` | Product structure is envelope only (ADR-054/064) |
| R3 | Product | Dual-compare still needs bootstrap emit for proof | Required by BARP law; never drop |
| R4 | Product | Seed-internal error packets / multi-file forge not implemented | ADR-061 direction; trackers false |
| R5 | Product | F-NATIVE is pure Whole pilot only; F-REGISTRY offline only | Authorized pilots; network/LLVM not claimed |
| R6 | Process | Package still `UNLICENSED` local technical preview | No public release claim |
| R7 | Historical | Root `AUDIT_REPORT.md` is migration history | Pointer to current MANIFEST/CORE_CLAIMS/progress report |

### Not defects

- Product-default toolchain (check/format/structure/LSP) seed path.  
- Product seed rebuild without `--bootstrap` (ADR-067) with dual-compare identity.  
- Zero ambient guest I/O without grants.  
- Workspace path jail / unlisted package reject.  
- Dual-compare corpus green.  

## BARP tracker honesty (spot-check)

| Tracker | Expected |
| --- | --- |
| `product_default_cli_toolchain` | true |
| `bootstrap_is_recovery_oracle_only` | true |
| `product_seed_rebuild_without_bootstrap` | true |
| `structural_edit_product_statement_and_record_ops` | true |
| `lsp_product_surface_hover_definition` | true |
| `seed_native_multi_module_elaboration` | **false** |
| `seed_internal_error_packets` | **false** |

## Section 0 self-audit (this audit deliverable)

| Item | Score |
| --- | --- |
| Complete / no stubs | PASS |
| Dependency-first | PASS (audit after green gate) |
| Zero warnings | PASS |
| Tests / gate | PASS release |
| DOC-SYNC | PASS (this delivery) |
| Secrets / untrusted input | N/A (docs+audit) |
| IP / multi-agent | N/A |
| Honest residual claims | PASS |

## Commands

```powershell
pwsh -NoProfile -File tools/aether-gate.ps1 -Mode release
# Expected: GATE PASS mode=release
```

## Verdict

**GREEN.** Safe to DOC-SYNC and refresh the comprehensive progress report.
Product remains a **local technical-preview pilot**, not a public 1.0.

---

*End of AUDIT_REPORT-2026-08-10-FULL-PROJECT.md.*
