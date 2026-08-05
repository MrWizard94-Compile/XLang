# M23 pure comptime weave calls — validation matrix

**Status:** Design gate ready; implementation pending  
**Date:** 2026-08-05  
**ADR:** [ADR-039](ADR-039-m23-comptime-pure-calls.md)  
**Design:** [DESIGN-M23-COMPTIME-PURE-CALLS.md](DESIGN-M23-COMPTIME-PURE-CALLS.md)

| ID | Case | Expected |
| --- | --- | --- |
| P1 | `comptime bind x <- call double 21` pure total Whole weave | Compiles; `x=42` at run |
| P2 | Call args from prior comptime names | Exit matches folded value |
| P3 | Multi-arg pure helper (`area w h`) | Exit 512 style corpus |
| P4 | Seed≡bootstrap for M23 example + M5/M15 regressions | Byte identity |
| N1 | Call host weave | AE-COMPTIME-001 |
| N2 | Call foreign weave | AE-COMPTIME-001 |
| N3 | Call `raises Whole` weave | AE-COMPTIME-001 |
| N4 | Callee with nested `call` | AE-COMPTIME-001 |
| N5 | Callee with `choose`/`while` (M23) | AE-COMPTIME-001 |
| N6 | Runtime name as call arg | AE-COMPTIME-001 |
| N7 | Forward callee (declared after) | AE-COMPTIME-001 |
| N8 | Non-Whole param/result | AE-COMPTIME-001 |
| N9 | Arithmetic overflow in callee | AE-COMPTIME-002 |
| H1 | Prior `examples/comptime.ae` / `comptime-chain.ae` | Unchanged dual-compare |

## Checklist

- [x] Design + ADR-039  
- [ ] Bootstrap eval + diagnostics  
- [ ] Seed dual-compare  
- [ ] DOC-SYNC 0.33  
- [ ] Delivery report  

---

*End of M23-VALIDATION-MATRIX.md*
