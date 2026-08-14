# Delivery Report - BARP exact task-checkpoint seed-SPEAK pilot

**Date:** 2026-08-11
**Status:** Delivered - full gate PASS
**Scope:** ADR-106 bounded direct-seed diagnostic authority reduction
**Rule IDs:** CONST-COMPLETE-001, CONST-DEP-001, CONST-DONE-001,
CONST-GATE-001, DOC-SYNC-001, ENG-WARN-001, RND-INVAR-001,
SEC-INPUT-001, TEST-BEHAVIOR-001

---

## Delivered behavior

The checked-in Aether seed compiler now line-scans canonical top-level task
weaves before normal compilation. Each task body must contain an exact indented
checkpoint statement before its next nonblank top-level line or end of source.
An unsatisfied task emits one structured aether.seed-error/v1 SPEAK packet with
code AE-SEED-015, origin seed-speak, stable position 1:1, and blank Bytes.

The scan is deliberately narrow:

- It recognizes only a canonical indentation-zero task header.
- It accepts only the exact statement checkpoint, including a nested task while
  body.
- It does not accept checkpointed, trailing-token variants, or a text literal
  containing the word checkpoint.
- Existing higher-priority seed pilots, including AE-SEED-014 reserved task
  surface, retain precedence.

The product host preflight now uses the same exact-checkpoint requirement and
labels its AE-SEED-015 packet origin as host-preflight. No accepted valid source,
AETH version/opcode, verifier, VM behavior, host grant, filesystem, process,
network, shell, or task-runtime authority changed.

## Delivered package

| Artifact | Purpose |
| --- | --- |
| seed/aether_seed.ae | Canonical stateful AE-SEED-015 direct seed-SPEAK scanner |
| seed/aether_seed.aeth | Regenerated checked-in seed artifact |
| crates/xlang-core/src/lib.rs | Exact host preflight, packet origin, pilot tracker, and direct-forge regressions |
| crates/xlang-core/tests/seed_self_host.rs | Self-host ABI assertion for ADR-106 |
| docs/ADR-106-barp-seed-speak-checkpoint-pilot.md | Decision, invariants, alternatives, and honesty boundary |
| docs/BARP-VALIDATION-MATRIX.md | Boundary, EOF, nested, typo, literal, priority, and host-origin evidence |
| docs/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md | BARP authority map through ADR-106 |
| docs/FORGE_CONTRACT.md and docs/SEED_PROFILE.md | Forge and seed-profile diagnostic contracts |
| docs/CORE_CLAIMS.md, docs/ROADMAP.md, docs/PROGRESS_REPORT-FULL-PROJECT.md | Current claims, next residuals, and gate stamp |
| README.md and MANIFEST.md | Product-facing seed-boundary synchronization; stale M23 materialization wording corrected |
| AGENTS.md | Project entry-point BARP record through ADR-106 |
| This report | Review package, gate evidence, Section 0 audit, residuals, and repeat commands |

## Verification evidence

| Check | Result |
| --- | --- |
| Bootstrap compile of edited seed | PASS |
| Direct forge regressions | PASS - boundary, EOF, typo/trailing-token/task-literal negatives, nested checkpoint, and AE-SEED-014 precedence |
| Product AE-SEED-015 packet origin regression | PASS - host-preflight |
| Seed self-host ABI regression | PASS |
| Constitution pack integrity | PASS - version 5.0.1, GOV-INT-001 |
| Full quality gate | PASS - pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode full |
| Full-gate seed identity | 86391C07D31069526D5FC33ADDC287FF1F79D4C35C663D388E3B6E2108F5ADCB |

The full gate ran formatting, deny-warning Clippy, all workspace tests, 32
top-level example seed-versus-bootstrap comparisons, host/project flows, and
bootstrap = product = forge = checked-in seed proof.

## Security, performance, and honesty boundary

The new scanner reads only caller-supplied source Text already delivered to the
seed. It performs a single bounded source-line pass, stops at the first error,
and has no file, process, network, shell, grant, model, or host-service
authority. Exact line matching prevents a checkpoint-shaped identifier or Text
literal from satisfying a cooperative-cancellation safety requirement.

This is not a full task parser, full source-span implementation, or full seed
diagnostic-parity claim. Remaining conformance codes are AE-SEED-010,
AE-SEED-011, and AE-SEED-013; seed-native multi-file elaboration is separate.
Task handles, timeouts, parallel execution, and manual cancellation remain
unimplemented.

## Section 0 gate

| # | Check | Result |
| --- | --- | --- |
| 1 | Completeness | PASS - seed source, artifact, host alignment, regressions, ADR, matrix, and docs ship together |
| 2 | Dependency-first | PASS - existing M19e/ADR-101 precondition and packet ABI were validated before seed authority reduction |
| 3 | Zero warnings/errors | PASS - full gate formatting and deny-warning Clippy clean |
| 4 | Tests exist and pass | PASS - direct/negative/priority/identity/workspace tests and full gate pass |
| 5 | Docs synchronized | PASS - contract, claims, roadmap, profile, forge, progress, manifest, README, project entry, ADR, and matrix updated together |
| 6 | Security and validation | PASS - bounded source scanner, exact matching, no authority expansion, no secrets |
| 7 | Performance reasoning | PASS - one early linear line scan; no VM runtime-path change |
| 8 | Version and stack fidelity | PASS - Rust 2021, Aether seed profile, and AETH v11/v12 contract retained |
| 9 | Full package ready | PASS - no source temporary files or half-applied artifacts remain |
| 10 | Resource and constraint check | PASS - offline deterministic work; no dependency, network, or hardware expansion |
| 11 | Reproducibility and determinism | PASS - four-way SHA-256 identity proven |
| 12 | IP and invention hygiene | PASS - mechanism, invariants, limits, and residuals explicitly recorded |
| 13 | Multi-agent coordination | N/A - one implementation agent performed the atomic increment |
| 14 | Review packaging | PASS - inventory, command evidence, risks, residuals, and commit scope included |
| 15 | Self-audit log | PASS - exact change-set scan and whitespace audit recorded below |

### Mechanical scan boundary

The prescribed gate_check.py root scan reports inherited historical
documentation markers, established resource-provenance terms, ignored target
logs, and nested workspace manifests outside this delivery. They are retained
because they are not new defects in this package. The same patterns applied to
only added/changed lines, plus git diff --check, passed for this delivery.

## Self-audit log

- Re-read BARP, M19e, and parser exact-statement behavior before design.
- Chose the narrow direct diagnostic invariant instead of widening task syntax.
- Found and corrected the false acceptance of checkpointed in the host preflight.
- Regenerated the seed artifact and proved bootstrap/product/forge/check-in identity.
- Tested both task-body boundary modes, exact token and trailing-token behavior,
  literal safety, nested checkpoints, priority, and packet origin.
- Ran the full pack, warning, test, example, project, and identity gate.
- Kept all broader parser parity, multi-file, and task-runtime claims residual.
- Audited the final changed package for temporary files, whitespace faults, and
  newly introduced unfinished-work markers.

## Exact repeat verification

    pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode full
    cargo test -p aether-core seed_speak_pilot_covers_lexical_structural_and_task_preflights --lib
    cargo test -p aether-core product_requires_checkpoint_in_task_weave --lib
    cargo test -p aether-core --test seed_self_host barp_adr072_product_seed_error_packet_abi

## Known residuals and next action

The next BARP increment should choose one bounded invariant for AE-SEED-010,
AE-SEED-011, or AE-SEED-013, or separately design a seed-native multi-file
forge ABI. It must preserve verifier-first artifacts, dual-compare proof, and
the no-ambient-authority boundary.

Suggested commit: feat: add checkpoint seed-SPEAK pilot

## Linked records

- ADR-106
- BARP validation matrix
- BARP design
- Forge contract
- Seed Profile

---

*End of DELIVERY_REPORT-2026-08-11-BARP-CHECKPOINT-SPEAK.md*
