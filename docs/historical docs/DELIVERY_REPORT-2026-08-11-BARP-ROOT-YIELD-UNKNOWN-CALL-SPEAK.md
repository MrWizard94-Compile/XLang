# Delivery Report — BARP canonical root-yield unknown-call seed-SPEAK pilot

**Date:** 2026-08-11
**Status:** Implementation and product release verification complete; approved Constitution scanner correction verified
**Scope:** ADR-111 bounded direct-seed `AE-SEED-011` root-yield diagnostic authority reduction
**Rule IDs:** `CONST-COMPLETE-001`, `CONST-DEP-001`, `CONST-DONE-001`, `CONST-GATE-001`, `DOC-SYNC-001`, `ENG-WARN-001`, `RND-INVAR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`

---

## Delivered behavior

The checked-in Aether seed's two-pass unknown-target scan now recognizes at most one canonical direct call in an ordinary `Whole` weave. Alongside ADR-110's two-space-indented `bind … <- call target value` shape, it recognizes an exact two-space-indented root `yield call target value` shape. The second pass checks exact indentation-zero declaration headers: `weave target `, `export weave target `, `host weave target `, `foreign weave target `, or `task weave target `.

If the target has no matching header, the seed SPEAKs one `aether.seed-error/v1` `AE-SEED-011` packet with `origin: seed-speak`, stable position `1:1`, and blank Bytes. Header recognition proves only that the target exists; the full compiler remains authoritative for declaration kind, effect, arity, ownership, result, and task-call legality.

- A root-yield unknown call now produces seed-native `AE-SEED-011`.
- Forward ordinary and declared host root-yield targets remain valid seed input.
- A call-shaped Text literal does not trigger the scan.
- Missing `world` remains the higher-priority `AE-SEED-006` packet.
- Existing ADR-110 direct-bind behavior remains in place.

No source syntax, AETH opcode/version, verifier, VM behavior, host grant, filesystem, process, network, shell, model, or guest authority changed.

## Delivered package

| Artifact | Purpose |
| --- | --- |
| `seed/aether_seed.ae` / `.aeth` | Extended two-pass direct-call scan and regenerated checked-in seed |
| `crates/xlang-core/src/lib.rs` | Expanded pilot boundary plus direct, valid, literal, and priority regressions |
| `crates/xlang-core/tests/seed_self_host.rs` | Product root-yield seed-packet-origin contract |
| `docs/ADR-111-barp-seed-speak-root-yield-unknown-call-pilot.md` | Decision, invariants, alternatives, and honesty boundary |
| `docs/BARP-VALIDATION-MATRIX.md` | Root-yield direct, valid, priority, and product-origin evidence |
| `docs/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md` | BARP map through ADR-111 |
| `docs/FORGE_CONTRACT.md`, `docs/SEED_PROFILE.md`, `docs/AETHER_0.37.md` | Current forge/seed/package-boundary contract |
| `docs/CORE_CLAIMS.md`, `docs/ROADMAP.md`, `docs/PROGRESS_REPORT-FULL-PROJECT.md` | Claims, residual ordering, and verification status |
| `docs/DESIGN-LAW-FORK-F-NATIVE.md`, `docs/DESIGN-LAW-FORK-F-REGISTRY.md` | Current authorized M35j/M24i boundaries; stale preauthorization language removed during scanner audit |
| `README.md`, `MANIFEST.md`, `AGENTS.md` | Product-facing scope and project-entry synchronization |
| This report | Review package, evidence, Section 0 audit, residuals, repeat commands |

## Verification evidence

| Check | Result |
| --- | --- |
| Bootstrap compile of edited seed | PASS |
| Direct seed forge | PASS — canonical root-yield unknown call produces `AE-SEED-011`, `seed-speak` |
| Boundary and priority negatives | PASS — forward and host headers / Text literal accept; missing world wins |
| Product seed-packet origin | PASS — canonical root-yield unknown call reports `AE-SEED-011`, `seed-speak` |
| Seed rebuild identity | PASS — targeted bootstrap/product/forge/check-in identity test |
| Constitution pack integrity | PASS — AGENTS Constitution 5.0.1 (`GOV-INT-001`) |
| Full quality gate | PASS — format, deny-warning Clippy, workspace tests, and all seed/product self-host checks |
| Release gate / consumer preview | PASS — 409-file local technical-preview package, consumer execution, and unlisted-file tamper rejection |

The completed release gate covered formatting, deny-warning Clippy, all workspace tests, seed/bootstrap comparison corpora, package/project flows, release packaging, consumer verification, and bootstrap = product = forge = checked-in seed proof. The four-way seed SHA-256 was `726301FC243CE36DA35C4CE08A428BB783E72B3165F1CA56957B91D89E747467`.

## Security, performance, and honesty boundary

The scan reads only caller-supplied source Text already delivered to the seed. It retains two bounded source-line passes and one target Text. It has no file, process, network, shell, grant, model, or host-service authority. Exact ordinary-Whole, two-space root-statement, target-boundary, and declaration-header checks prevent Text, nested statements, name prefixes, and valid declared targets from becoming false positives.

This is not a full name binder, source-span implementation, or general `AE-SEED-011` parity claim. Nested calls, no-argument calls, `handle`/`forward`/`spawn` forms, module-qualified paths, and seed-native multi-file elaboration remain outside this pilot.

## Section 0 gate

| # | Check | Result |
| --- | --- | --- |
| 1 | Completeness | PASS — no explicit completeness failure markers; all 37 contextual Item 1 findings were manually reviewed as implemented resource-provenance or contract language |
| 2 | Dependency-first | PASS — ADR-110's packet ABI, target-boundary scan, and product classification existed before this added shape |
| 3 | Zero warnings/errors | PASS — release gate ran `cargo fmt --check`, workspace Clippy with `-D warnings`, and workspace tests cleanly |
| 4 | Tests exist and pass | PASS — direct, forward, host, literal, priority, origin, and identity evidence |
| 5 | Docs synchronized | PASS — contract, roadmap, profile, forge, progress, manifest, README, ADR, matrix, and report updated together |
| 6 | Security and validation | PASS — bounded canonical scans, target-boundary validation, no authority expansion, no secrets |
| 7 | Performance reasoning | PASS — no new pass or allocation beyond the existing bounded ADR-110 scan; no VM runtime-path change |
| 8 | Version and stack fidelity | PASS — Rust 2021, Aether Seed Profile, AETH v11/v12 retained |
| 9 | Full package ready | PASS — 409-file technical-preview package passed exact-integrity and independent consumer verification |
| 10 | Resource and constraint check | PASS — offline deterministic work; no dependency, network, or hardware expansion |
| 11 | Reproducibility and determinism | PASS — bootstrap = product = forge = checked-in seed at SHA-256 `726301FC243CE36DA35C4CE08A428BB783E72B3165F1CA56957B91D89E747467` |
| 12 | IP and invention hygiene | PASS — mechanism, invariants, limits, and residuals recorded |
| 13 | Multi-agent coordination | N/A — one implementation agent performed the atomic increment |
| 14 | Review packaging | PASS — inventory, commands, evidence, risks, residuals, and commit boundary are present |
| 15 | Self-audit log | PASS — release evidence, named policy approval, scanner behavior tests, root scan, and contextual-term review are recorded |

## Self-audit log

- Ran the new root-yield behavior tests against the prior seed and observed the intended red state: no direct packet and `host-classify` product origin.
- Reused ADR-110's target-boundary and declaration-header scan rather than building a second or general binder.
- Proved forward and host headers, call-shaped Text literals, and missing-world priority at the named root-yield boundary.
- Proved product packet origin is seed-native for the named canonical form.
- Regenerated the seed and passed targeted bootstrap/product/forge/check-in identity evidence.
- Passed the release gate: pack 5.0.1 integrity, format, deny-warning Clippy, workspace tests, 32 example seed≡bootstrap comparisons, four-way seed identity, package integrity, consumer execution, and tamper rejection.
- Synchronized the authorized F-NATIVE and F-REGISTRY design packages with their implemented M35j and M24i boundaries, eliminating the remaining genuine stale scanner finding.
- Applied the named human-approved scanner correction; its 43 behavior tests pass, and the Aether-root Item 1 contextual findings were manually reviewed.
- Preserved broader `AE-SEED-011` / `AE-SEED-013` cases and seed-native multi-file elaboration as explicit residuals.

### Generic mechanical scanner boundary

Under the named human-approved policy, the mandatory `agents-constitution` `gate_check.py` prunes standard Rust `target` output, `.grok`, and copied Constitution trees only when they are descendants of the selected product root; an explicitly selected root remains fully in scope. It also keeps explicit no-deferral markers as `FAIL` while surfacing contextual language as visible Item 1 manual review. Its 43 behavior tests pass. The Aether-root scan completes in 2.6 seconds with zero failures/warnings, items 6/9/11 passing, and 37 Item 1 manual-review findings. Those findings comprise 18 verifier/compile resource-provenance references and 19 historical contract/documentation references; each describes an implemented state or contract, not deferred work. No ADR-111 delivery text matches an explicit completeness failure marker.

This is not a waiver: it is the named human-approved scanner correction, independently behavior-tested and manually reviewed under `CONST-GATE-001`. The scanner continues to stop delivery on explicit unfinished-work markers.

## Exact repeat verification

```powershell
pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release
cargo test -p aether-core seed_speak_pilot_covers_lexical_structural_and_task_preflights --lib
cargo test -p aether-core --test seed_self_host barp_phase3c_classifies_type_and_unknown_weave_product_failures
```

## Known residuals and next action

Choose a broader `AE-SEED-011` or `AE-SEED-013` invariant only under a new ADR, or separately design a seed-native multi-file forge ABI. Preserve verifier-first artifacts, dual-compare proof, and the no-ambient-authority boundary.

Suggested commit: `feat: add root-yield seed-SPEAK diagnostic pilot`

---

*End of DELIVERY_REPORT-2026-08-11-BARP-ROOT-YIELD-UNKNOWN-CALL-SPEAK.md*
