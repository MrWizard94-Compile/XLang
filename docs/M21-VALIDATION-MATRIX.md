# M21 foreign ABI validation matrix

**Status:** Product pilot green (package 0.31.0) after human authorize  
**Date:** 2026-08-04  
**ADR:** [ADR-025](ADR-025-m21-foreign-abi-pilot.md)

| ID | Case | Evidence |
| --- | --- | --- |
| P1 | foreign Whole→Whole with grant runs | `m21_foreign_whole_inc_requires_lib_grant_and_runs` exit 42 |
| N1 | missing `--grant-lib` | AE-FFI-003 |
| N2 | missing symbol | AE-FFI-003 |
| N3 | non-Whole foreign param | AE-FFI-001 |
| H1 | seed default compile of foreign | fails closed (seed does not parse foreign) |

## Honesty

- Native code is **not** sandboxed.  
- Seed dual-compare for foreign is **not** claimed.  

## Checklist

- [x] Human authorize phrase  
- [x] Design + ADR  
- [x] Pilot lib + bootstrap path  
- [x] Grant negatives  
- [x] DOC-SYNC 0.31  
