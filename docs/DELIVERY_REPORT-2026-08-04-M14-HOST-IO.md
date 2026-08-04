# Delivery Report — M14 capability-mediated host I/O

**Title:** Aether 0.19.0 grant-backed host I/O  
**Date:** 2026-08-04  
**Summary:** Operator grants on `aether run` install read/write/env host weaves under path-jail roots. Default run remains pure M8 fixtures. Seed dual-compares host-io declaration emit; path-escape and missing-grant negatives covered.

## Verify

```powershell
cargo test -p aether-core m14_
cargo test -p aether-core --test seed_self_host seed_profile_compiler_forges_m14
cargo test -p aether-cli run_with_grant
cargo clippy -p aether-core -p aether-cli -- -D warnings
```

## Rule ID self-audit

| Rule ID | Status |
| --- | --- |
| CONST-DONE-001 | Pass (M14) |
| TEST-BEHAVIOR-001 | Pass (P*/N* matrix) |
| SEC-INPUT-001 | Pass (path jail + deny-by-default) |
| DOC-SYNC-001 | Pass (0.19 package pin) |
| DOC-ADR-001 | Pass (ADR-018 implemented) |

## Scope honesty

- No network/shell/FFI  
- No ambient guest FS without grants  
- I/O failures fail closed (`AE-HOST-003`/`004`/`005`)  
- Seed emits host declarations; run-time I/O is host session authority  

*End of delivery report.*
