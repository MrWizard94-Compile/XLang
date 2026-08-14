# Delivery Report — BARP canonical unknown-call seed-SPEAK pilot

**Date:** 2026-08-11
**Status:** Implementation complete; product release gate PASS (2026-08-11, including the full-quality suite); approved Constitution scanner correction verified
**Scope:** ADR-110 bounded direct-seed `AE-SEED-011` diagnostic authority reduction
**Rule IDs:** `CONST-COMPLETE-001`, `CONST-DEP-001`, `CONST-DONE-001`,
`CONST-GATE-001`, `DOC-SYNC-001`, `ENG-WARN-001`, `RND-INVAR-001`,
`SEC-INPUT-001`, `TEST-BEHAVIOR-001`

---

## Delivered behavior

The checked-in Aether seed now finds at most one canonical direct-call statement
in an ordinary `Whole` weave: a trimmed `bind … <- call target value` line. It
then makes a second bounded source-line pass to find an exact top-level
declaration header: `weave target `, `export weave target `, `host weave target `,
`foreign weave target `, or `task weave target `. If none exists, it SPEAKs one
`aether.seed-error/v1` packet with `AE-SEED-011`, `origin: seed-speak`, stable
position `1:1`, and blank Bytes.

- A matching ordinary weave header declared after the call remains valid.
- A matching host weave header is not misclassified as an unknown target.
- A Text literal containing the same characters remains valid.
- Missing `world` remains the higher-priority `AE-SEED-006` packet.
- The normal product path merges this exact packet and reports
  `origin: seed-speak` for the named canonical form.

No accepted valid source, AETH opcode/version, verifier, VM behavior, host
grant, filesystem, process, network, shell, model, or guest authority changed.

## Delivered package

| Artifact | Purpose |
| --- | --- |
| `seed/aether_seed.ae` / `.aeth` | Two-pass direct-call/header scan and regenerated checked-in seed |
| `crates/xlang-core/src/lib.rs` | Pilot tracker plus direct, forward, host-header, literal, and priority regressions |
| `crates/xlang-core/tests/seed_self_host.rs` | Product seed-packet-origin contract |
| `docs/ADR-110-barp-seed-speak-unknown-call-pilot.md` | Decision, invariants, alternatives, and honesty boundary |
| `docs/BARP-VALIDATION-MATRIX.md` | Direct, valid, priority, and product-origin evidence |
| `docs/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md` | BARP map through ADR-110 |
| `docs/FORGE_CONTRACT.md`, `docs/SEED_PROFILE.md`, `docs/AETHER_0.37.md` | Current forge/seed/package-boundary contract |
| `docs/CORE_CLAIMS.md`, `docs/ROADMAP.md`, `docs/PROGRESS_REPORT-FULL-PROJECT.md` | Claims, residual ordering, and verification status |
| `README.md`, `MANIFEST.md`, `AGENTS.md` | Product-facing scope and project-entry synchronization |
| This report | Review package, evidence, Section 0 audit, residuals, repeat commands |

## Verification evidence

| Check | Result |
| --- | --- |
| Bootstrap compile of edited seed | PASS |
| Direct seed forge | PASS — canonical unknown call produces `AE-SEED-011`, `seed-speak` |
| Boundary and priority negatives | PASS — forward and host headers / Text literal accept; missing world wins |
| Product seed-packet origin | PASS — canonical unknown call reports `AE-SEED-011`, `seed-speak` |
| Seed self-host ABI | PASS — bootstrap = product = forge = checked-in seed, SHA-256 `593081B6C743FD3D857E979CDE6374589EFF89BA5EA6F603543474E3F936EAAB` |
| Constitution pack integrity | PASS — 5.0.1, `GOV-INT-001` |
| Full quality gate | PASS — release gate ran `cargo fmt --check`, deny-warning Clippy, and all workspace tests |
| Release gate / consumer preview | PASS — technical-preview package and 407-file exact-integrity consumer verification |

The release gate covered formatting, deny-warning Clippy, all workspace tests,
seed/bootstrap comparison corpora, package/project flows, release packaging,
consumer verification, and bootstrap = product = forge = checked-in seed proof.

## Security, performance, and honesty boundary

The scan reads only caller-supplied source Text already delivered to the seed.
It runs at most two bounded source-line passes, stores only one target Text, and
has no file, process, network, shell, grant, model, or host-service authority.
Exact ordinary-Whole, statement-prefix, target-boundary, and declaration-header
boundary checks prevent Text, name prefixes, forward declarations, and valid
host declarations from becoming false positives. Header recognition establishes
only target existence; the full compiler remains responsible for call-kind
legality.

This is not a full name binder, source-span implementation, or general
`AE-SEED-011` parity claim. Remaining BARP work includes broader
`AE-SEED-011` / `AE-SEED-013` cases and seed-native multi-file elaboration.

## Section 0 gate

| # | Check | Result |
| --- | --- | --- |
| 1 | Completeness | PASS — source/artifact, tracker, tests, ADR, matrix, docs, and report are wired together |
| 2 | Dependency-first | PASS — product forge/classification, packet ABI, and forward-call semantics existed before this narrow witness |
| 3 | Zero warnings/errors | PASS — `cargo fmt --check`, deny-warning Clippy, and workspace tests passed in release mode |
| 4 | Tests exist and pass | PASS — direct, forward, literal, priority, origin, and identity evidence |
| 5 | Docs synchronized | PASS — contract, claims, roadmap, profile, forge, progress, manifest, README, ADR, matrix, report updated together |
| 6 | Security and validation | PASS — bounded canonical scans, target-boundary validation, no authority expansion, no secrets |
| 7 | Performance reasoning | PASS — two bounded source-line passes, one target Text, no VM runtime-path change |
| 8 | Version and stack fidelity | PASS — Rust 2021, Aether Seed Profile, AETH v11/v12 retained |
| 9 | Full package ready | PASS — local technical-preview package and 407-file consumer verification passed |
| 10 | Resource and constraint check | PASS — offline deterministic work; no dependency, network, or hardware expansion |
| 11 | Reproducibility and determinism | PASS — bootstrap = product = forge = checked-in seed, SHA-256 `593081B6C743FD3D857E979CDE6374589EFF89BA5EA6F603543474E3F936EAAB` |
| 12 | IP and invention hygiene | PASS — mechanism, invariants, limits, and residuals recorded |
| 13 | Multi-agent coordination | N/A — one implementation agent performed the atomic increment |
| 14 | Review packaging | PASS — inventory, commands, evidence, risks, residuals, and commit boundary are present |
| 15 | Self-audit log | PASS — release evidence, named policy approval, scanner behavior tests, root scan, and contextual-term review are recorded |

## Self-audit log

- Re-read the product unknown-call classification and forward-call corpus
  before choosing one bounded seed witness.
- Implemented a source-line target/header check rather than a general binder.
- Proved that forward and host headers plus call-shaped Text literals remain valid.
- Proved product packet origin is seed-native for the named canonical form.
- Regenerated and proved bootstrap/product/forge/check-in identity.
- Passed the release gate, including pack integrity, zero-warning checks, full
  workspace tests, 32 example dual-compares, technical-preview packaging, and
  consumer verification.
- Applied the named human-approved scanner correction; its 43 behavior tests pass and all 37 contextual Item 1 findings were manually reviewed.
- Preserved broader `AE-SEED-011` / `AE-SEED-013` cases and seed-native
  multi-file elaboration as explicit residuals.

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
cargo test -p aether-core --test seed_self_host barp_phase3c_classifies_type_and_unknown_weave_product_failures
```

## Known residuals and next action

Choose a broader `AE-SEED-011` or `AE-SEED-013` invariant only under a new ADR,
or separately design a seed-native multi-file forge ABI. Preserve verifier-first
artifacts, dual-compare proof, and the no-ambient-authority boundary.

Suggested commit: `feat: add unknown-call seed-SPEAK pilot`

---

*End of DELIVERY_REPORT-2026-08-11-BARP-UNKNOWN-CALL-SPEAK.md*
