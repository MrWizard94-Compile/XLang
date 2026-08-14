# M19a explicit release validation matrix

**Status:** Product path green (package 0.25.0); seed≡bootstrap for release corpus  
**Date:** 2026-08-04  
**ADR:** [ADR-027](ADR-027-m19a-explicit-release.md)

| ID | Case | Evidence |
| --- | --- | --- |
| P1 | release Text then raise; handle | `compiles_verifies_and_runs_m19a_release_then_raise` exit 9 |
| N1 | release Whole | AE-RESOURCE-001 |
| N2 | double release | AE-RESOURCE-001 |
| N3 | raise without release of Text | AE-EFFECT-003 |
| P3 | seed≡bootstrap | `seed_profile_compiler_forges_m19a_release_raise_byte_identically` |

## Honesty

Default `aether compile` (seed) emits `RELEASE` (66) for root `release <name>`.
Corpus: `examples/release-raise.ae` matches bootstrap byte-for-byte.

## Checklist

- [x] Parse/format  
- [x] Validate + emit + VM (bootstrap)  
- [x] Verifier  
- [x] Seed emission dual-compare  
- [x] Tests (bootstrap + seed)  
- [x] DOC-SYNC 0.25 (seed product path)  

