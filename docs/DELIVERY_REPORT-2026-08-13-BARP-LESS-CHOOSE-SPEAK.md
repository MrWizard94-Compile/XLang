# Delivery Report — BARP canonical less-choose seed-SPEAK pilot

**Date:** 2026-08-13
**Status:** Implementation and release verification complete
**One-sentence summary:** ADR-112 gives direct forge a bounded seed-native
`AE-SEED-013` witness for canonical `choose less` nested `yield`, while ADR-070
retains the broader product preflight.
**Scope:** ADR-112 bounded direct-seed `AE-SEED-013` diagnostic authority reduction
**Rule IDs:** `CONST-COMPLETE-001`, `CONST-DEP-001`, `CONST-DONE-001`, `CONST-GATE-001`, `DOC-SYNC-001`, `ENG-WARN-001`, `RND-INVAR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`

---

## Delivered behavior

The checked-in Aether seed's bounded line-state scan now recognizes a canonical
ordinary-Whole `choose less` branch as well as ADR-109's canonical `choose same`
branch. If a deeper source line begins with `yield `, direct forge emits exactly
one `aether.seed-error/v1` packet with `AE-SEED-013`, `origin: seed-speak`,
position `1:1`, and blank Bytes. Its shared message accurately names the
comparison-choose witness.

ADR-070 remains authoritative on the product path: the same invalid source is
rejected before forge with `AE-SEED-013`, `origin: host-preflight`. A valid
`choose less` branch that revises a local then yields at weave root still
verifies and is byte-identical between the seed and bootstrap compilers.

This changes no valid source form, AETH schema or bytes, verifier, VM, forge
ABI, host capability, filesystem, process, network, shell, model, or guest
authority. Resource `choose` remains outside this scanner.

## Delivered package

| Artifact | Purpose |
| --- | --- |
| `seed/aether_seed.ae` / `.aeth` | Bounded `choose less` scan and regenerated checked-in seed |
| `crates/xlang-core/src/lib.rs` | Expanded direct-seed pilot contract and priority regressions |
| `crates/xlang-core/tests/seed_self_host.rs` | Direct seed, product-origin, blank-result, valid identity regression |
| `docs/ADR-112-barp-seed-speak-less-choose-yield-pilot.md` | Decision, invariants, alternatives, and honesty boundary |
| `docs/BARP-VALIDATION-MATRIX.md` | Direct, valid, priority, and product-origin evidence |
| `docs/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md` | BARP map through ADR-112 |
| `docs/FORGE_CONTRACT.md`, `docs/SEED_PROFILE.md`, `docs/AETHER_0.37.md` | Current forge, seed, and package-boundary contract |
| `docs/CORE_CLAIMS.md`, `docs/ROADMAP.md`, `docs/PROGRESS_REPORT-FULL-PROJECT.md` | Claims, residual ordering, and verification status |
| `README.md`, `MANIFEST.md`, `AGENTS.md` | Product-facing scope and project-entry synchronization |
| This report | Review package, evidence, Section 0 audit, residuals, and repeat commands |

## Verification evidence

| Check | Result |
| --- | --- |
| Test-driven regression | PASS — the new test was red against the prior checked-in seed, then green after the seed rebuild |
| Direct seed forge | PASS — canonical `choose less` nested yield yields one `AE-SEED-013` `seed-speak` packet and blank Bytes |
| Product safety boundary | PASS — the product retains one `AE-SEED-013` `host-preflight` packet |
| Valid comparison artifact | PASS — `choose less` revise plus root yield verifies and matches bootstrap byte-for-byte |
| Core semantic suite | PASS — 144 core tests, 31 self-host tests, effect/layout/comptime/nursery suites, and doctests |
| Constitution pack integrity | PASS — AGENTS Constitution 5.0.1 (`GOV-INT-001`) |
| Full quality gate | PASS — format, deny-warning Clippy, workspace tests, seed/product/self-host checks, and release proof |
| Release package / consumer preview | PASS — exact package integrity, examples, project/workspace, M25 lifecycle, and unlisted-file tamper rejection |

The completed release gate proved bootstrap = product = forge = checked-in seed
at SHA-256 `E09FA42287B106FC48068EC5CCB558F29CDFDA8EB8D9D5BD9CB1B4927F24C57D`.

## Security, performance, and honesty boundary

The new recognition reuses the existing second source-line pass; it adds one
literal prefix check and no parser, host operation, allocation class, VM path,
or runtime cost. It reads only the caller-supplied source Text already handed to
the seed. The ordinary-Whole, exact line-prefix, indentation, and root-yield
checks keep Text literals, resource choices, valid root yield, and broader
truth-condition syntax outside this bounded witness.

This is not a full comparison parser, complete `AE-SEED-013` parity, source-span
implementation, or seed-native multi-file elaboration claim. Broader
`AE-SEED-011` / `AE-SEED-013` forms and a multi-file forge ABI remain separate
design increments.

## Section 0 gate

| # | Check | Result |
| --- | --- | --- |
| 1 | Completeness | PASS — source, artifact, tests, contract docs, and report are present; no incomplete integration remains |
| 2 | Dependency-first | PASS — ADR-070 preflight, ADR-072 packet ABI, and ADR-109 line-state scan already established the needed boundary |
| 3 | Zero warnings/errors | PASS — release gate ran `cargo fmt --check`, workspace Clippy with `-D warnings`, and workspace tests cleanly |
| 4 | Tests exist and pass | PASS — direct, cardinality, blank-result, valid identity, priority, and product-origin evidence |
| 5 | Docs synchronized | PASS — ADR, matrix, design, profile, forge, claims, roadmap, progress, manifest, README, and entry point updated together |
| 6 | Security and validation | PASS — bounded canonical scan, no authority expansion, and no secrets |
| 7 | Performance reasoning | PASS — one constant prefix check inside an existing bounded seed scan; no VM runtime-path change |
| 8 | Version and stack fidelity | PASS — Rust 2021, Aether Seed Profile, and AETH v11/v12 retained |
| 9 | Full package ready | PASS — technical-preview package passed exact integrity and independent consumer verification |
| 10 | Resource and constraint check | PASS — offline deterministic work with no dependency, network, or hardware expansion |
| 11 | Reproducibility and determinism | PASS — bootstrap = product = forge = checked-in seed at the recorded SHA-256 |
| 12 | IP and invention hygiene | PASS — mechanism, invariants, limits, and residuals are dated and recorded |
| 13 | Multi-agent coordination | N/A — one implementation agent delivered this atomic increment |
| 14 | Review packaging | PASS — inventory, evidence, commands, risks, residuals, and commit boundary are recorded |
| 15 | Self-audit log | PASS — red/green behavior, full gate, package verification, and no-authority boundary reviewed |

## Self-audit log

- Started with a regression that showed the prior checked-in seed emitted no
  direct packet for the canonical `choose less` source.
- Reused ADR-109's bounded state machine instead of building a comparison parser.
- Asserted one direct packet, `seed-speak` origin, shared message, and blank Bytes.
- Preserved ADR-070's stronger product `host-preflight` boundary in the test.
- Proved a valid `choose less` revise/root-yield source verifies and remains
  seed/bootstrap byte-identical.
- Passed the full release gate, including pack integrity, deny-warning Clippy,
  workspace tests, four-way seed identity, package consumer execution, and
  tamper rejection.
- Left broader seed diagnostic parity and seed-native multi-file elaboration as
  explicit residuals.

## Exact repeat verification

```powershell
cargo test -p aether-core --test seed_self_host barp_adr112_seed_speaks_canonical_less_choose_yield
cargo test -p aether-core seed_speak_pilot_covers_lexical_structural_and_task_preflights --lib
cargo test -p aether-core
pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release
```

## Suggested commit message

```text
feat: extend seed SPEAK comparison diagnostics
```

## Known residuals and next action

Choose another bounded `AE-SEED-011` or `AE-SEED-013` invariant only under a
new ADR, or separately design a seed-native multi-file forge ABI. Preserve the
verifier-first artifact rule, dual-compare proof, and no-ambient-authority
boundary.

---

*End of DELIVERY_REPORT-2026-08-13-BARP-LESS-CHOOSE-SPEAK.md*
