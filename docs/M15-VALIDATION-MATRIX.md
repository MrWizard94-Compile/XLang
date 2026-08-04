# M15 comptime expansion validation matrix

**Status:** Design gate — **implementation not yet green**  
**Date:** 2026-08-04  
**Design:** [DESIGN-M15-COMPTIME-EXPANSION.md](DESIGN-M15-COMPTIME-EXPANSION.md)  
**ADR:** [ADR-019](ADR-019-m15-comptime-expansion.md)  
**Prior:** [M5-VALIDATION-MATRIX.md](M5-VALIDATION-MATRIX.md)

## Invariants

| ID | Evidence required at implement |
| --- | --- |
| M15-INV-001 | M5 `examples/comptime.ae` still dual-compares and exits as today |
| M15-INV-002 | Chain operands resolve only prior comptime Whole names or literals |
| M15-INV-003 | Forward ref / runtime name → `AE-COMPTIME-001` |
| M15-INV-004 | 1,025th directive still `AE-COMPTIME-003` |
| M15-INV-005 | No file/env/host weave usable from comptime evaluation |
| M15-INV-006 | Emission remains `COMPTIME_WHOLE`; run yields expected Whole |
| M15-INV-007 | Seed≡bootstrap on M15 chain corpus |
| M15-INV-008 | Calls/control/text still rejected in comptime bind |

## Positive

| ID | Case | Expect |
| --- | --- | --- |
| P1 | Literal-only M5 program | Unchanged |
| P2 | `product`/`sum` chain (4 binds) | Expected folded Whole exit |
| P3 | Mix literal + prior name | Evaluates |
| P4 | Signed/overflow-free chain | Matches VM arithmetic |
| P5 | Seed≡bootstrap on chain example | Byte identity |
| P6 | Authoring structure/edit round-trip chain | Canonical source |

## Negative

| ID | Case | Expect |
| --- | --- | --- |
| N1 | Forward reference to later comptime name | `AE-COMPTIME-001` |
| N2 | Runtime `bind` name as comptime operand | `AE-COMPTIME-001` |
| N3 | Unknown name | `AE-COMPTIME-001` |
| N4 | Overflow through named operand | `AE-COMPTIME-002` |
| N5 | Divide-by-zero via named zero | `AE-COMPTIME-002` |
| N6 | Mutable comptime bind | `AE-COMPTIME-001` (preserve M5) |
| N7 | Nested-block comptime bind | `AE-COMPTIME-001` |
| N8 | Call / text / host form in comptime | `AE-COMPTIME-001` |
| N9 | 1,025 directives | `AE-COMPTIME-003` |

## Implementation checklist

- [ ] Bootstrap comptime env map + chain eval  
- [ ] Negatives N1–N9  
- [ ] Example `examples/comptime-chain.ae` (or extend shipped corpus)  
- [ ] Seed dual-compare  
- [ ] Authoring round-trip  
- [ ] DOC-SYNC 0.20  
- [ ] Delivery report  

## Sign-off

| Gate | Status |
| --- | --- |
| Design + ADR-019 | **Ready / Accepted (implementable)** |
| Implementation | Pending |
| Seed dual-compare | Pending |

*End of M15-VALIDATION-MATRIX.md*
