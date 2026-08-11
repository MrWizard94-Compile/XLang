# M32a verified-execution benchmark suite — validation matrix

**Status:** Implemented — full gate PASS
**Date:** 2026-08-11
**Decision:** [ADR-104](ADR-104-m32a-verified-execution-benchmarks.md)
**Design:** [M32a verified-execution benchmarks](DESIGN-M32A-VERIFIED-EXECUTION-BENCHMARKS.md)
**Delivery evidence:** [M32a delivery report](DELIVERY_REPORT-2026-08-11-M32A-VERIFIED-EXECUTION-BENCHMARKS.md)

| ID | Invariant or case | Evidence / expected result |
| --- | --- | --- |
| M32A-P1 | `bench --list` exposes exactly the fixed workload names | Parser/unit test returns `welcome`, `arena-buffer`, and `task-loop` in declared order |
| M32A-P2 | Each named workload uses product seed compilation | Runner test invokes the default product compiler and accepts all embedded sources |
| M32A-P3 | A benchmark artifact is verified before sampling | Runner calls `verify_bytecode` after compilation and before warmup/measurement |
| M32A-P4 | `welcome` produces a complete one-sample report | Source/artifact SHA-256 fields, byte size, exit code, and min/median/max are populated |
| M32A-P5 | Aggregate selection has deterministic coverage order | `all` returns exactly the three workload reports in the documented order |
| M32A-P6 | Repeated observations preserve behavior | Every warmup and measured run must match captured stdout and exit code |
| M32A-P7 | Report output is structured and local | Explicit JSON file parses as `aether.benchmark-report/v1` with raw nanosecond samples |
| M32A-P8 | No runtime grant is installed | Runner uses `run_bytecode`, never `run_bytecode_with_grants` |
| M32A-N1 | Unknown workload fails before execution | Parser rejects it with a closed selection error |
| M32A-N2 | Missing selection or missing option value fails closed | Parser rejects incomplete command input |
| M32A-N3 | Duplicate `--warmup`, `--iterations`, or `--report` fails closed | Parser accepts each option once |
| M32A-N4 | Counts outside limits fail closed | Warmups reject values above 100; iterations reject zero and values above 1000 |
| M32A-N5 | `--list` cannot combine with selection or options | Parser rejects every trailing argument after `--list` |
| M32A-N6 | Report parent is never created implicitly | Writer rejects an explicit report path whose parent does not exist |
| M32A-N7 | Failed workload leaves no report | Command writes only after every selected workload completes successfully |
| M32A-N8 | Arbitrary source/artifact paths are unavailable | Grammar has no source/artifact argument or path flag |
| M32A-R1 | No language or bytecode semantic regression | Full workspace tests, example corpus, seed/bootstrap proof, and verifier tests pass |
| M32A-R2 | No warning debt | Format and workspace Clippy with `-D warnings` pass |
| M32A-R3 | Documentation and claims remain bounded | ADR, design, README, manifest, architecture, claims, roadmap, and progress report agree; no broad speed claim |
| M32A-R4 | Governance gate is complete | `aether-gate.ps1 -Mode full`, Constitution pack verification, scoped static scan, and delivery checklist pass |

## Required gates

- [x] Intended-behavior parser, runner, and report tests
- [x] Invalid-input and explicit-output-boundary tests
- [x] `cargo fmt --all -- --check`
- [x] `cargo clippy --workspace --all-targets -- -D warnings`
- [x] `cargo test --workspace`
- [x] `pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode full`
- [x] Constitution pack integrity through the full gate
- [x] Changed-set governance static scan and documented repository-wide inherited findings

---

*End of M32A-VALIDATION-MATRIX.md.*
