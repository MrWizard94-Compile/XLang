# Delivery Report — BARP canonical truth-choose seed-SPEAK pilot

**Date:** 2026-08-11
**Status:** Implementation complete; product full/release gate PASS; approved Constitution scanner correction verified
**Scope:** ADR-109 bounded direct-seed `AE-SEED-013` diagnostic authority reduction
**Rule IDs:** `CONST-COMPLETE-001`, `CONST-DEP-001`, `CONST-DONE-001`,
`CONST-GATE-001`, `DOC-SYNC-001`, `ENG-WARN-001`, `RND-INVAR-001`,
`SEC-INPUT-001`, `TEST-BEHAVIOR-001`

---

## Delivered behavior

The checked-in Aether seed now scans only a canonical ordinary,
indentation-zero `weave ` header containing `-> Whole:`. Within that body, a
trimmed line starting exactly `choose same ` arms a bounded truth-condition
state. An indented `yield ` beneath that state SPEAKs one
`aether.seed-error/v1` packet with `AE-SEED-013`, `origin: seed-speak`, stable
position `1:1`, and blank Bytes.

- A `choose same` that revises then reaches a root-level `yield` remains valid.
- Resource `choose` forms retain their permitted nested yields.
- Missing `world` remains the higher-priority `AE-SEED-006` seed packet.
- The normal product path deliberately retains its complete host preflight for
  `AE-SEED-013`; its packet remains `origin: host-preflight`.

No accepted valid source, AETH opcode/version, verifier, VM behavior, host
grant, filesystem, process, network, shell, model, or guest authority changed.

## Delivered package

| Artifact | Purpose |
| --- | --- |
| `seed/aether_seed.ae` / `.aeth` | Canonical `choose same` state scan and regenerated checked-in seed |
| `crates/xlang-core/src/lib.rs` | Pilot tracker plus direct, negative, priority, and resource regressions |
| `crates/xlang-core/tests/seed_self_host.rs` | Product preflight packet-origin contract |
| `docs/ADR-109-barp-seed-speak-truth-choose-yield-pilot.md` | Decision, invariants, alternatives, and honesty boundary |
| `docs/BARP-VALIDATION-MATRIX.md` | Direct, valid, priority, and product-origin evidence |
| `docs/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md` | BARP map through ADR-109 |
| `docs/FORGE_CONTRACT.md`, `docs/SEED_PROFILE.md`, `docs/AETHER_0.37.md` | Current forge/seed/package-boundary contract |
| `docs/CORE_CLAIMS.md`, `docs/ROADMAP.md`, `docs/PROGRESS_REPORT-FULL-PROJECT.md` | Claims, residual ordering, and verification status |
| `README.md`, `MANIFEST.md`, `AGENTS.md` | Product-facing scope and project-entry synchronization |
| This report | Review package, evidence, Section 0 audit, residuals, repeat commands |

## Verification evidence

| Check | Result |
| --- | --- |
| Bootstrap compile of edited seed | PASS |
| Direct seed forge | PASS — canonical `choose same` nested yield produces `AE-SEED-013`, `seed-speak` |
| Boundary and priority negatives | PASS — root yield / resource choose accept; missing world wins; product remains host-preflight |
| Seed self-host ABI | PASS — bootstrap = product = forge = checked-in seed |
| Core suite | PASS — 144 core unit tests, 10 M4, 4 M5, 3 M6, 4 M7, and 30 self-host tests |
| Constitution pack integrity | PASS — 5.0.1, `GOV-INT-001` |
| Full quality gate | PASS — `aether-gate.ps1 -Mode full` within release mode |
| Release gate / consumer preview | PASS — `aether-gate.ps1 -Mode release` |
| Final-gate seed identity | `21A001368799FC1A34FC029E0E69C4367980F3EA18EC2B42BA3E8BB51E721F70` |

The release gate covers formatting, deny-warning Clippy, all workspace tests,
seed/bootstrap comparison corpora, package/project flows, release packaging,
consumer verification, and bootstrap = product = forge = checked-in seed proof.

## Security, performance, and honesty boundary

The scan reads only caller-supplied source Text already delivered to the seed.
It is one bounded source-line pass, stops after the first packet, and has no
file, process, network, shell, grant, model, or host-service authority. Its
ordinary-Whole, `choose same`, indentation, and `yield ` checks prevent
resource forms and valid root yields from becoming false positives.

This is not a full control-flow parser, source-span implementation, or general
`AE-SEED-013` parity claim. Remaining BARP work includes `AE-SEED-011`, broader
`AE-SEED-013` cases, and seed-native multi-file elaboration.

## Section 0 gate

| # | Check | Result |
| --- | --- | --- |
| 1 | Completeness | PASS — source/artifact, tracker, tests, ADR, matrix, docs, and report are wired together |
| 2 | Dependency-first | PASS — ADR-070's fail-closed rule and the seed packet ABI existed before this narrow witness |
| 3 | Zero warnings/errors | PASS — release gate formatting and deny-warning Clippy clean |
| 4 | Tests exist and pass | PASS — direct, valid, priority, origin, identity, and core-suite evidence |
| 5 | Docs synchronized | PASS — contract, claims, roadmap, profile, forge, progress, manifest, README, ADR, matrix, report updated together |
| 6 | Security and validation | PASS — bounded canonical scanner, literal/form safety, no authority expansion, no secrets |
| 7 | Performance reasoning | PASS — one early linear source-line pass; no VM runtime-path change |
| 8 | Version and stack fidelity | PASS — Rust 2021, Aether Seed Profile, AETH v11/v12 retained |
| 9 | Full package ready | PASS — deterministic preview staging, SHA-256SUMS, and consumer verification complete |
| 10 | Resource and constraint check | PASS — offline deterministic work; no dependency, network, or hardware expansion |
| 11 | Reproducibility and determinism | PASS — four-way SHA-256 identity proven |
| 12 | IP and invention hygiene | PASS — mechanism, invariants, limits, and residuals recorded |
| 13 | Multi-agent coordination | N/A — one implementation agent performed the atomic increment |
| 14 | Review packaging | PASS — inventory, commands, evidence, risks, residuals, and commit boundary are present |
| 15 | Self-audit log | PASS — release evidence, named policy approval, scanner behavior tests, root scan, and contextual-term review are recorded |

## Self-audit log

- Re-read ADR-070's product preflight before selecting a direct-seed witness.
- Implemented the smallest line-state scan that distinguishes a nested
  `choose same` yield from a valid root yield and resource `choose` branches.
- Preserved the product host-preflight packet origin for the full invalid
  family; this change does not remove a safety barrier.
- Regenerated and proved bootstrap/product/forge/check-in identity.
- Passed the full/release gate, including release package consumer verification.
- Applied the named human-approved scanner correction; its 43 behavior tests pass and all 37 contextual Item 1 findings were manually reviewed.
- Preserved `AE-SEED-011`, broader `AE-SEED-013`, and seed-native multi-file
  elaboration as explicit residuals.

### Generic mechanical scanner boundary

The named human-approved scanner correction has 43 behavior tests passing. It
prunes build output, agent-kit, and copied Constitution descendants only; an
explicitly selected root remains in scope. Explicit unfinished-work markers
still fail the scan, while contextual language is visible Item 1 manual review.
The Aether-root scan completes in 2.6 seconds with zero failures/warnings,
items 6/9/11 passing, and 37 manual-review findings. The 18 verifier/compile
and 19 historical-contract references have been reviewed as implemented
semantics, not deferred work.

This is not a waiver: it is the named human-approved scanner correction,
independently behavior-tested and manually reviewed under `CONST-GATE-001`.

## Exact repeat verification

```powershell
pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release
cargo test -p aether-core seed_speak_pilot_covers_lexical_structural_and_task_preflights --lib
cargo test -p aether-core --test seed_self_host barp_adr070_product_rejects_yield_in_truth_choose
```

## Known residuals and next action

Choose a bounded `AE-SEED-011` invariant, broaden `AE-SEED-013` only under a
new ADR, or separately design a seed-native multi-file forge ABI. Preserve
verifier-first artifacts, dual-compare proof, and the no-ambient-authority
boundary.

Suggested commit: `feat: add truth-choose seed-SPEAK pilot`

---

*End of DELIVERY_REPORT-2026-08-11-BARP-TRUTH-CHOOSE-SPEAK.md*
