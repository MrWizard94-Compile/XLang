# M32a design — verified-execution benchmarks

**Status:** Implemented design contract — full gate PASS
**Date:** 2026-08-11
**Decision:** [ADR-104](ADR-104-m32a-verified-execution-benchmarks.md)
**Scope:** post-0.36 local CLI maturity tooling; no language, AETH, or grant change

## 1. Purpose

`aether bench` gives an operator a bounded, machine-readable local measurement
of Aether's existing verified execution path. It establishes a baseline and
does not assert an optimization, comparative ranking, latency guarantee, or
cross-machine result.

The measured operation is one call to `run_bytecode`: AETH verification,
decode, and VM execution. Seed compilation occurs once before sampling and is
outside the measurement interval.

## 2. Closed command grammar

```text
aether bench <all|welcome|arena-buffer|task-loop>
  [--warmup <0..=100>]
  [--iterations <1..=1000>]
  [--report <file.json>]

aether bench --list
```

`all` uses the declared workload order below. Each flag is accepted once.
Unknown selections, duplicate flags, missing values, non-UTF-8 text options,
out-of-range counts, and trailing input fail before a workload is compiled or
run. `--list` accepts no other argument.

## 3. Workload contract

| Name | Embedded source | Behavioral coverage | Authority |
| --- | --- | --- | --- |
| `welcome` | `examples/welcome.ae` | ordinary pure Text/output path | none |
| `arena-buffer` | `examples/arena-buffer.ae` | bounded arena and Copy-element buffer path | none |
| `task-loop` | `examples/task-loop.ae` | bounded structured-task/checkpoint path | none |

The sources are compiled into the CLI with `include_str!`; a benchmark command
does not read project files, artifacts, source paths, environment values, or
network data. Their checked-in gate coverage is the boundedness basis. A new
workload requires a reviewed source, an updated ADR/design/matrix, and intended
behavior evidence before it joins this table.

## 4. Execution protocol

For every selected workload, in fixed order:

1. Product-seed compile the embedded source exactly once.
2. Explicitly run `verify_bytecode` on the produced artifact before sampling.
3. Record SHA-256 digests of the embedded UTF-8 source and artifact, plus
   artifact byte length.
4. Run the artifact for the configured warmup count with `run_bytecode`.
5. Capture the first successful run's stdout and exit code as its expected
   observable result. Every later warmup and measured run must match exactly.
6. For every measured run, take an `Instant` immediately before `run_bytecode`
   and immediately after it. Convert the elapsed duration to a checked `u64`
   count of nanoseconds.
7. Sort a copy of exact samples to calculate `min`, `median`, and `max`.
   An even-count median uses the lower of the two central values; the command's
   default iteration count is odd.

The protocol accepts any normal exit value because the workload's observable
result, rather than a universal exit-zero convention, is the integrity check.
Any compiler, verifier, VM, timing-conversion, or output-divergence failure
fails the entire command and no report is written.

## 5. Report contract

An explicit report target receives UTF-8 JSON using this schema:

```json
{
  "schema": "aether.benchmark-report/v1",
  "language": "Aether",
  "version": "0.36.0",
  "measurement_scope": "verified-aeth-run/v1: verify + decode + execute; seed compilation excluded",
  "warmup_iterations": 3,
  "iterations": 11,
  "workloads": [
    {
      "name": "welcome",
      "source_sha256": "lowercase hex SHA-256",
      "artifact_sha256": "lowercase hex SHA-256",
      "artifact_bytes": 0,
      "exit_code": 0,
      "samples_ns": [0],
      "min_ns": 0,
      "median_ns": 0,
      "max_ns": 0
    }
  ]
}
```

The example uses illustrative numeric values to describe field shape, not a
benchmark result. The report contains no timestamp, host inventory, secret,
source text, stdout, grant, or ambient-path data. Its changing samples are
intentionally raw local evidence; source and artifact digests let a later
comparison confirm the workload identity. M32b retains this v1 shape and adds a
separate opt-in v2 comparison contract with a profile, safe environment fields,
and a stdout hash; see [ADR-105](ADR-105-m32b-profile-bound-comparisons.md).

The report writer rejects a missing parent directory and writes only after all
selected workloads have completed successfully. It creates or replaces only the
explicit report file; standard output remains a human-readable summary.

## 6. Security and authority review

| Boundary | Control |
| --- | --- |
| Workload selection | Closed enum; no caller source/artifact input |
| Execution | Product compiled, explicitly verified AETH only |
| Runtime authority | `run_bytecode`, no grants or foreign libraries |
| Bounded operator work | Strict warmup and iteration limits; fixed corpus |
| File output | Optional explicit report file, existing parent required |
| Network/process/model | No API, subprocess, service, or network access |
| Data disclosure | Report has hashes/statistics only; no source or stdout payload |

This command does not introduce guest capabilities. It intentionally avoids a
generic `--source`, `--artifact`, `--timeout`, or arbitrary command hook until a
separate execution-limit and authority design can prove those boundaries.

## 7. Verification plan

- Parser tests cover legal selection/options and rejected duplicate, unknown,
  missing, malformed, and out-of-range input.
- Runner tests prove each fixed workload seed-compiles, verifies, samples, and
  emits stable source/artifact identities without timing thresholds.
- Report tests inspect the JSON schema and confirm writes require an explicit,
  existing parent path.
- Full product gate exercises workspace tests, seed/boot dual-compare corpus,
  examples, formatter, Clippy with warnings denied, and Constitution pack
  integrity. The delivery report records exact results.

## 8. Non-goals

- No general benchmark runner for arbitrary Aether programs.
- No compiler-time, native, JIT, allocator, profile-guided, or cross-machine
  performance conclusion.
- No package version, source grammar, AETH opcode/version, verifier, VM, or
  capability change.
- No release, publication, telemetry, benchmark upload, or automatic baseline
  comparison within M32a itself. M32b provides a separate strict local data-only
  comparison method under [ADR-105](ADR-105-m32b-profile-bound-comparisons.md),
  not a performance conclusion.

---

*End of DESIGN-M32A-VERIFIED-EXECUTION-BENCHMARKS.md.*
