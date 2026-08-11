# ADR-104: M32a verified-execution benchmark suite

**Status:** Accepted and implemented — full gate PASS
**Date:** 2026-08-11
**Decision makers:** Human director (ongoing Aether mainstream-maturity direction); implementation agent
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `ENG-PERF-001`,
`RND-INVAR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`
**Depends on:** ADR-040, ADR-043, ADR-064, ADR-067

## Context

The mainstream-maturity roadmap identifies performance and deployment evidence
as a weak pillar. Aether has a scoped local release timing harness for seed
self-hosting, but it does not give an operator a stable, product-facing way to
measure verified AETH execution across representative bounded programs.

The VM correctly verifies artifacts before it runs them. It deliberately does
not expose a general execution-fuel limit through the CLI. A command that
accepted arbitrary caller source or arbitrary artifact input as a benchmark
workload could therefore turn a local timing request into an unbounded program
execution. That input surface is inappropriate for the first benchmark suite.

## Decision

1. Add `aether bench` as an offline, local-only command with a closed workload
   allow-list: `welcome`, `arena-buffer`, and `task-loop`, plus the explicit
   aggregate selection `all`.
2. Embed the exact checked-in Aether sources in the CLI binary. The command
   does not read workload source or artifacts from operator paths and never
   accepts a caller-supplied program as a benchmark target.
3. Compile each selected source exactly once with the default product seed
   compiler, explicitly verify the resulting AETH bytes, then measure repeated
   `verify + decode + VM execute` calls. Compilation time is excluded.
4. Execute with no grants, foreign libraries, shell, network, process, or
   model authority. Each measured result must equal the first observed stdout
   and exit code for its workload; any divergence fails the command.
5. Bound command-controlled work: warmups are `0..=100`; measured iterations
   are `1..=1000`; options are unique and unknown arguments fail closed.
6. Permit JSON output only through an explicit `--report <file.json>` path.
   The parent directory must already exist. Reports record source/artifact
   SHA-256 identities, artifact size, exact samples, and summary statistics.
7. Do not change Aether source syntax, AETH versions, verifier acceptance,
   VM semantics, host grants, package version, or native backend scope. This is
   a post-0.36 tooling maturity increment, not a language release.

## Consequences

- Operators gain reproducible local evidence for a declared set of bounded,
  pure workloads and can retain raw timing samples for later comparison.
- The command measures the actual safety boundary (`verify + decode + execute`)
  instead of making a verification-bypassing speed claim.
- Results remain local observations. They are not cross-machine comparisons,
  competitive claims, service-level objectives, compiler-time measurements, or
  proof that all Aether programs have the same performance profile.
- Adding a workload requires an ADR/design update, boundedness review,
  intended-behavior test, and documentation synchronization. An arbitrary
  benchmark-input flag remains outside this decision.

## Alternatives considered

| Option | Benefit | Reason not selected |
| --- | --- | --- |
| Closed embedded corpus | Bounded execution, fixed source identity, no path authority | Covers a deliberately small workload set |
| Caller-supplied source or artifact | Flexible ad hoc testing | No general fuel/timeout boundary; creates unbounded execution input |
| Test-only PowerShell harness | Minimal Rust/CLI change | Does not provide a stable operator interface or machine-readable product report |
| Native/JIT benchmark path | Could isolate lower-level throughput | Expands F-NATIVE/performance scope before a verified VM baseline exists |
| Static documentation only | No implementation risk | Produces no raw, repeatable measurement evidence |

## Links

- [M32a design](DESIGN-M32A-VERIFIED-EXECUTION-BENCHMARKS.md)
- [M32a validation matrix](M32A-VALIDATION-MATRIX.md)
- [Mainstream maturity roadmap](ROADMAP-MAINSTREAM-MATURITY.md)
- [RTP-001 timing precedent](ADR-040-runtime-text-ascii-fast-path.md)
- [Seed product-path authority](ADR-067-barp-product-seed-rebuild.md)

---

*End of ADR-104.*
