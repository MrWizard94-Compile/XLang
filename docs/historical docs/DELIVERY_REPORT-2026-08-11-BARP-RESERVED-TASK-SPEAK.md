# Delivery Report — BARP reserved-task seed-SPEAK pilot

**Date:** 2026-08-11
**Status:** Delivered — full gate PASS
**Scope:** ADR-103, bounded seed-side diagnostic authority reduction
**Rule IDs:** `CONST-COMPLETE-001`, `CONST-DEP-001`, `CONST-DONE-001`,
`CONST-GATE-001`, `DOC-SYNC-001`, `ENG-WARN-001`, `RND-INVAR-001`,
`SEC-INPUT-001`, `TEST-BEHAVIOR-001`

---

## Delivered behavior

The checked-in Aether seed compiler now recognizes the canonical lowercase
reserved-task prefixes after leading ASCII-space indentation on a source line:

- `timeout `
- `task handle `
- `handle task `
- `parallel together`
- `together parallel`

The first match emits one structured `aether.seed-error/v1` SPEAK packet with
`code: "AE-SEED-014"` and `origin: "seed-speak"`, then returns blank Bytes
without entering the normal compiler body. Existing empty, tab, legacy-`fn`,
raw-import, missing-world, and missing-main seed-pilot precedence is preserved.
A valid `speak "timeout 1"` Text literal does not match the line-aware scanner.

This delivery does not implement task handles, timeouts, parallel execution,
new AETH instructions, VM behavior, host grants, or new language syntax. It is
a bounded diagnostic pilot; full seed diagnostic parity and seed-native
multi-file elaboration remain residual.

## Delivered package

| Artifact | Purpose |
| --- | --- |
| `seed/aether_seed.ae` | Canonical line-aware `AE-SEED-014` scanner after the established seed pilots |
| `seed/aether_seed.aeth` | Regenerated checked-in product seed artifact |
| `crates/xlang-core/src/lib.rs` | Pilot tracker, direct-forge behavior, priority, and literal-safety regressions |
| `crates/xlang-core/tests/seed_self_host.rs` | Self-host ABI assertion for the ADR-103 pilot |
| `docs/ADR-103-barp-seed-speak-reserved-task-pilot.md` | Decision, scope, alternatives, and honesty boundary |
| `docs/BARP-VALIDATION-MATRIX.md` | Five-prefix behavior, literal negative, and precedence evidence; duplicate prior matrix row repaired |
| `docs/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md` | BARP authority map through ADR-103 |
| `docs/FORGE_CONTRACT.md`, `docs/SEED_PROFILE.md`, `MANIFEST.md` | Product and forge boundary synchronization |
| `docs/CORE_CLAIMS.md`, `docs/ROADMAP.md`, `docs/PROGRESS_REPORT-FULL-PROJECT.md`, `AGENTS.md` | Current claim, roadmap, progress, and project-entry synchronization |
| This report | Review package, verification commands, risks, and Section 0 evidence |

## Verification evidence

| Check | Result |
| --- | --- |
| Bootstrap seed check, formatter, and artifact regeneration | PASS |
| Direct-forge packet regression | PASS — five prefixes, higher-priority missing-world, and Text-literal negative |
| Seed packet ABI integration regression | PASS |
| Shipped-example seed-versus-bootstrap identity regression | PASS |
| Constitution pack integrity | PASS — version 5.0.1 (`GOV-INT-001`) |
| Full quality gate | PASS — `pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode full` |
| Full-gate seed identity | `A692BEEA7BEB9891E5D64028A8537126AF2C52929754CA89399A8783C74BC76B` |

The full gate ran formatting, deny-warning Clippy, all workspace tests, 32
top-level example seed-versus-bootstrap comparisons, project flows, and the
bootstrap = product = forge = checked-in seed proof.

## Security, performance, and honesty boundary

The scanner reads only already-supplied source Text and has no file, process,
network, grant, or host-service authority. It stops at the first match and adds
one bounded source-line pass before compilation; it does not affect VM runtime
paths. Canonical lowercase and ASCII-space handling are intentional limits, not
claims of parser-equivalent classification. The host preflight remains broader,
including Unicode trimming and ASCII case folding.

## Section 0 gate

| # | Check | Result |
| --- | --- | --- |
| 1 | Completeness | PASS — all requested seed, artifact, tracker, test, and documentation dependencies are present; change-set scan adds no unfinished-work markers |
| 2 | Dependency-first | PASS — seed source and regenerated artifact ship together; host packet ABI already exists |
| 3 | Zero warnings/errors | PASS — full gate formatting and deny-warning Clippy clean |
| 4 | Tests exist and pass | PASS — direct behavior, negative, priority, ABI, identity, workspace, and gate suites pass |
| 5 | Docs synchronized | PASS — ADR, matrix, contracts, claims, roadmap, progress, and project entry updated together |
| 6 | Security and validation | PASS — untrusted source scanner is bounded, literal-safe, and adds no authority |
| 7 | Performance reasoning | PASS — single early-exit line scan, outside VM runtime paths |
| 8 | Version and stack fidelity | PASS — Aether seed profile and Rust 2021 workspace conventions retained |
| 9 | Full package ready | PASS — clean scoped Git package; generated gate output remains ignored under `target/` |
| 10 | Resource and constraint check | PASS — offline deterministic work; no dependency, network, or hardware expansion |
| 11 | Reproducibility and determinism | PASS — workspace root `Cargo.lock` plus bootstrap = product = forge = checked-in SHA-256 identity proven |
| 12 | IP and invention hygiene | PASS — bounded mechanism and residual limits recorded; no broader invention claim made |
| 13 | Multi-agent coordination | N/A — one implementation agent performed this atomic increment |
| 14 | Review packaging | PASS — manifest-equivalent inventory, commit scope, verification, risks, and next action are present |
| 15 | Self-audit log | PASS — recorded below; mechanical scan boundary is documented before commit |

### Mechanical scan boundary

The prescribed `gate_check.py` was executed at the repository root. It reports
pre-existing historical documentation markers, established resource-provenance
terms, ignored `target/*.log` outputs, and nested workspace manifests outside
this delivery's changed package. Those findings are retained rather than
silently deleted or renamed because they are unrelated project history and
public semantics. An exact change-set scan using the same completeness patterns
and `git diff --check` passed: this delivery introduces neither an
unfinished-work marker nor whitespace errors. The normal full Aether gate is
the authoritative workspace build, test, warning, and identity evidence above.

## Self-audit log

- Re-read the BARP boundary and used the existing host classification as the reference behavior.
- Implemented exactly the five canonical reserved-task prefixes, without widening task syntax.
- Regenerated the checked-in seed artifact from the edited Aether source.
- Tested each prefix, preflight priority, literal safety, packet ABI, and shipped-example seed identity.
- Verified full workspace quality and deterministic four-way seed identity with the full gate.
- Kept documentation explicit that the full SPEAK matrix and seed-native multi-file remain residual.
- Reviewed source-input handling for false-positive and authority expansion risks.
- Prepared this scoped inventory, recorded the scanner's inherited findings, and passed the exact change-set check before atomic commit.

## Exact repeat verification

```powershell
pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode full
cargo test -p aether-core seed_speak_pilot_covers_lexical_structural_and_reserved_task_preflights --lib
cargo test -p aether-core --test seed_self_host barp_adr072_product_seed_error_packet_abi
```

## Known residuals and next action

The remaining explicitly unimplemented seed-SPEAK conformance codes are
`AE-SEED-010`, `AE-SEED-011`, `AE-SEED-013`, and `AE-SEED-015`; seed-native
multi-file elaboration is also still unimplemented. The next BARP increment
must select and document one bounded invariant before changing the seed.

Suggested commit: `feat: add reserved-task seed-SPEAK pilot`

## Linked records

- [ADR-103](ADR-103-barp-seed-speak-reserved-task-pilot.md)
- [BARP validation matrix](../Current%20state/BARP-VALIDATION-MATRIX.md)
- [BARP design](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
- [Forge contract](../Current%20state/FORGE_CONTRACT.md)
- [Seed Profile](../Current%20state/SEED_PROFILE.md)

---

*End of DELIVERY_REPORT-2026-08-11-BARP-RESERVED-TASK-SPEAK.md*
