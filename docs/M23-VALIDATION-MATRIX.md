# M23 pure comptime weave calls — validation matrix

**Status:** Implementation green — package **0.33.0**
**Date:** 2026-08-07
**ADR:** [ADR-039](ADR-039-m23-comptime-pure-calls.md)  
**Design:** [DESIGN-M23-COMPTIME-PURE-CALLS.md](DESIGN-M23-COMPTIME-PURE-CALLS.md)

| ID | Case | Expected |
| --- | --- | --- |
| P1 | `comptime bind x <- call double 21` pure total Whole weave | Folded helper result; behavior test |
| P2 | Call args from prior comptime names | `cell -> wide` in `comptime-calls.ae` |
| P3 | Multi-arg pure helper (`area w h`) | Exit 512 corpus |
| P4 | Seed-emitted product path ≡ bootstrap for M23 + M5/M15 (raw M23; no materialization) | Byte identity |
| N1 | Call host weave | AE-COMPTIME-001 |
| N2 | Call foreign weave | AE-COMPTIME-001 |
| N3 | Call `raises Whole` weave | AE-COMPTIME-001 |
| N4 | Callee with nested `call` | AE-COMPTIME-001 |
| N5 | Callee with `choose`/`while` (M23) | AE-COMPTIME-001 |
| N6 | Runtime name as call arg | AE-COMPTIME-001 |
| N7 | Forward callee (declared after) | AE-COMPTIME-001 |
| N8 | Non-Whole param/result | AE-COMPTIME-001 |
| N9 | Arithmetic overflow in callee | AE-COMPTIME-002 |
| H1 | Prior `examples/comptime.ae` / `comptime-chain.ae` | Unchanged seed dual-compare |

## Checklist

- [x] Design + ADR-039  
- [x] Bootstrap eval + diagnostics
- [x] Seed-emitted product-path dual-compare
- [x] DOC-SYNC 0.33
- [x] Delivery report
- [x] BARP Phase 1: seed-native raw M23 (ADR-043; no materialization bridge)

---

*End of M23-VALIDATION-MATRIX.md*
