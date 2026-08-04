# M10 multi-unit offline projects validation matrix

**Status:** Implementation gate for Aether 0.13 / M10 — satisfied by
[DELIVERY_REPORT-2026-08-04-M10-MULTI-UNIT-PROJECTS.md](DELIVERY_REPORT-2026-08-04-M10-MULTI-UNIT-PROJECTS.md)  
**Date:** 2026-08-04  
**Design:** [DESIGN-M10-MULTI-UNIT-PROJECTS.md](DESIGN-M10-MULTI-UNIT-PROJECTS.md)  
**ADR:** [ADR-013](ADR-013-m10-multi-unit-projects.md)

## Invariants under test

| Invariant | Evidence required |
| --- | --- |
| M10-INV-001 schema + path grammar | Nested path accepted; backslash/absolute/`..` rejected; unknown fields rejected |
| M10-INV-002 roles | Zero mains, two mains rejected; one main + N libs accepted |
| M10-INV-003 / 004 path bounds | `src/main.ae` resolves under root; escape after resolve fails |
| M10-INV-005 lock | Multi-unit lock exact match; missing unit / wrong digest fails `AE-PROJECT-003` |
| M10-INV-006 verify | Each unit seed-compiles; one bad unit fails closed |
| M10-INV-007 independence | No test requires cross-unit symbol resolution; each unit is a complete program |
| M10-INV-008 format | `project format` emits canonical text per unit; `--write` only listed paths |
| M10-INV-009 output-dir | Artifacts named with `__` mapping; verified AETH bytes |
| M10-INV-010 offline | No network APIs on project path |
| M10-INV-011 codes | Stable `AE-PROJECT-*` on failures |

## Positive corpus (must pass after implementation)

| Case | Setup | Expect |
| --- | --- | --- |
| P1 Single-unit regression | `examples/project` | verify OK (M9) |
| P2 Multi-unit nested | `examples/project-multi` with `src/main.ae` + `lib/helper.ae`, full lock | verify OK; two unit reports |
| P3 Multi-unit output-dir | verify `--output-dir` | `src__main.aeth` + `lib__helper.aeth` (names per design) exist and verify |
| P4 project format stdout | format without `--write` | both units appear in declaration order, canonical |
| P5 project format write | temp copy + `--write` | files match bootstrap `format_source` |
| P6 Dual role | one main + two libs | verify OK when all sources valid |

## Negative corpus (must fail closed)

| Case | Setup | Expect code / behavior |
| --- | --- | --- |
| N1 Path escape `..` | unit `../x.ae` | `AE-PROJECT-002` at parse |
| N2 Absolute path | `/tmp/x.ae` or `C:...` | `AE-PROJECT-002` |
| N3 Backslash path | `src\\main.ae` | `AE-PROJECT-002` |
| N4 Dot segment | `src/./main.ae` | `AE-PROJECT-002` |
| N5 Two mains | two `role: main` | `AE-PROJECT-001` |
| N6 Zero mains | only libs | `AE-PROJECT-001` |
| N7 Lock mismatch one unit | wrong sha for lib | `AE-PROJECT-003` |
| N8 Lock missing unit | lock omits lib | `AE-PROJECT-003` |
| N9 Bad unit source | lib has syntax error | `AE-PROJECT-004`; main not partially “shipped” as success without lib |
| N10 Unknown field | `"depends_on": []` | parse reject (unknown field) |
| N11 Cross-unit resolve (non-goal) | main calls weave defined only in lib file | **must fail** as ordinary single-file compile of main — documents independence |

## Non-tests (explicitly out of matrix)

- Import syntax  
- Registry fetch  
- Lib unit without `main` as language feature  
- Seed self-host changes  

## Implementation checklist

- [x] Schema path pattern updated  
- [x] Rust path validation matches schema  
- [x] Core tests for multi-unit / path / lock / independence  
- [x] CLI `project format`  
- [x] Example `examples/project-multi`  
- [x] DOC-SYNC product law  
- [x] Gate script multi-unit verify  
- [x] Delivery report  

## Sign-off

| Gate | Status |
| --- | --- |
| Design + ADR | **Accepted** |
| Implementation | **Done** (0.13.0) |
| Delivery | **Done** |
