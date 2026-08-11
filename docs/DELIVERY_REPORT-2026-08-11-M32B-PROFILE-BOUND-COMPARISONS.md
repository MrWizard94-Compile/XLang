# Delivery Report — M32b profile-bound benchmark comparisons

**Date:** 2026-08-11
**Status:** Delivered — final full gate PASS
**Scope:** ADR-105 post-0.36 local performance-evidence tooling
**Rule IDs:** `CONST-COMPLETE-001`, `CONST-DEP-001`, `CONST-DONE-001`,
`CONST-GATE-001`, `DOC-ADR-001`, `DOC-SYNC-001`, `ENG-WARN-001`,
`PERF-REASON-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`

---

## Delivered behavior

M32b extends the closed M32a benchmark surface without changing its
unprofiled report contract:

```text
aether bench <all|welcome|arena-buffer|task-loop>
  [--warmup <0..=100>]
  [--iterations <1..=1000>]
  [--profile <lowercase-ascii-id>]
  [--report <file.json>]

aether bench compare <baseline-report.json> <candidate-report.json>
  [--report <file.json>]

aether bench --list
```

An unprofiled run continues to emit `aether.benchmark-report/v1` exactly. A
profiled run emits v2: the same fixed pure workload evidence plus a canonical
non-secret profile, a safe OS/architecture/pointer-width/build-profile
fingerprint, and a SHA-256 hash of already-checked stdout. The command retains
product-seed compilation once per workload, explicit AETH verification, bounded
warmups and measurements, empty grants, and repeated observable-result equality.
Compilation remains outside the timed sample interval.

`aether bench compare` opens only the two caller-selected local report files.
It checks an opened regular file, caps each read at 256 KiB, decodes strict
UTF-8 JSON with unknown fields denied, validates schema, limits, digests,
current embedded-source identity, samples, and lower-median summaries, then
requires equal profile, environment, configuration, workload selection, source
identity, stdout identity, and exit code. It does not compile, execute, load,
grant, launch, or contact anything. Artifact SHA-256 values and versions may
differ; the comparison makes an artifact change explicit and reports signed
median deltas only after the evidence checks pass.

This is comparison methodology, not an optimization, timing threshold,
hardware/toolchain attestation, cross-machine normalization, competitive result,
or general speed claim. The profile is a human declaration and must not contain
secret or identifying data.

## Delivered package

| Artifact | Purpose |
| --- | --- |
| `apps/xlang-cli/src/bench_runner.rs` | M32b parser, canonical profile validation, v1/v2 report serialization, capped strict reader, pair compatibility rules, data-only comparison, arithmetic, and twelve focused tests |
| `apps/xlang-cli/src/main.rs` | Registers profile and comparison CLI help with the authority boundary |
| `docs/ADR-105-m32b-profile-bound-comparisons.md` | Decision, alternatives, report compatibility, input boundary, and honesty limits |
| `docs/DESIGN-M32B-PROFILE-BOUND-COMPARISONS.md` | Exact command/report protocol, validation, resource, security, and comparison semantics |
| `docs/M32B-VALIDATION-MATRIX.md` | Positive, negative, regression, and gate evidence |
| `docs/DESIGN-M32A-VERIFIED-EXECUTION-BENCHMARKS.md` | Records that v1 remains M32a scope while M32b is the separate comparison layer |
| `README.md`, `MANIFEST.md`, `docs/ARCHITECTURE.md` | User workflow, executable contract, and CLI authority-boundary synchronization |
| `docs/CORE_CLAIMS.md`, `docs/ROADMAP.md`, `docs/ROADMAP-MAINSTREAM-MATURITY.md`, `docs/PROGRESS_REPORT-FULL-PROJECT.md`, `AGENTS.md` | Claim control, maturity evidence, priority order, and project-entry synchronization |
| This report | Delivery inventory, verification evidence, residuals, and Section 0 self-audit |

No dependency, seed source/artifact, AETH version, package version, host grant,
native/JIT path, external service, or release package was added. Generated
benchmark reports stay under ignored `target/` paths and are not part of this
delivery.

## Verification evidence

| Check | Result |
| --- | --- |
| M32b focused CLI suite | PASS — 12 tests cover parser modes, v1 compatibility, v2 identity, strict reader, malformed data, pair mismatches, output alias prevention, writer boundary, and delta arithmetic |
| CLI unit suite | PASS — 39 tests in the full gate |
| Full workspace core suite | PASS — 138 core tests plus M4/M5/M6/M7 and seed/self-host suites in the full gate |
| Formatting and warnings | PASS — `cargo fmt --all -- --check`; workspace Clippy with `-D warnings` |
| Constitution pack integrity | PASS — version 5.0.1, `GOV-INT-001` |
| Example proof | PASS — all 32 top-level examples seed≡bootstrap |
| Project/VM flows | PASS — host pilot exit 48; project/module verification and build flows complete |
| Seed identity | PASS — bootstrap = product = forge = checked-in seed |
| Debug CLI end-to-end | PASS — two profiled `welcome` v2 reports compared into `aether.benchmark-comparison/v1`; matching output identity and report SHA-256 values were emitted |
| Optimized CLI smoke | PASS — `target\release\aether.exe` generated and compared v2 reports for all three fixed workloads with no grants |
| Final quality gate | PASS — `pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode full` |

Final full-gate seed SHA-256:

```text
A692BEEA7BEB9891E5D64028A8537126AF2C52929754CA89399A8783C74BC76B
```

## Security, performance, and honesty boundary

The only executable benchmark input remains the M32a closed embedded corpus.
M32b adds local report data input, not program input: a bounded reader accepts
only explicitly selected regular files and treats every field as untrusted.
Reports cannot provide source, artifact, grant, process, network, or library
authority. Strict parsing and semantic recomputation stop malformed timing data
from becoming a timing delta. An output path cannot alias either input report,
so an explicit comparison write cannot destroy its evidence source.

The profile/fingerprint pair prevents obvious comparison-context mismatches but
does not prove physical hardware, installed toolchain, scheduler state, or
thermal state. The release smoke intentionally produced different local timing
samples; those values are not retained as an improvement claim. Future scoped
performance work must preserve raw v2 reports, the comparison report hashes,
and the declared pinned context alongside all correctness and seed gates.

## Section 0 gate

| # | Check | Result |
| --- | --- | --- |
| 1 | Completeness | PASS — parser, v1/v2 report paths, strict input validation, comparison, output protection, tests, ADR, design, matrix, and contract updates are complete; changed-set scan is clean |
| 2 | Dependency-first | PASS — reuses existing product seed compiler, verifier, VM, SHA-256, and locked serde dependencies; no unresolved boundary remains |
| 3 | Zero warnings/errors | PASS — final full gate format and deny-warning workspace Clippy clean |
| 4 | Tests exist and pass | PASS — intended behavior, v1 compatibility, malformed command/report input, pair mismatch, writer boundary, workspace, seed, VM, and example suites pass |
| 5 | Docs synchronized | PASS — ADR/design/matrix, README, manifest, architecture, claims, roadmaps, progress, AGENTS, and this report agree on the bounded scope |
| 6 | Security and validation | PASS — closed workload enum; canonical profile; explicit regular, capped, strict report input; current-source/hash/statistic checks; no report execution; explicit distinct output |
| 7 | Performance reasoning | PASS — fixed measured scope, bounded sample/resource policy, declared profile methodology, and no unearned result claim |
| 8 | Version and stack fidelity | PASS — Rust 2021 / 1.88 workspace and Aether seed-product path retained; no package/AETH/language version change |
| 9 | Full package ready | PASS — atomic scoped package contains no generated reports, temporary delivery material, or overwritten input evidence |
| 10 | Resource and constraint check | PASS — three fixed workloads, 0–100 warmups, 1–1000 samples, 256 KiB inputs, and no network/process/model/grant/dependency expansion |
| 11 | Reproducibility and determinism | PASS — fixed embedded source identity, deterministic observable-result equality, raw samples, report hashes, root `Cargo.lock`, and four-way seed identity proof |
| 12 | IP and invention hygiene | PASS — mechanics and limits are documented without novelty, superiority, or broad performance assertions |
| 13 | Multi-agent coordination | N/A — one implementation agent performed this atomic increment; no delegated work occurred |
| 14 | Review packaging | PASS — inventory, verification, authority boundary, residuals, repeat commands, and commit guidance are present |
| 15 | Self-audit log | PASS — recorded below, including static scan boundary and residual risk |

### Mechanical scan boundary

The prescribed Constitution `gate_check.py` root scan reports inherited
completeness markers in frozen governance/history material and core semantic
identifiers, ignored `target/*.log` artifacts, and member Cargo manifests that
correctly share the root `Cargo.lock`; those repository-wide observations are
outside this delivery. An exact added-line scan applies the same completeness
and secret patterns only to this delivery and passes. `git diff --check` passes.
The normal full Aether gate is the authoritative workspace build, warning, test,
example, and seed-identity evidence.

## Self-audit log

- Chose the roadmap comparison-method dependency before any VM optimization; no language, artifact, native, registry, or grant scope moved.
- Preserved byte-compatible unprofiled v1 output and made v2 opt-in through a canonical, non-secret profile declaration.
- Limited report input to two explicit, regular, UTF-8 JSON files with metadata and streaming size bounds; no report data is executable.
- Recomputed report statistics and checked source/output behavior identities so an unlike workload cannot be described as faster.
- Used signed checked nanosecond deltas and string PPM to avoid numeric overflow; a zero baseline has an explicit undefined relative result.
- Added focused positive and negative tests, exercised debug and optimized CLI flows, and reran the full Constitution pack/workspace/seed gate.
- Kept generated reports ignored under `target/`, scanned the delivery scope, and documented that timing observations alone establish no performance claim.

## Exact repeat verification

```powershell
pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode full
cargo build --release -p aether-cli
.\target\release\aether.exe bench all --warmup 1 --iterations 3 --profile win11-rust-1.88-release-fixed --report .\target\m32b-baseline.json
.\target\release\aether.exe bench all --warmup 1 --iterations 3 --profile win11-rust-1.88-release-fixed --report .\target\m32b-candidate.json
.\target\release\aether.exe bench compare .\target\m32b-baseline.json .\target\m32b-candidate.json --report .\target\m32b-comparison.json
```

## Known residuals and next action

- The profile is a local declaration, not a CPU/toolchain attestation; report comparison does not normalize external machine or scheduler conditions.
- The corpus remains three fixed pure workloads and the scope remains verification, decode, and VM execution—not product compilation, native/JIT, allocator profiling, or arbitrary program timing.
- No performance threshold, historical baseline registry, trend rule, or public speed claim exists. A human-declared pinned baseline/candidate evidence set is required before considering a scoped performance result.
- BARP's remaining seed SPEAK conformance codes and seed-native multi-file work retain their documented top priority.

Suggested commit: `feat: add profile-bound benchmark comparisons`

## Linked records

- [ADR-104](ADR-104-m32a-verified-execution-benchmarks.md)
- [ADR-105](ADR-105-m32b-profile-bound-comparisons.md)
- [M32b design](DESIGN-M32B-PROFILE-BOUND-COMPARISONS.md)
- [M32b validation matrix](M32B-VALIDATION-MATRIX.md)
- [Current roadmap](ROADMAP.md)
- [Core claims](CORE_CLAIMS.md)

---

*End of DELIVERY_REPORT-2026-08-11-M32B-PROFILE-BOUND-COMPARISONS.md.*
