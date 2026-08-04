# M17 offline test runner validation matrix

**Status:** Implementation green (package 0.22.0)  
**Date:** 2026-08-04  
**Design:** [DESIGN-M17-OFFLINE-TEST-RUNNER.md](DESIGN-M17-OFFLINE-TEST-RUNNER.md)  
**ADR:** [ADR-021](ADR-021-m17-offline-test-runner.md)

## Invariants

| ID | Evidence |
| --- | --- |
| M17-INV-001 | Offline CLI path only |
| M17-INV-002 | Symlink skip in walk; roots via canonicalize |
| M17-INV-003 | `compile_with_seed` in `run_one_test` |
| M17-INV-004 | nonzero exit → FAIL unit test |
| M17-INV-005 | empty dir → error |
| M17-INV-006 | pure `run_bytecode` |

## Cases

| ID | Case | Evidence |
| --- | --- | --- |
| P1 | examples/tests pass | `shipped_examples_tests_directory_passes` |
| P2 | ok_test / bad_test mix | `test_runner_passes_zero_exit_and_fails_nonzero` |
| N3 | empty discovery | same unit test |

## Checklist

- [x] CLI command  
- [x] Discovery  
- [x] Examples  
- [x] Unit tests  
- [x] DOC-SYNC 0.22  
- [x] Delivery report  

## Sign-off

| Gate | Status |
| --- | --- |
| Design + ADR-021 | **Accepted / Implemented** |
| Implementation | **Green (0.22.0)** |
