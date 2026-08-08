# RTP-001 runtime Text ASCII fast path — validation matrix

**Status:** Implemented — package **0.34.0**
**Date:** 2026-08-08
**ADR:** [ADR-040](ADR-040-runtime-text-ascii-fast-path.md)
**Contract:** [AETHER_0.34.md](AETHER_0.34.md)

| ID | Invariant or case | Evidence / expected result |
| --- | --- | --- |
| RTP-P1 | ASCII `measure` uses scalar-equivalent byte length | `RuntimeText::scalar_len` test: `aether` is 6 |
| RTP-P2 | ASCII `glyph`, `cut`, and `seek` preserve scalar indexes | Internal helper test: `aether`, `th`, and `eth` |
| RTP-P3 | Unicode remains scalar-aware | Internal helper test and observable `Aé🙂Z` primitive test |
| RTP-P4 | Joined ASCII provenance is exact | Runtime implementation combines both operands with logical AND; full behavior suite passes |
| RTP-P5 | Text crossing host/invocation/VM boundaries remains valid | Full core/CLI host, invocation, and runtime tests pass |
| RTP-N1 | Invalid UTF-8 is not made executable by the cache | Existing malformed-artifact assertion still rejects invalid UTF-8 |
| RTP-N2 | No artifact/wire-format change is smuggled in | AETH v4–v11 verifier compatibility and seed/bootstrap parity suites pass |
| RTP-N3 | No source or capability expansion | Parser/authoring/host behavior unchanged; only private VM representation changed |
| RTP-H1 | Prior text, bytes, packing, resource, effect, seed, and CLI behavior remains | `cargo test --workspace` and `cargo test --release --workspace` pass |
| RTP-PERF-1 | Exact release self-host test has repeatable local timing evidence | Three direct process samples using `tools/measure-seed-self-host.ps1`; median 10,469 ms |
| RTP-PERF-2 | Full debug self-host remains a routine gate | Final `aether-gate -Mode full -SkipPack`: 17 seed/self-host tests passed in 19.97 s |

## Required gates

- [x] Intended-behavior ASCII and Unicode tests
- [x] Malformed UTF-8 rejection retained
- [x] `cargo fmt --all -- --check`
- [x] `cargo clippy --workspace --all-targets -- -D warnings`
- [x] `cargo test --workspace`
- [x] `cargo test --release --workspace`
- [x] Exact release self-host timing distribution
- [x] Seed/product CLI parity and forge identity (recorded in delivery report)
- [x] Documentation synchronized without upgrading a runtime optimization into a language feature

*End of RTP-001-VALIDATION-MATRIX.md*
