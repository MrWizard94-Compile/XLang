# ADR-105: M32b profile-bound benchmark comparisons

**Status:** Accepted — implementation in this delivery
**Date:** 2026-08-11
**Decision makers:** Human director (ongoing Aether mainstream-maturity direction); implementation agent
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `PERF-REASON-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`
**Depends on:** ADR-040, ADR-043, ADR-064, ADR-067, ADR-104

## Context

M32a provides bounded local raw measurements of the verified AETH execution
path, but its v1 reports intentionally contain no declared comparison context
or product comparison operation. The mainstream-maturity roadmap requires
pinned hardware, workload identity, and medians versus a prior Aether before a
scoped performance-improvement claim is credible. The current claim register
therefore identifies a profile-bound baseline and comparison method as the
next dependency.

An arbitrary report parser or arbitrary benchmark input would weaken M32a's
authority boundary. In particular, comparison must never execute a report,
replay embedded source, install grants, invoke a process, or accept a runtime
path beyond the caller-selected report files.

## Decision

1. Preserve `aether.benchmark-report/v1` exactly for existing `aether bench`
   calls without a profile. Add the opt-in `--profile <id>` flag to produce
   `aether.benchmark-report/v2`.
2. A v2 profile ID is an explicit, canonical, non-secret operator label. It is
   lowercase ASCII, 1–64 characters, using only letters, digits, `.`, `_`, and
   `-`, and begins and ends with a letter or digit. The operator is responsible
   for documenting the pinned host, toolchain, workload policy, and other
   comparison context behind that label; labels must not contain secrets or
   personal data.
3. Every v2 report records a safe runtime environment fingerprint: operating
   system, architecture, pointer width, and debug/release build profile. It
   intentionally does not inventory processors, memory, user names, paths,
   environment variables, toolchain executables, or network state. Each v2
   workload additionally records only the SHA-256 of its checked observable
   stdout, never the stdout itself.
4. Add the closed command
   `aether bench compare <baseline-report.json> <candidate-report.json>
   [--report <file.json>]`. It reads only the two explicitly named local report
   files and writes only an explicitly named output file after comparison
   succeeds.
5. The comparison reader accepts v2 only, caps each UTF-8 JSON input at 256 KiB,
   checks that the opened input is a regular file, rejects unknown JSON fields,
   and validates every bounded field, digest, sample count, canonical workload
   order, and derived min/median/max statistic before comparing.
6. Comparison requires exact equality of profile, environment fingerprint,
   measurement scope, warmup count, measured count, workload selection/order,
   source SHA-256, stdout SHA-256, and exit code. It may compare different
   Aether versions and artifact SHA-256 values; the output makes each artifact
   identity and whether it changed explicit.
7. The comparison report records raw input-file SHA-256 identities, versions,
   common configuration, per-workload signed median delta in nanoseconds, and
   an optional signed median delta in parts per million. PPM is `null` when the
   baseline median is zero and otherwise is a signed decimal string truncated
   toward zero, avoiding JSON integer overflow.
8. No timing threshold, speedup claim, language syntax, AETH format, verifier,
   VM behavior, guest capability, package version, native/JIT scope, telemetry,
   process, or network surface changes in M32b.

## Consequences

- Operators can create a declared local baseline and later compare it only to a
  report with exactly the same declared profile, environment, source corpus,
  observable behavior, and measurement policy.
- A future performance increment can cite raw input report digests and a
  machine-readable delta report rather than an anecdotal timing.
- `--profile` is intentionally a declaration, not a hardware or toolchain
  attestation. The safe fingerprint prevents obvious accidental mismatch but
  does not prove that a physical machine or compiler installation is unchanged.
- Existing v1 report consumers remain compatible. They cannot be compared by
  the new command because they lack the required context and stdout identity.
- The closed corpus and bounded report parser retain a small local authority
  surface. No arbitrary Aether program becomes executable through comparison.

## Alternatives considered

| Option | Benefit | Reason not selected |
| --- | --- | --- |
| Profile-bound v2 reports plus strict comparison | Supports honest local baseline deltas while retaining M32a boundaries | Requires an explicit operator profile and a new report schema |
| Compare v1 reports directly | Minimal implementation | Lacks profile, environment, and observable-output identity; unsafe basis for a performance claim |
| Auto-discover full CPU/toolchain inventory | More machine detail | Adds privacy, platform, subprocess, and stability complexity; does not itself prove a pinned setup |
| Accept arbitrary JSON and compare best-effort fields | Flexible | Lets malformed or incomplete evidence masquerade as comparable data |
| Run reports or referenced artifacts during comparison | Could independently reproduce samples | Reintroduces arbitrary execution and violates the closed M32a workload boundary |
| Publish/upload baseline data | Easier sharing | Adds telemetry/network authority outside this local-first increment |

## Links

- [M32a benchmark decision](ADR-104-m32a-verified-execution-benchmarks.md)
- [M32b design](DESIGN-M32B-PROFILE-BOUND-COMPARISONS.md)
- [M32b validation matrix](../Current%20state/M32B-VALIDATION-MATRIX.md)
- [Mainstream maturity roadmap](ROADMAP-MAINSTREAM-MATURITY.md)
- [Core claims register](../Current%20state/CORE_CLAIMS.md)

---

*End of ADR-105.*
