# M19b nursery × resource validation matrix

**Status:** Product path green (package 0.26.0); seed≡bootstrap for mix corpus  
**Date:** 2026-08-04  
**ADR:** [ADR-028](ADR-028-m19b-nursery-resource.md)

| ID | Case | Evidence |
| --- | --- | --- |
| P1 | main owns arena + pure together; yield sum | `examples/nursery-resource.ae` exit 7; dual-compare |
| P2 | prior nursery-total / cancel unchanged | existing M7 tests |
| N1 | spawn argument Arena/Buffer/access/table | AE-TASK-003 (existing) |
| N2 | spawn callee uses resource forms | AE-TASK-003 |
| N3 | live access loan across together | AE-EFFECT-003 |
| N4 | raise with live arena (no release) | AE-EFFECT-003 (M19a) |

## Honesty

Default seed compile admits the mix because bootstrap validation gates
`compile_with_seed`. No new AETH opcodes.

## Checklist

- [x] Design + ADR-028  
- [x] Bootstrap validation  
- [x] Example + dual-compare  
- [x] Negative cases  
- [x] DOC-SYNC 0.26  
