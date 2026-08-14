# M32b profile-bound benchmark comparisons — validation matrix

**Status:** Implemented — full gate PASS
**Date:** 2026-08-11
**Decision:** [ADR-105](../historical%20docs/ADR-105-m32b-profile-bound-comparisons.md)
**Design:** [M32b profile-bound comparisons](../historical%20docs/DESIGN-M32B-PROFILE-BOUND-COMPARISONS.md)
**Delivery evidence:** [M32b delivery report](../historical%20docs/DELIVERY_REPORT-2026-08-11-M32B-PROFILE-BOUND-COMPARISONS.md)

| ID | Invariant or case | Evidence / expected result |
| --- | --- | --- |
| M32B-P1 | Existing unprofiled benchmark output remains v1 | Runner test writes a report without `--profile` and observes no v2-only fields |
| M32B-P2 | A profiled benchmark emits a complete v2 identity | Runner test observes v2 schema, canonical profile, safe environment, and stdout SHA-256 with no stdout payload |
| M32B-P3 | Valid same-context reports compare deterministically | Unit test validates a compatible baseline/candidate pair and derives signed median deltas and report SHA-256 values |
| M32B-P4 | Artifact changes remain visible without blocking comparable behavior | Pair test accepts changed artifact SHA-256 while reporting `artifact_identity_changed: true` |
| M32B-P5 | Comparison recomputes and validates raw statistics | Synthetic report tests require min/lower-median/max to match exact samples before output |
| M32B-P6 | PPM arithmetic is total for report values | Tests cover positive, negative, truncating, and zero-baseline `null` outcomes |
| M32B-P7 | Comparison is data-only | Tests use report inputs; implementation does not invoke compiler, VM, grants, process, or network paths |
| M32B-N1 | Malformed profile and duplicate `--profile` fail before work | Parser tests reject empty, uppercase/invalid, overlong, duplicate, missing, and wrong-mode profiles |
| M32B-N2 | v1 and unknown-schema input cannot be compared | Strict reader accepts only `aether.benchmark-report/v2` |
| M32B-N3 | Malformed/oversize report input fails boundedly | Reader tests reject invalid UTF-8/JSON, unknown fields, and data beyond 256 KiB |
| M32B-N4 | Invalid semantic report fields fail closed | Tests reject invalid hashes, sizes, sample counts, stats, names/order, and environment/profile shapes |
| M32B-N5 | Context or behavior mismatch cannot become a timing delta | Pair tests reject profile, environment, policy, source, stdout SHA-256, exit-code, and selection/order differences |
| M32B-N6 | Output writes retain explicit-file discipline | Writer tests reject a missing parent and leave no comparison report before successful validation |
| M32B-R1 | No language or bytecode semantic regression | Full workspace tests, example corpus, seed/bootstrap proof, and verifier tests pass |
| M32B-R2 | No warning debt | Format and workspace Clippy with `-D warnings` pass |
| M32B-R3 | Claims remain bounded | ADR, design, README, manifest, architecture, claims, roadmaps, progress, and delivery report state methodology only—not a speed claim |
| M32B-R4 | Governance gate is complete | `aether-gate.ps1 -Mode full`, Constitution pack verification, scoped static scan, and delivery checklist pass |

## Required gates

- [x] Intended-behavior parser, v1/v2 runner, strict-reader, pair, and arithmetic tests
- [x] Invalid-input, bounded-read, and explicit-output-boundary tests
- [x] `cargo fmt --all -- --check`
- [x] `cargo clippy --workspace --all-targets -- -D warnings`
- [x] `cargo test --workspace`
- [x] `pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode full`
- [x] Constitution pack integrity through the full gate
- [x] Changed-set governance static scan and documented repository-wide inherited findings

---

*End of M32B-VALIDATION-MATRIX.md.*
