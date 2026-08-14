# Delivery Report — M12 fine-grained structural edits

**Title:** Aether 0.16.0 `aether.edit/v7` statement-level edits  
**Date:** 2026-08-04  
**Summary:** Statement replace/insert/delete under weave bodies with typed paths; seed-before-write preserved; v6 rejected.

## Rule ID self-audit

| Rule ID | Status |
| --- | --- |
| CONST-DONE-001 | Pass |
| TEST-BEHAVIOR-001 | Pass (authoring tests) |
| DOC-SYNC-001 | Pass |
| SEC-INPUT-001 | Pass (path/index fail-closed) |
| ENG-WARN-001 | Pass |

## Verify

```powershell
cargo test -p aether-core --lib authoring::
cargo test -p aether-cli
cargo clippy -p aether-core -p aether-cli -- -D warnings
```
