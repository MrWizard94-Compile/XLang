# Constitution full-project audit — 2026-08-04

**Branch:** `codex/xlang-local-first-studio`  
**HEAD at audit start:** `0533d57` (0.25 M19a release bootstrap)  
**Pack:** AGENTS Constitution 5.0.1 — `verify-pack.ps1` **PASS** (GOV-INT-001)  
**Rule IDs:** `CONST-GATE-001`, `CONST-DONE-001`, `ENG-WARN-001`, `DOC-SYNC-001`, `TEST-BEHAVIOR-001`, `SEC-INPUT-001`

## Scope

Full local quality gate + claim/docs honesty vs Level 4 AGENTS.md + MANIFEST +
CORE_CLAIMS. No network. No release packaging (TP-2 full package not re-run).

## Gates

| Gate | Result |
| --- | --- |
| Pack verify | **PASS** |
| `cargo fmt --check` | **FAIL → fixed** (`cargo fmt --all`) |
| `cargo clippy -p aether-core -p aether-cli -- -D warnings` | **PASS** (after fmt) |
| `cargo test -p aether-core --lib` | **PASS** (78) |
| `cargo test -p aether-cli` | **PASS** (13) |
| Seed compile `welcome.ae` | **PASS** |
| Seed compile `release-raise.ae` | **FAIL (expected)** — seed does not emit RELEASE |
| Bootstrap compile+run `release-raise.ae` | **PASS** exit 9 |
| `aether test examples/tests` | **PASS** 3/3 |
| `workspace verify examples/workspace` | **PASS** |

## Findings

### Fixed this audit

| ID | Severity | Finding | Fix |
| --- | --- | --- | --- |
| A1 | Gate fail | `cargo fmt --check` dirty (ENG-WARN-001 / zero-warning culture) | `cargo fmt --all` |
| A2 | DOC-SYNC | README still advertised **0.12.0** and overstated seed corpus | README updated to **0.25** + MANIFEST/claims pointers + honest seed/`release` note |
| A3 | DOC-SYNC | AGENTS product table omitted M19a ADR-027 / AETHER_0.25; artifact pin omitted RELEASE | AGENTS.md updated |
| A4 | Honesty UX | `examples/release-raise.ae` looked like a normal seed example | Header comment: bootstrap-only until seed dual-compare |

### Accepted residual (documented, not stop-ship if honest)

| ID | Severity | Finding | Disposition |
| --- | --- | --- | --- |
| R1 | Major product gap | Seed does not emit `RELEASE` (66); default `aether compile` fails closed on `release` programs | **Documented** in MANIFEST, AETHER_0.25, CORE_CLAIMS CLM-032; bootstrap path proven. Seed emission attempted in audit; nested indent risk to `aether_seed.ae` — **not force-merged**. Follow-up: seed engineer dual-compare. |
| R2 | Process | Full `aether-gate -Mode full` (seed self-host rebuild pole) not re-run this audit | Day-to-day quick gate + targeted tests green; full gate before release/TP package |
| R3 | Scope | FFI still design-blocked (ADR-025); nursery×resource deferred (ADR-023) | Correct under law |

### Not defects

- No ambient host I/O without grants (M14 tests green).  
- Workspace path jail / cycle tests green.  
- Pack law integrity green.  
- No unsafe / clippy deny regressions after fmt.  

## Definition of Done (this audit)

| Item | Status |
| --- | --- |
| Issues found fixed or honestly residual | Yes |
| Zero-warning + fmt clean | Yes (after fix) |
| Claims not overstated | Yes (README/AGENTS/`release` honesty) |
| Seed dual-compare for M19a | Residual R1 — not falsely claimed Done |

## Commands re-run after fixes

```powershell
pwsh -File "../../AGENTS Constitution/tools/verify-pack.ps1"
cargo fmt --all -- --check
cargo clippy -p aether-core -p aether-cli -- -D warnings
cargo test -p aether-core --lib
cargo test -p aether-cli
```

*End of audit report.*
