# M32b design — profile-bound benchmark comparisons

**Status:** Implemented design contract
**Date:** 2026-08-11
**Decision:** [ADR-105](ADR-105-m32b-profile-bound-comparisons.md)
**Scope:** post-0.36 local performance-evidence tooling; no language, AETH, VM, or grant change

## 1. Purpose

M32b turns M32a's closed-corpus local measurements into safe comparison
evidence. It does not optimize the VM or make a performance claim. Instead, it
requires the operator to declare a non-secret profile for like-for-like runs,
records a small safe environment fingerprint, and compares only reports that
prove the same source corpus and observable behavior.

The benchmark operation remains exactly one `run_bytecode` call: verification,
decode, and VM execution. Product-seed compilation is performed once before
sampling and remains outside the timed interval.

## 2. Closed command grammar

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

`--profile` is unique and accepted only for a benchmark run. A profile ID is
1–64 bytes of lowercase ASCII letters, digits, `.`, `_`, or `-`, beginning and
ending with a letter or digit; it must not contain a secret or identifying
information. It is a human-controlled label for the pinned
host/toolchain/workload policy, not an automatic attestation.

`compare` accepts exactly two UTF-8 report paths and one optional unique output
path. It does not accept benchmark options, profiles, source paths, artifact
paths, grants, shell hooks, or a command to execute. Unknown, duplicate,
missing, non-UTF-8, or trailing arguments fail before an input is opened.

## 3. Versioned report contracts

Calls without `--profile` continue to emit the unchanged
`aether.benchmark-report/v1` contract from M32a. A profiled call emits v2:

```json
{
  "schema": "aether.benchmark-report/v2",
  "language": "Aether",
  "version": "0.36.0",
  "measurement_scope": "verified-aeth-run/v1: verify + decode + execute; seed compilation excluded",
  "profile": "win11-rust-1.88-release-fixed",
  "environment": {
    "operating_system": "windows",
    "architecture": "x86_64",
    "pointer_width": 64,
    "build_profile": "release"
  },
  "warmup_iterations": 3,
  "iterations": 11,
  "workloads": [
    {
      "name": "welcome",
      "source_sha256": "lowercase hex SHA-256",
      "artifact_sha256": "lowercase hex SHA-256",
      "artifact_bytes": 0,
      "stdout_sha256": "lowercase hex SHA-256",
      "exit_code": 0,
      "samples_ns": [0],
      "min_ns": 0,
      "median_ns": 0,
      "max_ns": 0
    }
  ]
}
```

The JSON values above describe shape only, not a measured result. The runtime
provides `environment`; no user, machine name, path, CPU model, memory size,
environment variable, source text, stdout text, grant, or timestamp is stored.
The workload's `stdout_sha256` binds comparison to the verified observed result
without recording its content.

The comparison command produces this distinct, non-executable data report:

```json
{
  "schema": "aether.benchmark-comparison/v1",
  "measurement_scope": "verified-aeth-run/v1: verify + decode + execute; seed compilation excluded",
  "profile": "win11-rust-1.88-release-fixed",
  "environment": {
    "operating_system": "windows",
    "architecture": "x86_64",
    "pointer_width": 64,
    "build_profile": "release"
  },
  "baseline_report_sha256": "lowercase hex SHA-256",
  "candidate_report_sha256": "lowercase hex SHA-256",
  "baseline_version": "0.36.0",
  "candidate_version": "0.36.0",
  "warmup_iterations": 3,
  "iterations": 11,
  "workloads": [
    {
      "name": "welcome",
      "source_sha256": "lowercase hex SHA-256",
      "stdout_sha256": "lowercase hex SHA-256",
      "exit_code": 0,
      "baseline_artifact_sha256": "lowercase hex SHA-256",
      "candidate_artifact_sha256": "lowercase hex SHA-256",
      "artifact_identity_changed": false,
      "baseline_median_ns": 0,
      "candidate_median_ns": 0,
      "median_delta_ns": 0,
      "median_delta_ppm": "0"
    }
  ]
}
```

`median_delta_ns` is candidate minus baseline. `median_delta_ppm` is the same
delta divided by the baseline median and multiplied by one million, truncated
toward zero. It is a decimal string to keep every mathematically valid result
representable in JSON; it is `null` when the baseline median is zero.

## 4. Comparison acceptance protocol

For each caller-selected input path, the CLI opens the file, proves it is a
regular file, and reads at most 262,144 bytes. It then decodes UTF-8 JSON into a
`deny_unknown_fields` v2 data structure and validates:

1. exact v2 schema, Aether language name, and measurement scope;
2. canonical non-secret profile and safe environment field shapes;
3. bounded warmup (`0..=100`) and measured (`1..=1000`) counts;
4. exactly one allowed workload name, or all three in the declared order;
5. lowercase 64-character SHA-256 fields, a source digest equal to the current
   embedded workload source, and a positive bounded artifact size;
6. exactly one bounded sample per declared iteration, representable signed
   delta inputs, and min/lower-median/max values recomputed from exact samples.

The two validated reports must then have identical profile, environment,
warmup/iteration policy, workload names/order, source SHA-256, stdout SHA-256,
and exit codes. Their Aether versions and artifact identities may differ. A
comparison output carries both artifact SHA-256 values and makes the difference
explicit, so a compiler/runtime change is never hidden.

No report source, artifact, stdout, or JSON value is executed. Comparison reads
only data from the operator-selected paths; it does not call the compiler, VM,
foreign loader, network, process launcher, or grant subsystem.

## 5. Integrity, performance, and resource rules

- The fixed M32a workload corpus and product-seed compilation protocol are
  unchanged. A comparison is meaningful only for the same reported workload
  source and behavior; a different source or output is rejected rather than
  being labelled faster.
- v2 reports preserve exact raw samples. The comparison derives its summaries
  from validated raw input instead of trusting summary fields.
- The parser's 256 KiB cap, 1,000-sample maximum, three-workload maximum, and
  fixed field limits prevent a report from causing unbounded allocation or
  arithmetic. The comparison needs only two bounded report buffers plus small
  fixed corpus metadata.
- No threshold, trend test, benchmark score, baseline registry, telemetry,
  automatic upload, or cross-machine claim is added. The profile and safe
  environment equality are necessary evidence guards, not proof that a physical
  machine or toolchain did not change.
- Existing unprofiled callers retain v1. v1 input fails closed in `compare`
  because it cannot provide profile or observed-output identity evidence.

## 6. Security and authority review

| Boundary | Control |
| --- | --- |
| Benchmark program | Closed embedded workload enum from M32a; no caller code/artifact execution |
| Comparison input | Two explicit paths only; regular-file check, 256 KiB cap, UTF-8, strict schema, semantic validation |
| Report output | Optional explicit non-input path only; parent must already exist; writes after all validation/comparison succeeds |
| Runtime authority | Benchmark uses `run_bytecode` with no grants; comparison never calls VM or compiler |
| Data disclosure | Hashes, integer samples, safe environment fingerprint, and non-secret operator profile only; no stdout/source/path/secret payload |
| Network/process/model | No API, subprocess, model, service, telemetry, or network access |
| Arithmetic | Checked signed delta conversion; unbounded PPM represented as a decimal string; zero baseline produces `null` |

## 7. Verification plan

- Parser tests accept profiled runs and comparison syntax; reject malformed,
  duplicate, non-canonical, and cross-mode flags before I/O or execution.
- Runner tests prove v1 remains unchanged and v2 carries profile, safe
  environment, and stdout SHA-256 without exposing stdout.
- Report-validation tests cover v1 rejection, unknown fields, oversize input,
  invalid digests, invalid statistics, invalid order, mismatched counts, and
  non-regular/missing files.
- Pair tests cover a valid same-profile comparison, artifact identity change,
  and rejected profile/environment/configuration/source/output/exit mismatches.
- Arithmetic tests cover signed nanosecond deltas, PPM signs/truncation, and a
  zero baseline.
- Full product gate covers workspace tests, seed/bootstrap proof corpus,
  examples, formatter, Clippy with warnings denied, and Constitution pack
  integrity. The delivery report records exact results.

## 8. Non-goals

- No VM speed optimization or resulting performance claim.
- No arbitrary Aether benchmark target, compiler-time measurement, native/JIT
  timing, profiler, hardware probe, automatic baseline storage, trend analysis,
  or telemetry.
- No full host/toolchain attestation, secret-bearing profile metadata, or
  cross-machine comparability guarantee.
- No package version, Aether source grammar, AETH opcode/version, verifier, VM,
  host grant, F-NATIVE, or F-REGISTRY change.

---

*End of DESIGN-M32B-PROFILE-BOUND-COMPARISONS.md.*
