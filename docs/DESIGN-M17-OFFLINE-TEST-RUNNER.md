# M17 Design: offline `aether test` runner

**Status:** Accepted design for ADR-021 — **implemented in package 0.22.0**  
**Date:** 2026-08-04  
**Decision record:** [ADR-021](ADR-021-m17-offline-test-runner.md)  
**Validation:** [M17-VALIDATION-MATRIX.md](M17-VALIDATION-MATRIX.md)  
**Depends on:** M9 project tooling, seed-hosted compile, pure VM run  
**Portfolio:** tooling / maturity **T-TEST** (E3 M21 class)  
**Rule IDs:** `SEC-INPUT-001`, `TEST-BEHAVIOR-001`, `DOC-ADR-001`, `CONST-DEP-001`

---

## 1. Purpose and boundary

M17 adds a **local, offline test runner** so Aether programs can be checked by
the product CLI without editors, network, or a second compiler.

**In scope**

- Discover and run standalone `.ae` test programs  
- Seed-compile + verify + pure `run` (no grants by default)  
- Pass iff process completes with **exit code 0**  
- Human-readable summary; CLI exit 0/1  

**Out of scope**

- `#[test]`-style weave attributes  
- Project schema `role: test` (deferred)  
- JUnit/XML reports  
- Parallel execution, timeouts, flaky-retry  
- Host I/O grants during tests (use pure fixtures; M14 grants later ADR)  
- Network or package registry  

---

## 2. Core claim

> `aether test` discovers local test sources under caller-selected paths,
> seed-compiles each, runs the verified artifact with pure host fixtures, and
> reports pass/fail from `main`’s Whole exit code (0 = pass). Discovery never
> leaves the selected root; no ambient grants or network.

---

## 3. Design decisions

### D1 — Test program shape

Each test is a complete Aether program with total `main [] -> Whole`.  
**Pass:** compile + verify + run succeeds and `exit_code == 0`.  
**Fail:** compile/run error or nonzero exit.

### D2 — Discovery

```text
aether test [path...]
```

| Argument | Behavior |
| --- | --- |
| omitted | Discover under `.` |
| file `*.ae` | Run that file (explicit; need not end in `_test.ae`) |
| directory | Recursively collect `*_test.ae` under that root |

- Skip symlinks (files and dirs) to reduce path-escape risk.  
- After canonicalize, every discovered path must stay under the discovery root.  
- Stable sort by path string for deterministic order.

### D3 — Compile path

Product path only: **`compile_with_seed`**, then `verify_bytecode`, then
`run_bytecode` (empty grants). No silent bootstrap product path.

### D4 — Reporting

```text
ok   path/to/foo_test.ae
FAIL path/to/bar_test.ae: <message>
...
Aether 0.22.0 test: N passed; M failed
```

CLI process exit: `0` if M == 0 and N+M > 0; `1` if any fail or zero tests found.

### D5 — Package pin

**0.22.0** at ship.

### D6 — Stop conditions

- Network or registry  
- Ambient host grants without flags  
- Executing non-Aether scripts  
- Claiming coverage/parallel CI product without design  

---

## 4. Invariants

| ID | Invariant |
| --- | --- |
| M17-INV-001 | Offline only; no network |
| M17-INV-002 | Discovery confined under selected roots; symlinks skipped |
| M17-INV-003 | Seed-compile is product path |
| M17-INV-004 | Pass requires exit code 0 |
| M17-INV-005 | Empty discovery fails closed (nonzero CLI exit) |
| M17-INV-006 | Pure run (no grants) in M17 |

---

## 5. Implementation plan

1. CLI `test` command + discovery walk  
2. Example `examples/tests/*_test.ae`  
3. CLI unit tests (temp dir pass/fail/empty)  
4. DOC-SYNC 0.22 + delivery report  

---

*End of DESIGN-M17-OFFLINE-TEST-RUNNER.md*
