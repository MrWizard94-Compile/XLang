# M19a explicit release validation matrix

**Status:** Bootstrap implementation green (package 0.25.0); **seed dual-compare pending**  
**Date:** 2026-08-04  
**ADR:** [ADR-027](ADR-027-m19a-explicit-release.md)

| ID | Case | Evidence |
| --- | --- | --- |
| P1 | release Text then raise; handle | `compiles_verifies_and_runs_m19a_release_then_raise` exit 9 |
| N1 | release Whole | AE-RESOURCE-001 |
| N2 | double release | AE-RESOURCE-001 |
| N3 | raise without release of Text | AE-EFFECT-003 |
| P3 | seed≡bootstrap | **Pending** seed emission of RELEASE |

## Honesty

Default `aether compile` (seed) does **not** yet emit `RELEASE`. Use
`compile --bootstrap` for release programs until seed dual-compare lands.

## Checklist

- [x] Parse/format  
- [x] Validate + emit + VM (bootstrap)  
- [x] Verifier  
- [ ] Seed emission dual-compare  
- [x] Tests (bootstrap)  
- [x] DOC-SYNC 0.25 (honest seed note)  
