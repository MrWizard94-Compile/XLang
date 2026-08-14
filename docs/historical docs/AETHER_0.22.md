# Aether 0.22 Toolchain Contract (M17 offline test runner)

**Status:** Historical package contract — language surface remains **0.11** / AETH **v11**; package **0.22** adds `aether test`; current package contract is [AETHER_0.35.md](AETHER_0.35.md)
**Depends on:** seed-hosted compile, pure VM run, [ADR-021](ADR-021-m17-offline-test-runner.md)

## Purpose

Offline discovery and execution of standalone Aether test programs via the
product CLI.

## CLI

```text
aether test [path...]
```

| Input | Behavior |
| --- | --- |
| (none) | Discover `*_test.ae` under `.` |
| directory | Recursive `*_test.ae` under that root (symlinks skipped) |
| file `.ae` | Run that file as one test |

**Pass:** seed-compile + verify + pure run with **exit code 0**.  
**CLI exit:** 0 if all passed and at least one test ran; 1 otherwise.

No host grants in M17 (pure fixtures only).

## Examples

`examples/tests/zero_test.ae`, `examples/tests/arithmetic_test.ae`

## Evidence

- Design: [DESIGN-M17-OFFLINE-TEST-RUNNER.md](DESIGN-M17-OFFLINE-TEST-RUNNER.md)
- Matrix: [M17-VALIDATION-MATRIX.md](M17-VALIDATION-MATRIX.md)
- CLI module: `apps/xlang-cli/src/test_runner.rs`

*End of AETHER_0.22.md*
