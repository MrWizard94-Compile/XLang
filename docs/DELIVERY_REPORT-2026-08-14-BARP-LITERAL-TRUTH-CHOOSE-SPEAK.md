# Delivery Report — BARP literal Truth choose seed-SPEAK pilot

**Date:** 2026-08-14
**Status:** Implementation and release verification complete
**One-sentence summary:** ADR-113 adds exact literal `choose bright:` and
`choose dim:` direct-forge `AE-SEED-013` witnesses without broadening product
authority or claiming general Truth-expression parsing.
**Scope:** ADR-113 bounded direct-seed literal Truth `AE-SEED-013` diagnostic authority reduction
**Rule IDs:** `CONST-COMPLETE-001`, `CONST-DEP-001`, `CONST-DONE-001`, `CONST-GATE-001`, `DOC-SYNC-001`, `ENG-WARN-001`, `RND-INVAR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`

---

## Delivered behavior

The checked-in Aether seed's bounded line-state scan now recognizes exact
trimmed `choose bright:` and `choose dim:` lines inside a canonical ordinary
`weave ... -> Whole:` body. If a deeper line begins with `yield `, direct forge
emits exactly one `aether.seed-error/v1` packet with `AE-SEED-013`,
`origin: seed-speak`, position `1:1`, and blank Bytes. The shared packet message
now accurately says that yield is not allowed inside a canonical truth choose
branch, covering the existing `same` / `less` and new literal witnesses.

ADR-070 remains authoritative on the product path: each invalid literal source
is rejected before forge with a single `AE-SEED-013` `host-preflight` packet.
Valid `bright` and `dim` branches that revise a local then yield at weave root
still verify, match bootstrap byte-for-byte, and exit with their expected value.
Valid bare Truth variables and `choose not dim:` remain outside this literal
pilot, produce verified seed AETH, and match bootstrap.

This changes no valid source form, AETH schema or bytes, verifier, VM, forge
ABI, host capability, filesystem, process, network, shell, model, or guest
authority. Resource `choose` remains outside this scanner.

## Delivered package

| Artifact | Purpose |
| --- | --- |
| `seed/aether_seed.ae` / `.aeth` | Exact bright/dim literal scan, generalized message, and regenerated checked-in seed |
| `crates/xlang-core/src/lib.rs` | Expanded pilot contract, direct packet coverage, and missing-world priority regressions |
| `crates/xlang-core/tests/seed_self_host.rs` | TDD direct, cardinality, blank-result, product-origin, literal identity/run, and nonliteral-boundary regression |
| `docs/ADR-113-barp-seed-speak-literal-truth-choose-yield-pilot.md` | Decision, invariants, alternatives, and honesty boundary |
| `docs/BARP-VALIDATION-MATRIX.md` | Literal, valid, nonliteral-boundary, priority, and product-origin evidence |
| `docs/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md` | BARP map through ADR-113 |
| `docs/FORGE_CONTRACT.md`, `docs/SEED_PROFILE.md`, `docs/AETHER_0.37.md` | Current forge, seed, and package-boundary contract |
| `docs/CORE_CLAIMS.md`, `docs/ROADMAP.md`, `docs/PROGRESS_REPORT-FULL-PROJECT.md` | Claims, residual ordering, and verification status |
| `README.md`, `MANIFEST.md`, `AGENTS.md` | Product-facing scope and project-entry synchronization |
| This report | Review package, evidence, Section 0 audit, residuals, and repeat commands |

## Verification evidence

| Check | Result |
| --- | --- |
| Test-driven regression | PASS — the direct bright witness was red against the prior checked-in seed, then green after the seed rebuild |
| Direct literal seed forge | PASS — exact `bright` and `dim` nested yields each emit one `AE-SEED-013` `seed-speak` packet with blank Bytes |
| Product safety boundary | PASS — each literal source retains one `AE-SEED-013` `host-preflight` packet |
| Valid literal artifacts | PASS — bright/dim revise-plus-root-yield sources verify, match bootstrap, and exit 42 |
| Nonliteral boundary | PASS — valid bare Truth variable and `choose not dim:` sources emit no seed packet, verify, and match bootstrap |
| Core semantic suite | PASS — 144 core tests, 32 self-host tests, effect/layout/comptime/nursery suites, and doctests |
| Constitution pack integrity | PASS — AGENTS Constitution 5.0.1 (`GOV-INT-001`) |
| Full quality gate | PASS — format, deny-warning Clippy, workspace tests, seed/product/self-host checks, and release proof |
| Release package / consumer preview | PASS — exact package integrity, examples, project/workspace, M25 lifecycle, and unlisted-file tamper rejection |

The completed release gate proved bootstrap = product = forge = checked-in seed
at SHA-256 `2360EF2AA9B2700AF07A8FEAF907E795E07922403352ACCFE46AF321CDB815DB`.

## Security, performance, and honesty boundary

The extension reuses the existing bounded second source-line pass. It adds two
exact Text equality checks and no parser, host operation, allocation class, VM
path, or execution cost. It reads only caller-supplied source Text already
handed to the seed. Exact literal comparisons and existing indentation/root-yield
state keep bare Truth variables, `not` expressions, Text literals, resource
choices, valid root yield, and broader truth-condition syntax outside this pilot.

This is not a general Truth parser, complete `AE-SEED-013` parity, source-span
implementation, or seed-native multi-file elaboration claim. Broader
`AE-SEED-011` / `AE-SEED-013` forms and a multi-file forge ABI remain separate
design increments.

## Section 0 gate

| # | Check | Result |
| --- | --- | --- |
| 1 | Completeness | PASS — source, artifact, tests, contract docs, and report are present; no incomplete integration remains |
| 2 | Dependency-first | PASS — ADR-070 preflight, ADR-072 packet ABI, and ADR-109/112 state machine established the needed boundary |
| 3 | Zero warnings/errors | PASS — release gate ran `cargo fmt --check`, workspace Clippy with `-D warnings`, and workspace tests cleanly |
| 4 | Tests exist and pass | PASS — red/green, direct, cardinality, blank-result, valid identity/run, nonliteral, priority, and product-origin evidence |
| 5 | Docs synchronized | PASS — ADR, matrix, design, profile, forge, claims, roadmap, progress, manifest, README, and entry point updated together |
| 6 | Security and validation | PASS — exact canonical scan, no authority expansion, and no secrets |
| 7 | Performance reasoning | PASS — two constant equality checks inside an existing bounded seed scan; no VM runtime-path change |
| 8 | Version and stack fidelity | PASS — Rust 2021, Aether Seed Profile, and AETH v11/v12 retained |
| 9 | Full package ready | PASS — technical-preview package passed exact integrity and independent consumer verification |
| 10 | Resource and constraint check | PASS — offline deterministic work with no dependency, network, or hardware expansion |
| 11 | Reproducibility and determinism | PASS — bootstrap = product = forge = checked-in seed at the recorded SHA-256 |
| 12 | IP and invention hygiene | PASS — mechanism, invariants, limits, and residuals are dated and recorded |
| 13 | Multi-agent coordination | N/A — one implementation agent delivered this atomic increment |
| 14 | Review packaging | PASS — inventory, evidence, commands, risks, residuals, and commit boundary are recorded |
| 15 | Self-audit log | PASS — red/green behavior, full gate, package verification, and no-authority boundary reviewed |

## Self-audit log

- Added the bright/dim regression first and observed the prior seed emit no
  direct packet for `choose bright:`.
- Reused ADR-109/112 state instead of introducing a Truth-expression parser.
- Required exact literal line equality, one packet, `seed-speak` origin, shared
  message, and blank Bytes.
- Preserved ADR-070's stronger product `host-preflight` boundary in the test.
- Proved bright/dim valid root-yield artifacts run correctly and retain
  seed/bootstrap identity.
- Proved valid bare Truth-variable and `not` forms stay outside the literal pilot.
- Passed the full release gate, including pack integrity, deny-warning Clippy,
  workspace tests, four-way seed identity, package consumer execution, and
  tamper rejection.
- Left broader seed diagnostic parity and seed-native multi-file elaboration as
  explicit residuals.

## Exact repeat verification

```powershell
cargo test -p aether-core --test seed_self_host barp_adr113_seed_speaks_canonical_literal_truth_choose_yield
cargo test -p aether-core seed_speak_pilot_covers_lexical_structural_and_task_preflights --lib
cargo test -p aether-core
pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release
```

## Suggested commit message

```text
feat: extend literal Truth seed SPEAK diagnostics
```

## Known residuals and next action

Choose another bounded `AE-SEED-011` or `AE-SEED-013` invariant only under a
new ADR, or separately design a seed-native multi-file forge ABI. Preserve the
verifier-first artifact rule, dual-compare proof, and no-ambient-authority
boundary.

---

*End of DELIVERY_REPORT-2026-08-14-BARP-LITERAL-TRUTH-CHOOSE-SPEAK.md*
