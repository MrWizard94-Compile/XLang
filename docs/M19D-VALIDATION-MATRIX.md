# M19d multi-weave arena / resourceful spawn validation matrix

**Status:** Product target package 0.32.0  
**Date:** 2026-08-04  
**ADR:** [ADR-035](ADR-035-m19d-spawn-scoped-arena.md)  
**Design:** [DESIGN-M19D-SPAWN-SCOPED-ARENA.md](DESIGN-M19D-SPAWN-SCOPED-ARENA.md)

| ID | Case | Expected |
| --- | --- | --- |
| P1 | Total non-main weave declares arena + buffer | Compiles; runs |
| P2 | Spawn resourceful total worker; parent may own arena | Exit 7; seed≡bootstrap |
| P3 | Header capacity = sum of arenas | Match sum in artifact |
| N1 | Two arenas in one weave | Reject AE-RESOURCE / M2 |
| N2 | Erroring weave declares arena/buffer | AE-EFFECT-003 |
| N3 | Resource spawn **argument** | AE-TASK-003 |
| N4 | Capacity sum > 1_000_000 | Reject |
| H1 | Prior main-only examples unchanged | Dual-compare corpus green |

## Checklist

- [x] Design + ADR-035  
- [x] Bootstrap + verifier + VM  
- [x] Seed capacity sum + dual-compare  
- [x] DOC-SYNC 0.32  
