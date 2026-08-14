# M16 resource ↔ effect validation matrix

**Status:** Implementation green (package 0.21.0)  
**Date:** 2026-08-04  
**Design:** [DESIGN-M16-RESOURCE-EFFECT.md](DESIGN-M16-RESOURCE-EFFECT.md)  
**ADR:** [ADR-020](ADR-020-m16-resource-effect.md)

## Invariants

| ID | Evidence |
| --- | --- |
| M16-INV-001 | `resource-handle` exit 7; unit + seed dual-compare |
| M16-INV-002 | Erroring weave + buffer + raise → AE-EFFECT-003 |
| M16-INV-003 | raise with live Text still AE-EFFECT-003 |
| M16-INV-004 | (access span) reject path present in validator/verifier |
| M16-INV-005 | Nursery + resource still AE-TASK-003 |
| M16-INV-006 | M4/M2 regression tests green |
| M16-INV-007 | Seed≡bootstrap `resource-handle` |
| M16-INV-008 | No resource error payload (unchanged signatures) |

## Implementation checklist

- [x] Abortive vs handle weave bans  
- [x] Handle boundary (access-only reject)  
- [x] Verifier handle path  
- [x] Example + tests  
- [x] Seed dual-compare  
- [x] DOC-SYNC 0.21  
- [x] Delivery report  

## Sign-off

| Gate | Status |
| --- | --- |
| Design + ADR-020 | **Accepted / Implemented** |
| Implementation | **Green (0.21.0)** |
| Seed dual-compare | **Green** |
