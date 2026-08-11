# Delivery Report — M32a verified-execution benchmark suite

**Date:** 2026-08-11
**Status:** Delivered — final full gate PASS
**Scope:** ADR-104 post-0.36 local performance-evidence tooling
**Rule IDs:** `CONST-COMPLETE-001`, `CONST-DEP-001`, `CONST-DONE-001`,
`CONST-GATE-001`, `DOC-ADR-001`, `DOC-SYNC-001`, `ENG-PERF-001`,
`ENG-WARN-001`, `RND-INVAR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`

---

## Delivered behavior

`aether bench` is a closed, local command:

```text
aether bench <all|welcome|arena-buffer|task-loop>
  [--warmup <0..=100>]
  [--iterations <1..=1000>]
  [--report <file.json>]

aether bench --list
```

It embeds the exact checked-in `welcome`, `arena-buffer`, and `task-loop`
sources in the CLI. For each selected workload it product-seed-compiles once,
explicitly verifies the resulting AETH bytes, runs bounded warmups, and records
samples for the normal `verify + decode + execute` VM path. Seed compilation is
outside the sample interval.

Every warmup and measured execution must match the first observed stdout and
exit code. An optional report writes only after every selected workload has
succeeded, only to a caller-specified file whose parent directory already
exists. The JSON report is `aether.benchmark-report/v1` and records raw samples,
summary values, source SHA-256, artifact SHA-256, artifact size, and exit code.

The command accepts no caller source or artifact path and installs no grants,
foreign libraries, process, shell, network, or model authority. It makes no
cross-machine, competitive, compiler-time, all-program, or speed-improvement
claim. Package version remains **0.36.0**; source grammar, AETH format,
verifier, VM semantics, seed artifact, and capability surface are unchanged.

## Delivered package

| Artifact | Purpose |
| --- | --- |
| `apps/xlang-cli/src/bench_runner.rs` | Closed workload selection, option validation, product-seed compile, verification, measurement, report serialization, and six focused tests |
| `apps/xlang-cli/src/main.rs` | Registers `aether bench` and exposes closed usage help |
| `docs/ADR-104-m32a-verified-execution-benchmarks.md` | Decision, alternatives, authority boundary, and consequences |
| `docs/DESIGN-M32A-VERIFIED-EXECUTION-BENCHMARKS.md` | Command/report protocol, boundedness, security controls, and test plan |
| `docs/M32A-VALIDATION-MATRIX.md` | Positive, negative, regression, and gate evidence |
| `README.md`, `MANIFEST.md`, `docs/ARCHITECTURE.md` | User interface, product contract, and CLI authority synchronization |
| `docs/CORE_CLAIMS.md`, `docs/ROADMAP.md`, `docs/PROGRESS_REPORT-FULL-PROJECT.md`, `AGENTS.md` | Claim boundary, current roadmap, maturity scorecard, and project-entry synchronization |
| This report | Delivery inventory, verification evidence, risks, and Section 0 self-audit |

No dependency, seed source/artifact, AETH file, release package, or external
service was added.

## Verification evidence

| Check | Result |
| --- | --- |
| M32a parser/runner/report tests | PASS — six focused cases cover closed syntax, malformed input, one-workload evidence, aggregate ordering, report boundary, and median rule |
| CLI unit suite | PASS — 33 tests |
| Full workspace core suite | PASS — 138 core tests plus M4 (10), M5 (4), M6 (3), M7 (4), and seed/self-host (30) suites |
| Formatting and warnings | PASS — `cargo fmt --all -- --check`; workspace Clippy with `-D warnings` |
| Constitution pack integrity | PASS — version 5.0.1, `GOV-INT-001` |
| Example proof | PASS — all 32 top-level examples seed≡bootstrap |
| Project/VM flows | PASS — host pilot exits 48; project/module checks complete |
| Seed identity | PASS — bootstrap = product = forge = checked-in seed |
| Optimized CLI smoke | PASS — `target\release\aether.exe bench all --warmup 1 --iterations 3 --report ...`; valid three-workload report; temporary report removed |
| Final quality gate | PASS — `pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode full` |

Final full-gate seed SHA-256:

```text
A692BEEA7BEB9891E5D64028A8537126AF2C52929754CA89399A8783C74BC76B
```

## Security, performance, and honesty boundary

The benchmark command narrows its execution input to three compiled-in,
reviewed, bounded sources because the VM deliberately has no general CLI
fuel/timeout control. This avoids turning an ad hoc timing request into an
unbounded caller-program execution surface. `run_bytecode` preserves the normal
verify-before-run path, and the runner explicitly verifies once before it
starts sampling. Empty grants retain the pure host fixture set only.

Report output is an explicit local write. The writer neither creates a parent
directory nor emits a partial report after a workload error. It stores no source
text, stdout, secrets, host inventory, or timestamp.

M32a is measurement infrastructure, not an optimization. The release smoke
confirmed the command works across the fixed corpus, but its timing values are
not retained here as a benchmark baseline because the command did not establish
a pinned hardware/toolchain comparison protocol. M32b is the next evidence step
before any scoped performance-change claim.

## Section 0 gate

| # | Check | Result |
| --- | --- | --- |
| 1 | Completeness | PASS — command, parser, runner, report path, negative handling, tests, ADR, matrix, and contract documents are all present; changed-set scanner is clean |
| 2 | Dependency-first | PASS — uses existing seed compile, verifier, VM, SHA-256, and serde dependencies; no unresolved boundary remains |
| 3 | Zero warnings/errors | PASS — final full gate format and deny-warning Clippy clean |
| 4 | Tests exist and pass | PASS — intended behavior, malformed command input, report-write boundary, workspace, seed, VM, and example suites pass |
| 5 | Docs synchronized | PASS — ADR/design/matrix, README, manifest, architecture, claims, roadmap, progress report, and AGENTS agree on the closed scope |
| 6 | Security and validation | PASS — closed workload enum, bounded counts, strict duplicate/unknown rejection, explicit verified artifact, empty grants, and explicit report output |
| 7 | Performance reasoning | PASS — sample scope and limits are declared; raw local evidence is separated from performance assertions |
| 8 | Version and stack fidelity | PASS — Rust 2021 / 1.88 workspace and Aether seed-product path retained; no package/AETH/language version change |
| 9 | Full package ready | PASS — scoped Git package contains no generated benchmark report or temporary delivery material |
| 10 | Resource and constraint check | PASS — bounded corpus and operator counts; no network, process, model, native, grant, or dependency expansion |
| 11 | Reproducibility and determinism | PASS — fixed embedded source and artifact SHA-256 identities, deterministic observable-result equality, root `Cargo.lock`, and four-way seed identity proof |
| 12 | IP and invention hygiene | PASS — mechanism and limits are documented as a bounded local tool; no novelty, superiority, or broad performance claim made |
| 13 | Multi-agent coordination | N/A — one implementation agent performed this atomic increment; no delegated work occurred |
| 14 | Review packaging | PASS — manifest-equivalent inventory, verification, security/performance boundary, residuals, and repeat commands are included |
| 15 | Self-audit log | PASS — recorded below, including mechanical scan boundary and residual risk |

### Mechanical scan boundary

The prescribed `gate_check.py` root scan was run. It reports inherited historical
completeness terms under frozen governance/history material and core semantic
identifiers, ignored `target/*.log` artifacts, and member Cargo manifests that
correctly share the root `Cargo.lock`; none are introduced by this delivery. The
scan also caught one new explanatory word in the initial M32a design draft. It
was replaced before this final report.

An exact added-line scan applies the same completeness and secret patterns only
to this delivery's new text and passes. `git diff --check` passes. The normal
full Aether gate remains the authoritative workspace build, warning, test,
example, and seed-identity evidence.

## Self-audit log

- Derived the benchmark scope from the mainstream P5 evidence gap without
  changing the language, artifact, native, registry, or grant boundary.
- Reused checked-in bounded examples and embedded them to prevent arbitrary
  source/artifact execution through the timing command.
- Compiled once per workload with the product seed, explicitly verified AETH,
  measured the normal verify-before-run operation, and checked observations for
  behavioral divergence.
- Enforced unique options, bounded warmup/iteration counts, existing report
  parents, and no report write before whole-command success.
- Added intended-behavior and invalid-input tests; manually exercised list,
  report, and all-workload optimized flows.
- Re-ran the complete Constitution pack/workspace/full seed gate after final
  contract synchronization.
- Removed generated report material and documented that performance samples
  remain local evidence rather than a speed claim.
- Scanned root and exact delivery scope; repaired the sole new scan finding and
  preserved unrelated repository history.

## Exact repeat verification

```powershell
pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode full
cargo build --release -p aether-cli
.\target\release\aether.exe bench --list
.\target\release\aether.exe bench all --warmup 1 --iterations 3 --report .\target\aether-bench.json
```

## Known residuals and next action

- The corpus is intentionally three fixed pure workloads; it is not a generic
  benchmark runner and should remain closed until an execution-limit design
  proves a broader input boundary.
- Samples include verification, decode, and VM execution, but exclude product
  seed compilation and native lowering.
- Results are local observations with no pinned host/toolchain baseline,
  cross-machine normalization, historical comparison store, percentile policy,
  or performance acceptance threshold.
- Continue **M32b** only after recording a reviewed measurement methodology;
  BARP's remaining seed SPEAK codes and seed-native multi-file work retain their
  documented priority.

Suggested commit: `feat: add verified-execution benchmarks`

## Linked records

- [ADR-104](ADR-104-m32a-verified-execution-benchmarks.md)
- [M32a design](DESIGN-M32A-VERIFIED-EXECUTION-BENCHMARKS.md)
- [M32a validation matrix](M32A-VALIDATION-MATRIX.md)
- [Current roadmap](ROADMAP.md)
- [Core claims](CORE_CLAIMS.md)

---

*End of DELIVERY_REPORT-2026-08-11-M32A-VERIFIED-EXECUTION-BENCHMARKS.md.*
