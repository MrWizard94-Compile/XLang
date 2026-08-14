# M15 comptime expansion validation matrix

**Status:** Implementation green (package 0.20.0)  
**Date:** 2026-08-04  
**Design:** [DESIGN-M15-COMPTIME-EXPANSION.md](DESIGN-M15-COMPTIME-EXPANSION.md)  
**ADR:** [ADR-019](ADR-019-m15-comptime-expansion.md)  
**Prior:** [M5-VALIDATION-MATRIX.md](M5-VALIDATION-MATRIX.md)

## Invariants

| ID | Evidence |
| --- | --- |
| M15-INV-001 | M5 `examples/comptime.ae` dual-compares exit 150 |
| M15-INV-002 | Chain operands resolve prior comptime names (`m15_comptime_name_chaining`) |
| M15-INV-003 | Forward ref / runtime / unknown → `AE-COMPTIME-001` |
| M15-INV-004 | 1,025th directive still `AE-COMPTIME-003` (M5 corpus) |
| M15-INV-005 | No host/env in comptime evaluator |
| M15-INV-006 | `COMPTIME_WHOLE` emission; chain exit 288 |
| M15-INV-007 | Seed≡bootstrap on chain (`seed_profile_compiler_forges_m15_*`) |
| M15-INV-008 | Calls/control still rejected in comptime bind |

## Positive

| ID | Case | Expect | Evidence |
| --- | --- | --- | --- |
| P1 | Literal-only M5 | Unchanged exit 150 | M5 tests + seed |
| P2 | 4-bind chain | exit 288 | example + tests |
| P3 | Mix literal + name | product base 2 → 30 | unit test |
| P5 | Seed≡bootstrap chain | Byte identity | seed_self_host |

## Negative

| ID | Case | Expect | Evidence |
| --- | --- | --- | --- |
| N1 | Forward reference | `AE-COMPTIME-001` | unit test |
| N2 | Runtime bind name | `AE-COMPTIME-001` | M5/M15 unit test |
| N3 | Unknown name | `AE-COMPTIME-001` | unit test |
| N4 | Overflow through name | `AE-COMPTIME-002` | unit test |

## Implementation checklist

- [x] Bootstrap comptime env map + chain eval  
- [x] Negatives N1–N4  
- [x] Example `examples/comptime-chain.ae`  
- [x] Seed dual-compare  
- [x] DOC-SYNC 0.20  
- [x] Delivery report  

## Sign-off

| Gate | Status |
| --- | --- |
| Design + ADR-019 | **Accepted / Implemented** |
| Implementation | **Green (0.20.0)** |
| Seed dual-compare | **Green** |
